#!/bin/bash

set -eu

function usage() {
    echo "Usage: $(basename "${0}") shellcheck dirs..."
    exit 1
}

test -z "${1:-}" && usage
SHELLCHECK="${1}"

find -L "${@:2}" -type f -print0 | xargs -0 "${SHELLCHECK}" --norc
