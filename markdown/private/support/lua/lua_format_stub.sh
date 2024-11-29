#!/bin/bash

set -eu

function usage() {
    echo "Usage: $(basename "${0}") lua_format src args..."
    exit 1
}

test -z "${1:-}" && usage
LUA_FORMAT="${1}"
test -z "${2:-}" && usage
SRC="${2}"

OUTPUT="${TEST_TMPDIR}/output.txt"

"${LUA_FORMAT}" "${SRC}" "${@:3}" >"${OUTPUT}"

if ! diff "${SRC}" "${OUTPUT}"; then
    exit 1
fi

exit 0
