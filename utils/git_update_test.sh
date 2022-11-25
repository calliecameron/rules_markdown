#!/bin/bash

set -eux

function usage() {
    echo "Usage: $(basename "${0}") git_update_script bindump docdump pdfdump zipdump gitattributes gitconfig gitignore precommit"
    exit 1
}

test -z "${1:-}" && usage
SCRIPT="${1}"
test -z "${2:-}" && usage
BINDUMP="${2}"
test -z "${3:-}" && usage
DOCDUMP="${3}"
test -z "${4:-}" && usage
PDFDUMP="${4}"
test -z "${5:-}" && usage
ZIPDUMP="${5}"
test -z "${6:-}" && usage
GITATTRIBUTES="${6}"
test -z "${7:-}" && usage
GITCONFIG="${7}"
test -z "${8:-}" && usage
GITIGNORE="${8}"
test -z "${9:-}" && usage
PRECOMMIT="${9}"

(
    cd "${TEST_TMPDIR}"
    mkdir a
    cd a
    git init .
)

BUILD_WORKSPACE_DIRECTORY="${TEST_TMPDIR}" "${SCRIPT}" \
    "${BINDUMP}" \
    "${DOCDUMP}" \
    "${PDFDUMP}" \
    "${ZIPDUMP}" \
    "${GITATTRIBUTES}" \
    "${GITCONFIG}" \
    "${GITIGNORE}" \
    "${PRECOMMIT}" \
    a

diff "${BINDUMP}" "${TEST_TMPDIR}/a/.bin/bindump"
[ "$(stat -c '%a' "${TEST_TMPDIR}/a/.bin/bindump")" = '700' ]
diff "${DOCDUMP}" "${TEST_TMPDIR}/a/.bin/docdump"
[ "$(stat -c '%a' "${TEST_TMPDIR}/a/.bin/docdump")" = '700' ]
diff "${PDFDUMP}" "${TEST_TMPDIR}/a/.bin/pdfdump"
[ "$(stat -c '%a' "${TEST_TMPDIR}/a/.bin/pdfdump")" = '700' ]
diff "${ZIPDUMP}" "${TEST_TMPDIR}/a/.bin/zipdump"
[ "$(stat -c '%a' "${TEST_TMPDIR}/a/.bin/zipdump")" = '700' ]
diff "${GITATTRIBUTES}" "${TEST_TMPDIR}/a/.gitattributes"
[ "$(stat -c '%a' "${TEST_TMPDIR}/a/.gitattributes")" = '600' ]
diff "${GITCONFIG}" "${TEST_TMPDIR}/a/.gitconfig"
[ "$(stat -c '%a' "${TEST_TMPDIR}/a/.gitconfig")" = '600' ]
diff "${GITIGNORE}" "${TEST_TMPDIR}/a/.gitignore"
[ "$(stat -c '%a' "${TEST_TMPDIR}/a/.gitignore")" = '600' ]
diff "${PRECOMMIT}" "${TEST_TMPDIR}/a/.git/hooks/pre-commit"
[ "$(stat -c '%a' "${TEST_TMPDIR}/a/.git/hooks/pre-commit")" = '700' ]
grep 'path = ../.gitconfig' "${TEST_TMPDIR}/a/.git/config" >/dev/null
