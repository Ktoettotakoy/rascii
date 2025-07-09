# RASCII

Generate ascii from an image or video cli tool

Usage: rascii <COMMAND>

### Commands:
#### convert
Convert an image to ASCII
Usage: rascii convert [OPTIONS] --input <INPUT>

```
Options:
  -i, --input <INPUT>    Path to the input image
  -w, --width <WIDTH>    Output width [default: 100]
  -s, --style <STYLE>    Style of ASCII art (0 for basic, 1 for extended, 2 for inverted) Default is 0 (basic)
  -o, --output <OUTPUT>  Output file (optional)
```
#### help
Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
