"""Markdown rules."""

load("//markdown/private/utils:lib.bzl", "key_value_arg")

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
        metadata_args += [
            "--metadata-file",
            key_value_arg(
                dep.label.package + ":" + dep.label.name,
                dep[MdFileInfo].metadata.path,
            ),
        ]
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
            default = "//markdown/private/core/group:combine_deps_metadata",
            executable = True,
            cfg = "exec",
        ),
    },
)

def _standard_lint(ctx):
    standard_lint_ok = ctx.actions.declare_file(ctx.label.name + "_standard_lint_ok.txt")
    ctx.actions.run(
        outputs = [standard_lint_ok],
        inputs = [
            ctx.file.src,
            ctx.executable._markdownlint,
            ctx.file._markdownlint_config,
        ],
        executable = ctx.executable._standard_lint,
        arguments = [
            ctx.executable._markdownlint.path,
            ctx.file._markdownlint_config.path,
            ctx.file.src.path,
            standard_lint_ok.path,
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
    custom_dictionary = ctx.actions.declare_file(ctx.label.name + "_dictionary.dic")
    if ctx.attr.dictionaries:
        dict_inputs = []
        dict_args = []
        for d in ctx.attr.dictionaries:
            dict_inputs += d.files.to_list()
            dict_args += [f.path for f in d.files.to_list()]
        ctx.actions.run(
            outputs = [custom_dictionary],
            inputs = dict_inputs,
            executable = ctx.executable._gen_dictionary,
            arguments = [custom_dictionary.path] +
                        dict_args,
            progress_message = "%{label}: generating dictionary",
        )
    else:
        ctx.actions.write(
            output = custom_dictionary,
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
        inputs = [
            ctx.executable._hunspell,
            ctx.file._locale_archive,
            custom_dictionary,
            spellcheck_input,
        ] + ctx.files._hunspell_dicts,
        executable = ctx.executable._spellcheck,
        arguments = [
            ctx.executable._hunspell.path,
            ctx.files._hunspell_dicts[0].dirname,
            ctx.file._locale_archive.path,
            custom_dictionary.path,
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
        dep_args += [
            "--dep",
            key_value_arg(
                dep.label.package + ":" + dep.label.name,
                dep[MdFileInfo].output.path,
            ),
        ]
    image_args = []
    for image in ctx.attr.images:
        image_args += [
            "--image",
            key_value_arg(
                image.label.package + ":" + image.label.name,
                image[DefaultInfo].files.to_list()[0].path,
            ),
        ]
    ctx.actions.run(
        outputs = [preprocessed],
        inputs = [ctx.file.src],
        executable = ctx.executable._preprocess,
        arguments = dep_args + image_args + [
            ctx.file.src.path,
            preprocessed.path,
            ctx.label.package,
        ],
        progress_message = "%{label}: preprocessing markdown",
    )

    compiled = ctx.actions.declare_file(ctx.label.name + "_stage2_compiled.json")
    input_metadata_raw = ctx.actions.declare_file(ctx.label.name + "_input_metadata_raw.json")
    extra_args = []
    if ctx.attr.increment_included_headers:
        extra_args.append("--metadata=increment-included-headers:t")
    ctx.actions.run(
        outputs = [compiled, input_metadata_raw],
        inputs = [
            preprocessed,
            ctx.file._validate_ids,
            ctx.file._spellcheck_cleanup,
            ctx.file._validate_quotes,
            ctx.file._include,
            ctx.file._write_metadata,
            ctx.file._paragraph_annotations,
            ctx.file._header_auto_ids,
            ctx.file._wordcount,
            ctx.file._poetry_lines,
        ] + [dep[MdFileInfo].output for dep in ctx.attr.deps[MdGroupInfo].deps],
        executable = ctx.executable._pandoc,
        arguments = [
            "--lua-filter=" + ctx.file._validate_ids.path,
            "--lua-filter=" + ctx.file._spellcheck_cleanup.path,
            "--lua-filter=" + ctx.file._validate_quotes.path,
            "--lua-filter=" + ctx.file._include.path,
            "--lua-filter=" + ctx.file._write_metadata.path,
            "--lua-filter=" + ctx.file._paragraph_annotations.path,
            "--lua-filter=" + ctx.file._header_auto_ids.path,
            "--lua-filter=" + ctx.file._wordcount.path,
            "--lua-filter=" + ctx.file._poetry_lines.path,
            "--metadata=metadata-out-file:" + input_metadata_raw.path,
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

    input_metadata = ctx.actions.declare_file(ctx.label.name + "_input_metadata.json")
    ctx.actions.run(
        outputs = [input_metadata],
        inputs = [
            input_metadata_raw,
        ],
        executable = ctx.executable._validate_input_metadata,
        arguments = [
            input_metadata_raw.path,
            input_metadata.path,
        ],
        progress_message = "%{label}: validating input metadata",
    )

    raw_version = ctx.actions.declare_file(ctx.label.name + "_raw_version.json")
    if ctx.file.version_file:
        inputs = [ctx.file.version_file]
        args = ["--version-file", ctx.file.version_file.path]
    else:
        inputs = [ctx.info_file]
        args = ["--info-file", ctx.info_file.path]
    ctx.actions.run(
        outputs = [raw_version],
        inputs = inputs,
        executable = ctx.executable._raw_version,
        arguments = args + [raw_version.path],
        progress_message = "%{label}: computing raw version",
    )

    version = ctx.actions.declare_file(ctx.label.name + "_version.json")
    extra_args = []
    if ctx.attr.version_override:
        extra_args += ["--version-override", ctx.attr.version_override]
    if ctx.attr.repo_override:
        extra_args += ["--repo-override", ctx.attr.repo_override]
    ctx.actions.run(
        outputs = [version],
        inputs = [raw_version, ctx.attr.deps[MdGroupInfo].metadata],
        executable = ctx.executable._version,
        arguments = extra_args + [
            raw_version.path,
            ctx.attr.deps[MdGroupInfo].metadata.path,
            version.path,
        ],
        progress_message = "%{label}: computing version",
    )

    source_hash = ctx.actions.declare_file(ctx.label.name + "_source_hash.json")
    ctx.actions.run(
        outputs = [source_hash],
        inputs = [ctx.file.src, ctx.attr.deps[MdGroupInfo].metadata],
        executable = ctx.executable._source_hash,
        arguments = [
            ctx.file.src.path,
            ctx.attr.deps[MdGroupInfo].metadata.path,
            source_hash.path,
        ],
        progress_message = "%{label}: computing source hash",
    )

    parsed_dates = ctx.actions.declare_file(ctx.label.name + "_parsed_dates.json")
    ctx.actions.run(
        outputs = [parsed_dates],
        inputs = [input_metadata],
        executable = ctx.executable._parse_date,
        arguments = [
            input_metadata.path,
            parsed_dates.path,
        ],
        progress_message = "%{label}: parsing date",
    )

    versioned = ctx.actions.declare_file(ctx.label.name + "_stage3_versioned.json")
    versioned_metadata_raw = ctx.actions.declare_file(ctx.label.name + "_stage3_versioned_metadata_raw.json")
    ctx.actions.run(
        outputs = [versioned, versioned_metadata_raw],
        inputs = [
            compiled,
            input_metadata,
            version,
            source_hash,
            parsed_dates,
            ctx.file._write_metadata,
        ],
        executable = ctx.executable._pandoc,
        arguments = [
            "--lua-filter=" + ctx.file._write_metadata.path,
            "--metadata-file=" + version.path,
            "--metadata-file=" + source_hash.path,
            "--metadata-file=" + parsed_dates.path,
            "--metadata=metadata-out-file:" + versioned_metadata_raw.path,
            "--metadata=lang:en-GB",
            "--from=json",
            "--to=json",
            "--fail-if-warnings",
            "--output=" + versioned.path,
            compiled.path,
        ],
        progress_message = "%{label}: adding version information",
    )

    versioned_metadata = ctx.actions.declare_file(ctx.label.name + "_stage3_versioned_metadata.json")
    ctx.actions.run(
        outputs = [versioned_metadata],
        inputs = [
            versioned_metadata_raw,
        ],
        executable = ctx.executable._validate_output_metadata,
        arguments = [
            versioned_metadata_raw.path,
            versioned_metadata.path,
        ],
        progress_message = "%{label}: validating output metadata",
    )

    output = ctx.actions.declare_file(ctx.label.name + ".json")
    ctx.actions.run(
        outputs = [output],
        inputs = [versioned, versioned_metadata] + lint_ok,
        executable = "cp",
        arguments = [versioned.path, output.path],
        progress_message = "%{label}: generating output",
    )

    output_metadata = ctx.actions.declare_file(ctx.label.name + "_metadata.json")
    ctx.actions.run(
        outputs = [output_metadata],
        inputs = [output, versioned_metadata],
        executable = "cp",
        arguments = [versioned_metadata.path, output_metadata.path],
        progress_message = "%{label}: generating metadata",
    )

    data = depset(
        ctx.attr.data + ctx.attr.images,
        transitive = [dep[MdFileInfo].data for dep in ctx.attr.deps[MdGroupInfo].deps],
    )
    return [
        DefaultInfo(files = depset([output, output_metadata])),
        MdFileInfo(
            name = ctx.label.name,
            output = output,
            metadata = output_metadata,
            data = data,
        ),
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
        "version_file": attr.label(
            allow_single_file = [".json"],
            doc = "File with version info.",
        ),
        "version_override": attr.string(
            default = "",
            doc = "Set the document version to this value, rather than the computed value. Should only be used for testing.",
        ),
        "repo_override": attr.string(
            default = "",
            doc = "Set the document repo to this value, rather than the computed value. Should only be used for testing.",
        ),
        "_pandoc": attr.label(
            default = "//markdown/private/external:pandoc",
            executable = True,
            cfg = "exec",
        ),
        "_markdownlint": attr.label(
            default = "//markdown/private/external:markdownlint",
            executable = True,
            cfg = "exec",
        ),
        "_markdownlint_config": attr.label(
            allow_single_file = True,
            default = "//markdown/private/core/lint:default_markdownlintrc",
        ),
        "_standard_lint": attr.label(
            default = "//markdown/private/core/lint:standard_lint",
            executable = True,
            cfg = "exec",
        ),
        "_custom_lint": attr.label(
            default = "//markdown/private/core/lint:custom_lint",
            executable = True,
            cfg = "exec",
        ),
        "_gen_dictionary": attr.label(
            default = "//markdown/private/core/spelling:gen_dictionary",
            executable = True,
            cfg = "exec",
        ),
        "_spellcheck_input_template": attr.label(
            allow_single_file = True,
            default = "//markdown/private/core/spelling:spellcheck_input_template.md",
        ),
        "_spellcheck_filter": attr.label(
            allow_single_file = True,
            default = "//markdown/private/core/spelling:spellcheck_filter.lua",
        ),
        "_hunspell": attr.label(
            default = "//markdown/private/external:hunspell",
            executable = True,
            cfg = "exec",
        ),
        "_hunspell_dicts": attr.label(
            default = "//markdown/private/external:hunspell_dicts",
        ),
        "_locale_archive": attr.label(
            allow_single_file = True,
            default = "//markdown/private/external:locale_archive",
        ),
        "_spellcheck": attr.label(
            default = "//markdown/private/core/spelling:spellcheck",
            executable = True,
            cfg = "exec",
        ),
        "_preprocess": attr.label(
            default = "//markdown/private/core:preprocess",
            executable = True,
            cfg = "exec",
        ),
        "_validate_ids": attr.label(
            allow_single_file = True,
            default = "//markdown/private/core/filters:validate_ids.lua",
        ),
        "_spellcheck_cleanup": attr.label(
            allow_single_file = True,
            default = "//markdown/private/core/spelling:spellcheck_cleanup.lua",
        ),
        "_validate_quotes": attr.label(
            allow_single_file = True,
            default = "//markdown/private/core/filters:validate_quotes.lua",
        ),
        "_include": attr.label(
            allow_single_file = True,
            default = "//markdown/private/core/filters:include.lua",
        ),
        "_paragraph_annotations": attr.label(
            allow_single_file = True,
            default = "//markdown/private/core/filters:paragraph_annotations.lua",
        ),
        "_header_auto_ids": attr.label(
            allow_single_file = True,
            default = "//markdown/private/core/filters:header_auto_ids.lua",
        ),
        "_wordcount": attr.label(
            allow_single_file = True,
            default = "//markdown/private/core/filters:wordcount.lua",
        ),
        "_poetry_lines": attr.label(
            allow_single_file = True,
            default = "//markdown/private/core/filters:poetry_lines.lua",
        ),
        "_validate_input_metadata": attr.label(
            default = "//markdown/private/core:validate_input_metadata",
            executable = True,
            cfg = "exec",
        ),
        "_raw_version": attr.label(
            default = "//markdown/private/core:raw_version",
            executable = True,
            cfg = "exec",
        ),
        "_version": attr.label(
            default = "//markdown/private/core:version",
            executable = True,
            cfg = "exec",
        ),
        "_source_hash": attr.label(
            default = "//markdown/private/core:source_hash",
            executable = True,
            cfg = "exec",
        ),
        "_parse_date": attr.label(
            default = "//markdown/private/core:parse_date",
            executable = True,
            cfg = "exec",
        ),
        "_write_metadata": attr.label(
            allow_single_file = True,
            default = "//markdown/private/core/filters:write_metadata.lua",
        ),
        "_validate_output_metadata": attr.label(
            default = "//markdown/private/core:validate_output_metadata",
            executable = True,
            cfg = "exec",
        ),
    },
)
