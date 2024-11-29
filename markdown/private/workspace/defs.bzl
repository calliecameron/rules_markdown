"""Workspace macros."""

load("@buildifier_prebuilt//:rules.bzl", "buildifier_test")
load("//markdown/private/summary:defs.bzl", "md_summary")
load("//markdown/private/utils:defs.bzl", "extend_file", "required_files")

visibility("//markdown/private")

def md_workspace(name = None, extra_bazelrc_lines = None):  # buildifier: disable=unused-variable
    """Workspace setup.

    Args:
        name: unused
        extra_bazelrc_lines: extra lines to add to the generated bazelrc
    """

    if native.package_name():
        fail("md_workspace may only be used in the workspace root")

    buildifier_test(
        name = "buildifier_test",
        no_sandbox = True,
        workspace = "//:WORKSPACE",
    )

    native.sh_binary(
        name = "new",
        srcs = ["@rules_markdown//markdown/private/workspace:new_package.sh"],
        visibility = ["//visibility:private"],
    )

    extend_file(
        name = "bazeliskrc",
        src = "@rules_markdown//markdown/private/workspace:default_bazeliskrc",
        prepend_lines = ["# Auto-generated; do not edit."],
    )

    extend_file(
        name = "bazelrc",
        src = "@rules_markdown//markdown/private/workspace:default_bazelrc",
        prepend_lines = ["# Auto-generated; edit extra_bazelrc_lines in md_workspace."],
        append_lines = extra_bazelrc_lines,
    )

    required_files(
        name = "workspace",
        copy = [
            (
                "@rules_markdown//markdown/private/workspace:workspace_status",
                ".markdown_workspace/workspace_status",
                "700",
            ),
            (
                "@rules_markdown//markdown/private/utils:git_repo_version",
                ".markdown_workspace/git_repo_version",
                "700",
            ),
            (
                ":bazeliskrc",
                ".bazeliskrc",
                "600",
            ),
            (
                ":bazelrc",
                ".bazelrc",
                "600",
            ),
            # We need both markdownlint configs to ensure both vscode and the CLI in subfolders find
            # the config.
            (
                "@rules_markdown//markdown/private/core/lint:default_markdownlintrc",
                ".markdownlintrc",
                "600",
            ),
            (
                "@rules_markdown//markdown/private/core/lint:default_markdownlintrc",
                ".markdownlint.json",
                "600",
            ),
        ],
    )

    md_summary()
