import os
import os.path
import subprocess
from collections.abc import Mapping
from typing import Any

from markdown.private.utils import test_utils

GOOD = {
    "title": "Foo",
    "author": "Bar",
    "publications": [
        {
            "venue": "Foo",
            "submitted": "2022-12-01",
            "accepted": "2022-12-02",
            "published": "2022-12-03",
            "paid": "£10",
            "urls": ["http://example.com"],
            "notes": "foo bar",
        },
    ],
    "notes": "baz quux",
    "finished": True,
}

GOOD2 = {
    "title": "Foo",
    "author": "Bar",
    "date": "10 June 2023",
    "publications": [
        {
            "venue": "Foo",
            "submitted": "2022-12-01",
            "accepted": "2022-12-02",
            "published": "2022-12-03",
            "paid": "£10",
            "urls": ["http://example.com"],
            "notes": "foo bar",
        },
    ],
    "notes": "baz quux",
    "finished": True,
}

GOOD_MINIMAL: Mapping[str, Any] = {}

GOOD_CONVERSIONS = {
    "title": "",
    "date": "",
    "notes": "",
}


class TestValidate(test_utils.ScriptTestCase):
    def run_script(  # type: ignore[override]
        self,
        content: Mapping[str, Any],
    ) -> dict[str, Any]:
        in_file = os.path.join(self.tmpdir(), "in.json")
        self.dump_json(in_file, content)

        out_file = os.path.join(self.tmpdir(), "out.json")

        super().run_script(
            args=[
                in_file,
                out_file,
            ],
        )

        return self.load_json(out_file)

    def test_validate_succeeds(self) -> None:
        self.assertEqual(
            self.run_script(GOOD),
            {
                "author": ["Bar"],
                "finished": True,
                "notes": "baz quux",
                "publications": [
                    {
                        "accepted": "2022-12-02",
                        "notes": "foo bar",
                        "paid": "£10",
                        "published": "2022-12-03",
                        "submitted": "2022-12-01",
                        "urls": ["http://example.com"],
                        "venue": "Foo",
                    },
                ],
                "title": "Foo",
            },
        )
        self.assertEqual(
            self.run_script(GOOD2),
            {
                "author": ["Bar"],
                "date": "10 June 2023",
                "finished": True,
                "notes": "baz quux",
                "publications": [
                    {
                        "accepted": "2022-12-02",
                        "notes": "foo bar",
                        "paid": "£10",
                        "published": "2022-12-03",
                        "submitted": "2022-12-01",
                        "urls": ["http://example.com"],
                        "venue": "Foo",
                    },
                ],
                "title": "Foo",
            },
        )

        self.assertEqual(
            self.run_script(GOOD_MINIMAL),
            {},
        )

        self.assertEqual(
            self.run_script(GOOD_CONVERSIONS),
            {},
        )

    def test_validate_fails(self) -> None:
        # Unknown key
        with self.assertRaises(subprocess.CalledProcessError):
            self.run_script(
                {"foo": "bar"},
            )

        # Wrong type (should be string)
        with self.assertRaises(subprocess.CalledProcessError):
            self.run_script(
                {"title": ["foo"]},
            )

        # Wrong type (should be list of string or string)
        with self.assertRaises(subprocess.CalledProcessError):
            self.run_script(
                {"author": [True]},
            )

        # Wrong type (should be list of string or string)
        with self.assertRaises(subprocess.CalledProcessError):
            self.run_script(
                {"author": True},
            )

        # Wrong type (should be string)
        with self.assertRaises(subprocess.CalledProcessError):
            self.run_script(
                {"date": True},
            )

        # Wrong type (should be list)
        with self.assertRaises(subprocess.CalledProcessError):
            self.run_script(
                {"publications": "foo"},
            )

        # Unknown key
        with self.assertRaises(subprocess.CalledProcessError):
            self.run_script(
                {"publications": [{"foo": "bar"}]},
            )

        # Missing required key
        with self.assertRaises(subprocess.CalledProcessError):
            self.run_script(
                {"publications": [{"notes": "foo"}]},
            )

        # Missing required key
        with self.assertRaises(subprocess.CalledProcessError):
            self.run_script(
                {"publications": [{"venue": "bar"}]},
            )

        # Wrong type (should be string)
        with self.assertRaises(subprocess.CalledProcessError):
            self.run_script(
                {"publications": [{"venue": ["foo"], "submitted": "2022-10-10"}]},
            )
        with self.assertRaises(subprocess.CalledProcessError):
            self.run_script(
                {"publications": [{"venue": "foo", "submitted": "2022-10-10", "paid": ["foo"]}]},
            )
        with self.assertRaises(subprocess.CalledProcessError):
            self.run_script(
                {"publications": [{"venue": "foo", "submitted": "2022-10-10", "notes": ["foo"]}]},
            )

        # Wrong type (should be date)
        with self.assertRaises(subprocess.CalledProcessError):
            self.run_script(
                {"publications": [{"venue": "foo", "submitted": "foo"}]},
            )
        with self.assertRaises(subprocess.CalledProcessError):
            self.run_script(
                {"publications": [{"venue": "foo", "accepted": "foo"}]},
            )
        with self.assertRaises(subprocess.CalledProcessError):
            self.run_script(
                {"publications": [{"venue": "foo", "rejected": "foo"}]},
            )
        with self.assertRaises(subprocess.CalledProcessError):
            self.run_script(
                {"publications": [{"venue": "foo", "withdrawn": "foo"}]},
            )
        with self.assertRaises(subprocess.CalledProcessError):
            self.run_script(
                {"publications": [{"venue": "foo", "abandoned": "foo"}]},
            )
        with self.assertRaises(subprocess.CalledProcessError):
            self.run_script(
                {"publications": [{"venue": "foo", "published": "foo"}]},
            )
        with self.assertRaises(subprocess.CalledProcessError):
            self.run_script(
                {"publications": [{"venue": "foo", "self-published": "foo"}]},
            )

        # Wrong type (should be list)
        with self.assertRaises(subprocess.CalledProcessError):
            self.run_script(
                {"publications": [{"venue": "foo", "submitted": "2022-10-10", "urls": "foo"}]},
            )

        # Wrong type (should be list of string)
        with self.assertRaises(subprocess.CalledProcessError):
            self.run_script(
                {
                    "publications": [
                        {"venue": "foo", "submitted": "2022-10-10", "urls": [{"foo": "bar"}]},
                    ],
                },
            )

        # Mutually exclusive keys
        with self.assertRaises(subprocess.CalledProcessError):
            self.run_script(
                {
                    "publications": [
                        {"venue": "foo", "accepted": "2022-12-01", "rejected": "2022-12-02"},
                    ],
                },
            )
        with self.assertRaises(subprocess.CalledProcessError):
            self.run_script(
                {
                    "publications": [
                        {"venue": "foo", "accepted": "2022-12-01", "withdrawn": "2022-12-02"},
                    ],
                },
            )
        with self.assertRaises(subprocess.CalledProcessError):
            self.run_script(
                {
                    "publications": [
                        {"venue": "foo", "accepted": "2022-12-01", "abandoned": "2022-12-02"},
                    ],
                },
            )
        with self.assertRaises(subprocess.CalledProcessError):
            self.run_script(
                {
                    "publications": [
                        {"venue": "foo", "rejected": "2022-12-01", "withdrawn": "2022-12-02"},
                    ],
                },
            )
        with self.assertRaises(subprocess.CalledProcessError):
            self.run_script(
                {
                    "publications": [
                        {"venue": "foo", "rejected": "2022-12-01", "abandoned": "2022-12-02"},
                    ],
                },
            )
        with self.assertRaises(subprocess.CalledProcessError):
            self.run_script(
                {
                    "publications": [
                        {"venue": "foo", "rejected": "2022-12-01", "published": "2022-12-02"},
                    ],
                },
            )
        with self.assertRaises(subprocess.CalledProcessError):
            self.run_script(
                {
                    "publications": [
                        {"venue": "foo", "withdrawn": "2022-12-01", "published": "2022-12-02"},
                    ],
                },
            )
        with self.assertRaises(subprocess.CalledProcessError):
            self.run_script(
                {
                    "publications": [
                        {"venue": "foo", "abandoned": "2022-12-01", "published": "2022-12-02"},
                    ],
                },
            )
        with self.assertRaises(subprocess.CalledProcessError):
            self.run_script(
                {
                    "publications": [
                        {"venue": "foo", "self-published": "2022-12-01", "published": "2022-12-02"},
                    ],
                },
            )
        with self.assertRaises(subprocess.CalledProcessError):
            self.run_script(
                {
                    "publications": [
                        {"venue": "foo", "self-published": "2022-12-01", "submitted": "2022-12-02"},
                    ],
                },
            )
        with self.assertRaises(subprocess.CalledProcessError):
            self.run_script(
                {
                    "publications": [
                        {"venue": "foo", "self-published": "2022-12-01", "rejected": "2022-12-02"},
                    ],
                },
            )
        with self.assertRaises(subprocess.CalledProcessError):
            self.run_script(
                {
                    "publications": [
                        {"venue": "foo", "self-published": "2022-12-01", "withdrawn": "2022-12-02"},
                    ],
                },
            )
        with self.assertRaises(subprocess.CalledProcessError):
            self.run_script(
                {
                    "publications": [
                        {"venue": "foo", "self-published": "2022-12-01", "abandoned": "2022-12-02"},
                    ],
                },
            )
        with self.assertRaises(subprocess.CalledProcessError):
            self.run_script(
                {
                    "publications": [
                        {"venue": "foo", "self-published": "2022-12-01", "accepted": "2022-12-02"},
                    ],
                },
            )

        # Wrong type (should be string)
        with self.assertRaises(subprocess.CalledProcessError):
            self.run_script(
                {"notes": ["foo"]},
            )

        # Wrong type (should be bool)
        with self.assertRaises(subprocess.CalledProcessError):
            self.run_script(
                {"finished": "foo"},
            )


if __name__ == "__main__":
    test_utils.ScriptTestCase.main()
