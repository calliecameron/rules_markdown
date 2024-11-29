"""Rules for word-processor outputs."""

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

MdDocxInfo = provider(
    "Info for docx output",
    fields = {
        "output": "Docx file",
    },
)

def _md_odt_impl(ctx):
    intermediate = ctx.actions.declare_file(ctx.label.name + "_intermediate.odt")
    pandoc(
        ctx = ctx,
        extension = "odt",
        variant = None,
        to_format = "odt",
        inputs = [
            filters.add_subject.file(ctx),
            filters.cleanup_metadata.file(ctx),
            filters.remove_collection_separators.file(ctx),
        ],
        args = [
            filters.add_subject.arg(ctx),
            filters.cleanup_metadata.arg(ctx),
            filters.remove_collection_separators.arg(ctx),
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
        extension = "odt",
        variant = None,
        file_to_open = output,
    )

    return [
        default_info(ctx, output, script),
    ]

md_odt = rule(
    implementation = _md_odt_impl,
    executable = True,
    doc = docstring("odt", None),
    attrs = {
                "file": attr.label(
                    providers = [MdFileInfo],
                    doc = "An md_file target.",
                ),
                "extra_pandoc_flags": attr.string_list(
                    doc = "Extra flags to pass to pandoc",
                ),
                "out": attr.output(),
            } |
            tools.pandoc.attr |
            tools.write_open_script.attr |
            tools.zip_cleaner.attr |
            filters.add_subject.attr |
            filters.cleanup_metadata.attr |
            filters.remove_collection_separators.attr |
            timestamp_override.attr,
)

def _md_docx_impl(ctx):
    intermediate = ctx.actions.declare_file(ctx.label.name + "_intermediate.docx")
    pandoc(
        ctx = ctx,
        extension = "docx",
        variant = None,
        to_format = "docx",
        inputs = [
            ctx.file._template,
            filters.add_subject.file(ctx),
            filters.cleanup_metadata.file(ctx),
            filters.remove_collection_separators_before_headers.file(ctx),
            ctx.file._docx_filter,
        ],
        args = [
            "--reference-doc=" + ctx.file._template.path,
            filters.add_subject.arg(ctx),
            filters.cleanup_metadata.arg(ctx),
            filters.remove_collection_separators_before_headers.arg(ctx),
            "--lua-filter=" + ctx.file._docx_filter.path,
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
        extension = "docx",
        variant = None,
        file_to_open = output,
    )

    return [
        default_info(ctx, output, script),
        MdDocxInfo(output = output),
        ctx.attr.file[MdFileInfo],
    ]

md_docx = rule(
    implementation = _md_docx_impl,
    executable = True,
    doc = docstring("docx", None),
    attrs = {
                "file": attr.label(
                    providers = [MdFileInfo],
                    doc = "An md_file target.",
                ),
                "extra_pandoc_flags": attr.string_list(
                    doc = "Extra flags to pass to pandoc",
                ),
                "out": attr.output(),
                "_template": attr.label(
                    allow_single_file = True,
                    default = "//markdown/private/formats/word:reference.docx",
                ),
                "_docx_filter": attr.label(
                    allow_single_file = True,
                    default = "//markdown/private/formats/word:docx_filter.lua",
                ),
            } |
            tools.pandoc.attr |
            tools.write_open_script.attr |
            tools.zip_cleaner.attr |
            filters.add_subject.attr |
            filters.cleanup_metadata.attr |
            filters.remove_collection_separators_before_headers.attr |
            timestamp_override.attr,
)

def _md_doc_impl(ctx):
    output = ctx.outputs.out
    ctx.actions.run(
        outputs = [output],
        inputs = [
            ctx.attr.docx[MdDocxInfo].output,
            ctx.executable._unoconv_bin,
        ],
        executable = ctx.executable._unoconv,
        arguments = [
            ctx.executable._unoconv_bin.path,
            "--format",
            "doc",
            "--output",
            output.path,
            ctx.attr.docx[MdDocxInfo].output.path,
        ],
        env = {"HOME": "/tmp"},
        progress_message = progress_message("doc", None),
    )

    script = write_open_script(
        ctx = ctx,
        extension = "doc",
        variant = None,
        file_to_open = output,
    )

    return [
        default_info(ctx, output, script),
    ]

md_doc = rule(
    implementation = _md_doc_impl,
    executable = True,
    doc = docstring("doc", None),
    attrs = {
                "docx": attr.label(
                    providers = [MdFileInfo, MdDocxInfo],
                    doc = "An md_docx target.",
                ),
                "out": attr.output(),
                "_unoconv": attr.label(
                    default = "//markdown/private/formats/word:unoconv",
                    executable = True,
                    cfg = "exec",
                ),
                "_unoconv_bin": attr.label(
                    default = "//markdown/private/external:unoconv",
                    executable = True,
                    cfg = "exec",
                ),
            } |
            tools.write_open_script.attr,
)

def _md_shunnmodern_docx_impl(ctx):
    metadata = ctx.actions.declare_file(ctx.label.name + "_shunnmodern_metadata.json")
    ctx.actions.run(
        outputs = [metadata],
        inputs = [ctx.attr.file[MdFileInfo].metadata],
        executable = ctx.executable._shunn_metadata,
        arguments = [
            ctx.attr.file[MdFileInfo].metadata.path,
            metadata.path,
        ],
        progress_message = "%{label}: generating shunn metadata",
    )

    intermediate_docx = ctx.actions.declare_file(ctx.label.name + "_shunnmodern_intermediate.docx")
    env = timestamp_override.env(ctx)
    data_inputs = []
    for target in ctx.attr.file[MdFileInfo].data.to_list():
        data_inputs += target.files.to_list()
    ctx.actions.run(
        outputs = [intermediate_docx],
        inputs = data_inputs + [
            ctx.attr.file[MdFileInfo].output,
            metadata,
            filters.add_subject.file(ctx),
            filters.cleanup_metadata.file(ctx),
            ctx.file._filter,
            ctx.executable._md2short_bin,
            tools.zip.executable(ctx),
            tools.unzip.executable(ctx),
            ctx.executable._strip_nondeterminism,
            tools.pandoc.executable(ctx),
        ],
        executable = ctx.executable._md2short,
        arguments = [
            ctx.executable._md2short_bin.path,
            tools.pandoc.executable(ctx).path,
            tools.zip.executable(ctx).path,
            tools.unzip.executable(ctx).path,
            "--overwrite",
            "--modern",
            "--from",
            "json",
            "--output",
            intermediate_docx.path,
            "--metadata-file=" + metadata.path,
            "--metadata=strip-nondeterminism:" + ctx.executable._strip_nondeterminism.path,
            filters.add_subject.arg(ctx),
            filters.cleanup_metadata.arg(ctx),
            "--lua-filter=" + ctx.file._filter.path,
            ctx.attr.file[MdFileInfo].output.path,
        ],
        env = env,
        progress_message = progress_message("docx", "shunnmodern"),
    )

    output = ctx.outputs.out
    clean_zip(
        ctx = ctx,
        in_file = intermediate_docx,
        out_file = output,
    )

    script = write_open_script(
        ctx = ctx,
        extension = "docx",
        variant = "shunnmodern",
        file_to_open = output,
    )

    return [
        default_info(ctx, output, script),
    ]

md_shunnmodern_docx = rule(
    implementation = _md_shunnmodern_docx_impl,
    executable = True,
    doc = docstring("docx", "shunnmodern"),
    attrs = {
                "file": attr.label(
                    providers = [MdFileInfo],
                    doc = "An md_file target.",
                ),
                "out": attr.output(),
                "_shunn_metadata": attr.label(
                    default = "//markdown/private/formats/word:shunn_metadata",
                    executable = True,
                    cfg = "exec",
                ),
                "_md2short": attr.label(
                    default = "//markdown/private/formats/word:md2short",
                    executable = True,
                    cfg = "exec",
                ),
                "_md2short_bin": attr.label(
                    default = "//markdown/private/external:md2short",
                    executable = True,
                    cfg = "exec",
                ),
                "_filter": attr.label(
                    allow_single_file = True,
                    default = "//markdown/private/formats/word:shunn_docx_filter.lua",
                ),
                "_strip_nondeterminism": attr.label(
                    default = "//markdown/private/external:strip_nondeterminism",
                    executable = True,
                    cfg = "exec",
                ),
            } |
            tools.pandoc.attr |
            tools.write_open_script.attr |
            tools.zip.attr |
            tools.unzip.attr |
            tools.zip_cleaner.attr |
            filters.add_subject.attr |
            filters.cleanup_metadata.attr |
            timestamp_override.attr,
)
