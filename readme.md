
# Truth Table Generator

A fast and flexible truth table generator implemented in Rust. This tool allows users to input logical formulas and generates complete truth tables, making it ideal for logic analysis, education, and verification tasks.

[![Rust](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- Support for complex logical formulas with multiple operators
- Fast parsing and evaluation using Shunting Yard algorithm
- Generates complete truth tables for given formulas
- Supports common logical operators: AND, OR, NOT, IF, IFF
- Handles parentheses for precise operator precedence
- Efficient memory usage with boxed AST nodes
- Command-line interface for easy integration into scripts or larger projects

## Installation

To use the Truth Table Generator, you need to have Rust installed on your system. If you don't have Rust installed, you can get it from [rustup.rs](https://rustup.rs/).

Once Rust is installed, clone this repository and build the project:

```bash
git clone https://github.com/yourusername/truth-table-rs.git
cd truth-table-rs
cargo build --release
```

The compiled binary will be available in `target/release/truth-table-rs`.

## Usage

To generate a truth table, run the program with your logical formula as an argument:

```bash
./target/release/truth-table-rs "a & b | ~c"
```

This will output a nicely formatted truth table:

```
|   a   |   b   |   c   | a & b | ~c |
--------------------------------------
|   T   |   T   |   T   |     T      |
|   T   |   T   |   F   |     T      |
|   T   |   F   |   T   |     F      |
|   T   |   F   |   F   |     T      |
|   F   |   T   |   T   |     F      |
|   F   |   T   |   F   |     T      |
|   F   |   F   |   T   |     F      |
|   F   |   F   |   F   |     T      |
--------------------------------------
T: True, F: False

```

You can use the following operators in your formulas:
- `&` or `&&` for AND
- `|` or `||` for OR
- `~` or `!` for NOT
- `->` for IF (implication)
- `<->` for IFF (bi-implication)

Parentheses can be used to specify operator precedence.

## Contributing

Contributions to the Truth Table Generator are welcome! Here's how you can contribute:

1. Fork the repository
2. Create a new branch for your feature or bug fix
3. Make your changes and write tests if applicable
4. Run `cargo test` to ensure all tests pass
5. Submit a pull request with a clear description of your changes

Please adhere to the existing code style and add comments where necessary.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

Built with ❤️ using Rust. Stars ⭐ and contributions are appreciated!
