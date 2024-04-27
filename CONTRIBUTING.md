# Contributing to the Datadog Agent (Rust Rewrite)

Thank you for your interest in contributing to the Datadog Agent! This document provides guidelines and instructions for contributing to the Rust rewrite of the Datadog Agent.

## Setting Up Your Development Environment

To contribute to the Rust rewrite of the Datadog Agent, you will need to set up your development environment:

1. Install Rust and Cargo using [rustup](https://rustup.rs/). This will install the latest stable version of Rust and Cargo, Rust's package manager and build system.
2. Fork the [Datadog Agent repository](https://github.com/DataDog/datadog-agent) and clone your fork to your local machine.
3. Navigate to the cloned repository and switch to the `rust-rewrite` branch.

## Building the Project

To build the Agent, run the following command in the root directory of the project:

```sh
cargo build
```

This will compile the project and produce an executable in the `target/debug` directory.

## Running Tests

To ensure that your changes do not break existing functionality, run the tests using the following command:

```sh
cargo test
```

Please make sure all tests pass before submitting a pull request.

## Contributing Code

When you're ready to contribute your changes, please keep the following in mind:

- Write clear, concise commit messages that explain the changes made.
- Make sure your code adheres to the Rust [style guide](https://doc.rust-lang.org/book/appendix-04-useful-development-tools.html#rustfmt-for-formatting-code) and best practices.
- Use `cargo fmt` to format your code before committing.
- Use `cargo clippy` to catch common mistakes and improve your Rust code.
- Create a new branch for your changes and push it to your fork.
- Submit a pull request to the `rust-rewrite` branch of the original repository.

## Review Process

After you submit a pull request, the maintainers will review your changes. They may provide feedback or request changes before merging your contribution. Please be responsive to their feedback to ensure a smooth review process.

## Additional Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust and WebAssembly](https://rustwasm.github.io/docs/book/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

We appreciate your contributions to making the Datadog Agent better!
