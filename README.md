# fgener

A rust command line utility to generate n number of files with random characters in each one of them.

## Installation

```bash
$ cd fgener
$ cargo build --release
$ cargo install --path .
```
## Usage

```
$ fgener <path_to_output_folder> <number_of_files_to_generate>
```

## Example

```
$ fgener ./outputs 10
```
This should generate 10 files in an output folder if output folder exists.

