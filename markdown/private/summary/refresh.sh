#!/bin/bash
# Auto-generated.

set -eu

THIS_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "${THIS_DIR}/.."

TMPFILE="$(mktemp)"

bazel query 'kind(md_file, ...)' | LC_ALL=C sort >"${TMPFILE}"

OUTFILE='.markdown_summary/contents.bzl'

cat >"${OUTFILE}" <<EOF
"""Auto-generated bzl file."""

visibility("private")

_CONTENTS = [
EOF

while read -r line; do
    echo "    \"${line}\"," >>"${OUTFILE}"
done <"${TMPFILE}"

cat >>"${OUTFILE}" <<EOF
]

def contents():
    return _CONTENTS
EOF

chmod u=rw,go= "${OUTFILE}"

rm "${TMPFILE}"
