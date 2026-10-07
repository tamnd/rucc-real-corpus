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
# disagree about what can be known either. There is one exception. gcc removes a fortify check
# when the range of the length proves that the copy fits, and rucc does not do this yet
# (tamnd/rucc#3355). So when gcc says that a program has only unprotected functions and rucc
# says that it has protected ones, the rucc build has more checks and not fewer. This is
# counted as the same, and the summary line says how many files it was.

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
    : > "$result"
    if ! "$rrc" --rucc "$out/bin/$name" build "$project" --level O2 > "$out/$name/$project.log" 2>&1; then
        echo "build failed" > "$result"
        return
    fi
    tree=$(awk '$1 == "tree" { print $2 }' "$out/$name/$project.log")
    linked "$tree" | while IFS= read -r file; do
        "$check" "$tree/$file" 2>&1 | sed -n 's/^ \(.*\): \(.*\)$/\1: \2/p' | sed "s|^|$file: |" >> "$result"
    done
}

# The rucc result, with the one fortify difference of the header put back to what gcc said.
accepted() {
    awk -F': ' 'NR == FNR { gcc[$1 FS $2] = $0; next }
        $2 == "Fortify Source functions" && $3 ~ /^yes/ && gcc[$1 FS $2] ~ /: no, only unprotected functions found!$/ { print gcc[$1 FS $2]; next }
        { print }' "$out/gcc/$1.txt" "$out/rucc/$1.txt"
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
        if [ "$more" -gt 0 ]; then
            echo "$project: the same for $files files, and in $more of them rucc keeps a fortify check that gcc removed"
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
