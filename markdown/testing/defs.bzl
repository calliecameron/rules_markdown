"""Test utils."""

load("@bazel_skylib//lib:shell.bzl", "shell")
load(
    "//markdown/private/formats:defs.bzl",
    "ext_var_dot",
    "ext_var_underscore",
)

visibility("public")

def _dump_test(target, extension, variant, tool, tool_target = None, tool_helper_targets = None, tool_helper_args = None):
    native.sh_test(
        name = "%s_%s_dump_test" % (target, ext_var_underscore(extension, variant)),
        srcs = [Label("//markdown/testing:dump_test.sh")],
        data = [
                   "output/%s.%s" % (target, ext_var_dot(extension, variant)),
               ] + ([tool_target] if tool_target else []) +
               (tool_helper_targets if tool_helper_targets else []),
        args = [
            "$(rootpath output/%s.%s)" % (target, ext_var_dot(extension, variant)),
            tool,
        ] + (tool_helper_args if tool_helper_args else []),
    )

def _diff_test(target, extension, variant, tool, tool_target = None, tool_helper_targets = None, tool_helper_args = None):
    native.sh_test(
        name = "%s_%s_diff_test" % (target, ext_var_underscore(extension, variant)),
        srcs = [Label("//markdown/testing:diff_test.sh")],
        data = [
                   "output/%s.%s" % (target, ext_var_dot(extension, variant)),
                   "saved/%s.%s" % (target, ext_var_dot(extension, variant)),
               ] + ([tool_target] if tool_target else []) +
               (tool_helper_targets if tool_helper_targets else []),
        args = [
            "$(rootpath output/%s.%s)" % (target, ext_var_dot(extension, variant)),
            "$(rootpath saved/%s.%s)" % (target, ext_var_dot(extension, variant)),
            tool,
        ] + (tool_helper_args if tool_helper_args else []),
    )

def _zip_cleaned_test(target, extension, variant):
    native.sh_test(
        name = "%s_%s_zip_cleaned_test" % (target, ext_var_underscore(extension, variant)),
        srcs = [Label("//markdown/testing:zip_cleaned_test.sh")],
        data = [
            "output/%s.%s" % (target, ext_var_dot(extension, variant)),
            Label("//markdown/private/external:zipinfo"),
        ],
        args = [
            "$(rootpath output/%s.%s)" % (target, ext_var_dot(extension, variant)),
            "$(rootpath %s)" % Label("//markdown/private/external:zipinfo"),
        ],
    )

def _version_test(target, version_regex):
    native.sh_test(
        name = "%s_version_test" % target,
        srcs = [Label("//markdown/testing:version_test.sh")],
        data = [
            "output/%s.%s" % (target, ext_var_dot("json", "metadata")),
        ],
        args = [
            "$(rootpath output/%s.%s)" % (target, ext_var_dot("json", "metadata")),
            shell.quote(version_regex),
        ],
    )

def _dump_and_diff_tests(target, extension, variant, reproducible, tool, tool_target = None, tool_helper_targets = None, tool_helper_args = None):
    _dump_test(
        target,
        extension,
        variant,
        tool,
        tool_target,
        tool_helper_targets,
        tool_helper_args,
    )
    if reproducible:
        _diff_test(
            target,
            extension,
            variant,
            tool,
            tool_target,
            tool_helper_targets,
            tool_helper_args,
        )

def _cat_tests(target, extension, variant, reproducible):
    _dump_and_diff_tests(
        target,
        extension,
        variant,
        reproducible,
        "cat",
    )

def _zip_tests(target, extension, variant, reproducible):
    _dump_and_diff_tests(
        target,
        extension,
        variant,
        reproducible,
        "$(rootpath %s)" % Label("//markdown/private/utils:zipdump"),
        Label("//markdown/private/utils:zipdump"),
        [Label("//markdown/private/external:unzip")],
        ["$(rootpath %s)" % Label("//markdown/private/external:unzip")],
    )
    _zip_cleaned_test(
        target,
        extension,
        variant,
    )

def _pdf_tests(target, extension, variant, reproducible):
    _dump_and_diff_tests(
        target,
        extension,
        variant,
        reproducible,
        "$(rootpath %s)" % Label("//markdown/private/utils:pdfdump"),
        Label("//markdown/private/utils:pdfdump"),
        [
            Label("//markdown/private/external:pdfinfo"),
            Label("//markdown/private/utils:pdf2txt"),
        ],
        [
            "$(rootpath %s)" % Label("//markdown/private/external:pdfinfo"),
            "$(rootpath %s)" % Label("//markdown/private/utils:pdf2txt"),
        ],
    )

def _bin_tests(target, extension, variant):
    # Always nondeterministic
    _dump_test(
        target,
        extension,
        variant,
        "$(rootpath %s)" % Label("//markdown/private/utils:bindump"),
        Label("//markdown/private/utils:bindump"),
        [Label("//markdown/private/external:hexdump")],
        ["$(rootpath %s)" % Label("//markdown/private/external:hexdump")],
    )

def _doc_tests(target, extension, variant):
    # Always nondeterministic
    _dump_test(
        target,
        extension,
        variant,
        "$(rootpath %s)" % Label("//markdown/private/utils:docdump"),
        Label("//markdown/private/utils:docdump"),
    )

def output_test(target, reproducible, name = None):  # buildifier: disable=unused-variable
    """Test the target's outputs.

    Args:
        target: name of the output.
        reproducible: whether target has reproducible output, and can use golden tests.
        name: unused.
    """
    _cat_tests(target, "md", None, reproducible)
    _cat_tests(target, "md", "plain", reproducible)
    _cat_tests(target, "md", "tumblr", reproducible)
    _cat_tests(target, "txt", None, reproducible)
    _cat_tests(target, "html", None, reproducible)

    _cat_tests(target, "tex", None, reproducible)
    _pdf_tests(target, "pdf", None, reproducible)

    _zip_tests(target, "epub", None, reproducible)
    _bin_tests(target, "mobi", None)

    _zip_tests(target, "odt", None, reproducible)
    _zip_tests(target, "docx", None, reproducible)
    _doc_tests(target, "doc", None)

    _zip_tests(target, "docx", "shunnmodern", reproducible)

    _cat_tests(target, "json", "metadata", reproducible)
    _cat_tests(target, "json", "deps_metadata", reproducible)

def versioned_test(target, name = None):  # buildifier: disable=unused-variable
    _version_test(target, "[0-9a-f]+(-dirty)?, [0-9]{4}-[0-9]{2}-[0-9]{2} [0-9]{2}:[0-9]{2}:[0-9]{2}\\+00:00(, dirty deps)?")

def unversioned_test(target, name = None):  # buildifier: disable=unused-variable
    _version_test(target, "unversioned")
