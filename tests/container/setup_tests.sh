#!/bin/bash

set -eu

export GIT_AUTHOR_NAME='test'
export GIT_COMMITTER_NAME='test'
export GIT_AUTHOR_EMAIL='test@example.com'
export GIT_COMMITTER_EMAIL='test@example.com'

# Main repo
TEST_DIR="${HOME}/rules_markdown"
cd "${TEST_DIR}"
git -c init.defaultBranch=main init .
git add --all
git commit --quiet -m 'Test'
bazel run :git_update
bazel clean

# Unversioned copy
TEST_DIR="${HOME}/other-workspace-unversioned"
cp -a "${HOME}/rules_markdown/tests/other_workspace" "${TEST_DIR}"
rm "${TEST_DIR}/.gitignore"
sed -i 's|path = "../.."|path = "../rules_markdown"|g' "${TEST_DIR}/MODULE.bazel"
sed -i 's|versioned_test|unversioned_test|g' "${TEST_DIR}/tests/test6/BUILD"
mkdir "${TEST_DIR}/new"
cd "${TEST_DIR}/new"
bazel run //:new
printf "\nTest text\n" >>'new.md'
buildozer 'new_load @rules_markdown//markdown/testing:defs.bzl output_test' '//new:all'
buildozer 'new_load @rules_markdown//markdown/testing:defs.bzl versioned_test' '//new:all'
cat <<EOF >>BUILD

output_test(
    reproducible = False,
    target = "new",
)

versioned_test(
    target = "new",
)
EOF
bazel run :git_update
"${TEST_DIR}/.markdown_summary/refresh"
bazel clean
git add --all
git commit --no-verify --quiet -m 'Test'

# Versioned copy
TEST_DIR="${HOME}/other-workspace-versioned"
cp -a "${HOME}/rules_markdown/tests/other_workspace" "${TEST_DIR}"
rm "${TEST_DIR}/.gitignore"
sed -i 's|path = "../.."|path = "../rules_markdown"|g' "${TEST_DIR}/MODULE.bazel"
cd "${TEST_DIR}"
buildozer 'new_load @markdown//:defs.bzl md_git_repo' '//:all'
printf '\nmd_git_repo()\n' >>BUILD
git -c init.defaultBranch=main init .
bazel run :git_update
mkdir "${TEST_DIR}/new"
cd "${TEST_DIR}/new"
bazel run //:new
printf "\nTest text\n" >>'new.md'
buildozer 'new_load @rules_markdown//markdown/testing:defs.bzl output_test' '//new:all'
buildozer 'new_load @rules_markdown//markdown/testing:defs.bzl versioned_test' '//new:all'
cat <<EOF >>BUILD

output_test(
    reproducible = False,
    target = "new",
)

versioned_test(
    target = "new",
)
EOF
"${TEST_DIR}/.markdown_summary/refresh"
bazel clean
git add --all
git commit --no-verify --quiet -m 'Test'

rm -rf "${HOME}/.cache/bazel"
