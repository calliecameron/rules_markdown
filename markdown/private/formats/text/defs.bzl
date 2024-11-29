"""Rules for text outputs."""

load(
    "//markdown/private/formats:lib.bzl",
    "filters",
    "simple_pandoc_output_impl",
    "simple_pandoc_output_rule",
)
load(
    "//markdown/private/formats:types.bzl",
    "filter",
)

visibility("//markdown/private/formats")

_md_plain_filter = filter(
    attr = {
        "_plain_markdown_filter": attr.label(
            allow_single_file = True,
            default = "//markdown/private/formats/text:plain_markdown_filter.lua",
        ),
    },
    file = lambda ctx: ctx.file._plain_markdown_filter,
    arg = lambda ctx: "--lua-filter=" + ctx.file._plain_markdown_filter.path,
)

_md_tumblr_filter = filter(
    attr = {
        "_tumblr_markdown_filter": attr.label(
            allow_single_file = True,
            default = "//markdown/private/formats/text:tumblr_markdown_filter.lua",
        ),
    },
    file = lambda ctx: ctx.file._tumblr_markdown_filter,
    arg = lambda ctx: "--lua-filter=" + ctx.file._tumblr_markdown_filter.path,
)

def _md_output_impl(ctx, variant, extra_args, extra_filters):
    return simple_pandoc_output_impl(
        ctx = ctx,
        extension = "md",
        variant = variant,
        to_format = "markdown-smart",
        inputs = [
            filters.cleanup_metadata.file(ctx),
            filters.remove_paragraph_annotations.file(ctx),
            filters.remove_collection_separators.file(ctx),
        ] + [f.file(ctx) for f in extra_filters],
        args = extra_args + [
            filters.cleanup_metadata.arg(ctx),
            filters.remove_paragraph_annotations.arg(ctx),
            filters.remove_collection_separators.arg(ctx),
        ] + [f.arg(ctx) for f in extra_filters],
        env = {},
        file = ctx.attr.file,
    )

def _md_output_rule(impl, variant, extra_filters):
    return simple_pandoc_output_rule(
        impl = impl,
        extension = "md",
        variant = variant,
        filters = [
            filters.cleanup_metadata,
            filters.remove_paragraph_annotations,
            filters.remove_collection_separators,
        ] + extra_filters,
    )

def _md_md_impl(ctx):
    return _md_output_impl(
        ctx = ctx,
        variant = None,
        extra_args = ["--standalone", "--wrap=none"],
        extra_filters = [],
    )

md_md = _md_output_rule(
    impl = _md_md_impl,
    variant = None,
    extra_filters = [],
)

def _md_plain_md_impl(ctx):
    return _md_output_impl(
        ctx = ctx,
        variant = "plain",
        extra_args = ["--wrap=auto", "--columns=80"],
        extra_filters = [_md_plain_filter],
    )

md_plain_md = _md_output_rule(
    impl = _md_plain_md_impl,
    variant = "plain",
    extra_filters = [_md_plain_filter],
)

def _md_tumblr_md_impl(ctx):
    return _md_output_impl(
        ctx = ctx,
        variant = "tumblr",
        extra_args = ["--standalone", "--wrap=none"],
        extra_filters = [_md_tumblr_filter],
    )

md_tumblr_md = _md_output_rule(
    impl = _md_tumblr_md_impl,
    variant = "tumblr",
    extra_filters = [_md_tumblr_filter],
)

def _md_txt_impl(ctx):
    return simple_pandoc_output_impl(
        ctx = ctx,
        extension = "txt",
        variant = None,
        to_format = "plain",
        inputs = [
            filters.cleanup_metadata.file(ctx),
            filters.remove_collection_separators.file(ctx),
        ],
        args = [
            "--standalone",
            "--wrap=none",
            filters.cleanup_metadata.arg(ctx),
            filters.remove_collection_separators.arg(ctx),
        ],
        env = {},
        file = ctx.attr.file,
    )

md_txt = simple_pandoc_output_rule(
    impl = _md_txt_impl,
    extension = "txt",
    variant = None,
    filters = [
        filters.cleanup_metadata,
        filters.remove_collection_separators,
    ],
)
