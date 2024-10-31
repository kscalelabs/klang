"""Defines the PyKlang CLI."""

from pathlib import Path

import click

from pyklang.bindings import parse_file


@click.command()
@click.argument("input_file")
@click.option("-o", "--output", help="The output file to compile.")
@click.option("-i", "--inplace", is_flag=True, help="Overwrite the input file.")
@click.option("-t", "--text", is_flag=True, help="Output the text representation of the program.")
def main(input_file: str, output: str | None, inplace: bool, text: bool) -> None:
    """Kompile a Klang program."""
    program = parse_file(input_file)

    if inplace:
        if output is not None:
            raise click.UsageError("Cannot specify both -o and -i")
        output = Path(input_file).with_suffix(".ko").as_posix()

    if output is None:
        click.echo(program)
    else:
        Path(output).parent.mkdir(parents=True, exist_ok=True)
        if text:
            program.save_text(output)
        else:
            program.save_binary(output)


if __name__ == "__main__":
    main()
