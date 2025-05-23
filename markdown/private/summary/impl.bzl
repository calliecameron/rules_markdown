"""Summary implementation macros."""

load("@rules_markdown//markdown/private/utils:defs.bzl", "required_files")

visibility("public")

def md_summary_impl(contents, md_group):
    required_files(
        name = "contents",
        copy = [
            (
                Label("//markdown/private/summary:contents.build"),
                "BUILD",
                "600",
            ),
            (
                Label("//markdown/private/summary:refresh.sh"),
                "refresh",
                "700",
            ),
        ],
        create = [
            (
                Label("//markdown/private/summary:contents.bzl"),
                "contents.bzl",
                "600",
            ),
        ],
    )

    md_group(
        name = "contents",
        deps = contents,
    )
