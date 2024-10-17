load("@markdown//:defs.bzl", "md_git_repo", "md_workspace")
load("@rules_python//python:pip.bzl", "compile_pip_requirements")
load("//markdown/private/utils:defs.bzl", "required_files")

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
        "/.mypy_cache/",
        "/.vscode/",
        "/rust-project.json",
    ],
    extra_precommit = "//tests:extra_precommit.sh",
)

exports_files(
    [
        "pyproject.toml",
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
