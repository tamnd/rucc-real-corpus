#!/bin/sh
# The house style, checked.
#
# Three rules, all of them mechanical:
#
#   1. No em dash and no en dash. Write the sentence with a comma, a full stop or a pair of
#      brackets. This is not a typographic preference. Everything written here gets read by
#      somebody deciding whether a project belongs on the list or whether an exclusion is
#      honest, and the plainest possible prose is the point.
#   2. No horizontal rules. A document that needs a page break needs a heading.
#   3. No non-breaking spaces. They arrive by accident from a paste, they are invisible in a
#      diff, and they break a grep six months later.
#
# The patterns are built with printf rather than written literally, so that this file stays
# plain ASCII and cannot fail its own check.
#
# It runs over tracked files only, so a scratch note in the working tree is nobody's business.

set -eu

status=0

em_dash=$(printf '\342\200\224')
en_dash=$(printf '\342\200\223')
nbsp=$(printf '\302\240')

check() {
    pattern=$1
    complaint=$2
    matches=$(git ls-files -z '*.md' '*.rs' '*.toml' '*.yml' '*.sh' |
        xargs -0 grep -n -E "$pattern" 2>/dev/null || true)
    if [ -n "$matches" ]; then
        echo "$complaint"
        echo "$matches"
        echo
        status=1
    fi
}

check "$em_dash|$en_dash" 'a typographic dash, which the house style does not use:'
check '^(-{3,}|\*{3,}|_{3,})[[:space:]]*$' 'a horizontal rule, which the house style does not use:'
check "$nbsp" 'a non-breaking space, which is almost always an accident:'

if [ "$status" -eq 0 ]; then
    echo "prose style is clean"
fi

exit "$status"
