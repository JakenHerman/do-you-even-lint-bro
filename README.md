# Do You Even Lint, Bro?

A Rust-based tool that scans through your Python projects and counts how many times each unique linter ignore pattern (like # type: ignore[...] for mypy or # noqa for flake8) appears.

TLDR; a tool that uses Rust to track pesky annotations that Python’s type checking simply can’t manage without a constant stream of # type: ignore comments.

## Overview

While ignoring linter warnings in Python is possible, it also leaves you with hidden technical debt and less-than-ideal type safety. I wrote this program to:
- Scan a Python project for linter ignore comments.
- Count the occurrences of each unique ignore pattern.
- Output the results to the terminal or write them to a file.

## Features
- Splits ignore directives like `# type: ignore[has-type, arg-type]` into separate keys and counts each occurrence individually.
- CLI interface via Clap: Specify the directory, choose the linter (mypy or flake8), and optionally output the results to a text file.

## Installation
Clone the repository:
```
git clone https://github.com/jakenherman/do-you-even-lint-bro.git
cd do-you-even-lint-bro
```

Build the project:
Ensure you have Rust installed via [rustup](https://rustup.rs).
```
cargo build --release
```

## Usage
Run the tool from the command line. Here are a few examples:

**Count mypy ignores in a directory**:
```
cargo run -- --dir /path/to/python/project --linter mypy
```

**Count flake8 ignores in a directory and output to a file**:
```
cargo run -- --dir /path/to/python/project --linter flake8 --output flake8_ignores.txt
```

The tool will output the number of unique ignore patterns along with how many times each appears. Example:
```
Unique 'flake8' patterns found: 2
'D106' ignored 6 times
'DTZ007' ignored 2 times
```

## Contributing
Contributions, suggestions, and even snarky comments comparing Python’s type system to Rust’s are welcome! Please open an issue or submit a pull request. Enjoy the power and reliability of Rust, and may your linter ignores be ever counted—and swiftly eliminated!
