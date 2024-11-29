"""Helpers for output formats."""

load("//markdown/private/core:defs.bzl", "MdFileInfo")
load(":types.bzl", "filter")

visibility("//markdown/private/formats/...")

tools = struct(
    pandoc = struct(
        attr = {
            "_pandoc": attr.label(
                default = "//markdown/private/external:pandoc",
                executable = True,
                cfg = "exec",
            ),
        },
        executable = lambda ctx: ctx.executable._pandoc,
    ),
    write_open_script = struct(
        attr = {
            "_write_open_script": attr.label(
                default = "//markdown/private/formats:write_open_script",
                executable = True,
                cfg = "exec",
            ),
        },
        executable = lambda ctx: ctx.executable._write_open_script,
    ),
    zip = struct(
        attr = {
            "_zip": attr.label(
                default = "//markdown/private/external:zip",
                executable = True,
                cfg = "exec",
            ),
        },
        executable = lambda ctx: ctx.executable._zip,
    ),
    unzip = struct(
        attr = {
            "_unzip": attr.label(
                default = "//markdown/private/external:unzip",
                executable = True,
                cfg = "exec",
            ),
        },
        executable = lambda ctx: ctx.executable._unzip,
    ),
    zip_cleaner = struct(
        attr = {
            "_strip_nondeterminism": attr.label(
                default = "//markdown/private/external:strip_nondeterminism",
                executable = True,
                cfg = "exec",
            ),
            "_zip_cleaner": attr.label(
                default = "//markdown/private/formats:zip_cleaner",
                executable = True,
                cfg = "exec",
            ),
        },
        executable = lambda ctx: ctx.executable._zip_cleaner,
        strip_nondeterminism = lambda ctx: ctx.executable._strip_nondeterminism,
    ),
)

filters = struct(
    add_title = filter(
        attr = {
            "_add_title": attr.label(
                allow_single_file = True,
                default = "//markdown/private/formats/filters:add_title.lua",
            ),
        },
        file = lambda ctx: ctx.file._add_title,
        arg = lambda ctx: "--lua-filter=" + ctx.file._add_title.path,
    ),
    add_subject = filter(
        attr = {
            "_add_subject": attr.label(
                allow_single_file = True,
                default = "//markdown/private/formats/filters:add_subject.lua",
            ),
        },
        file = lambda ctx: ctx.file._add_subject,
        arg = lambda ctx: "--lua-filter=" + ctx.file._add_subject.path,
    ),
    cleanup_metadata = filter(
        attr = {
            "_cleanup_metadata": attr.label(
                allow_single_file = True,
                default = "//markdown/private/formats/filters:cleanup_metadata.lua",
            ),
        },
        file = lambda ctx: ctx.file._cleanup_metadata,
        arg = lambda ctx: "--lua-filter=" + ctx.file._cleanup_metadata.path,
    ),
    remove_paragraph_annotations = filter(
        attr = {
            "_remove_paragraph_annotations": attr.label(
                allow_single_file = True,
                default = "//markdown/private/formats/filters:remove_paragraph_annotations.lua",
            ),
        },
        file = lambda ctx: ctx.file._remove_paragraph_annotations,
        arg = lambda ctx: "--lua-filter=" + ctx.file._remove_paragraph_annotations.path,
    ),
    remove_collection_separators = filter(
        attr = {
            "_remove_collection_separators": attr.label(
                allow_single_file = True,
                default = "//markdown/private/formats/filters:remove_collection_separators.lua",
            ),
        },
        file = lambda ctx: ctx.file._remove_collection_separators,
        arg = lambda ctx: "--lua-filter=" + ctx.file._remove_collection_separators.path,
    ),
    remove_collection_separators_before_headers = filter(
        attr = {
            "_remove_collection_separators_before_headers": attr.label(
                allow_single_file = True,
                default = "//markdown/private/formats/filters:remove_collection_separators_before_headers.lua",
            ),
        },
        file = lambda ctx: ctx.file._remove_collection_separators_before_headers,
        arg = lambda ctx: "--lua-filter=" + ctx.file._remove_collection_separators_before_headers.path,
    ),
)

def _timestamp_override(ctx):
    env = {}
    if ctx.attr.timestamp_override:
        env["SOURCE_DATE_EPOCH"] = ctx.attr.timestamp_override
    return env

timestamp_override = struct(
    attr = {
        "timestamp_override": attr.string(),
    },
    env = _timestamp_override,
)

def expand_locations(ctx, file, args):
    data = file[MdFileInfo].data.to_list()
    return [ctx.expand_location(arg, targets = data) for arg in args]

def _ext_var(extension, variant, joiner):
    return (variant + joiner if variant else "") + extension

def ext_var_underscore(extension, variant):
    return _ext_var(extension, variant, "_")

def ext_var_dot(extension, variant):
    return _ext_var(extension, variant, ".")

def docstring(extension, variant):
    return ("md_" + ext_var_underscore(extension, variant) + " generates " +
            ext_var_dot(extension, variant) + " output from an md_file.")

def _progress_message_without_label(extension, variant):
    return "generating " + ext_var_dot(extension, variant) + " output"

def progress_message(extension, variant):
    return "%{label}: " + _progress_message_without_label(extension, variant)

def default_info(ctx, output, script):
    return DefaultInfo(
        files = depset([output, script]),
        runfiles = ctx.runfiles(files = [output]),
        executable = script,
    )

def write_open_script(ctx, extension, variant, file_to_open):
    script = ctx.actions.declare_file(ctx.label.name + ".sh")
    ctx.actions.run(
        outputs = [script],
        inputs = [file_to_open],
        executable = tools.write_open_script.executable(ctx),
        arguments = [ctx.workspace_name, file_to_open.short_path, script.path],
        progress_message = "%{label}: generating " + ext_var_dot(extension, variant) + " open script",
    )
    return script

def clean_zip(ctx, in_file, out_file):
    ctx.actions.run(
        outputs = [out_file],
        inputs = [
            tools.zip_cleaner.strip_nondeterminism(ctx),
            in_file,
        ],
        executable = tools.zip_cleaner.executable(ctx),
        arguments = [
            tools.zip_cleaner.strip_nondeterminism(ctx).path,
            in_file.path,
            out_file.path,
        ],
        progress_message = "%{label}: cleaining zip file",
    )

def pandoc(ctx, extension, variant, to_format, inputs, args, env, file, output, progress_message = None, sandbox = True):
    """Run pandoc.

    Args:
        ctx: rule ctx.
        extension: file extension of the output format.
        variant: file variant of the output format.
        to_format: pandoc output format.
        inputs: action inputs.
        args: extra action args.
        env: environment variables to pass to pandoc.
        file: something that provides MdFileInfo.
        output: the output file.
        progress_message: message to display when running the action.
        sandbox: whether to run sandboxed.
    """
    if not progress_message:
        progress_message = _progress_message_without_label(extension, variant)
    progress_message = "%{label}: " + progress_message
    data_inputs = []
    for target in file[MdFileInfo].data.to_list():
        data_inputs += target.files.to_list()

    ctx.actions.run(
        outputs = [output],
        inputs = [
            file[MdFileInfo].output,
        ] + data_inputs + inputs,
        executable = tools.pandoc.executable(ctx),
        arguments = [
            "--from=json",
            "--to=" + to_format,
            "--fail-if-warnings",
            "--output=" + output.path,
        ] + args + [
            file[MdFileInfo].output.path,
        ],
        env = env,
        progress_message = progress_message,
        execution_requirements = {"local": "1"} if not sandbox else {},
    )

def simple_pandoc_output_impl(ctx, extension, variant, to_format, inputs, args, env, file, sandbox = True):
    output = ctx.outputs.out
    pandoc(
        ctx = ctx,
        extension = extension,
        variant = variant,
        to_format = to_format,
        inputs = inputs,
        args = args + expand_locations(ctx, file, ctx.attr.extra_pandoc_flags),
        env = env,
        file = file,
        output = output,
        sandbox = sandbox,
    )
    script = write_open_script(
        ctx = ctx,
        extension = extension,
        variant = variant,
        file_to_open = output,
    )

    return [default_info(ctx, output, script)]

def simple_pandoc_output_rule(impl, extension, variant, filters = None):
    attrs = {
        "file": attr.label(
            providers = [MdFileInfo],
            doc = "An md_file target.",
        ),
        "extra_pandoc_flags": attr.string_list(
            doc = "Extra flags to pass to pandoc",
        ),
        "out": attr.output(),
    } | tools.pandoc.attr | tools.write_open_script.attr

    for filter in filters or []:
        attrs |= filter.attr

    return rule(
        implementation = impl,
        executable = True,
        doc = docstring(extension, variant),
        attrs = attrs,
    )
