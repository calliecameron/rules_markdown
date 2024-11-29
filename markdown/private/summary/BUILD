load("//markdown/private/support/shell:defs.bzl", "sh_source")

exports_files(
    [
        "contents.build",
        "contents.bzl",
    ],
    visibility = ["//visibility:public"],
)

sh_source(
    name = "refresh",
    src = "refresh",
    visibility = ["//visibility:public"],
)

sh_source(
    name = "summarise_publications",
    src = "summarise_publications",
    visibility = ["//visibility:public"],
)

sh_source(
    name = "summarise_contents",
    src = "summarise_contents",
    visibility = ["//visibility:public"],
)
