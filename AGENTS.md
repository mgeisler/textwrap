## Project Overview

This project is a Rust library named `textwrap` for wrapping and indenting text.
It is designed to be used in command-line programs to format dynamic output. The
library provides functions for wrapping text to a given width, filling text
(wrapping and joining with newlines), indenting, and dedenting.

The library is highly configurable and supports:

- **Optimal-fit algorithm:** for finding the best line breaks.
- **Hyphenation:** for about 70 languages.
- **Unicode support:** for handling non-ASCII characters correctly.
- **Terminal width detection:** for automatically adjusting the output to the
  terminal size.

The library is modular, and features can be enabled or disabled via Cargo
features to keep the binary size small.

## Building and Running

The project is a standard Rust library. It can be built and tested using
`cargo`.

### Building

To build the library, run:

```sh
cargo build
```

To build the library with all features, run:

```sh
cargo build --all-features
```

### Running Examples

The project includes a number of examples in the `examples` directory. To run an
example, use the `cargo run --example` command. For example, to run the
`interactive` example, which demonstrates most of the available features, run:

```sh
cargo run --example interactive
```

### Running Tests

To run the tests, run:

```sh
cargo test
```

To run the tests with all features, run:

```sh
cargo test --all-features
```

## Development Conventions

The project follows standard Rust conventions.

- **Formatting:** The code is formatted with `dprint fmt`. This will format all
  project files.
- **Linting:** The project uses `clippy` for linting.
- **Continuous Integration:** The project uses GitHub Actions for continuous
  integration. The configuration is in the `.github/workflows` directory.
- **Dependencies:** The project uses `dependabot` to keep dependencies up to
  date. The configuration is in the `.github/dependabot.yml` file.
- **Documentation:** The project has extensive documentation, which can be
  generated with `cargo doc`. The documentation is also available on
  [docs.rs](https://docs.rs/textwrap/).
