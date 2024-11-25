import os
import os.path
import subprocess
from collections.abc import Sequence

from markdown.private.utils import test_utils

GOOD = """Foo bar.

!include %s

!include %s

An image ![foo](%s "bar"){.baz} goes here.
"""


class TestPreprocess(test_utils.ScriptTestCase):
    def run_script(  # type: ignore[override]
        self,
        content: str,
        current_package: str,
        deps: Sequence[tuple[str, str]],
        images: Sequence[tuple[str, str]],
    ) -> str:
        in_file = os.path.join(self.tmpdir(), "in.md")
        self.dump_file(in_file, content)

        out_file = os.path.join(self.tmpdir(), "out.md")

        dep_args = []
        for dep, file in deps:
            dep_args += ["--dep", dep + "=" + file]

        image_args = []
        for image, file in images:
            image_args += ["--image", image + "=" + file]

        super().run_script(
            args=[
                in_file,
                out_file,
                current_package,
                *dep_args,
                *image_args,
            ],
        )

        return self.load_file(out_file)

    def test_main(self) -> None:
        output = self.run_script(
            GOOD % (":bar", "//baz:quux", ":foo"),
            "a",
            [("a:bar", "a/bar.json"), ("baz:quux", "baz/quux.json")],
            [("a:foo", "a/foo.jpg")],
        )
        self.assertEqual(output, GOOD % ("a/bar.json", "baz/quux.json", "a/foo.jpg"))

    def test_main_root_package(self) -> None:
        output = self.run_script(
            GOOD % (":bar", "//baz:quux", ":foo"),
            "",
            [(":bar", "bar.json"), ("baz:quux", "baz/quux.json")],
            [(":foo", "foo.jpg")],
        )
        self.assertEqual(output, GOOD % ("bar.json", "baz/quux.json", "foo.jpg"))

    def test_main_fails(self) -> None:
        with self.assertRaises(subprocess.CalledProcessError):
            self.run_script("!include", "a", [], [])


if __name__ == "__main__":
    test_utils.ScriptTestCase.main()
