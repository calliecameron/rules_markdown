#!/bin/bash

set -eu

THIS_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

export GIT_AUTHOR_NAME='test'
export GIT_COMMITTER_NAME='test'
export GIT_AUTHOR_EMAIL='test@example.com'
export GIT_COMMITTER_EMAIL='test@example.com'

# Main repo
cd "${THIS_DIR}/rules_markdown"
git -c init.defaultBranch=main init .
git add --all
git commit --quiet -m 'Test'
bazel run :git_update
bazel clean

# Unversioned copy
cd "${THIS_DIR}"
cp -a rules_markdown/tests/other_workspace other-workspace-unversioned
rm other-workspace-unversioned/.gitignore
sed -i 's|path = "../.."|path = "../rules_markdown"|g' other-workspace-unversioned/MODULE.bazel
sed -i 's|versioned_test|unversioned_test|g' other-workspace-unversioned/tests/test6/BUILD

# Versioned copy
cd "${THIS_DIR}"
cp -a rules_markdown/tests/other_workspace other-workspace-versioned
rm other-workspace-versioned/.gitignore
sed -i 's|path = "../.."|path = "../rules_markdown"|g' other-workspace-versioned/MODULE.bazel
cd other-workspace-versioned
buildozer "new_load @markdown//:defs.bzl md_git_repo" "//:all"
printf '\nmd_git_repo()\n' >>BUILD
git -c init.defaultBranch=main init .
bazel run :git_update
bazel clean
git add --all
git commit --no-verify --quiet -m 'Test'

rm -rf ~/.cache/bazel
