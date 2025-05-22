"""Python rules."""

load("@bazel_skylib//rules:native_binary.bzl", "native_test")

visibility("//...")

def lua_source(name, src, visibility = None):
    native.exports_files(
        [src],
        visibility = visibility or ["//visibility:private"],
    )

    native_test(
        name = name + "_luacheck_test",
        src = "//markdown/private/external:luacheck",
        out = name + "_luacheck",
        args = [
            "--config=$(rootpath //:.luacheckrc)",
            "--no-cache",
            "$(location %s)" % src,
        ],
        data = [
            "//:.luacheckrc",
            src,
        ],
    )

    native_test(
        name = name + "_luaformat_test",
        src = "//tools/lua:lua_format_stub.sh",
        out = name + "_luaformat",
        args = [
            "$(rootpath //markdown/private/external:lua_format)",
            "$(location %s)" % src,
            "--config=$(rootpath //:.lua-format)",
        ],
        data = [
            "//markdown/private/external:lua_format",
            "//:.lua-format",
            src,
        ],
    )
