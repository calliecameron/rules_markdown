"""Hunspell dicts dependency."""

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

visibility("private")

def _hunspell_dicts_impl(module_ctx):
    http_archive(
        name = "hunspell_dicts",
        build_file = "//markdown/private/external:hunspell_dicts.build",
        url = "https://github.com/LibreOffice/dictionaries/archive/refs/tags/libreoffice-25.2.3.2.tar.gz",
        sha256 = "179508f52a2bcc6517ba8dd8250f614625168a2e6ce050d180fa9144ef3c8689",
        strip_prefix = "dictionaries-libreoffice-25.2.3.2",
    )

    return module_ctx.extension_metadata(reproducible = True)

hunspell_dicts = module_extension(
    implementation = _hunspell_dicts_impl,
)
