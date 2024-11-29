"""Aggregation rules."""

load("//markdown/private/core:defs.bzl", "MdGroupInfo")

visibility(["//markdown/private", "//markdown/private/summary"])

def _md_group_summary_impl(ctx):
    script = ctx.actions.declare_file(ctx.label.name + ".sh")
    ctx.actions.run(
        outputs = [script],
        executable = ctx.executable._write_group_summary_script,
        arguments = [
            ctx.workspace_name,
            ctx.attr.deps[MdGroupInfo].metadata.short_path,
            ctx.executable._group_summary.short_path,
            script.path,
        ],
        progress_message = "%{label}: generating summary script",
    )

    return [
        DefaultInfo(
            files = depset([script]),
            runfiles = ctx.runfiles(
                files = [ctx.attr.deps[MdGroupInfo].metadata],
                transitive_files = ctx.attr._group_summary[DefaultInfo].default_runfiles.files,
            ),
            executable = script,
        ),
    ]

md_group_summary = rule(
    implementation = _md_group_summary_impl,
    executable = True,
    doc = "md_group_summary summarises the contents of an md_group.",
    attrs = {
        "deps": attr.label(
            providers = [MdGroupInfo],
            doc = "md_group to summarise.",
        ),
        "_group_summary": attr.label(
            default = "//markdown/private/group:group_summary",
            executable = True,
            cfg = "exec",
        ),
        "_write_group_summary_script": attr.label(
            default = "//markdown/private/group:write_group_summary_script",
            executable = True,
            cfg = "exec",
        ),
    },
)

def _md_group_publications_impl(ctx):
    publications = ctx.actions.declare_file(ctx.label.name + ".html")
    ctx.actions.run(
        outputs = [publications],
        inputs = [ctx.attr.deps[MdGroupInfo].metadata],
        executable = ctx.executable._group_publications,
        arguments = [ctx.attr.deps[MdGroupInfo].metadata.path, publications.path],
        progress_message = "%{label}: generating publications",
    )

    script = ctx.actions.declare_file(ctx.label.name + ".sh")
    ctx.actions.run(
        outputs = [script],
        inputs = [publications],
        executable = ctx.executable._write_group_publications_script,
        arguments = [ctx.workspace_name, publications.short_path, script.path],
        progress_message = "%{label}: generating publications script",
    )

    return [
        DefaultInfo(
            files = depset([publications, script]),
            runfiles = ctx.runfiles(files = [publications]),
            executable = script,
        ),
    ]

md_group_publications = rule(
    implementation = _md_group_publications_impl,
    executable = True,
    doc = "md_group_publications displays the publications of an md_group.",
    attrs = {
        "deps": attr.label(
            providers = [MdGroupInfo],
            doc = "md_group to process.",
        ),
        "_group_publications": attr.label(
            default = "//markdown/private/group:group_publications",
            executable = True,
            cfg = "exec",
        ),
        "_write_group_publications_script": attr.label(
            default = "//markdown/private/group:write_group_publications_script",
            executable = True,
            cfg = "exec",
        ),
    },
)
