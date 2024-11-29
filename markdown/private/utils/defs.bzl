"""Utils macros."""

load("@bazel_skylib//lib:shell.bzl", "shell")
load("@pip//:requirements.bzl", "requirement")
load("//markdown/private/support/python:defs.bzl", "py_test")

visibility(["//markdown/private/...", "//readme", "//"])

def extend_file(name, src, out = None, prepend_lines = None, append_lines = None):
    """A file with lines prepended and appended.

    Args:
        name: name of the generated file
        src: existing file
        out: output filename
        prepend_lines: lines to prepend to src
        append_lines: lines to append to src
    """
    cmd = []
    out = out or name + ".txt"

    for line in prepend_lines or []:
        cmd.append("echo %s" % shell.quote(line))

    cmd.append("cat '$<'")

    for line in append_lines or []:
        cmd.append("echo %s" % shell.quote(line))

    native.genrule(
        name = name,
        srcs = [src],
        outs = [out],
        cmd = "( " + " && ".join(cmd) + " ) > '$@'",
        visibility = ["//visibility:private"],
    )

def _required_files_update(name, copy, create, extra_update):
    # without the inner quotes, sh_binary will discard this instead of passing
    # an empty arg
    args = [native.package_name() or "''"]
    data = []

    if extra_update:
        args += ["--extra_script", "$(rootpath %s)" % extra_update]
        data.append(extra_update)

    for src, dst, dst_mode in copy:
        args += ["--copy", "$(rootpath %s)" % src, dst, dst_mode]
        if src not in data:
            data.append(src)

    for src, dst, dst_mode in create:
        args += ["--create", "$(rootpath %s)" % src, dst, dst_mode]
        if src not in data:
            data.append(src)

    native.sh_binary(
        name = name + "_update",
        srcs = ["@rules_markdown//markdown/private/utils:required_files_update.sh"],
        args = args,
        data = data,
        visibility = ["//visibility:private"],
    )

def _required_files_test(name, check, check_mode_only, extra_check):
    # without the inner quotes, sh_binary will discard this instead of passing
    # an empty arg
    args = [native.package_name() or "''", "//%s:%s_update" % (native.package_name(), name)]
    data = []

    for src, dst, dst_mode in check:
        dsts = native.glob([dst])
        if not dsts:
            args += ["--missing_file", dst]
        elif len(dsts) != 1:
            fail("found multiple files matching", dst)
        else:
            args += ["--check", "$(rootpath %s)" % src, "$(rootpath %s)" % dst, dst_mode]
            if src not in data:
                data.append(src)
            data.append(dst)

    for _, dst, dst_mode in check_mode_only:
        dsts = native.glob([dst])
        if not dsts:
            args += ["--missing_file", dst]
        elif len(dsts) != 1:
            fail("found multiple files matching", dst)
        else:
            args += ["--check_mode_only", "$(rootpath %s)" % dst, dst_mode]
            data.append(dst)

    if extra_check:
        args += ["--extra_check", "$(rootpath %s)" % extra_check]
        data.append(extra_check)

    native.sh_test(
        name = name + "_test",
        srcs = ["@rules_markdown//markdown/private/utils:required_files_test.sh"],
        args = args,
        data = data,
        visibility = ["//visibility:private"],
    )

def required_files(name, copy = None, create = None, extra_check = None, extra_update = None):
    """A set of files that must exist in the source tree with specific contents.

    Creates a test to check that the files are correct, and a binary to update
    them.

    Args:
        name: name of the set of files.
        copy: list of (src, dst, dst_mode) tuples of files to copy; dst is
            relative to the package.
        create: list of (src, dst, dst_mode) tuples of files to create; dst is
            relative to the package.
        extra_check: extra script to run in the test
        extra_update: extra script to run in the update
    """
    copy = copy or []
    create = create or []

    if not copy and not create:
        fail("no files specified")

    for f in copy + create:
        if len(f) != 3:
            fail("each entry in copy and create must be a tuple (src, dst, dst_mode); got", f)

    _required_files_update(
        name = name,
        copy = copy,
        create = create,
        extra_update = extra_update,
    )
    _required_files_test(
        name = name,
        check = copy,
        check_mode_only = create,
        extra_check = extra_check,
    )

def script_py_test(name, src, script, deps = None, data = None, args = None):
    deps = deps or []
    data = data or []
    args = args or []
    py_test(
        name = name,
        srcs = [src],
        args = ["$(rootpath %s)" % script] + args,
        data = [script] + data,
        deps = ["//markdown/private/utils:test_utils"] + deps,
    )

def pandoc_filter_py_test(name, src, filter):
    py_test(
        name = name,
        srcs = [src],
        args = [
            "$(rootpath //markdown/private/external:pandoc)",
            "$(rootpath %s)" % filter,
        ],
        data = [
            filter,
            "//markdown/private/external:pandoc",
        ],
        deps = [
            "//markdown/private/utils:test_utils",
            requirement("panflute"),
        ],
    )
