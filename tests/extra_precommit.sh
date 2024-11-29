#!/bin/bash

set -eu

if ! grep "$(date '+%Y')" 'LICENSE' &>/dev/null; then
    echo 'Error: update LICENSE to contain the current year'
    exit 1
fi

MODULE_VERSION="$(tr '\n' '\t' <MODULE.bazel |
    grep -o -E 'module\([^\)]+version = "([0-9]|\.)+"[^\)]+\)' |
    tr '\t' '\n' |
    grep 'version' |
    grep -o '".*"' |
    sed 's/"//g')"
USED_VERSION="$(tr '\n' '\t' <tests/other_workspace/MODULE.bazel |
    grep -o -E 'bazel_dep\([^\)]+name = "rules_markdown"[^\)]+version = "([0-9]|\.)+"[^\)]+\)' |
    tr '\t' '\n' |
    grep 'version' |
    grep -o '".*"' |
    sed 's/"//g')"

if [ "${USED_VERSION}" != "${MODULE_VERSION}" ]; then
    echo "Error: rules_markdown version used in tests/other_workspace/MODULE.bazel doesn't match main MODULE.bazel"
    exit 1
fi

exit 0
