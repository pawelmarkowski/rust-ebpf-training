## Prerequsites

If you installed rustup with script, cargo is already installed on your laptop

```bash
cargo --help
```

## Project initialization

```bash
cargo init
cargo init asdf --lib
```

above script creates `Cargo.toml` file and `src/main.rs`

```bash
cargo update
```

The `cargo update` command updates the dependencies listed in your `Cargo.toml` file to the latest versions allowed by the version constraints.

```bash
cargo build
cargo build -release
cargo test
cargo run
```
