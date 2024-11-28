"""Hunspell dicts dependency."""

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

def _hunspell_dicts_impl(module_ctx):
    http_archive(
        name = "hunspell_dicts",
        build_file = "//markdown/private/external:hunspell_dicts.build",
        url = "https://github.com/LibreOffice/dictionaries/archive/refs/tags/libreoffice-24.8.3.2.tar.gz",
        sha256 = "87d9cc02974c9eb2c0b95e00f9df18c7ea698b99fe0fc191c672640759d170da",
        strip_prefix = "dictionaries-libreoffice-24.8.3.2",
    )

    return module_ctx.extension_metadata(reproducible = True)

hunspell_dicts = module_extension(
    implementation = _hunspell_dicts_impl,
)
