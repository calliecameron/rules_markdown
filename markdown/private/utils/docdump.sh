#!/bin/bash

set -eu

function usage() {
    echo "Usage: $(basename "${0}") catdoc file"
    exit 1
}

test -z "${1:-}" && usage
CATDOC="${1}"
test -z "${2:-}" && usage
FILE="${2}"

if [[ "${FILE}" != /* ]]; then
    FILE="${BUILD_WORKING_DIRECTORY}/${FILE}"
fi

printf 'File hash: %s\n' "$(md5sum <"${FILE}")"
"${CATDOC}" "${FILE}"
