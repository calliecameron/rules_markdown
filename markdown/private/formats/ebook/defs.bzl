"""Rules for ebook outputs."""

load("//markdown/private/core:defs.bzl", "MdFileInfo")
load(
    "//markdown/private/formats:lib.bzl",
    "clean_zip",
    "default_info",
    "docstring",
    "expand_locations",
    "filters",
    "pandoc",
    "progress_message",
    "timestamp_override",
    "tools",
    "write_open_script",
)

visibility("//markdown/private/formats")

MdEpubInfo = provider(
    "Info for epub output",
    fields = {
        "output": "Epub file",
    },
)

def _md_epub_impl(ctx):
    intermediate = ctx.actions.declare_file(ctx.label.name + "_intermediate.epub")
    pandoc(
        ctx = ctx,
        extension = "epub",
        variant = None,
        to_format = "epub",
        inputs = [
            ctx.file._css,
            filters.add_title.file(ctx),
            filters.add_subject.file(ctx),
            filters.cleanup_metadata.file(ctx),
            filters.remove_collection_separators_before_headers.file(ctx),
        ],
        args = [
            "--css=" + ctx.file._css.path,
            filters.add_title.arg(ctx),
            filters.add_subject.arg(ctx),
            filters.cleanup_metadata.arg(ctx),
            filters.remove_collection_separators_before_headers.arg(ctx),
        ] + expand_locations(ctx, ctx.attr.file, ctx.attr.extra_pandoc_flags),
        env = timestamp_override.env(ctx),
        file = ctx.attr.file,
        output = intermediate,
    )

    output = ctx.outputs.out
    clean_zip(
        ctx = ctx,
        in_file = intermediate,
        out_file = output,
    )

    script = write_open_script(
        ctx = ctx,
        extension = "epub",
        variant = None,
        file_to_open = output,
    )

    return [
        default_info(ctx, output, script),
        MdEpubInfo(output = output),
        ctx.attr.file[MdFileInfo],
    ]

md_epub = rule(
    implementation = _md_epub_impl,
    executable = True,
    doc = docstring("epub", None),
    attrs = {
                "file": attr.label(
                    providers = [MdFileInfo],
                    doc = "An md_file target.",
                ),
                "extra_pandoc_flags": attr.string_list(
                    doc = "Extra flags to pass to pandoc",
                ),
                "out": attr.output(),
                "_css": attr.label(
                    allow_single_file = True,
                    default = "//markdown/private/formats/ebook:epub.css",
                ),
            } |
            tools.pandoc.attr |
            tools.write_open_script.attr |
            tools.zip_cleaner.attr |
            filters.add_title.attr |
            filters.add_subject.attr |
            filters.cleanup_metadata.attr |
            filters.remove_collection_separators_before_headers.attr |
            timestamp_override.attr,
)

def _md_mobi_impl(ctx):
    output = ctx.outputs.out
    ctx.actions.run(
        outputs = [output],
        inputs = [
            ctx.attr.epub[MdEpubInfo].output,
            ctx.executable._ebook_convert_bin,
        ],
        executable = ctx.executable._ebook_convert,
        arguments = [
            ctx.executable._ebook_convert_bin.path,
            ctx.attr.epub[MdEpubInfo].output.path,
            output.path,
        ],
        progress_message = progress_message("mobi", None),
    )

    script = write_open_script(
        ctx = ctx,
        extension = "mobi",
        variant = None,
        file_to_open = output,
    )

    return [default_info(ctx, output, script)]

md_mobi = rule(
    implementation = _md_mobi_impl,
    executable = True,
    doc = docstring("mobi", None),
    attrs = {
                "epub": attr.label(
                    providers = [MdFileInfo, MdEpubInfo],
                    doc = "An md_epub target.",
                ),
                "out": attr.output(),
                "_ebook_convert": attr.label(
                    default = "//markdown/private/formats/ebook:ebook_convert",
                    executable = True,
                    cfg = "exec",
                ),
                "_ebook_convert_bin": attr.label(
                    default = "//markdown/private/external:ebook_convert",
                    executable = True,
                    cfg = "exec",
                ),
            } |
            tools.write_open_script.attr,
)
