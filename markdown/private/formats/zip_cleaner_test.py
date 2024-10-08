import os
import os.path
import subprocess
import sys

from markdown.private.utils import test_utils

STRIP_NONDETERMINISM = ""
ZIPINFO = ""
ZIP = ""


class TestZipCleaner(test_utils.ScriptTestCase):
    def test_zip_cleaner(self) -> None:
        txt_file = os.path.join(self.tmpdir(), "in.txt")
        self.dump_file(txt_file, "foo\n")

        in_file = os.path.join(self.tmpdir(), "in.zip")

        subprocess.run(
            [
                ZIP,
                in_file,
                txt_file,
            ],
            check=True,
        )

        output = subprocess.run(
            [
                ZIPINFO,
                "-T",
                in_file,
            ],
            check=True,
            capture_output=True,
            encoding="utf-8",
        )
        self.assertNotIn("19800101", output.stdout)

        out_file = os.path.join(self.tmpdir(), "out.zip")

        self.run_script(
            args=[
                STRIP_NONDETERMINISM,
                in_file,
                out_file,
            ],
        )

        output = subprocess.run(
            [
                ZIPINFO,
                "-T",
                out_file,
            ],
            check=True,
            capture_output=True,
            encoding="utf-8",
        )
        self.assertIn("19800101", output.stdout)


if __name__ == "__main__":
    STRIP_NONDETERMINISM = sys.argv[1]
    del sys.argv[1]
    ZIPINFO = sys.argv[1]
    del sys.argv[1]
    ZIP = sys.argv[1]
    del sys.argv[1]
    test_utils.ScriptTestCase.main()