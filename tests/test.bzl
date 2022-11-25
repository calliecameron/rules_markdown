"""Test utils."""

def _diff_test(target, ext, tool, tool_target = None):
    native.sh_test(
        name = "%s_%s_diff_test" % (target, ext.replace(".", "_")),
        srcs = ["//tests:diff_test.sh"],
        data = [
            "output/%s.%s" % (target, ext),
            "saved/%s.%s" % (target, ext),
        ] + ([tool_target] if tool_target else []),
        args = [
            "$(rootpath output/%s.%s)" % (target, ext),
            "$(rootpath saved/%s.%s)" % (target, ext),
            tool,
        ],
    )

def diff_test(target, name = None):  # buildifier: disable=unused-variable
    _diff_test(target, "md", "cat")
    _diff_test(target, "txt", "cat")
    _diff_test(target, "tex", "cat")
    _diff_test(target, "pdf", "$(rootpath //utils:pdfdump)", "//utils:pdfdump")
