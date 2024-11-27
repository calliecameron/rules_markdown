import os
import os.path
from collections.abc import Sequence

from markdown.private.utils import test_utils

DATA = {
    "test1:foo": {
        "title": "Foo",
        "author": ["A", "B"],
        "date": "2022",
        "wordcount": "10",
        "poetry-lines": "3",
        "finished": True,
        "version": "bar",
        "publications": [
            {
                "venue": "foo",
                "submitted": "2023-05-18",
                "accepted": "2023-05-18",
                "published": "2023-05-18",
            },
        ],
        "lang": "en-GB",
        "repo": "bar",
        "source-hash": "1",
        "parsed-dates": ["2022"],
    },
    "test1:bar": {
        "title": "Bar\nbaz",
        "author": "A",
        "wordcount": "5",
        "poetry-lines": "0",
        "version": "quux, dirty",
        "lang": "en-GB",
        "repo": "bar",
        "source-hash": "1",
        "parsed-dates": [],
    },
    "test2:baz": {
        "title": "baz",
        "author": "B",
        "date": "from August 2020 to 1 March 2023",
        "wordcount": "20",
        "poetry-lines": "5",
        "finished": False,
        "version": "baz",
        "publications": [
            {
                "venue": "baz",
                "submitted": "2023-05-18",
                "rejected": "2023-05-18",
            },
        ],
        "lang": "en-GB",
        "repo": "bar",
        "source-hash": "1",
        "parsed-dates": ["2020/08", "2023/03/01"],
    },
}


class TestSummary(test_utils.ScriptTestCase):
    maxDiff = None

    def run_script(self, args: Sequence[str]) -> str:  # type: ignore[override]
        filename = os.path.join(self.tmpdir(), "in.json")
        self.dump_json(filename, DATA)

        return (
            super()
            .run_script(
                args=[
                    filename,
                    *args,
                ],
            )
            .stdout
        )

    def test_summary_pretty(self) -> None:
        self.assertEqual(
            self.run_script([]),
            """
| target    | title    | author | raw date                         | date                | wordcount | poetry lines | finished | publication | version     | status |
|-----------|----------|--------|----------------------------------|---------------------|-----------|--------------|----------|-------------|-------------|--------|
| test1:bar | Bar\\nbaz | A      |                                  |                     |         5 |            0 | no       |             | quux, dirty | DIRTY  |
| test1:foo | Foo      | A, B   | 2022                             | 2022                |        10 |            3 | yes      | published   | bar         | ok     |
| test2:baz | baz      | B      | from August 2020 to 1 March 2023 | 2020/08, 2023/03/01 |        20 |            5 | no       | attempted   | baz         | ok     |
""".lstrip(),  # noqa: E501
        )

    def test_summary_raw(self) -> None:
        self.assertEqual(
            self.run_script(["--raw"]),
            """target,title,author,raw date,date,wordcount,poetry lines,finished,publication,version,status
test1:bar,Bar\\nbaz,A,,,5,0,no,,"quux, dirty",DIRTY
test1:foo,Foo,"A, B",2022,2022,10,3,yes,published,bar,ok
test2:baz,baz,B,from August 2020 to 1 March 2023,"2020/08, 2023/03/01",20,5,no,attempted,baz,ok
""",  # noqa: E501
        )

        self.assertEqual(
            self.run_script(["--raw", "--include-target", "test1"]),
            """target,title,author,raw date,date,wordcount,poetry lines,finished,publication,version,status
test1:bar,Bar\\nbaz,A,,,5,0,no,,"quux, dirty",DIRTY
test1:foo,Foo,"A, B",2022,2022,10,3,yes,published,bar,ok
""",  # noqa: E501
        )

        self.assertEqual(
            self.run_script(["--raw", "--exclude-target", "test1"]),
            """target,title,author,raw date,date,wordcount,poetry lines,finished,publication,version,status
test2:baz,baz,B,from August 2020 to 1 March 2023,"2020/08, 2023/03/01",20,5,no,attempted,baz,ok
""",  # noqa: E501
        )

        self.assertEqual(
            self.run_script(["--raw", "--include-target", "test1", "--exclude-title", "ba.*"]),
            """target,title,author,raw date,date,wordcount,poetry lines,finished,publication,version,status
test1:foo,Foo,"A, B",2022,2022,10,3,yes,published,bar,ok
""",  # noqa: E501
        )

        self.assertEqual(
            self.run_script(["--raw", "--sort-target"]),
            """target,title,author,raw date,date,wordcount,poetry lines,finished,publication,version,status
test1:bar,Bar\\nbaz,A,,,5,0,no,,"quux, dirty",DIRTY
test1:foo,Foo,"A, B",2022,2022,10,3,yes,published,bar,ok
test2:baz,baz,B,from August 2020 to 1 March 2023,"2020/08, 2023/03/01",20,5,no,attempted,baz,ok
""",  # noqa: E501
        )

        self.assertEqual(
            self.run_script(["--raw", "--sort-target", "--reverse"]),
            """target,title,author,raw date,date,wordcount,poetry lines,finished,publication,version,status
test2:baz,baz,B,from August 2020 to 1 March 2023,"2020/08, 2023/03/01",20,5,no,attempted,baz,ok
test1:foo,Foo,"A, B",2022,2022,10,3,yes,published,bar,ok
test1:bar,Bar\\nbaz,A,,,5,0,no,,"quux, dirty",DIRTY
""",  # noqa: E501
        )

        self.assertEqual(
            self.run_script(["--raw", "--sort-title"]),
            """target,title,author,raw date,date,wordcount,poetry lines,finished,publication,version,status
test1:bar,Bar\\nbaz,A,,,5,0,no,,"quux, dirty",DIRTY
test2:baz,baz,B,from August 2020 to 1 March 2023,"2020/08, 2023/03/01",20,5,no,attempted,baz,ok
test1:foo,Foo,"A, B",2022,2022,10,3,yes,published,bar,ok
""",  # noqa: E501
        )

        self.assertEqual(
            self.run_script(["--raw", "--sort-title", "--reverse"]),
            """target,title,author,raw date,date,wordcount,poetry lines,finished,publication,version,status
test1:foo,Foo,"A, B",2022,2022,10,3,yes,published,bar,ok
test2:baz,baz,B,from August 2020 to 1 March 2023,"2020/08, 2023/03/01",20,5,no,attempted,baz,ok
test1:bar,Bar\\nbaz,A,,,5,0,no,,"quux, dirty",DIRTY
""",  # noqa: E501
        )

        self.assertEqual(
            self.run_script(["--raw", "--sort-author"]),
            """target,title,author,raw date,date,wordcount,poetry lines,finished,publication,version,status
test1:bar,Bar\\nbaz,A,,,5,0,no,,"quux, dirty",DIRTY
test1:foo,Foo,"A, B",2022,2022,10,3,yes,published,bar,ok
test2:baz,baz,B,from August 2020 to 1 March 2023,"2020/08, 2023/03/01",20,5,no,attempted,baz,ok
""",  # noqa: E501
        )

        self.assertEqual(
            self.run_script(["--raw", "--sort-author", "--reverse"]),
            """target,title,author,raw date,date,wordcount,poetry lines,finished,publication,version,status
test2:baz,baz,B,from August 2020 to 1 March 2023,"2020/08, 2023/03/01",20,5,no,attempted,baz,ok
test1:foo,Foo,"A, B",2022,2022,10,3,yes,published,bar,ok
test1:bar,Bar\\nbaz,A,,,5,0,no,,"quux, dirty",DIRTY
""",  # noqa: E501
        )

        self.assertEqual(
            self.run_script(["--raw", "--sort-raw-date"]),
            """target,title,author,raw date,date,wordcount,poetry lines,finished,publication,version,status
test1:bar,Bar\\nbaz,A,,,5,0,no,,"quux, dirty",DIRTY
test1:foo,Foo,"A, B",2022,2022,10,3,yes,published,bar,ok
test2:baz,baz,B,from August 2020 to 1 March 2023,"2020/08, 2023/03/01",20,5,no,attempted,baz,ok
""",  # noqa: E501
        )

        self.assertEqual(
            self.run_script(["--raw", "--sort-raw-date", "--reverse"]),
            """target,title,author,raw date,date,wordcount,poetry lines,finished,publication,version,status
test2:baz,baz,B,from August 2020 to 1 March 2023,"2020/08, 2023/03/01",20,5,no,attempted,baz,ok
test1:foo,Foo,"A, B",2022,2022,10,3,yes,published,bar,ok
test1:bar,Bar\\nbaz,A,,,5,0,no,,"quux, dirty",DIRTY
""",  # noqa: E501
        )

        self.assertEqual(
            self.run_script(["--raw", "--sort-date"]),
            """target,title,author,raw date,date,wordcount,poetry lines,finished,publication,version,status
test1:bar,Bar\\nbaz,A,,,5,0,no,,"quux, dirty",DIRTY
test2:baz,baz,B,from August 2020 to 1 March 2023,"2020/08, 2023/03/01",20,5,no,attempted,baz,ok
test1:foo,Foo,"A, B",2022,2022,10,3,yes,published,bar,ok
""",  # noqa: E501
        )

        self.assertEqual(
            self.run_script(["--raw", "--sort-date", "--reverse"]),
            """target,title,author,raw date,date,wordcount,poetry lines,finished,publication,version,status
test2:baz,baz,B,from August 2020 to 1 March 2023,"2020/08, 2023/03/01",20,5,no,attempted,baz,ok
test1:foo,Foo,"A, B",2022,2022,10,3,yes,published,bar,ok
test1:bar,Bar\\nbaz,A,,,5,0,no,,"quux, dirty",DIRTY
""",  # noqa: E501
        )

        self.assertEqual(
            self.run_script(["--raw", "--sort-wordcount"]),
            """target,title,author,raw date,date,wordcount,poetry lines,finished,publication,version,status
test1:bar,Bar\\nbaz,A,,,5,0,no,,"quux, dirty",DIRTY
test1:foo,Foo,"A, B",2022,2022,10,3,yes,published,bar,ok
test2:baz,baz,B,from August 2020 to 1 March 2023,"2020/08, 2023/03/01",20,5,no,attempted,baz,ok
""",  # noqa: E501
        )

        self.assertEqual(
            self.run_script(["--raw", "--sort-wordcount", "--reverse"]),
            """target,title,author,raw date,date,wordcount,poetry lines,finished,publication,version,status
test2:baz,baz,B,from August 2020 to 1 March 2023,"2020/08, 2023/03/01",20,5,no,attempted,baz,ok
test1:foo,Foo,"A, B",2022,2022,10,3,yes,published,bar,ok
test1:bar,Bar\\nbaz,A,,,5,0,no,,"quux, dirty",DIRTY
""",  # noqa: E501
        )

        self.assertEqual(
            self.run_script(["--raw", "--sort-poetry-lines"]),
            """target,title,author,raw date,date,wordcount,poetry lines,finished,publication,version,status
test1:bar,Bar\\nbaz,A,,,5,0,no,,"quux, dirty",DIRTY
test1:foo,Foo,"A, B",2022,2022,10,3,yes,published,bar,ok
test2:baz,baz,B,from August 2020 to 1 March 2023,"2020/08, 2023/03/01",20,5,no,attempted,baz,ok
""",  # noqa: E501
        )

        self.assertEqual(
            self.run_script(["--raw", "--sort-poetry-lines", "--reverse"]),
            """target,title,author,raw date,date,wordcount,poetry lines,finished,publication,version,status
test2:baz,baz,B,from August 2020 to 1 March 2023,"2020/08, 2023/03/01",20,5,no,attempted,baz,ok
test1:foo,Foo,"A, B",2022,2022,10,3,yes,published,bar,ok
test1:bar,Bar\\nbaz,A,,,5,0,no,,"quux, dirty",DIRTY
""",  # noqa: E501
        )

        self.assertEqual(
            self.run_script(["--raw", "--sort-finished"]),
            """target,title,author,raw date,date,wordcount,poetry lines,finished,publication,version,status
test1:bar,Bar\\nbaz,A,,,5,0,no,,"quux, dirty",DIRTY
test2:baz,baz,B,from August 2020 to 1 March 2023,"2020/08, 2023/03/01",20,5,no,attempted,baz,ok
test1:foo,Foo,"A, B",2022,2022,10,3,yes,published,bar,ok
""",  # noqa: E501
        )

        self.assertEqual(
            self.run_script(["--raw", "--sort-finished", "--reverse"]),
            """target,title,author,raw date,date,wordcount,poetry lines,finished,publication,version,status
test1:foo,Foo,"A, B",2022,2022,10,3,yes,published,bar,ok
test2:baz,baz,B,from August 2020 to 1 March 2023,"2020/08, 2023/03/01",20,5,no,attempted,baz,ok
test1:bar,Bar\\nbaz,A,,,5,0,no,,"quux, dirty",DIRTY
""",  # noqa: E501
        )

        self.assertEqual(
            self.run_script(["--raw", "--sort-publication"]),
            """target,title,author,raw date,date,wordcount,poetry lines,finished,publication,version,status
test1:bar,Bar\\nbaz,A,,,5,0,no,,"quux, dirty",DIRTY
test2:baz,baz,B,from August 2020 to 1 March 2023,"2020/08, 2023/03/01",20,5,no,attempted,baz,ok
test1:foo,Foo,"A, B",2022,2022,10,3,yes,published,bar,ok
""",  # noqa: E501
        )

        self.assertEqual(
            self.run_script(["--raw", "--sort-publication", "--reverse"]),
            """target,title,author,raw date,date,wordcount,poetry lines,finished,publication,version,status
test1:foo,Foo,"A, B",2022,2022,10,3,yes,published,bar,ok
test2:baz,baz,B,from August 2020 to 1 March 2023,"2020/08, 2023/03/01",20,5,no,attempted,baz,ok
test1:bar,Bar\\nbaz,A,,,5,0,no,,"quux, dirty",DIRTY
""",  # noqa: E501
        )

        self.assertEqual(
            self.run_script(["--raw", "--sort-version"]),
            """target,title,author,raw date,date,wordcount,poetry lines,finished,publication,version,status
test1:foo,Foo,"A, B",2022,2022,10,3,yes,published,bar,ok
test2:baz,baz,B,from August 2020 to 1 March 2023,"2020/08, 2023/03/01",20,5,no,attempted,baz,ok
test1:bar,Bar\\nbaz,A,,,5,0,no,,"quux, dirty",DIRTY
""",  # noqa: E501
        )

        self.assertEqual(
            self.run_script(["--raw", "--sort-version", "--reverse"]),
            """target,title,author,raw date,date,wordcount,poetry lines,finished,publication,version,status
test1:bar,Bar\\nbaz,A,,,5,0,no,,"quux, dirty",DIRTY
test2:baz,baz,B,from August 2020 to 1 March 2023,"2020/08, 2023/03/01",20,5,no,attempted,baz,ok
test1:foo,Foo,"A, B",2022,2022,10,3,yes,published,bar,ok
""",  # noqa: E501
        )

        self.assertEqual(
            self.run_script(["--raw", "--sort-status"]),
            """target,title,author,raw date,date,wordcount,poetry lines,finished,publication,version,status
test1:bar,Bar\\nbaz,A,,,5,0,no,,"quux, dirty",DIRTY
test1:foo,Foo,"A, B",2022,2022,10,3,yes,published,bar,ok
test2:baz,baz,B,from August 2020 to 1 March 2023,"2020/08, 2023/03/01",20,5,no,attempted,baz,ok
""",  # noqa: E501
        )

        self.assertEqual(
            self.run_script(["--raw", "--sort-status", "--reverse"]),
            """target,title,author,raw date,date,wordcount,poetry lines,finished,publication,version,status
test2:baz,baz,B,from August 2020 to 1 March 2023,"2020/08, 2023/03/01",20,5,no,attempted,baz,ok
test1:foo,Foo,"A, B",2022,2022,10,3,yes,published,bar,ok
test1:bar,Bar\\nbaz,A,,,5,0,no,,"quux, dirty",DIRTY
""",  # noqa: E501
        )


if __name__ == "__main__":
    test_utils.ScriptTestCase.main()
