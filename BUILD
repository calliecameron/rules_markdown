load("@markdown//:defs.bzl", "md_git_repo", "md_workspace")
load("@rules_python//python:pip.bzl", "compile_pip_requirements")
load("//markdown/private/utils:defs.bzl", "required_files")
load("//tools/shell:defs.bzl", "sh_test")

compile_pip_requirements(
    name = "requirements",
    requirements_in = "requirements.txt",
    requirements_txt = "requirements_lock.txt",
    tags = ["requires-network"],
)

md_workspace(
    extra_bazelrc_lines = [
        "build --deleted_packages=tests/other_workspace,tests/other_workspace/.markdown_summary,tests/other_workspace/tests/test1,tests/other_workspace/tests/test2,tests/other_workspace/tests/test3,tests/other_workspace/tests/test3/.markdown_summary,tests/other_workspace/tests/test4,tests/other_workspace/tests/test5,tests/other_workspace/tests/test6",
        "query --deleted_packages=tests/other_workspace,tests/other_workspace/.markdown_summary,tests/other_workspace/tests/test1,tests/other_workspace/tests/test2,tests/other_workspace/tests/test3,tests/other_workspace/tests/test3/.markdown_summary,tests/other_workspace/tests/test4,tests/other_workspace/tests/test5,tests/other_workspace/tests/test6",
        "build --aspects=@rules_rust//rust:defs.bzl%rust_clippy_aspect",
        "build --output_groups=+clippy_checks",
        "build --aspects=@rules_rust//rust:defs.bzl%rustfmt_aspect",
        "build --output_groups=+rustfmt_checks",
    ],
)

md_git_repo(
    extra_gitignore_lines = [
        "/.env",
        "/.env.leave",
        "/.vscode/",
        "/target/",
    ],
    extra_precommit = "//tests:extra_precommit.sh",
    precommit_build_all = True,
)

exports_files(
    [
        "pyproject.toml",
        ".luacheckrc",
        ".lua-format",
        ".shellcheckrc",
    ],
    visibility = ["//:__subpackages__"],
)

exports_files(
    [
        "MODULE.bazel",
    ],
    visibility = ["//readme:__pkg__"],
)

required_files(
    name = "readme",
    copy = [
        (
            "//readme:readme_generated",
            "README.md",
            "600",
        ),
    ],
)

sh_test(
    name = "python_version_test",
    srcs = ["python_version_test.sh"],
    args = [
        "$(rootpath MODULE.bazel)",
        "$(rootpath .python-version)",
        "$(rootpath pyproject.toml)",
        "$(rootpath //tools/python:defs.bzl)",
    ],
    data = [
        ".python-version",
        "MODULE.bazel",
        "pyproject.toml",
        "//tools/python:defs.bzl",
    ],
)
