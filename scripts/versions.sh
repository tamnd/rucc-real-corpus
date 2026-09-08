#!/bin/sh
# Every path dependency in the workspace has to ask for the version the workspace is on.
#
# A path requirement carries a version as well as a path, and cargo checks the version even
# though it resolves through the path. Leaving one behind on an older patch works for as long as
# the requirement is still compatible and then stops working on the release commit, which is the
# worst commit to find out on, because nothing in that commit is what broke it.
#
# So it is checked here, on every pull request, where the fix is one line and belongs to whoever
# wrote the change rather than to whoever cut the release.

set -eu

want=$(grep -m1 '^version = ' Cargo.toml | sed 's/.*"\(.*\)".*/\1/')
if [ -z "$want" ]; then
    echo "no version in the workspace package section of Cargo.toml"
    exit 1
fi

status=0
while IFS= read -r line; do
    name=${line%% *}
    got=$(echo "$line" | sed 's/.*version = "\([^"]*\)".*/\1/')
    if [ "$got" != "$want" ]; then
        echo "$name asks for $got and the workspace is on $want"
        status=1
    fi
done <<EOF
$(grep -E '^[a-z0-9-]+ = \{ path = ' Cargo.toml)
EOF

if [ "$status" -eq 0 ]; then
    echo "every path requirement asks for $want"
fi

exit "$status"
