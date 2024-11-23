import os
import os.path
import subprocess
from collections.abc import Mapping

from markdown.private.utils import test_utils


class TestRawVersion(test_utils.ScriptTestCase):
    def run_script(self, version: Mapping[str, str], info: str) -> str:  # type: ignore[override]
        args = []

        if version:
            version_file = os.path.join(self.tmpdir(), "version.json")
            self.dump_json(version_file, version)
            args += ["--version-file", version_file]

        if info:
            info_file = os.path.join(self.tmpdir(), "info.txt")
            self.dump_file(info_file, info)
            args += ["--info-file", info_file]

        out_file = os.path.join(self.tmpdir(), "out.json")

        super().run_script(args=[*args, out_file])

        return self.load_file(out_file)

    def test_version_file(self) -> None:
        self.assertEqual(
            self.run_script(
                {
                    "version": "1",
                    "repo": "a/b",
                },
                "",
            ),
            """{
    "repo": "a/b",
    "version": "1"
}""",
        )

    def test_info_file_present(self) -> None:
        self.assertEqual(
            self.run_script(
                {},
                """
STABLE_WORKSPACE_PARENT_REPO a/b
STABLE_WORKSPACE_PARENT_VERSION 1
""",
            ),
            """{
    "repo": "a/b",
    "version": "1"
}""",
        )

    def test_info_file_missing(self) -> None:
        self.assertEqual(
            self.run_script(
                {},
                """
FOO 1
""",
            ),
            """{
    "repo": "unversioned",
    "version": "unversioned"
}""",
        )

    def test_neither(self) -> None:
        with self.assertRaises(subprocess.CalledProcessError):
            self.run_script({}, "")

    def test_both(self) -> None:
        with self.assertRaises(subprocess.CalledProcessError):
            self.run_script(
                {
                    "version": "1",
                    "repo": "a/b",
                },
                "a",
            )


if __name__ == "__main__":
    test_utils.ScriptTestCase.main()
