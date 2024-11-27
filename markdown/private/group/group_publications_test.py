import os
import os.path

from markdown.private.utils import test_utils


class TestPublications(test_utils.ScriptTestCase):
    def test_publications(self) -> None:
        metadata = os.path.join(self.tmpdir(), "metadata.json")
        self.dump_json(
            metadata,
            {
                "//foo:bar": {
                    "title": "Foo <Bar>",
                    "wordcount": "10",
                    "version": "bar",
                    "publications": [
                        {
                            "venue": "Foo",
                            "submitted": "2022-12-13",
                            "accepted": "2022-12-13",
                            "published": "2022-12-14",
                        },
                        {
                            "venue": "Bar",
                            "submitted": "2022-10-13",
                            "rejected": "2022-10-14",
                        },
                    ],
                    "poetry-lines": "0",
                    "lang": "en-GB",
                    "repo": "bar",
                    "source-hash": "1",
                    "parsed-dates": ["2020"],
                },
                "//baz:quux": {
                    "title": "Baz",
                    "wordcount": "20",
                    "version": "quux, dirty",
                    "publications": [
                        {
                            "venue": "Foo",
                            "self-published": "2022-11-14",
                        },
                    ],
                    "poetry-lines": "0",
                    "lang": "en-GB",
                    "repo": "bar",
                    "source-hash": "1",
                    "parsed-dates": ["2021"],
                },
            },
        )

        outfile = os.path.join(self.tmpdir(), "out.html")

        self.run_script(
            args=[
                metadata,
                outfile,
            ],
        )

        self.assertEqual(
            self.load_file(outfile),
            """<!doctype html>
<html lang="en-GB">
<head>
<meta charset="utf-8">
<title>Publications</title>
<style>
table { border-collapse: collapse; }
th, td { border: 1px solid; padding: 5px; }
a:link { color: black; }
a:visited { color: black; }
.submitted { background-color: #ffff00; }
.rejected { background-color: #ff6d6d; }
.withdrawn { background-color: #ff972f; }
.abandoned { background-color: #cccccc; }
.accepted { background-color: #729fcf; }
.self_published { background-color: #158466; }
.published { background-color: #81d41a; }
</style>
</head>
<body>
<h1>Publications</h1>
<table>
<thead>
<tr>
<th title="Target">Target</th>
<th title="Title">Title</th>
<th title="Wordcount">Wordcount</th>
<th style="border-right: 3px solid" title="Notes">Notes</th>
<th title="Bar">Bar</th>
<th title="Foo">Foo</th>
</tr>
</thead>
<tbody>
<tr>
<td class="self_published" title="//baz:quux"><a href="#//baz:quux">//baz:quux</a></td>
<td title="Baz">Baz</td>
<td title="20">20</td>
<td style="border-right: 3px solid" title=""></td>
<td></td>
<td class="self_published" title="//baz:quux, Foo"><a href="#//baz:quux">2022-11-14 Self-published</a></td>
</tr>
<tr>
<td class="published" title="//foo:bar"><a href="#//foo:bar">//foo:bar</a></td>
<td title="Foo &lt;Bar&gt;">Foo &lt;Bar&gt;</td>
<td title="10">10</td>
<td style="border-right: 3px solid" title=""></td>
<td class="rejected" title="//foo:bar, Bar"><a href="#//foo:bar">2022-10-13 Submitted<br>2022-10-14 Rejected</a></td>
<td class="published" title="//foo:bar, Foo"><a href="#//foo:bar">2022-12-13 Submitted<br>2022-12-13 Accepted<br>2022-12-14 Published</a></td>
</tr>
</tbody>
</table>
<h2>Details</h2>
<h3 id="//baz:quux">//baz:quux</h3>
<code><pre>{
  "lang": "en-GB",
  "parsed-dates": [
    "2021"
  ],
  "poetry-lines": 0,
  "publications": [
    {
      "self-published": "2022-11-14",
      "venue": "Foo"
    }
  ],
  "repo": "bar",
  "source-hash": "1",
  "title": "Baz",
  "version": "quux, dirty",
  "wordcount": 20
}</pre></code>
<h3 id="//foo:bar">//foo:bar</h3>
<code><pre>{
  "lang": "en-GB",
  "parsed-dates": [
    "2020"
  ],
  "poetry-lines": 0,
  "publications": [
    {
      "accepted": "2022-12-13",
      "published": "2022-12-14",
      "submitted": "2022-12-13",
      "venue": "Foo"
    },
    {
      "rejected": "2022-10-14",
      "submitted": "2022-10-13",
      "venue": "Bar"
    }
  ],
  "repo": "bar",
  "source-hash": "1",
  "title": "Foo &lt;Bar&gt;",
  "version": "bar",
  "wordcount": 10
}</pre></code>
</body>
</html>
""",  # noqa: E501
        )


if __name__ == "__main__":
    test_utils.ScriptTestCase.main()
