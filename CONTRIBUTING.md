# Contributing to Klang

Thank you for your interest in contributing to Klang! We welcome contributions from everyone. This document provides guidelines and instructions for contributing to the project.

## Development Setup

1. Ensure you have Rust and Cargo installed. If not, follow the instructions [here](https://www.rust-lang.org/tools/install).

2. Build the project:

```
cargo build
```

3. Run the tests:

```
cargo test
```

## Development Workflow

1. Create a new branch for your feature or bug fix:

```
git checkout -b feature-or-fix-name
```

2. Make your changes and commit them with a clear commit message.

3. Push your changes to your fork:

```
git push origin feature-or-fix-name
```

4. Open a pull request against the main repository.

## Coding Standards

- Follow the Rust style guide. You can use `rustfmt` to automatically format your code:

```
cargo fmt
```

- Run `clippy` to catch common mistakes and improve your code:

```
cargo clippy
```

## Running Klang

To run the Klang interpreter:
