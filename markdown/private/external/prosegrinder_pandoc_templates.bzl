"""Prosegrinder pandoc templates dependency."""

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

visibility("private")

def _prosegrinder_pandoc_templates_impl(module_ctx):
    http_archive(
        name = "prosegrinder_pandoc_templates",
        build_file = "//markdown/private/external:prosegrinder_pandoc_templates.build",
        url = "https://github.com/prosegrinder/pandoc-templates/archive/71d6e9e47ec142a763fff1398b468ba1a008ced0.zip",
        sha256 = "7978b6122ecf7c751c2f1a54f0fcb500f2fa982cc400653a6d77fe88489471f8",
        strip_prefix = "pandoc-templates-71d6e9e47ec142a763fff1398b468ba1a008ced0",
        patches = ["//markdown/private/external:md2short.sh.patch"],
    )

    return module_ctx.extension_metadata(reproducible = True)

prosegrinder_pandoc_templates = module_extension(
    implementation = _prosegrinder_pandoc_templates_impl,
)
