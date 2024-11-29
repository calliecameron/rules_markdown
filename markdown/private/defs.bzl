"""Public API of the module."""

load("@bazel_skylib//rules:build_test.bzl", "build_test")
load(
    "//markdown/private/collection:defs.bzl",
    _md_collection_src = "md_collection_src",
)
load(
    "//markdown/private/core:defs.bzl",
    _md_file = "md_file",
    _md_group = "md_group",
)
load(
    "//markdown/private/formats:defs.bzl",
    "ext_var_dot",
    "ext_var_underscore",
    _md_deps_metadata_json = "md_deps_metadata_json",
    _md_doc = "md_doc",
    _md_docx = "md_docx",
    _md_epub = "md_epub",
    _md_html = "md_html",
    _md_md = "md_md",
    _md_metadata_json = "md_metadata_json",
    _md_mobi = "md_mobi",
    _md_odt = "md_odt",
    _md_pdf = "md_pdf",
    _md_plain_md = "md_plain_md",
    _md_shunnmodern_docx = "md_shunnmodern_docx",
    _md_tex = "md_tex",
    _md_tex_intermediate = "md_tex_intermediate",
    _md_tumblr_md = "md_tumblr_md",
    _md_txt = "md_txt",
)
load(
    "//markdown/private/git:defs.bzl",
    _md_git_repo = "md_git_repo",
)
load(
    "//markdown/private/group:defs.bzl",
    _md_group_publications = "md_group_publications",
    _md_group_summary = "md_group_summary",
)
load(
    "//markdown/private/summary:defs.bzl",
    _md_summary = "md_summary",
)
load(
    "//markdown/private/workspace:defs.bzl",
    _md_workspace = "md_workspace",
)

visibility("public")

_FORMATS = [
    ("md", None),
    ("md", "plain"),
    ("md", "tumblr"),
    ("txt", None),
    ("html", None),
    ("tex", None),
    ("pdf", None),
    ("epub", None),
    ("mobi", None),
    ("odt", None),
    ("docx", None),
    ("doc", None),
    ("docx", "shunnmodern"),
    ("json", "metadata"),
    ("json", "deps_metadata"),
]

def _name(name, extension, variant):
    return name + "_" + ext_var_underscore(extension, variant)

def _output(name, extension, variant):
    return "output/%s.%s" % (name, ext_var_dot(extension, variant))

def md_file(
        name,
        src = None,
        deps = None,
        extra_dictionaries = None,
        data = None,
        images = None,
        increment_included_headers = False,
        version_file = None,
        version_override = "",
        repo_override = ""):
    """md_file represents a markdown source file.

    Args:
        name: the name of the file.
        src: the source file, if different from <name>.md.
        deps: other md_file targets used in !include statements in src.
        extra_dictionaries: extra dictionaries for spellchecking.
        data: data dependencies.
        images: image dependencies.
        increment_included_headers: if true, header level in included files is
            incremented, e.g. level 1 headers become level 2 headers. If false,
            headers are unchanged.
        version_file: file with version info.
        version_override: set the document version to this value, rather than
           the computed value. Should only be used for testing.
        repo_override: set the document repo to this value, rather than
           the computed value. Should only be used for testing.
    """
    if not src:
        src = name + ".md"
    extra_dictionaries = extra_dictionaries or []

    md_group(
        name = name + "_deps",
        deps = deps,
    )

    _md_file(
        name = name,
        src = src,
        deps = name + "_deps",
        dictionaries = native.glob([name + ".dic"]) + extra_dictionaries,
        data = data,
        images = images,
        increment_included_headers = increment_included_headers,
        version_file = version_file,
        version_override = version_override,
        repo_override = repo_override,
        visibility = ["//visibility:public"],
    )

    build_test(
        name = name + "_test",
        targets = [name],
    )

def md_document(
        name,
        src = None,
        deps = None,
        extra_dictionaries = None,
        data = None,
        images = None,
        increment_included_headers = False,
        extra_pandoc_flags = None,
        extra_latex_flags = None,
        version_file = None,
        version_override = None,
        repo_override = None,
        timestamp_override = None,
        cache_dir = None,
        existing_file = None,
        main_document = True,
        output_visibility = None):
    """md_document compiles a markdown source file into many formats.

    Args:
        name: the name of the document.
        src: the source file, if different from <name>.md.
        deps: other md_file targets used in !include statements in src.
        extra_dictionaries: extra dictionaries for spellchecking.
        data: data dependencies.
        images: image dependencies.
        increment_included_headers: if true, header level in included files is
            incremented, e.g. level 1 headers become level 2 headers. If false,
            headers are unchanged.
        extra_pandoc_flags: extra flags to pass to pandoc.
        extra_latex_flags: extra flags to pass to pandoc for latex-based
            formats.
        version_file: file with version info.
        version_override: set the document version to this value, rather than
            the computed value. Should only be used for testing.
        repo_override: set the document repo to this value, rather than
            the computed value. Should only be used for testing.
        timestamp_override: set the build timestamp to this value, rather than
            the current value. Should only be used for testing.
        cache_dir: persistent writable cache dir.
        existing_file: use an existing md_file rather than creating one; if
            set, most other args must not be set.
        main_document: whether this is the main document in the package; creates
            some convenience aliases.
        output_visibility: visibilty of the generated outputs.
    """
    if existing_file:
        if src or deps or extra_dictionaries or data or images or increment_included_headers or version_override or repo_override:
            native.fail("Other args must not be set when existing_file is set")
        file = existing_file
    else:
        md_file(
            name = name,
            src = src,
            deps = deps,
            extra_dictionaries = extra_dictionaries,
            data = data,
            images = images,
            increment_included_headers = increment_included_headers,
            version_file = version_file,
            version_override = version_override,
            repo_override = repo_override,
        )
        file = name

    output_visibility = output_visibility or ["//visibility:private"]
    extra_pandoc_flags = extra_pandoc_flags or []
    extra_latex_flags = extra_latex_flags or []

    _md_md(
        name = _name(name, "md", None),
        file = file,
        extra_pandoc_flags = extra_pandoc_flags,
        out = _output(name, "md", None),
        visibility = output_visibility,
    )
    _md_plain_md(
        name = _name(name, "md", "plain"),
        file = file,
        extra_pandoc_flags = extra_pandoc_flags,
        out = _output(name, "md", "plain"),
        visibility = output_visibility,
    )
    _md_tumblr_md(
        name = _name(name, "md", "tumblr"),
        file = file,
        extra_pandoc_flags = extra_pandoc_flags,
        out = _output(name, "md", "tumblr"),
        visibility = output_visibility,
    )
    _md_txt(
        name = _name(name, "txt", None),
        file = file,
        extra_pandoc_flags = extra_pandoc_flags,
        out = _output(name, "txt", None),
        visibility = output_visibility,
    )
    _md_html(
        name = _name(name, "html", None),
        file = file,
        extra_pandoc_flags = extra_pandoc_flags,
        out = _output(name, "html", None),
        visibility = output_visibility,
    )
    _md_tex_intermediate(
        name = _name(name, "tex_intermediate", None),
        file = file,
        extra_pandoc_flags = extra_pandoc_flags + extra_latex_flags,
        visibility = ["//visibility:private"],
    )
    _md_tex(
        name = _name(name, "tex", None),
        intermediate = _name(name, "tex_intermediate", None),
        extra_pandoc_flags = extra_pandoc_flags + extra_latex_flags,
        out = _output(name, "tex", None),
        timestamp_override = timestamp_override,
        visibility = output_visibility,
    )
    _md_pdf(
        name = _name(name, "pdf", None),
        intermediate = _name(name, "tex_intermediate", None),
        extra_pandoc_flags = extra_pandoc_flags + extra_latex_flags,
        out = _output(name, "pdf", None),
        timestamp_override = timestamp_override,
        cache_dir = cache_dir,
        visibility = output_visibility,
    )
    _md_epub(
        name = _name(name, "epub", None),
        file = file,
        extra_pandoc_flags = extra_pandoc_flags,
        out = _output(name, "epub", None),
        timestamp_override = timestamp_override,
        visibility = output_visibility,
    )
    _md_mobi(
        name = _name(name, "mobi", None),
        epub = _name(name, "epub", None),
        out = _output(name, "mobi", None),
        visibility = output_visibility,
    )
    _md_odt(
        name = _name(name, "odt", None),
        file = file,
        extra_pandoc_flags = extra_pandoc_flags,
        out = _output(name, "odt", None),
        timestamp_override = timestamp_override,
        visibility = output_visibility,
    )
    _md_docx(
        name = _name(name, "docx", None),
        file = file,
        extra_pandoc_flags = extra_pandoc_flags,
        out = _output(name, "docx", None),
        timestamp_override = timestamp_override,
        visibility = output_visibility,
    )
    _md_doc(
        name = _name(name, "doc", None),
        docx = _name(name, "docx", None),
        out = _output(name, "doc", None),
        visibility = output_visibility,
    )
    _md_shunnmodern_docx(
        name = _name(name, "docx", "shunnmodern"),
        file = file,
        out = _output(name, "docx", "shunnmodern"),
        timestamp_override = timestamp_override,
        visibility = output_visibility,
    )
    _md_metadata_json(
        name = _name(name, "json", "metadata"),
        file = file,
        out = _output(name, "json", "metadata"),
        visibility = output_visibility,
    )
    _md_deps_metadata_json(
        name = _name(name, "json", "deps_metadata"),
        group = file + "_deps",
        out = _output(name, "json", "deps_metadata"),
        visibility = output_visibility,
    )

    native.filegroup(
        name = name + "_all",
        srcs = [_name(name, ext, var) for (ext, var) in _FORMATS],
        data = [_name(name, ext, var) for (ext, var) in _FORMATS],
        visibility = ["//visibility:private"],
    )

    native.genrule(
        name = name + "_save_sh",
        outs = [name + "_save.sh"],
        cmd = "$(location @rules_markdown//markdown/private/formats:write_save_script) $@ %s" % native.package_name(),
        tools = ["@rules_markdown//markdown/private/formats:write_save_script"],
        visibility = ["//visibility:private"],
    )

    native.sh_binary(
        name = name + "_save",
        srcs = [name + "_save.sh"],
        data = [name + "_all"],
        visibility = ["//visibility:private"],
    )

    if main_document:
        for (ext, var) in _FORMATS:
            native.alias(
                name = ext_var_underscore(ext, var),
                actual = _name(name, ext, var),
                visibility = ["//visibility:private"],
            )
        native.alias(
            name = "save",
            actual = name + "_save",
            visibility = ["//visibility:private"],
        )
        native.alias(
            name = "deps_summary",
            actual = name + "_deps_summary",
            visibility = ["//visibility:private"],
        )
        native.alias(
            name = "deps_publications",
            actual = name + "_deps_publications",
            visibility = ["//visibility:private"],
        )

def md_collection(
        name,
        title,
        author,
        deps,
        date = None,
        extra_metadata = None,
        version_file = None,
        version_override = None,
        repo_override = None,
        timestamp_override = None,
        cache_dir = None,
        main_document = True,
        output_visibility = None):
    """md_collection collects multiple documents into a single document.

    Args:
        name: the name of the document.
        title: the title of the collection.
        author: the author of the collection.
        date: the date of the collection.
        deps: md_file targets to include in the collection.
        extra_metadata: a metadata file to include.
        version_file: file with version info.
        version_override: set the document version to this value, rather than
            the computed value. Should only be used for testing.
        repo_override: set the document repo to this value, rather than
            the computed value. Should only be used for testing.
        timestamp_override: set the build timestamp to this value, rather than
            the current value. Should only be used for testing.
        cache_dir: persistent writable cache dir.
        main_document: whether this is the main document in the package; creates
            some convenience aliases.
        output_visibility: visibilty of the generated outputs.
    """
    _md_collection_src(
        name = name + "_src",
        title = title,
        author = author,
        date = date,
        deps = name + "_deps",
    )

    md_document(
        name = name,
        src = name + "_src",
        deps = deps,
        data = [
            "@rules_markdown//markdown/private/collection:collection_header.tex",
            "@rules_markdown//markdown/private/collection:collection_before.tex",
        ] + ([extra_metadata] if extra_metadata else []),
        increment_included_headers = True,
        extra_pandoc_flags = [
            "--table-of-contents",
            "--toc-depth=1",
        ] + (["--metadata-file=$(rootpath %s)" % extra_metadata] if extra_metadata else []),
        extra_latex_flags = [
            "--variable=section-page-break",
            "--include-in-header=$(rootpath @rules_markdown//markdown/private/collection:collection_header.tex)",
            "--include-before-body=$(rootpath @rules_markdown//markdown/private/collection:collection_before.tex)",
        ],
        version_file = version_file,
        version_override = version_override,
        repo_override = repo_override,
        timestamp_override = timestamp_override,
        cache_dir = cache_dir,
        main_document = main_document,
        output_visibility = output_visibility,
    )

def md_group(name, deps):
    """md_group is a group of md_file targets.

    Args:
        name: the name of the group.
        deps: md_file targets to include in the group.
    """
    _md_group(
        name = name,
        deps = deps,
        visibility = ["//visibility:private"],
    )

    _md_group_summary(
        name = name + "_summary",
        deps = name,
        visibility = ["//visibility:private"],
    )

    _md_group_publications(
        name = name + "_publications",
        deps = name,
        visibility = ["//visibility:private"],
    )

md_summary = _md_summary
md_git_repo = _md_git_repo
md_workspace = _md_workspace
