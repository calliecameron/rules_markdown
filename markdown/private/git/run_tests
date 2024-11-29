#!/bin/bash

set -eu

function usage() {
    echo "Usage $(basename "${0}") root_dir build_all"
    exit 1
}

test -z "${1:-}" && usage
ROOT_DIR="$(readlink -f "${1}")"
BUILD_ALL="${2:-}"

TMP_DIR="$(mktemp -d)"

function cleanup() {
    # shellcheck disable=SC2317
    rm -rf "${TMP_DIR}"
}

trap cleanup EXIT

REPOS_TMP="${TMP_DIR}/repos-tmp"
REPOS="${TMP_DIR}/repos"
TEST_ROOTS_TMP="${TMP_DIR}/test-roots-tmp"
TEST_ROOTS="${TMP_DIR}/test-roots"
SUMMARIES="${TMP_DIR}/summaries"
BEFORE_STATE_DIR="${TMP_DIR}/before"
AFTER_STATE_DIR="${TMP_DIR}/after"
FAKE_XDG_OPEN="${TMP_DIR}/fake-xdg-open"

mkdir -p "${BEFORE_STATE_DIR}" "${AFTER_STATE_DIR}" "${FAKE_XDG_OPEN}"

cat >"${FAKE_XDG_OPEN}/xdg-open" <<EOF
#!/bin/bash

true
EOF
chmod u+x "${FAKE_XDG_OPEN}/xdg-open"

function find-repos() {
    find "${ROOT_DIR}" -type d -name '.git' -prune -print |
        while read -r line; do
            dirname "$(readlink -f "${line}")"
        done >"${REPOS_TMP}"

    if [ ! -d "${ROOT_DIR}/.git" ] && (cd "${ROOT_DIR}" && git rev-parse --git-dir &>/dev/null); then
        echo "${ROOT_DIR}" >>"${REPOS_TMP}"
    fi

    LC_ALL=C sort <"${REPOS_TMP}" | uniq >"${REPOS}"
}

function find-test-roots() {
    find "${ROOT_DIR}" -type f -regex '.*/WORKSPACE\(\.bazel\)?' -print |
        while read -r line; do
            dirname "$(readlink -f "${line}")"
        done >"${TEST_ROOTS_TMP}"

    echo "${ROOT_DIR}" >>"${TEST_ROOTS_TMP}"

    LC_ALL=C sort <"${TEST_ROOTS_TMP}" | uniq >"${TEST_ROOTS}"
}

function find-summaries() {
    find "${ROOT_DIR}" -type d -name '.markdown_summary' |
        while read -r line; do
            readlink -f "${line}"
        done |
        LC_ALL=C sort |
        uniq >"${SUMMARIES}"
}

function snapshot-repo() {
    local REPO="${1}"
    local OUT_FILE="${2}"

    (
        cd "${REPO}"
        git diff >"${OUT_FILE}"

        git status --porcelain |
            grep '??' |
            sed 's|?? *||g' |
            LC_ALL=C sort |
            while read -r line; do
                {
                    echo "UNTRACKED"
                    echo "${line}"
                    cat "$(git rev-parse --show-toplevel)/${line}"
                } >>"${OUT_FILE}"
            done
    )
}

function repo-state-file() {
    echo "${1}" | sed 's|_|__|g' | sed 's|/|_SLASH_|g'
}

function snapshot-repos() {
    local STATE_DIR="${1}"
    while read -r line; do
        snapshot-repo "${line}" "${STATE_DIR}/$(repo-state-file "${line}")"
    done <"${REPOS}"
}

function refresh-summaries() {
    while read -r line; do
        echo -e "\e[34mRefreshing summary in ${line}\e[0m"
        "${line}/refresh"
    done <"${SUMMARIES}"
}

function run-tests() {
    local TEST_ROOT="${1}"
    (
        cd "${TEST_ROOT}"
        if [ -n "${BUILD_ALL}" ]; then
            echo -e "\e[34mBuilding in ${line}\e[0m"
            bazel build ...:all
        fi
        echo -e "\e[34mRunning tests in ${line}\e[0m"
        bazel test --build_tests_only //:buildifier_test ...:all
    )
}

function run-all-tests() {
    while read -r line; do
        run-tests "${line}"
    done <"${TEST_ROOTS}"
}

function run-summaries() {
    while read -r line; do
        echo -e "\e[34mTesting summary scripts in ${line}\e[0m"
        "${line}/../summarise_contents" >/dev/null
        PATH="${FAKE_XDG_OPEN}:${PATH}" "${line}/../summarise_publications"
    done <"${SUMMARIES}"
}

find-repos
find-test-roots
find-summaries

snapshot-repos "${BEFORE_STATE_DIR}"

refresh-summaries
run-all-tests
run-summaries

snapshot-repos "${AFTER_STATE_DIR}"

if ! diff -r "${BEFORE_STATE_DIR}" "${AFTER_STATE_DIR}" >/dev/null; then
    echo
    echo -e '\e[31;1mERROR:\e[0m files changed during the test run'
    exit 1
fi

exit 0
