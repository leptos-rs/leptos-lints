# leptos-lints

[Dylint] lints for apps created with the [Leptos] framework.

## Quick start

Install [Dylint] with

```sh
cargo install cargo-dylint dylint-link
```

Put the next configuration in the _Cargo.toml_ of your workspace.

```toml
[workspace.metadata.dylint]
libraries = [{ git = "https://github.com/leptos-rs/leptos-lints", tag = "v0.1.0" }]
```

Run the lints with

```sh
cargo dylint --all
```

See `cargo dylint --help` for more information.

## Configuration

### Workspace

```toml
[workspace.metadata.dylint]
libraries = [{ git = "https://github.com/leptos-rs/leptos-lints", tag = "v0.1.0" }]
```

### Lint levels

#### RUSTFLAGS

Use the `RUSTFLAGS` environment variable to set custom lint levels for each lint.

For example, to set [`leptos_print_stdout`] lint to `deny`, run the next command.

```sh
RUSTFLAGS="-Dleptos_print_stdout" cargo dylint --all
```

Or in the file _.cargo/config.toml_ to avoid repeating the command.

```toml
[target.'cfg(all())']
rustflags = ["-Dleptos_print_stdout"]
```

The downside of this approach is that the project will be compiled from scratch
every time you edit the `RUSTFLAGS` variable.

#### Cargo.toml

Use `[lints.rust]` table in _Cargo.toml_ to set custom lint levels for each lint.

For example, to set [`leptos_print_stdout`] lint to `deny` in a workspace,
add the next lines to the _Cargo.toml_ file.

```toml
[workspace.lints.rust]
unknown_lints = "allow"
leptos_print_stdout = "deny"
```

The downside of this approach is that unknown lints will be allowed by default.

## Lints

<!-- lints table start -->

| Rule | Description | Level |
|---|---|:-:|
| [`leptos_print_stdout`] | Check for calls to `leptos::logging::log!`. | Warn |

[`leptos_print_stdout`]: https://github.com/leptos-rs/leptos-lints/tree/main/lints/leptos_print_stdout#readme

<!-- lints table end -->

[Dylint]: https://github.com/trailofbits/dylint
[Leptos]: https://leptos.dev
