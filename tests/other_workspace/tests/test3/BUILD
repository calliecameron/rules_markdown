load("@markdown//:defs.bzl", "md_document", "md_file", "md_summary")
load("@rules_markdown//markdown/testing:defs.bzl", "output_test")

md_file(
    name = "include1",
    repo_override = "reproducible",
    version_override = "reproducible",
    deps = [":include2"],
)

md_file(
    name = "include2",
    repo_override = "reproducible",
    version_override = "reproducible",
)

md_document(
    name = "test3",
    data = ["custom-toc.tex"],
    extra_dictionaries = ["extra.dic"],
    extra_latex_flags = ["--include-in-header=$(rootpath custom-toc.tex)"],
    extra_pandoc_flags = [
        "--table-of-contents",
        "--toc-depth=1",
    ],
    repo_override = "reproducible",
    timestamp_override = "1618243321",
    version_override = "reproducible",
    deps = [":include1"],
)

output_test(
    reproducible = True,
    target = "test3",
)

md_document(
    name = "test3a",
    existing_file = ":test3",
    extra_latex_flags = ["--include-in-header=$(rootpath custom-toc.tex)"],
    extra_pandoc_flags = [
        "--table-of-contents",
        "--toc-depth=1",
    ],
    main_document = False,
    timestamp_override = "1618243321",
)

output_test(
    reproducible = True,
    target = "test3a",
)

md_summary()
