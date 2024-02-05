# fgener

A rust command line utility to generate n number of files with random characters in each one of them.

## Installation

```bash
$ cargo install fgener
```

```bash
$ cd fgener
$ cargo build --release
$ cargo install --path .
```
## Usage

```
$ fgener --help
Usage: fgener [OPTIONS] <OUTPUT_PATH> <NUMBER_OF_FILES>

Arguments:
  <OUTPUT_PATH>      The output folder where files are generated
  <NUMBER_OF_FILES>  Number of files to be generated

Options:
  -l, --lowest-number-of-chars <LOWEST_NUMBER_OF_CHARS>
          least number of characters in a file (Optional) [default: 100]
  -h <HIGHEST_NUMBER_OF_CHARS>
          highest number of characters in a file (Optional) [default: 2621440]
  -h, --help
          Print help
  -V, --version
          Print version
```

## Example

```
$ fgener ./outputs 10
```
This should generate 10 files in an output folder if output folder exists.

