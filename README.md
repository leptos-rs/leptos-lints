# leptos-lints

[Dylint] lints for apps created with the [Leptos] framework.

## Installation

Install [Dylint] with

```sh
cargo install cargo-dylint dylint-link
```

## Configuration

### Workspace

Put the next configuration in the _Cargo.toml_ of your workspace.

```toml
[workspace.metadata.dylint]
libraries = [{ git = "https://github.com/leptos-rs/leptos-lints", tag = "v0.1.0" }]
```

## Usage

Run the lints with

```sh
cargo dylint --all
```

See `cargo dylint --help` for more options.

## Lints

| Rule                    | Description                                 |
| ----------------------- | ------------------------------------------- |
| [`leptos_print_stdout`] | Check for calls to `leptos::logging::log!`. |

[`leptos_print_stdout`]: https://github.com/leptos-rs/leptos-lints/tree/main/lints/leptos_print_stdout#readme

## Configuring lint levels

### RUSTFLAGS

Lint levels can be configured using the `RUSTFLAGS` environment variable.
For example, to set the `leptos_print_stdout` lint to `allow`, run the next command.

```sh
RUSTFLAGS="-Aleptos_print_stdout" cargo dylint --all
```

[Dylint]: https://github.com/trailofbits/dylint
[Leptos]: https://leptos.dev
