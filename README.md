# laks

[![image](https://img.shields.io/pypi/l/ruff.svg)](https://github.com/astral-sh/ruff/blob/main/LICENSE)
[![CI](https://github.com/geyerandreas/laks/actions/workflows/ci.yml/badge.svg)](https://github.com/geyerandreas/laks/actions/workflows/ci.yml)

## Getting Started

To start the development server you just run

```sh
bacon run-long
```

## Using custom settings

You can generate custom settings via

```sh
cargo run -p laks config init
```

This will create a `laks.toml`.
To use this while developing, run the following:

```sh
bacon run--long -- -c laks.toml
```
