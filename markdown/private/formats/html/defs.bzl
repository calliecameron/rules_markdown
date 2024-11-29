"""Rules for html outputs."""

load(
    "//markdown/private/formats:lib.bzl",
    "filters",
    "simple_pandoc_output_impl",
    "simple_pandoc_output_rule",
)

visibility("//markdown/private/formats")

def _md_html_impl(ctx):
    return simple_pandoc_output_impl(
        ctx = ctx,
        extension = "html",
        variant = None,
        to_format = "html",
        inputs = [
            filters.add_title.file(ctx),
            filters.cleanup_metadata.file(ctx),
            filters.remove_collection_separators.file(ctx),
        ],
        args = [
            "--standalone",
            filters.add_title.arg(ctx),
            filters.cleanup_metadata.arg(ctx),
            filters.remove_collection_separators.arg(ctx),
        ],
        env = {},
        file = ctx.attr.file,
    )

md_html = simple_pandoc_output_rule(
    impl = _md_html_impl,
    extension = "html",
    variant = None,
    filters = [
        filters.add_title,
        filters.cleanup_metadata,
        filters.remove_collection_separators,
    ],
)
