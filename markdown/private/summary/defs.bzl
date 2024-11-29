"""Summary macros."""

load("@bazel_skylib//lib:subpackages.bzl", "subpackages")
load("//markdown/private/utils:defs.bzl", "required_files")

visibility(["//markdown/private", "//markdown/private/workspace"])

def md_summary(name = None):  # buildifier: disable=unused-variable
    """Summaries of subpackage contents.

    Args:
        name: unused
    """
    subpackage_name = ".markdown_summary"
    has_subpackage = subpackages.exists(subpackage_name)

    copy = [
        (
            "@rules_markdown//markdown/private/summary:summarise_contents",
            "summarise_contents",
            "700",
        ),
        (
            "@rules_markdown//markdown/private/summary:summarise_publications",
            "summarise_publications",
            "700",
        ),
    ]
    if not has_subpackage:
        copy += [
            (
                "@rules_markdown//markdown/private/summary:contents.build",
                subpackage_name + "/BUILD",
                "600",
            ),
            (
                "@rules_markdown//markdown/private/summary:refresh",
                subpackage_name + "/refresh",
                "700",
            ),
        ]

    create = []
    if not has_subpackage:
        create.append(
            (
                "@rules_markdown//markdown/private/summary:contents.bzl",
                subpackage_name + "/contents.bzl",
                "600",
            ),
        )

    required_files(
        name = "contents",
        copy = copy,
        create = create,
    )
