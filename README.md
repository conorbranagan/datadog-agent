# Datadog Agent

[![CircleCI](https://circleci.com/gh/DataDog/datadog-agent/tree/main.svg?style=svg)](https://circleci.com/gh/DataDog/datadog-agent/tree/main)
[![Windows unit tests](https://github.com/DataDog/datadog-agent/actions/workflows/windows-unittests.yml/badge.svg)](https://github.com/DataDog/datadog-agent/actions/workflows/windows-unittests.yml)
[![Coverage status](https://codecov.io/github/DataDog/datadog-agent/coverage.svg?branch=main)](https://codecov.io/github/DataDog/datadog-agent?branch=main)
[![GoDoc](https://godoc.org/github.com/DataDog/datadog-agent?status.svg)](https://godoc.org/github.com/DataDog/datadog-agent)
[![Go Report Card](https://goreportcard.com/badge/github.com/DataDog/datadog-agent)](https://goreportcard.com/report/github.com/DataDog/datadog-agent)

The present repository contains the source code of the Datadog Agent version 7 and version 6. Please refer to the [Agent user documentation](docs/agent) for information about differences between Agent v5, Agent v6 and Agent v7. Additionally, we provide a list of prepackaged binaries for an easy install process [here](https://app.datadoghq.com/account/settings#agent)

**Note:** the source code of Datadog Agent v5 is located in the
[dd-agent](https://github.com/DataDog/dd-agent) repository.

## Documentation

The general documentation of the project, including instructions for installation
and development, is located under [the docs directory](docs) of the present repo.

## Getting started

To build the Agent in Rust, you need:
* [Rust](https://www.rust-lang.org/tools/install) and Cargo (usually installed with Rust by default).
* CMake version 3.12 or later and a C++ compiler for building dependencies.

Builds are orchestrated with Cargo, Rust's package manager and build system. To build the Agent, follow these steps:

1. Clone the repository: `git clone https://github.com/conorbranagan/datadog-agent.git`.
2. Navigate into the project folder: `cd datadog-agent`.
3. Build the agent with `cargo build`.

To run the unit tests, use `cargo test` in the project directory.

Please refer to the [Rust Agent Developer Guide](docs/dev/rust/README.md) for more details on development and testing.

## Testing

Run unit tests using `cargo test`.

## Run

You can run the agent with:
