#!/usr/bin/env python3

import argparse
import re
from os.path import basename

def parse():
    parser = argparse.ArgumentParser(
        description="The normalizer for maxwell protocol in rust."
    )
    parser.add_argument("--proto_file", required=True,
                        type=argparse.FileType("r"))
    args = parser.parse_args()
    return args.proto_file


def capitalize(name):
    return "".join(map(lambda s: s.capitalize(), name.lower().split("_")))


def normalize(content, output_file_name):
    output = re.sub(
        r"(enum\s+)([a-zA-Z0-9_]+_t)(\s+\{)",
        lambda match: match.group(1)+ capitalize(match.group(2).rstrip("_t")) + match.group(3), 
        content
    )
    output = re.sub(
        r"(message\s+)([a-zA-Z0-9_]+_t)(\s+\{)",
        lambda match: match.group(1) + capitalize(match.group(2).rstrip("_t")) + match.group(3),
        output
    )
    output = re.sub(
        r"([a-zA-Z0-9_]+_t)(\s+[a-zA-Z0-9_]+\s+=\s+[0-9]+\s*;)",
        lambda match: capitalize(match.group(1).rstrip("_t")) + match.group(2),
        output
    )
    with open(output_file_name, "w") as output_file:
        output_file.write(output)


if __name__ == "__main__":
    proto_file = parse()
    content = proto_file.read()
    output_file_name = proto_file.name.rstrip(".proto") + ".normalized.proto"
    normalize(content, output_file_name)