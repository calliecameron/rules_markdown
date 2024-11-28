#!/bin/bash

set -eu

THIS_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "${THIS_DIR}"

./rules_markdown/markdown/private/git/run_tests rules_markdown
./rules_markdown/markdown/private/git/run_tests other-workspace-unversioned
./rules_markdown/markdown/private/git/run_tests other-workspace-versioned
