"""Git repo macros."""

load("//markdown/private/utils:defs.bzl", "extend_file", "required_files")

visibility("//markdown/private")

def md_git_repo(
        name = None,
        extra_gitignore_lines = None,
        extra_precommit = None,
        precommit_build_all = False):  # buildifier: disable=unused-variable
    """Git repo setup.

    Args:
        name: unused
        extra_gitignore_lines: extra lines to add to the generated gitignore
        extra_precommit: an extra script to run at precommit
        precommit_build_all: when true, build all targets in precommit; when
            false, only build dependencies of tests
    """
    native.sh_binary(
        name = "git_test_extra",
        srcs = [Label("//markdown/private/git:git_test_extra.sh")],
        data = native.glob([".git/config"], allow_empty = True),
        visibility = ["//visibility:private"],
    )

    extend_file(
        name = "gitattributes",
        src = Label("//markdown/private/git:default_gitattributes"),
        prepend_lines = ["# Auto-generated; do not edit."],
    )

    extend_file(
        name = "gitconfig",
        src = Label("//markdown/private/git:default_gitconfig"),
        prepend_lines = ["# Auto-generated; do not edit."],
    )

    extend_file(
        name = "gitignore",
        src = Label("//markdown/private/git:default_gitignore"),
        prepend_lines = ["# Auto-generated; edit extra_gitignore_lines in md_git_repo."],
        append_lines = extra_gitignore_lines,
    )

    native.genrule(
        name = "precommit",
        srcs = [Label("//markdown/private/git:precommit_template.sh")],
        outs = ["precommit.sh"],
        cmd = "sed 's/@@@@@/%s/g' <$< >$@" % ("t" if precommit_build_all else ""),
    )

    copy = [
        (
            ":gitattributes",
            ".gitattributes",
            "600",
        ),
        (
            ":gitconfig",
            ".gitconfig",
            "600",
        ),
        (
            ":gitignore",
            ".gitignore",
            "600",
        ),
        (
            Label("//markdown/private/git:run_tests.sh"),
            ".git/hooks/markdown_run_tests",
            "700",
        ),
        (
            ":precommit",
            ".git/hooks/pre-commit",
            "700",
        ),
    ]
    if extra_precommit:
        copy.append(
            (
                extra_precommit,
                ".git/hooks/markdown_extra_precommit",
                "700",
            ),
        )
    required_files(
        name = "git",
        copy = copy,
        extra_check = ":git_test_extra",
        extra_update = Label("//markdown/private/git:git_update_extra"),
    )
