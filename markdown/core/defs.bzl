"""Markdown rules."""

_SRC_FORMAT = "markdown+smart-pandoc_title_block-auto_identifiers"

MdGroupInfo = provider(
    "Info for a group of markdown files.",
    fields = {
        "deps": "The files in the group",
        "metadata": "Metadata of all files in the group, as json",
    },
)

MdFileInfo = provider(
    "Info for a markdown file.",
    fields = {
        "name": "The name of the document",
        "output": "Compiled document, as json",
        "metadata": "Document metadata, as json",
        "data": "Data deps of the document",
    },
)

def _md_group_impl(ctx):
    output = []
    for dep in ctx.attr.deps:
        output += dep[DefaultInfo].files.to_list()

    metadata = ctx.actions.declare_file(ctx.label.name + "_metadata.json")
    metadata_args = []
    for dep in ctx.attr.deps:
        metadata_args += ["--metadata_file", dep.label.package + ":" + dep.label.name, dep[MdFileInfo].metadata.path]
    ctx.actions.run(
        outputs = [metadata],
        inputs = [dep[MdFileInfo].metadata for dep in ctx.attr.deps],
        executable = ctx.executable._combine_deps_metadata,
        arguments = metadata_args + [metadata.path],
        progress_message = "%{label}: combining deps metadata",
    )

    return [
        DefaultInfo(files = depset([metadata] + output)),
        MdGroupInfo(deps = ctx.attr.deps, metadata = metadata),
    ]

md_group = rule(
    implementation = _md_group_impl,
    doc = "md_group is a group of md_file targets.",
    attrs = {
        "deps": attr.label_list(
            allow_empty = True,
            providers = [DefaultInfo, MdFileInfo],
            doc = "md_file targets to include in the group.",
        ),
        "_combine_deps_metadata": attr.label(
            default = "//markdown/core:combine_deps_metadata",
            executable = True,
            cfg = "exec",
        ),
    },
)

def _standard_lint(ctx):
    standard_lint_input = ctx.actions.declare_file(ctx.label.name + "_standard_lint_input.md")
    ctx.actions.run(
        outputs = [standard_lint_input],
        inputs = [
            ctx.file.src,
        ],
        executable = ctx.executable._gen_standard_lint_input,
        arguments = [
            ctx.file.src.path,
            standard_lint_input.path,
        ],
        progress_message = "%{label}: generating input for standard linter",
    )

    standard_lint_ok = ctx.actions.declare_file(ctx.label.name + "_standard_lint_ok.txt")
    ctx.actions.run(
        outputs = [standard_lint_ok],
        inputs = [
            standard_lint_input,
            ctx.file._pymarkdown_config,
        ],
        executable = ctx.executable._standard_lint,
        arguments = [
            standard_lint_ok.path,
            "--strict-config",
            "--config",
            ctx.file._pymarkdown_config.path,
            "scan",
            standard_lint_input.path,
        ],
        progress_message = "%{label}: linting markdown with standard linter",
    )

    return standard_lint_ok

def _custom_lint(ctx):
    custom_lint_ok = ctx.actions.declare_file(ctx.label.name + "_custom_lint_ok.txt")
    ctx.actions.run(
        outputs = [custom_lint_ok],
        inputs = [
            ctx.file.src,
        ],
        executable = ctx.executable._custom_lint,
        arguments = [
            ctx.file.src.path,
            custom_lint_ok.path,
        ],
        progress_message = "%{label}: linting markdown with custom linter",
    )

    return custom_lint_ok

def _spellcheck(ctx):
    dictionary = ctx.actions.declare_file(ctx.label.name + "_dictionary.dic")
    if ctx.attr.dictionaries:
        dict_inputs = []
        dict_args = []
        for d in ctx.attr.dictionaries:
            dict_inputs += d.files.to_list()
            dict_args += [f.path for f in d.files.to_list()]
        ctx.actions.run(
            outputs = [dictionary],
            inputs = dict_inputs,
            executable = ctx.executable._gen_dictionary,
            arguments = [dictionary.path] +
                        dict_args,
            progress_message = "%{label}: generating dictionary",
        )
    else:
        ctx.actions.write(
            output = dictionary,
            content = "",
        )

    spellcheck_input = ctx.actions.declare_file(ctx.label.name + "_spellcheck_input.md")
    ctx.actions.run(
        outputs = [spellcheck_input],
        inputs = [
            ctx.file.src,
            ctx.file._spellcheck_input_template,
            ctx.file._spellcheck_filter,
        ],
        executable = ctx.executable._pandoc,
        arguments = [
            "--lua-filter=" + ctx.file._spellcheck_filter.path,
            "--from=" + _SRC_FORMAT,
            "--to=markdown-smart",
            "--template=" + ctx.file._spellcheck_input_template.path,
            "--strip-comments",
            "--fail-if-warnings",
            "--output=" + spellcheck_input.path,
            ctx.file.src.path,
        ],
        progress_message = "%{label}: generating input for spellchecking",
    )

    spellcheck_ok = ctx.actions.declare_file(ctx.label.name + "_spellcheck_ok.txt")
    ctx.actions.run(
        outputs = [spellcheck_ok],
        inputs = [dictionary, spellcheck_input],
        executable = ctx.executable._spellcheck,
        arguments = [
            dictionary.path,
            spellcheck_input.path,
            spellcheck_ok.path,
            "en_GB",
        ],
        progress_message = "%{label}: spellchecking",
    )

    return spellcheck_ok

def _md_file_impl(ctx):
    lint_ok = [
        _standard_lint(ctx),
        _custom_lint(ctx),
        _spellcheck(ctx),
    ]

    preprocessed = ctx.actions.declare_file(ctx.label.name + "_stage1_preprocessed.md")
    dep_args = []
    for dep in ctx.attr.deps[MdGroupInfo].deps:
        dep_args += ["--dep", dep.label.package + ":" + dep.label.name, dep[MdFileInfo].output.path]
    image_args = []
    for image in ctx.attr.images:
        image_args += ["--image", image.label.package + ":" + image.label.name, image[DefaultInfo].files.to_list()[0].path]
    ctx.actions.run(
        outputs = [preprocessed],
        inputs = [ctx.file.src],
        executable = ctx.executable._preprocess,
        arguments = dep_args + image_args + [ctx.file.src.path, preprocessed.path, ctx.label.package],
        progress_message = "%{label}: preprocessing markdown",
    )

    compiled = ctx.actions.declare_file(ctx.label.name + "_stage2_compiled.json")
    extra_args = []
    if ctx.attr.increment_included_headers:
        extra_args.append("--metadata=increment-included-headers:t")
    ctx.actions.run(
        outputs = [compiled],
        inputs = [
            preprocessed,
            ctx.file._validate_ids,
            ctx.file._spellcheck_cleanup,
            ctx.file._validate_quotes,
            ctx.file._include,
            ctx.file._starts_with_text,
            ctx.file._header_auto_ids,
            ctx.file._wordcount,
            ctx.file._poetry_lines,
        ] + [dep[MdFileInfo].output for dep in ctx.attr.deps[MdGroupInfo].deps],
        executable = ctx.executable._pandoc,
        tools = [ctx.executable._validate_metadata],
        arguments = [
            "--lua-filter=" + ctx.file._validate_ids.path,
            "--lua-filter=" + ctx.file._spellcheck_cleanup.path,
            "--lua-filter=" + ctx.file._validate_quotes.path,
            "--filter=" + ctx.executable._validate_metadata.path,
            "--lua-filter=" + ctx.file._include.path,
            "--lua-filter=" + ctx.file._starts_with_text.path,
            "--lua-filter=" + ctx.file._header_auto_ids.path,
            "--lua-filter=" + ctx.file._wordcount.path,
            "--lua-filter=" + ctx.file._poetry_lines.path,
            "--metadata=lang:en-GB",
            "--from=" + _SRC_FORMAT,
            "--to=json",
            "--strip-comments",
            "--fail-if-warnings",
            "--output=" + compiled.path,
        ] + extra_args + [
            preprocessed.path,
        ],
        progress_message = "%{label}: compiling markdown",
    )

    raw_version = ctx.actions.declare_file(ctx.label.name + "_raw_version.json")
    ctx.actions.run(
        outputs = [raw_version],
        inputs = [ctx.info_file],
        executable = ctx.executable._raw_version,
        arguments = [ctx.info_file.path, raw_version.path, ctx.label.package],
        progress_message = "%{label}: computing raw version",
    )

    version = ctx.actions.declare_file(ctx.label.name + "_version.json")
    extra_args = []
    if ctx.attr.version_override:
        extra_args += ["--version_override", ctx.attr.version_override]
    ctx.actions.run(
        outputs = [version],
        inputs = [raw_version, ctx.attr.deps[MdGroupInfo].metadata],
        executable = ctx.executable._version,
        arguments = extra_args + [raw_version.path, ctx.attr.deps[MdGroupInfo].metadata.path, version.path],
        progress_message = "%{label}: computing version",
    )

    versioned = ctx.actions.declare_file(ctx.label.name + "_stage3_versioned.json")
    versioned_metadata = ctx.actions.declare_file(ctx.label.name + "_stage3_versioned_metadata.json")
    ctx.actions.run(
        outputs = [versioned, versioned_metadata],
        inputs = [
            compiled,
            version,
            ctx.file._write_metadata,
            ctx.file._cleanup,
        ],
        executable = ctx.executable._pandoc,
        arguments = [
            "--lua-filter=" + ctx.file._write_metadata.path,
            "--lua-filter=" + ctx.file._cleanup.path,
            "--metadata-file=" + version.path,
            "--metadata=metadata-out-file:" + versioned_metadata.path,
            "--from=json",
            "--to=json",
            "--fail-if-warnings",
            "--output=" + versioned.path,
            compiled.path,
        ],
        progress_message = "%{label}: adding version information",
    )

    output = ctx.actions.declare_file(ctx.label.name + ".json")
    ctx.actions.run(
        outputs = [output],
        inputs = [versioned] + lint_ok,
        executable = "cp",
        arguments = [versioned.path, output.path],
        progress_message = "%{label}: generating output",
    )

    metadata = ctx.outputs.metadata_out
    ctx.actions.run(
        outputs = [metadata],
        inputs = [output, versioned_metadata],
        executable = "cp",
        arguments = [versioned_metadata.path, metadata.path],
        progress_message = "%{label}: generating metadata",
    )

    data = depset(
        ctx.attr.data + ctx.attr.images,
        transitive = [dep[MdFileInfo].data for dep in ctx.attr.deps[MdGroupInfo].deps],
    )
    return [
        DefaultInfo(files = depset([output, metadata])),
        MdFileInfo(name = ctx.label.name, output = output, metadata = metadata, data = data),
    ]

md_file = rule(
    implementation = _md_file_impl,
    doc = "md_file compiles and validates a single markdown file.",
    attrs = {
        "src": attr.label(
            allow_single_file = [".md"],
            doc = "A markdown source file.",
        ),
        "deps": attr.label(
            providers = [MdGroupInfo],
            doc = "Other md_file targets used in !include statements in src.",
        ),
        "dictionaries": attr.label_list(
            allow_empty = True,
            allow_files = [".dic"],
            doc = "Dictionary files for spellchecking.",
        ),
        "data": attr.label_list(
            allow_empty = True,
            allow_files = True,
            doc = "Data dependencies.",
        ),
        "images": attr.label_list(
            allow_empty = True,
            allow_files = True,
            doc = "Image dependencies.",
        ),
        "increment_included_headers": attr.bool(
            default = False,
            doc = "If true, header level in included files is incremented, e.g. level 1 headers become level 2 headers. If false, headers are unchanged.",
        ),
        "version_override": attr.string(
            default = "",
            doc = "Set the document version to this value, rather than the computed value. Should only be used for testing.",
        ),
        "metadata_out": attr.output(
            doc = "Label of the output metadata file.",
        ),
        "_pandoc": attr.label(
            default = "//markdown/external:pandoc",
            executable = True,
            cfg = "exec",
        ),
        "_gen_standard_lint_input": attr.label(
            default = "//markdown/core/lint:gen_standard_lint_input",
            executable = True,
            cfg = "exec",
        ),
        "_standard_lint": attr.label(
            default = "//markdown/core/lint:standard_lint",
            executable = True,
            cfg = "exec",
        ),
        "_pymarkdown_config": attr.label(
            allow_single_file = True,
            default = "//:pymarkdown.json",
        ),
        "_custom_lint": attr.label(
            default = "//markdown/core/lint:custom_lint",
            executable = True,
            cfg = "exec",
        ),
        "_gen_dictionary": attr.label(
            default = "//markdown/core/spelling:gen_dictionary",
            executable = True,
            cfg = "exec",
        ),
        "_spellcheck_input_template": attr.label(
            allow_single_file = True,
            default = "//markdown/core/spelling:spellcheck_input_template.md",
        ),
        "_spellcheck_filter": attr.label(
            allow_single_file = True,
            default = "//markdown/core/spelling:spellcheck_filter.lua",
        ),
        "_spellcheck": attr.label(
            default = "//markdown/core/spelling:spellcheck",
            executable = True,
            cfg = "exec",
        ),
        "_preprocess": attr.label(
            default = "//markdown/core:preprocess",
            executable = True,
            cfg = "exec",
        ),
        "_validate_ids": attr.label(
            allow_single_file = True,
            default = "//markdown/core/filters:validate_ids.lua",
        ),
        "_spellcheck_cleanup": attr.label(
            allow_single_file = True,
            default = "//markdown/core/spelling:spellcheck_cleanup.lua",
        ),
        "_validate_quotes": attr.label(
            allow_single_file = True,
            default = "//markdown/core/filters:validate_quotes.lua",
        ),
        "_validate_metadata": attr.label(
            default = "//markdown/core/filters:validate_metadata",
            executable = True,
            cfg = "exec",
        ),
        "_include": attr.label(
            allow_single_file = True,
            default = "//markdown/core/filters:include.lua",
        ),
        "_starts_with_text": attr.label(
            allow_single_file = True,
            default = "//markdown/core/filters:starts_with_text.lua",
        ),
        "_header_auto_ids": attr.label(
            allow_single_file = True,
            default = "//markdown/core/filters:header_auto_ids.lua",
        ),
        "_wordcount": attr.label(
            allow_single_file = True,
            default = "//markdown/core/filters:wordcount.lua",
        ),
        "_poetry_lines": attr.label(
            allow_single_file = True,
            default = "//markdown/core/filters:poetry_lines.lua",
        ),
        "_raw_version": attr.label(
            default = "//markdown/core:raw_version",
            executable = True,
            cfg = "exec",
        ),
        "_version": attr.label(
            default = "//markdown/core:version",
            executable = True,
            cfg = "exec",
        ),
        "_write_metadata": attr.label(
            allow_single_file = True,
            default = "//markdown/core/filters:write_metadata.lua",
        ),
        "_cleanup": attr.label(
            allow_single_file = True,
            default = "//markdown/core/filters:cleanup.lua",
        ),
    },
)
