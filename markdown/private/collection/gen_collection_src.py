import argparse

import yaml

from markdown.private.utils.metadata import InputMetadata, MetadataMap


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("title")
    parser.add_argument("author")
    parser.add_argument("metadata_file")
    parser.add_argument("out_file")
    parser.add_argument("--date")
    parser.add_argument("--dep", action="append", default=[])
    args = parser.parse_args()

    main_author = args.author

    main_metadata = InputMetadata(
        title=args.title,
        author=[main_author],
        date=args.date if args.date else "",
    )

    output = ["---"]
    output += (
        yaml.dump(
            main_metadata.model_dump(
                mode="json",
                by_alias=True,
                exclude_unset=True,
                exclude_defaults=True,
            ),
        )
        .strip()
        .split("\n")
    )
    output.append("---")
    output.append("")

    with open(args.metadata_file, encoding="utf-8") as f:
        metadata = MetadataMap.model_validate_json(f.read())

    for target in args.dep:
        m = metadata[target]
        output += ["::: nospellcheck", "", f"# {m.title}"]
        author = m.author[0] if m.author else ""
        print_author = author != args.author
        print_date = m.date != ""
        tagline = []
        if print_author:
            tagline.append(author)
        if print_date:
            tagline.append(m.date)
        if tagline:
            output += [
                "",
                "**" + ", ".join(tagline) + "**",
                "",
                "::: collectionseparator",
                "&nbsp;",
                ":::",
            ]
        output += ["", ":::", "", f"!include //{target}", ""]

    with open(args.out_file, "w", encoding="utf-8") as f:
        f.write("\n".join(output))


if __name__ == "__main__":
    main()
