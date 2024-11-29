#!/bin/bash

set -eu

THIS_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
RUN_TESTS="${THIS_DIR}/markdown_run_tests"
EXTRA_PRECOMMIT="${THIS_DIR}/markdown_extra_precommit"

# Allow subprocesses to handle any git repo, identifying this one by working
# directory, not by env vars.
# shellcheck disable=SC2046
unset $(git rev-parse --local-env-vars)

if [ ! -x "${RUN_TESTS}" ]; then
    echo "Can't find test runner; 'bazel run :git_update' to fix"
    exit 1
fi

"${RUN_TESTS}" . '@@@@@'

if [ -x "${EXTRA_PRECOMMIT}" ]; then
    "${EXTRA_PRECOMMIT}"
fi

exit 0
