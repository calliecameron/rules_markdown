#!/bin/bash

set -eu

PACKAGE="$(realpath "--relative-to=${BUILD_WORKSPACE_DIRECTORY}" "${BUILD_WORKING_DIRECTORY}")"

if [ "${PACKAGE}" = '.' ]; then
    echo "ERROR: Can't use this script in the workspace root; set up the BUILD file manually." >&2
    exit 1
fi

SOURCE_DIR="${BUILD_WORKSPACE_DIRECTORY}/${PACKAGE}"

function is-git-repo() {
    local DIR
    DIR="$(readlink -f "${1}")"

    while true; do
        if [ -d "${DIR}/.git" ]; then
            echo 't'
            return 0
        elif [ "${DIR}" = '/' ]; then
            echo
            return 0
        else
            DIR="$(readlink -f "${DIR}/..")"
        fi
    done
}

if [ -f "${SOURCE_DIR}/BUILD" ]; then
    echo 'ERROR: BUILD file already exists' >&2
    exit 1
fi

BASENAME="$(basename "${PACKAGE}")"
MD_FILE="${SOURCE_DIR}/${BASENAME}.md"
GIT_NAME="$(git config user.name || true)"

if [ ! -f "${MD_FILE}" ]; then
    echo '---' >"${MD_FILE}"
    echo "title: ${BASENAME}" >>"${MD_FILE}"

    if [ -n "${GIT_NAME}" ]; then
        echo "author: ${GIT_NAME}" >>"${MD_FILE}"
    fi
    echo 'finished: false' >>"${MD_FILE}"
    echo '---' >>"${MD_FILE}"
fi

IS_GIT_REPO="$(is-git-repo "${SOURCE_DIR}")"

if [ -z "${IS_GIT_REPO}" ]; then
    (
        cd "${SOURCE_DIR}"
        git init .
    )
fi

if [ -d "${SOURCE_DIR}/.git" ]; then
    cat >"${SOURCE_DIR}/BUILD" <<EOF
load("@markdown//:defs.bzl", "md_document", "md_git_repo")

md_git_repo()

md_document(
    name = "${BASENAME}",
)
EOF
else
    cat >"${SOURCE_DIR}/BUILD" <<EOF
load("@markdown//:defs.bzl", "md_document")

md_document(
    name = "${BASENAME}",
)
EOF
fi
