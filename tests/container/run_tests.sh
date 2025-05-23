#!/bin/bash

set -eu

THIS_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "${THIS_DIR}"

RUN_TESTS="${HOME}/rules_markdown/markdown/private/git/run_tests"

"${RUN_TESTS}" "${HOME}/rules_markdown" 't'
"${RUN_TESTS}" "${HOME}/other-workspace-unversioned" 't'
"${RUN_TESTS}" "${HOME}/other-workspace-versioned" 't'

# We have to build first so that MODULE.bazel.lock doesn't change during the
# test run
(
    cd "${HOME}/new-workspace"
    bazel build ...:all
)
"${RUN_TESTS}" "${HOME}/new-workspace" 't'
