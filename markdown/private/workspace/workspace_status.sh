#!/bin/bash
# This catches the corner case, not handled by module extensions, where a parent
# of the workspace is a git repo.

set -eu

function usage() {
    echo "Usage: $(basename "${0}") git_repo_version_script"
    exit 1
}

test -z "${1:-}" && usage
GIT_REPO_VERSION_SCRIPT="$(readlink -f "${1}")"

function find-git-repo() {
    local DIR
    DIR="$(readlink -f "${1}")"

    while true; do
        if [ -d "${DIR}/.git" ]; then
            echo "${DIR}"
            return 0
        elif [ "${DIR}" = '/' ]; then
            echo
            return 0
        else
            DIR="$(readlink -f "${DIR}/..")"
        fi
    done
}

GIT_REPO="$(find-git-repo ..)"

if [ -n "${GIT_REPO}" ]; then
    echo "STABLE_WORKSPACE_PARENT_REPO ${GIT_REPO}"
    echo "STABLE_WORKSPACE_PARENT_VERSION $("${GIT_REPO_VERSION_SCRIPT}" "${GIT_REPO}")"
fi

exit 0
