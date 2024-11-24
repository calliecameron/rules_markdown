import os
import os.path
import subprocess

from markdown.private.utils import test_utils

GOOD = """Foo bar.

\\“Lots \\”of \\‘quotes\\’.

Some -- dashes---
"""  # noqa: RUF001


class TestCustomLint(test_utils.ScriptTestCase):
    def run_script(  # type: ignore[override]
        self,
        content: str,
    ) -> None:
        in_file = os.path.join(self.tmpdir(), "in.md")
        self.dump_file(in_file, content)

        out_file = os.path.join(self.tmpdir(), "out.txt")

        super().run_script(
            args=[
                in_file,
                out_file,
            ],
        )

        self.assertEqual(self.load_file(out_file), "OK\n")

    def test_main(self) -> None:
        self.run_script(GOOD)

    def test_main_fails(self) -> None:
        with self.assertRaises(subprocess.CalledProcessError):
            self.run_script("“")


if __name__ == "__main__":
    test_utils.ScriptTestCase.main()
