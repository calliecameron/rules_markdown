load("@markdown//:defs.bzl", "md_document")
load("@rules_markdown//markdown/testing:defs.bzl", "output_test")

md_document(
    name = "test2",
    extra_latex_flags = ["--variable=section-page-break"],
    images = ["image.jpg"],
    repo_override = "reproducible",
    timestamp_override = "1618243321",
    version_override = "reproducible",
)

output_test(
    reproducible = True,
    target = "test2",
)
