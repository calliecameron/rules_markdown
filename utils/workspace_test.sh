#!/bin/bash

set -eu

function usage() {
    echo "Usage: $(basename "${0}") workspace_status workspace_contents_build workspace_contents_bzl workspace_summary workspace_publications bazelversion bazelrc local_workspace_status local_workspace_contents_build local_workspace_contents_bzl local_workspace_summary local_workspace_publications local_bazelversion local_bazelrc registry"
    exit 1
}

test -z "${1:-}" && usage
WORKSPACE_STATUS="${1}"
test -z "${2:-}" && usage
WORKSPACE_CONTENTS_BUILD="${2}"
test -z "${3:-}" && usage
WORKSPACE_CONTENTS_BZL="${3}"
test -z "${4:-}" && usage
WORKSPACE_SUMMARY="${4}"
test -z "${5:-}" && usage
WORKSPACE_PUBLICATIONS="${5}"
test -z "${6:-}" && usage
BAZELVERSION="${6}"
test -z "${7:-}" && usage
BAZELRC="${7}"
test -z "${8:-}" && usage
LOCAL_WORKSPACE_STATUS="${8}"
test -z "${9:-}" && usage
LOCAL_WORKSPACE_CONTENTS_BUILD="${9}"
test -z "${10:-}" && usage
LOCAL_WORKSPACE_CONTENTS_BZL="${10}"
test -z "${11:-}" && usage
LOCAL_WORKSPACE_SUMMARY="${11}"
test -z "${12:-}" && usage
LOCAL_WORKSPACE_PUBLICATIONS="${12}"
test -z "${13:-}" && usage
LOCAL_BAZELVERSION="${13}"
test -z "${14:-}" && usage
LOCAL_BAZELRC="${14}"
test -z "${15:-}" && usage
REGISTRY="${15}"

DIFF=''

function diff_file() {
    local MODE
    MODE="$(stat -L -c '%a' "${2}")"
    echo "Diffing $(basename "${2}")"
    if ! diff "${1}" "${2}"; then
        DIFF='t'
    elif [ "${MODE}" != "${3}" ]; then
        echo "Modes differ: want ${3}, got ${MODE}"
        DIFF='t'
    else
        echo 'OK'
    fi
    echo
}

function diff_mode_only() {
    local MODE
    MODE="$(stat -L -c '%a' "${2}")"
    echo "Diffing $(basename "${2}")"
    if [ "${MODE}" != "${3}" ]; then
        echo "Modes differ: want ${3}, got ${MODE}"
        DIFF='t'
    else
        echo 'OK'
    fi
    echo
}

TMP_BAZELRC="${TEST_TMPDIR}/tmp_bazelrc"
sed "s|@@@@@|${REGISTRY}|g" <"${BAZELRC}" >"${TMP_BAZELRC}"

diff_file "${WORKSPACE_STATUS}" "${LOCAL_WORKSPACE_STATUS}" '700'
diff_file "${WORKSPACE_CONTENTS_BUILD}" "${LOCAL_WORKSPACE_CONTENTS_BUILD}" '600'
diff_mode_only "${WORKSPACE_CONTENTS_BZL}" "${LOCAL_WORKSPACE_CONTENTS_BZL}" '600'
diff_file "${WORKSPACE_SUMMARY}" "${LOCAL_WORKSPACE_SUMMARY}" '700'
diff_file "${WORKSPACE_PUBLICATIONS}" "${LOCAL_WORKSPACE_PUBLICATIONS}" '700'
diff_file "${BAZELVERSION}" "${LOCAL_BAZELVERSION}" '600'
diff_file "${TMP_BAZELRC}" "${LOCAL_BAZELRC}" '600'

if [ -n "${DIFF}" ]; then
    echo "Found diff; 'bazel run :workspace_update' to fix"
    exit 1
fi

echo 'All OK'
exit 0
