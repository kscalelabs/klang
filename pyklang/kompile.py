"""Defines the PyKlang CLI."""

import argparse
from pathlib import Path

from pyklang.bindings import parse_file


def main() -> None:
    parser = argparse.ArgumentParser(description="Kompile a Klang program.")
    parser.add_argument("input", help="The input file to compile.")
    parser.add_argument("-o", "--output", help="The output file to compile.")
    parser.add_argument("-t", "--text", action="store_true", help="Output the text representation of the program.")
    args = parser.parse_args()

    program = parse_file(args.input)

    if args.output is None:
        print(program)
    else:
        Path(args.output).parent.mkdir(parents=True, exist_ok=True)
        if args.text:
            program.save_text(args.output)
        else:
            program.save_binary(args.output)


if __name__ == "__main__":
    main()
