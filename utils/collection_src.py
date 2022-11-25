import argparse
import json


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument('title')
    parser.add_argument('author')
    parser.add_argument('date')
    parser.add_argument('out_file')
    parser.add_argument('--dep', action='append', nargs=2, default=[])
    args = parser.parse_args()

    main_author = args.author

    output = [
        f'% {args.title}',
        f'% {main_author}',
    ]
    if args.date:
        output.append(f'% {args.date}')
    output.append('')

    for target, metadata_file in args.dep:
        with open(metadata_file, encoding='utf-8') as f:
            j = json.load(f)
        output += [f'# {j["title"]}', '']
        author = j['author'][0]
        print_author = author != args.author
        print_date = 'date' in j and j['date']
        tagline = []
        if print_author:
            tagline.append(author)
        if print_date:
            tagline.append(j['date'])
        if tagline:
            output += ['### ' + ', '.join(tagline), '']
        output += [f'!include //{target}', '']

    with open(args.out_file, 'w', encoding='utf-8') as f:
        f.write('\n'.join(output))


if __name__ == '__main__':
    main()
