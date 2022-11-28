#!/bin/bash

set -eu

function usage() {
    echo "Usage: $(basename "${0}") workspace_status workspace_summary bazelversion"
    exit 1
}

test -z "${1:-}" && usage
WORKSPACE_STATUS="${1}"
test -z "${2:-}" && usage
WORKSPACE_SUMMARY="${2}"
test -z "${3:-}" && usage
BAZELVERSION="${3}"

SOURCE_DIR="${BUILD_WORKSPACE_DIRECTORY}"
BIN_DIR="${SOURCE_DIR}/.bin"

mkdir -p "${BIN_DIR}"
cp "${WORKSPACE_STATUS}" "${BIN_DIR}/workspace_status"
chmod u=rwx,go= "${BIN_DIR}/workspace_status"
cp "${WORKSPACE_SUMMARY}" "${SOURCE_DIR}/workspace_summary"
chmod u=rwx,go= "${SOURCE_DIR}/workspace_summary"
cp "${BAZELVERSION}" "${SOURCE_DIR}/.bazelversion"
chmod u=rw,go= "${SOURCE_DIR}/.bazelversion"