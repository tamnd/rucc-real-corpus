#!/bin/sh
# Build each project with the flags that a distribution passes, once with rucc and once with gcc,
# and run hardening-check on each program and library that the two builds made. Then say where
# the two results are not the same.
#
#   scripts/hardening.sh RUCC GCC [PROJECT...]
#
# With no project names it does all of projects/. It is run from the top of the repository.
#
# The flags are what dpkg-buildflags gives on Ubuntu 24.04 with hardening=+all, which is the
# longest of the distribution lines. The file prefix map is not there, because its path is the
# path of the machine that made the line. rrc puts the level after these flags, so a manifest
# that wants a level still gets it.
#
# HARDENING_CHECK names the hardening-check script from devscripts, and defaults to the one on
# PATH. RRC names the harness, and defaults to target/release/rrc. OUT names the directory for
# the results, and defaults to runs/hardening.
#
# A program has the same result from both compilers when each line of hardening-check is the
# same. A line that says "unknown" counts as a result, because the two compilers should not
# disagree about what can be known either. There are three exceptions, and in each of them the rucc
# build keeps every check that the gcc build has.
#
# The first is the fortify line. hardening-check reads the names of the libc functions that the
# file calls. The answer changes when one compiler writes a copy or a fill with a known length as
# moves and the other calls memcpy or memset for it (tamnd/rucc#3360), or when gcc removes a check
# because it can prove that the copy fits (tamnd/rucc#3355). So the script also writes the set of
# _chk functions that each file calls. When each _chk function of the gcc build is also in the
# rucc build, the fortify line counts as the same. gcc also writes some strcpy and strcat calls as
# strlen and memcpy, so it calls __memcpy_chk where rucc calls __strcpy_chk. So the fortify line
# also counts as the same when rucc says a plain "yes" and gcc says "yes" with more words. A plain
# "yes" means that each call to a function that has a _chk form goes to the _chk form.
#
# The second is the stack protector line, when rucc says "yes" and gcc says "no". gcc decides which
# functions get a canary after it removes the locals that it does not need, and rucc decides it
# before (tamnd/rucc#3361). So rucc puts a canary in a few functions where gcc puts none.
#
# The third is the stack clash line, when rucc says "yes" and gcc says "unknown". hardening-check
# looks for one order of instructions: the compare, the jump out, the step and the touch. gcc
# sometimes moves the loop of a variable length array out of line, and then writes the step and
# the touch before the compare. The loop is there, but the script does not find it. rucc writes the
# loop in the order that the script reads.
#
# The summary line says how many lines of each project were counted as the same in this way.

set -eu

if [ "$#" -lt 2 ]; then
    echo "usage: scripts/hardening.sh RUCC GCC [PROJECT...]"
    exit 2
fi
rucc=$(command -v "$1")
gcc=$(command -v "$2")
shift 2

check=${HARDENING_CHECK:-hardening-check}
rrc=${RRC:-target/release/rrc}
out=${OUT:-runs/hardening}

flags="-g -O2 -fno-omit-frame-pointer -mno-omit-leaf-frame-pointer -flto=auto -ffat-lto-objects"
flags="$flags -fstack-protector-strong -fstack-clash-protection -Wformat -Werror=format-security"
flags="$flags -fcf-protection -Wdate-time -D_FORTIFY_SOURCE=3"
flags="$flags -Wl,-Bsymbolic-functions -Wl,-z,relro -Wl,-z,now"

mkdir -p "$out/bin" "$out/rucc" "$out/gcc"
for name in rucc gcc; do
    eval "real=\$$name"
    printf '#!/bin/sh\nexec %s %s "$@"\n' "$real" "$flags" > "$out/bin/$name"
    chmod +x "$out/bin/$name"
done

if [ "$#" -eq 0 ]; then
    set -- $(ls projects)
fi

# Each ELF file in the tree that is a program or a shared library, by its path in the tree.
# Objects and archives are left out, because hardening-check reads a linked file.
linked() {
    find "$1" -type f ! -name '*.o' ! -name '*.a' | sort | while IFS= read -r file; do
        case $(readelf -h "$file" 2>/dev/null | awk '$1 == "Type:" { print $2 }') in
            EXEC | DYN) echo "${file#"$1"/}" ;;
        esac
    done
}

# Builds the project with one compiler, and writes one line for each check of each file.
build() {
    project=$1
    name=$2
    result="$out/$name/$project.txt"
    checks="$out/$name/$project.chk"
    : > "$result"
    : > "$checks"
    if ! "$rrc" --rucc "$out/bin/$name" build "$project" --level O2 > "$out/$name/$project.log" 2>&1; then
        echo "build failed" > "$result"
        return
    fi
    tree=$(awk '$1 == "tree" { print $2 }' "$out/$name/$project.log")
    linked "$tree" | while IFS= read -r file; do
        "$check" "$tree/$file" 2>&1 | sed -n 's/^ \(.*\): \(.*\)$/\1: \2/p' | sed "s|^|$file: |" >> "$result"
        called=$(readelf -sW "$tree/$file" | awk '$7 == "UND" { sub(/@.*/, "", $8); print $8 }' | grep -E '^__.+_chk$' | sort -u | tr '\n' ' ')
        echo "$file: $called" >> "$checks"
    done
}

# The rucc result, with each line that keeps every check of the gcc build put back to what gcc
# said. See the top of this file.
accepted() {
    awk -F': ' -v text="$out/gcc/$1.txt" -v ours="$out/gcc/$1.chk" -v theirs="$out/rucc/$1.chk" '
        function kept(want, have,   n, i, w, got) {
            n = split(have, w, " ")
            for (i = 1; i <= n; i++) got[w[i]] = 1
            n = split(want, w, " ")
            for (i = 1; i <= n; i++) if (!(w[i] in got)) return 0
            return 1
        }
        FILENAME == text { gcc[$1 FS $2] = $0; answer[$1 FS $2] = $3; next }
        FILENAME == ours { want[$1] = $2; next }
        FILENAME == theirs { have[$1] = $2; next }
        $2 == "Fortify Source functions" && ($1 FS $2) in gcc && ($1 in want) && kept(want[$1], have[$1]) { print gcc[$1 FS $2]; next }
        $2 == "Fortify Source functions" && $3 == "yes" && answer[$1 FS $2] ~ /^yes / { print gcc[$1 FS $2]; next }
        $2 == "Stack protected" && $3 == "yes" && answer[$1 FS $2] == "no, not found!" { print gcc[$1 FS $2]; next }
        $2 == "Stack clash protection" && $3 == "yes" && answer[$1 FS $2] ~ /^unknown/ { print gcc[$1 FS $2]; next }
        { print }' "$out/gcc/$1.txt" "$out/gcc/$1.chk" "$out/rucc/$1.chk" "$out/rucc/$1.txt"
}

same=0
differ=0
failed=0
for project in "$@"; do
    build "$project" rucc
    build "$project" gcc
    if grep -q "^build failed" "$out/rucc/$project.txt" "$out/gcc/$project.txt"; then
        echo "$project: the build failed with $(grep -l "^build failed" "$out/rucc/$project.txt" "$out/gcc/$project.txt" | xargs -n1 dirname | xargs -n1 basename | paste -sd' ' -)"
        failed=$((failed + 1))
        continue
    fi
    files=$(cut -d: -f1 "$out/rucc/$project.txt" | sort -u | wc -l | tr -d ' ')
    accepted "$project" > "$out/rucc/$project.accepted"
    if diff "$out/gcc/$project.txt" "$out/rucc/$project.accepted" > "$out/$project.diff"; then
        more=$(diff "$out/rucc/$project.txt" "$out/rucc/$project.accepted" | grep -c '^<' || true)
        if [ "$more" -eq 1 ]; then
            echo "$project: the same for $files files, with 1 line where rucc keeps each check of the gcc build"
        elif [ "$more" -gt 1 ]; then
            echo "$project: the same for $files files, with $more lines where rucc keeps each check of the gcc build"
        else
            echo "$project: the same for $files files"
        fi
        rm -f "$out/$project.diff"
        same=$((same + 1))
    else
        echo "$project: not the same, see $out/$project.diff"
        differ=$((differ + 1))
    fi
done

echo "$same the same, $differ not the same, $failed with a build that failed"
[ "$differ" -eq 0 ]
