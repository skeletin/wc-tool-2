# ccwc: A Rust Implementation of the Unix `wc` Tool

This project is a solution to the John Crickett Code Challenge to build your own version of the classic Unix command line tool `wc` (word count), implemented in Rust.

## About

The Unix command line tools are a great metaphor for good software engineering, following the Unix Philosophy:
- **Write simple parts connected by clean interfaces**: Each tool does one thing well and provides a simple CLI for text input from files or streams.
- **Design programs to be connected to other programs**: Tools can be easily composed to create powerful pipelines.

This project, `ccwc` (Coding Challenges Word Count), is inspired by these principles and the challenge to re-create `wc` from scratch.

## Challenge Description

The goal is to build a tool that matches the functionality of the Unix `wc` command, which counts lines, words, characters, and bytes in a file. The requirements are based on the `wc` man page:

- `-c`: Output the number of bytes in a file
- `-l`: Output the number of lines in a file
- `-w`: Output the number of words in a file
- `-m`: Output the number of characters in a file (may differ from bytes for multibyte locales)
- No options: Output lines, words, and bytes (default `wc` behavior)

## Usage

Build the project with Cargo:

```sh
cargo build --release
```

Run `ccwc` with the desired options:

```sh
# Count bytes
./target/release/wc -c test.txt
# Count lines
./target/release/wc -l test.txt
# Count words
./target/release/wc -w test.txt
# Count characters
./target/release/wc -m test.txt
# Default: lines, words, bytes
./target/release/wc test.txt
```

Example output:

```
$ ./target/release/wc -c test.txt
  342190 test.txt
$ ./target/release/wc -l test.txt
    7145 test.txt
$ ./target/release/wc -w test.txt
   58164 test.txt
$ ./target/release/wc -m test.txt
  339292 test.txt
$ ./target/release/wc test.txt
    7145   58164  342190 test.txt
```

## How It Works

- Parses command line arguments for options and file names
- Reads the file and counts lines, words, bytes, and characters
- Supports multiple files and prints totals when more than one file is provided
- Handles invalid options and missing files gracefully

## References
- [The Art of Unix Programming](http://catb.org/~esr/writings/taoup/)
- [man wc](https://man7.org/linux/man-pages/man1/wc.1.html)
- [John Crickett Code Challenges](https://johncrickett.co.uk/)

## License

This project is for educational purposes as part of the John Crickett Code Challenges.
