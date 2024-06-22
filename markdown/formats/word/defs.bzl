"""Rules for word-processor outputs."""

load("//markdown/core:defs.bzl", "MdFileInfo")
load(
    "//markdown/formats:lib.bzl",
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
        ext = "odt",
        to_format = "odt",
        inputs = [filters.remove_collection_separators.file(ctx)],
        args = [
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
        ext = "odt",
        file_to_open = output,
    )

    return [
        default_info(ctx, output, script),
    ]

md_odt = rule(
    implementation = _md_odt_impl,
    executable = True,
    doc = docstring("odt"),
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
            filters.remove_collection_separators.attr |
            timestamp_override.attr,
)

def _md_docx_impl(ctx):
    intermediate = ctx.actions.declare_file(ctx.label.name + "_intermediate.docx")
    pandoc(
        ctx = ctx,
        ext = "docx",
        to_format = "docx",
        inputs = [
            ctx.file._template,
            filters.remove_collection_separators_before_headers.file(ctx),
            ctx.file._docx_filter,
        ],
        args = [
            "--reference-doc=" + ctx.file._template.path,
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
        ext = "docx",
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
    doc = docstring("docx"),
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
                    default = "//markdown/formats/word:reference.docx",
                ),
                "_docx_filter": attr.label(
                    allow_single_file = True,
                    default = "//markdown/formats/word:docx_filter.lua",
                ),
            } |
            tools.pandoc.attr |
            tools.write_open_script.attr |
            tools.zip_cleaner.attr |
            filters.remove_collection_separators_before_headers.attr |
            timestamp_override.attr,
)

def _md_doc_impl(ctx):
    output = ctx.outputs.out
    ctx.actions.run(
        outputs = [output],
        inputs = [ctx.attr.docx[MdDocxInfo].output],
        executable = ctx.executable._unoconv,
        arguments = [
            "--format",
            "doc",
            "--output",
            output.path,
            ctx.attr.docx[MdDocxInfo].output.path,
        ],
        env = {"HOME": "/tmp"},
        progress_message = progress_message("doc"),
    )

    script = write_open_script(
        ctx = ctx,
        ext = "doc",
        file_to_open = output,
    )

    return [
        default_info(ctx, output, script),
    ]

md_doc = rule(
    implementation = _md_doc_impl,
    executable = True,
    doc = docstring("doc"),
    attrs = {
                "docx": attr.label(
                    providers = [MdFileInfo, MdDocxInfo],
                    doc = "An md_docx target.",
                ),
                "out": attr.output(),
                "_unoconv": attr.label(
                    default = "//markdown/formats/word:unoconv",
                    executable = True,
                    cfg = "exec",
                ),
            } |
            tools.write_open_script.attr,
)

def _md_ms_docx_impl(ctx):
    metadata = ctx.actions.declare_file(ctx.label.name + "_ms_metadata.json")
    ctx.actions.run(
        outputs = [metadata],
        inputs = [ctx.attr.file[MdFileInfo].metadata],
        executable = ctx.executable._ms_metadata,
        arguments = [
            ctx.attr.file[MdFileInfo].metadata.path,
            metadata.path,
        ],
        progress_message = "%{label}: generating ms metadata",
    )

    intermediate_docx = ctx.actions.declare_file(ctx.label.name + "_ms_intermediate.docx")
    env = timestamp_override.env(ctx)
    env["PANDOC"] = tools.pandoc.wrapped_executable(ctx).path
    data_inputs = []
    for target in ctx.attr.file[MdFileInfo].data.to_list():
        data_inputs += target.files.to_list()
    ctx.actions.run(
        outputs = [intermediate_docx],
        inputs = data_inputs + [
            ctx.attr.file[MdFileInfo].output,
            metadata,
            ctx.file._filter,
            tools.pandoc.wrapped_executable(ctx),
        ],
        executable = ctx.executable._md2short,
        arguments = [
            "--overwrite",
            "--modern",
            "--from",
            "json",
            "--output",
            intermediate_docx.path,
            "--metadata-file=" + metadata.path,
            "--lua-filter=" + ctx.file._filter.path,
            ctx.attr.file[MdFileInfo].output.path,
        ],
        env = env,
        progress_message = progress_message("ms.docx"),
    )

    output = ctx.outputs.out
    clean_zip(
        ctx = ctx,
        in_file = intermediate_docx,
        out_file = output,
    )

    script = write_open_script(
        ctx = ctx,
        ext = "ms.docx",
        file_to_open = output,
    )

    return [
        default_info(ctx, output, script),
    ]

md_ms_docx = rule(
    implementation = _md_ms_docx_impl,
    executable = True,
    doc = docstring("ms.docx"),
    attrs = {
                "file": attr.label(
                    providers = [MdFileInfo],
                    doc = "An md_file target.",
                ),
                "out": attr.output(),
                "_ms_metadata": attr.label(
                    default = "//markdown/formats/word:ms_metadata",
                    executable = True,
                    cfg = "exec",
                ),
                "_md2short": attr.label(
                    default = "//markdown/external:md2short",
                    executable = True,
                    cfg = "exec",
                ),
                "_filter": attr.label(
                    allow_single_file = True,
                    default = "//markdown/formats/word:ms_docx_filter.lua",
                ),
            } |
            tools.pandoc.attr |
            tools.write_open_script.attr |
            tools.zip_cleaner.attr |
            timestamp_override.attr,
)
