#!/bin/bash

set -eu

function usage() {
    echo "Usage: $(basename "${0}") bazel_version"
    exit 1
}

test -z "${1:-}" && usage
BAZEL_VERSION="${1}"

git config --global user.name 'test'
git config --global user.email 'test@example.com'
git config --global init.defaultBranch 'main'

function new-package() {
    mkdir "${TEST_DIR}/newpackage"
    cd "${TEST_DIR}/newpackage"
    bazel run //:new
    printf "\nTest text\n" >>'newpackage.md'
    echo 'newpackage' >'newpackage.dic'
    buildozer 'new_load @rules_markdown//markdown/testing:defs.bzl output_test' '//newpackage:all'
    buildozer 'new_load @rules_markdown//markdown/testing:defs.bzl versioned_test' '//newpackage:all'
    cat <<EOF >>BUILD

output_test(
    reproducible = False,
    target = "newpackage",
)

versioned_test(
    target = "newpackage",
)
EOF
    if grep md_git_repo BUILD >/dev/null; then
        bazel run :git_update
    fi
    "${TEST_DIR}/.markdown_summary/refresh"
}

# Main repo
TEST_DIR="${HOME}/rules_markdown"
cd "${TEST_DIR}"
git init .
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
new-package
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
git init .
bazel run :git_update
new-package
bazel clean
git add --all
git commit --no-verify --quiet -m 'Test'

# New workspace
TEST_DIR="${HOME}/new-workspace"
mkdir "${TEST_DIR}"
cp "${HOME}/rules_markdown/markdown/private/workspace/default_bazeliskrc" "${TEST_DIR}/.bazeliskrc"
cp "${HOME}/rules_markdown/markdown/private/workspace/default_bazelrc" "${TEST_DIR}/.bazelrc"
touch "${TEST_DIR}/WORKSPACE"
sed "s/@@@@@/${BAZEL_VERSION}/g" <"${HOME}/rules_markdown/readme/module_template.bazel" |
    tr '\n' '\t' |
    perl -pe 's|archive_override\([^\)]*?\)|local_path_override(\t    module_name = "rules_markdown",\t    path = "../rules_markdown",\t)|g' |
    tr '\t' '\n' >"${TEST_DIR}/MODULE.bazel"
cp "${HOME}/rules_markdown/readme/root.build" "${TEST_DIR}/BUILD"
cd "${TEST_DIR}"
git init .
bazel run :workspace_update
bazel run :contents_update
bazel run :git_update
new-package
bazel clean
git add --all
git commit --no-verify --quiet -m 'Test'

rm -rf "${HOME}/.cache/bazel"
