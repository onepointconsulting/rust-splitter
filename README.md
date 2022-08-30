# Rust Splitter

Simple splitter that can be used to split files line by line.
It can choose files using recursive patterns. 

## Compilation

```ps1
& cargo build --release
```

## Usage

```
splitter
Simple program used to split files using glob patterns

USAGE:
    splitter.exe [OPTIONS] --pattern <PATTERN> --length-str <LENGTH_STR>

OPTIONS:
    -h, --help                       Print help information
    -l, --length-str <LENGTH_STR>    The length of the split in terms of lines, like e.g. 50
    -p, --pattern <PATTERN>          The pattern used to list files, like e.g. *.csv or
                                     /media/**/*.csv
    -t, --target-dir <TARGET_DIR>    A target directory for all of the files
(base) PS C:\development\playground\rust\first_steps\splitter>

```