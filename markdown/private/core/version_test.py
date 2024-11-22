import os
import os.path
import subprocess
from collections.abc import Mapping, Sequence

from markdown.private.utils import test_utils


class TestVersion(test_utils.ScriptTestCase):
    def run_script(  # type: ignore[override]
        self,
        raw_version: Mapping[str, str],
        deps_metadata: Mapping[str, Mapping[str, str | Sequence[str]]],
        args: Sequence[str],
    ) -> str:
        raw_version_file = os.path.join(self.tmpdir(), "raw_version.json")
        self.dump_json(raw_version_file, raw_version)

        deps_metadata_file = os.path.join(self.tmpdir(), "deps_metadata.json")
        self.dump_json(deps_metadata_file, deps_metadata)

        metadata_out_file = os.path.join(self.tmpdir(), "metadata_out.json")

        super().run_script(
            args=[
                raw_version_file,
                deps_metadata_file,
                metadata_out_file,
                *args,
            ],
        )

        return self.load_file(metadata_out_file)

    def test_main_simple(self) -> None:
        metadata_out = self.run_script(
            {"version": "foo", "repo": "bar"},
            {},
            [],
        )

        self.assertEqual(
            metadata_out,
            """{
    "repo": "bar",
    "version": "foo"
}""",
        )

    def test_main_complex(self) -> None:
        metadata_out = self.run_script(
            {"version": "foo", "repo": "bar"},
            {
                "dep1": {
                    "version": "2, dirty",
                    "repo": "bar",
                    "wordcount": "0",
                    "poetry-lines": "0",
                    "lang": "",
                    "source-hash": "",
                    "parsed-dates": [],
                },
                "dep2": {
                    "version": "3",
                    "repo": "quux",
                    "wordcount": "0",
                    "poetry-lines": "0",
                    "lang": "",
                    "source-hash": "",
                    "parsed-dates": [],
                },
            },
            [],
        )

        self.assertEqual(
            metadata_out,
            """{
    "repo": "bar",
    "version": "foo, dirty deps"
}""",
        )

    def test_main_version_override(self) -> None:
        metadata_out = self.run_script(
            {"version": "foo", "repo": "bar"},
            {
                "dep1": {
                    "version": "2, dirty",
                    "repo": "bar",
                    "wordcount": "0",
                    "poetry-lines": "0",
                    "lang": "",
                    "source-hash": "",
                    "parsed-dates": [],
                },
                "dep2": {
                    "version": "3",
                    "repo": "quux",
                    "wordcount": "0",
                    "poetry-lines": "0",
                    "lang": "",
                    "source-hash": "",
                    "parsed-dates": [],
                },
            },
            ["--version_override", "override"],
        )

        self.assertEqual(
            metadata_out,
            """{
    "repo": "bar",
    "version": "override"
}""",
        )

    def test_main_repo_override(self) -> None:
        metadata_out = self.run_script(
            {"version": "foo", "repo": "bar"},
            {
                "dep1": {
                    "version": "2, dirty",
                    "repo": "bar",
                    "wordcount": "0",
                    "poetry-lines": "0",
                    "lang": "",
                    "source-hash": "",
                    "parsed-dates": [],
                },
                "dep2": {
                    "version": "3",
                    "repo": "quux",
                    "wordcount": "0",
                    "poetry-lines": "0",
                    "lang": "",
                    "source-hash": "",
                    "parsed-dates": [],
                },
            },
            ["--repo_override", "override"],
        )

        self.assertEqual(
            metadata_out,
            """{
    "repo": "override",
    "version": "foo, dirty deps"
}""",
        )

    def test_main_fails(self) -> None:
        with self.assertRaises(subprocess.CalledProcessError):
            self.run_script(
                {"version": "foo", "repo": "bar"},
                {
                    "dep1": {
                        "version": "2, dirty",
                        "repo": "baz",
                        "wordcount": "0",
                        "poetry-lines": "0",
                        "lang": "",
                        "source-hash": "",
                        "parsed-dates": [],
                    },
                    "dep2": {
                        "version": "3",
                        "repo": "quux",
                        "wordcount": "0",
                        "poetry-lines": "0",
                        "lang": "",
                        "source-hash": "",
                        "parsed-dates": [],
                    },
                },
                ["--version_override", "override"],
            )


if __name__ == "__main__":
    test_utils.ScriptTestCase.main()
