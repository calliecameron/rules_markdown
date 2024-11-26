load("@bazel_skylib//:bzl_library.bzl", "bzl_library")
load("@crates//:defs.bzl", "aliases", "all_crate_deps")
load("@pip//:requirements.bzl", "requirement")
load("@rules_rust//rust:defs.bzl", "rust_library", "rust_test")
load("//markdown/private/support/python:defs.bzl", "py_binary", "py_library", "py_source", "py_test")
load("//markdown/private/support/shell:defs.bzl", "sh_binary", "sh_source", "sh_test")

bzl_library(
    name = "defs_bzl",
    srcs = ["defs.bzl"],
    visibility = ["//markdown/private:__subpackages__"],
)

bzl_library(
    name = "lib_bzl",
    srcs = ["lib.bzl"],
    visibility = ["//markdown/private/core:__pkg__"],
)

py_library(
    name = "test_utils",
    testonly = True,
    srcs = ["test_utils.py"],
    visibility = ["//markdown/private:__subpackages__"],
    deps = [requirement("panflute")],
)

py_library(
    name = "metadata",
    srcs = ["metadata.py"],
    visibility = ["//markdown/private:__subpackages__"],
    deps = [
        requirement("pydantic"),
        ":publications",
    ],
)

rust_library(
    name = "utils",
    srcs = [
        "args.rs",
        "bazel.rs",
        "deserializers.rs",
        "field_validators.rs",
        "json.rs",
        "metadata.rs",
        "problems.rs",
        "publications.rs",
        "utils.rs",
    ],
    aliases = aliases(),
    crate_name = "markdown",
    proc_macro_deps = all_crate_deps(
        proc_macro = True,
    ),
    visibility = ["//markdown/private:__subpackages__"],
    deps = all_crate_deps(
        normal = True,
    ),
)

rust_test(
    name = "utils_test",
    crate = ":utils",
)

py_test(
    name = "metadata_test",
    srcs = ["metadata_test.py"],
    deps = [":metadata"],
)

py_library(
    name = "publications",
    srcs = ["publications.py"],
    visibility = ["//markdown/private:__subpackages__"],
    deps = [requirement("pydantic")],
)

py_test(
    name = "publications_test",
    srcs = ["publications_test.py"],
    deps = [":publications"],
)

sh_binary(
    name = "bindump",
    srcs = ["bindump.sh"],
    args = [
        "$(rootpath //markdown/private/external:hexdump)",
    ],
    data = [
        "//markdown/private/external:hexdump",
    ],
    visibility = ["//visibility:public"],
)

sh_binary(
    name = "docdump",
    srcs = ["docdump.sh"],
    visibility = ["//visibility:public"],
)

py_binary(
    name = "pdf2txt",
    srcs = ["pdf2txt.py"],
    visibility = ["//visibility:public"],
    deps = [requirement("pdfminer.six")],
)

sh_binary(
    name = "pdfdump",
    srcs = ["pdfdump.sh"],
    args = [
        "$(rootpath //markdown/private/external:pdfinfo)",
        "$(rootpath pdf2txt)",
    ],
    data = [
        ":pdf2txt",
        "//markdown/private/external:pdfinfo",
    ],
    visibility = ["//visibility:public"],
)

sh_binary(
    name = "zipdump",
    srcs = ["zipdump.sh"],
    args = [
        "$(rootpath //markdown/private/external:unzip)",
    ],
    data = [
        "//markdown/private/external:unzip",
    ],
    visibility = ["//visibility:public"],
)

sh_source(
    name = "required_files_test",
    src = "required_files_test.sh",
    visibility = ["//visibility:public"],
)

sh_test(
    name = "required_files_test_test",
    srcs = ["required_files_test_test.sh"],
    args = ["$(rootpath required_files_test.sh)"],
    data = ["required_files_test.sh"],
)

sh_source(
    name = "required_files_update",
    src = "required_files_update.sh",
    visibility = ["//visibility:public"],
)

sh_test(
    name = "required_files_update_test",
    srcs = ["required_files_update_test.sh"],
    args = ["$(rootpath required_files_update.sh)"],
    data = ["required_files_update.sh"],
)

py_source(
    name = "git_repo_version",
    src = "git_repo_version",
    visibility = ["//visibility:public"],
)

sh_test(
    name = "git_repo_version_test",
    srcs = ["git_repo_version_test.sh"],
    args = ["$(rootpath git_repo_version)"],
    data = ["git_repo_version"],
)
