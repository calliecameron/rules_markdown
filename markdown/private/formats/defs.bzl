"""Misc output formats."""

load(
    "//markdown/private/formats/ebook:defs.bzl",
    _md_epub = "md_epub",
    _md_mobi = "md_mobi",
)
load(
    "//markdown/private/formats/html:defs.bzl",
    _md_html = "md_html",
)
load(
    "//markdown/private/formats/latex:defs.bzl",
    _md_pdf = "md_pdf",
    _md_tex = "md_tex",
    _md_tex_intermediate = "md_tex_intermediate",
)
load(
    "//markdown/private/formats/metadata:defs.bzl",
    _md_deps_metadata_json = "md_deps_metadata_json",
    _md_metadata_json = "md_metadata_json",
)
load(
    "//markdown/private/formats/text:defs.bzl",
    _md_md = "md_md",
    _md_plain_md = "md_plain_md",
    _md_tumblr_md = "md_tumblr_md",
    _md_txt = "md_txt",
)
load(
    "//markdown/private/formats/word:defs.bzl",
    _md_doc = "md_doc",
    _md_docx = "md_docx",
    _md_odt = "md_odt",
    _md_shunnmodern_docx = "md_shunnmodern_docx",
)
load(
    ":lib.bzl",
    _ext_var_dot = "ext_var_dot",
    _ext_var_underscore = "ext_var_underscore",
)

visibility(["//markdown/private", "//markdown/testing"])

md_md = _md_md
md_plain_md = _md_plain_md
md_tumblr_md = _md_tumblr_md
md_txt = _md_txt
md_html = _md_html
md_epub = _md_epub
md_mobi = _md_mobi
md_pdf = _md_pdf
md_tex = _md_tex
md_tex_intermediate = _md_tex_intermediate
md_doc = _md_doc
md_docx = _md_docx
md_shunnmodern_docx = _md_shunnmodern_docx
md_odt = _md_odt
md_metadata_json = _md_metadata_json
md_deps_metadata_json = _md_deps_metadata_json

ext_var_underscore = _ext_var_underscore
ext_var_dot = _ext_var_dot
