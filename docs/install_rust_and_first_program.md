# Installing Rust and Your First Program

## Installing Rust

To install Rust on your system, use the official installer script:

```bash
curl https://sh.rustup.rs -sSf | sh
```

This command will:

- Download the Rust installation script
- Install `rustup` (the Rust toolchain installer)
- Install the latest stable version of Rust
- Set up your PATH environment variable

To verify the installation, check the Rust version:

```bash
rustc --version
```

## Next Steps

Once Rust is installed, you'll be ready to start developing Rust applications and exploring eBPF programming with Rust.

### Compiling Your First Program

To compile a Rust program, use the `rustc` compiler. It's a good practice to organize your binaries in a separate folder:

```bash
rustc -o bin/first_program src/first_program.rs
```

This command will:

- Create a `bin` directory (if it doesn't exist)
- Compile the Rust source file `src/first_program.rs`
- Create an executable named `first` in the `bin` directory

### Variables and type inference

`let myvar = "inferred"`

### Read

- loops_and_structures.rs
- sequences.rs
- cargo.md
- trait_impl.rs // Traits and implementations Rust ~ interfaces and classes OOP
