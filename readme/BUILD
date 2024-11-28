load("@bazel_skylib//:bzl_library.bzl", "bzl_library")
load("@markdown//:defs.bzl", "md_document")
load("//markdown/private/support/shell:defs.bzl", "sh_source")
load("//markdown/testing:defs.bzl", "output_test")
load("defs.bzl", "wrap_file")

bzl_library(
    name = "defs_bzl",
    srcs = ["defs.bzl"],
    visibility = ["//visibility:private"],
    deps = [
        "//markdown/private/utils:defs_bzl",
        "@markdown//:defs.bzl",
    ],
)

sh_source(
    name = "version",
    src = "version.sh",
    visibility = ["//visibility:private"],
)

wrap_file(
    name = "bazeliskrc",
    src = "//markdown/private/workspace:default_bazeliskrc",
    language = "text",
)

wrap_file(
    name = "bazelrc",
    src = "//markdown/private/workspace:default_bazelrc",
    language = "text",
)

wrap_file(
    name = "root_build",
    src = "root.build",
    language = "starlark",
)

wrap_file(
    name = "basic_build",
    src = "basic.build",
    language = "starlark",
)

genrule(
    name = "module_bazel_src",
    srcs = [
        "//:MODULE.bazel",
        "module_template.bazel",
    ],
    outs = ["module.bazel"],
    cmd = "$(rootpath version.sh) $(SRCS) >$@",
    tools = ["version.sh"],
    visibility = ["//visibility:private"],
)

wrap_file(
    name = "module_bazel",
    src = "module.bazel",
    language = "starlark",
)

wrap_file(
    name = "spelling_build",
    src = "spelling.build",
    language = "starlark",
)

wrap_file(
    name = "include_build",
    src = "include.build",
    language = "starlark",
)

wrap_file(
    name = "collection_build",
    src = "collection.build",
    language = "starlark",
)

md_document(
    name = "readme",
    repo_override = "reproducible",
    timestamp_override = "1618243321",
    version_override = "reproducible",
    deps = [
        ":basic_build",
        ":bazeliskrc",
        ":bazelrc",
        ":collection_build",
        ":include_build",
        ":module_bazel",
        ":root_build",
        ":spelling_build",
    ],
)

genrule(
    name = "readme_generated",
    srcs = ["output/readme.plain.md"],
    outs = ["readme_generated.md"],
    cmd = "sed 's/\\\\!/!/g' <$< | sed 's/\\\\\\$$/\\$$/g' | sed 's/ / /g' >$@",
    visibility = ["//:__pkg__"],
)

output_test(
    # It is reproducible, but we don't need diff tests since we use
    # requried_files elsewhere.
    reproducible = False,
    target = "readme",
)
