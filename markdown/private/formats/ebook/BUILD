load("//tools/shell:defs.bzl", "sh_binary")

exports_files(
    [
        "epub.css",
    ],
    visibility = ["//visibility:private"],
)

sh_binary(
    name = "ebook_convert",
    srcs = ["ebook_convert.sh"],
    args = ["$(rootpath //tools/external:ebook_convert)"],
    data = ["//tools/external:ebook_convert"],
    visibility = ["//markdown/private/formats:__pkg__"],
)
