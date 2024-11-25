load("@bazel_skylib//:bzl_library.bzl", "bzl_library")
load("@crates//:defs.bzl", "all_crate_deps")
load("@pip//:requirements.bzl", "requirement")
load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_test")
load("//markdown/private/support/python:defs.bzl", "py_binary")
load("//markdown/private/utils:defs.bzl", "script_py_test")

bzl_library(
    name = "defs_bzl",
    srcs = ["defs.bzl"],
    visibility = ["//markdown/private:__subpackages__"],
)

rust_binary(
    name = "preprocess",
    srcs = ["preprocess.rs"],
    deps = [
        "//markdown/private/utils",
    ] + all_crate_deps(
        normal = True,
    ),
)

rust_test(
    name = "preprocess_rs_test",
    crate = ":preprocess",
)

script_py_test(
    name = "preprocess_test",
    src = "preprocess_test.py",
    script = ":preprocess",
)

rust_binary(
    name = "validate_input_metadata",
    srcs = ["validate_input_metadata.rs"],
    deps = [
        "//markdown/private/utils",
    ] + all_crate_deps(
        normal = True,
    ),
)

script_py_test(
    name = "validate_input_metadata_test",
    src = "validate_input_metadata_test.py",
    script = ":validate_input_metadata",
)

rust_binary(
    name = "source_hash",
    srcs = ["source_hash.rs"],
    deps = [
        "//markdown/private/utils",
    ] + all_crate_deps(
        normal = True,
    ),
)

script_py_test(
    name = "source_hash_test",
    src = "source_hash_test.py",
    script = ":source_hash",
)

rust_binary(
    name = "raw_version",
    srcs = ["raw_version.rs"],
    deps = [
        "//markdown/private/utils",
    ] + all_crate_deps(
        normal = True,
    ),
)

script_py_test(
    name = "raw_version_test",
    src = "raw_version_test.py",
    script = ":raw_version",
)

rust_binary(
    name = "version",
    srcs = ["version.rs"],
    deps = [
        "//markdown/private/utils",
    ] + all_crate_deps(
        normal = True,
    ),
)

rust_test(
    name = "version_rs_test",
    crate = ":version",
)

script_py_test(
    name = "version_test",
    src = "version_test.py",
    script = ":version",
)

py_binary(
    name = "parse_date",
    srcs = ["parse_date.py"],
    type_stub_deps = [
        requirement("types-dateparser"),
    ],
    deps = [
        requirement("dateparser"),
    ],
)

script_py_test(
    name = "parse_date_test",
    src = "parse_date_test.py",
    script = ":parse_date",
)

rust_binary(
    name = "validate_output_metadata",
    srcs = ["validate_output_metadata.rs"],
    deps = [
        "//markdown/private/utils",
    ] + all_crate_deps(
        normal = True,
    ),
)

script_py_test(
    name = "validate_output_metadata_test",
    src = "validate_output_metadata_test.py",
    script = ":validate_output_metadata",
)
