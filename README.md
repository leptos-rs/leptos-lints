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

## Lints

| Rule                    | Description                                 |
| ----------------------- | ------------------------------------------- |
| [`leptos_print_stdout`] | Check for calls to `leptos::logging::log!`. |

[`leptos_print_stdout`]: https://github.com/leptos-rs/leptos-lints/tree/main/lints/leptos_print_stdout#readme

[Dylint]: https://github.com/trailofbits/dylint
[Leptos]: https://leptos.dev
