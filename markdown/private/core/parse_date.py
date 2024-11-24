import argparse
import json

from dateparser.date import DateDataParser
from dateparser.search import search_dates


def parse_date(date: str) -> frozenset[str]:
    settings = {"DATE_ORDER": "DMY", "PARSERS": ["custom-formats", "absolute-time"]}
    parser = DateDataParser(["en"], ["en-GB"], settings=settings)  # type: ignore[arg-type]

    out = set()
    for text, _ in search_dates(date, languages=["en"], settings=settings) or []:
        data = parser.get_date_data(text)
        if data.date_obj:
            if data.period == "year":
                out.add(data.date_obj.strftime("%Y"))
            elif data.period == "month":
                out.add(data.date_obj.strftime("%Y/%m"))
            elif data.period == "day":
                out.add(data.date_obj.strftime("%Y/%m/%d"))

    return frozenset(out)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("in_file")
    parser.add_argument("out_file")
    args = parser.parse_args()

    with open(args.in_file, encoding="utf-8") as f:
        metadata = json.load(f)

    date = metadata.get("date", "")
    dates: frozenset[str] = frozenset()
    if date:
        dates = parse_date(date)

    with open(args.out_file, "w", encoding="utf-8") as f:
        json.dump(
            {"parsed-dates": sorted(dates)},
            f,
            sort_keys=True,
            indent=2,
        )


if __name__ == "__main__":
    main()
