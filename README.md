# cargo-trust

<p align="center">
  <a href="https://github.com/rust-lang/cargo">
    <img src="https://img.shields.io/badge/cargo-subcommand-orange?style=flat&logo=rust" alt="Cargo Subcommand" />
  </a>
  <a href="https://github.com/clap-rs/clap">
    <img src="https://img.shields.io/badge/cli%20with-clap-blue?style=flat" alt="CLI with Clap" />
  </a>
  <a href="https://github.com/BurntSushi/walkdir">
    <img src="https://img.shields.io/badge/traversal-walkdir-green?style=flat" alt="Traversal with WalkDir" />
  </a>
  <a href="https://github.com/serde-rs/serde">
    <img src="https://img.shields.io/badge/serialization-serde-red?style=flat" alt="Serialization with Serde" />
  </a>
  <br />
  <a href="https://github.com/rust-lang/rustfmt">
    <img src="https://img.shields.io/badge/code--style-rustfmt-fc8d62?style=flat" alt="Code Style: rustfmt" />
  </a>
  <a href="https://github.com/rust-lang/rust-clippy">
    <img src="https://img.shields.io/badge/linted%20with-clippy-ffc832?style=flat" alt="Linted with Clippy" />
  </a>
  <a href="https://doc.rust-lang.org/edition-guide/rust-2024/index.html">
    <img src="https://img.shields.io/badge/edition-2024-brightgreen?style=flat&logo=rust" alt="Rust 2024 Edition" />
  </a>
  <br />
  <br />
</p>

A security-focused Cargo subcommand for auditing `unsafe` code in Rust projects. Recursively scans your codebase to identify and report all unsafe blocks, functions, traits, and implementations.

## Why cargo-trust?

Rust's memory safety guarantees are bypassed when using `unsafe` code. While sometimes necessary for FFI, performance optimizations, or low-level operations, unsafe code introduces potential security vulnerabilities that can be difficult to track across a growing codebase.

`cargo-trust` makes unsafe code visible, helping teams maintain security awareness and audit unsafe usage patterns.

## Installation

```bash
cargo install cargo-trust
```

## Usage

```bash
# Audit current project
cargo trust

# Audit specific directory
cargo trust /path/to/project

# JSON output for tooling integration
cargo trust --format json

# Compact output for grep/scripts
cargo trust --format compact
```

## What it detects

- `unsafe fn` - Unsafe functions
- `unsafe {}` - Unsafe blocks
- `unsafe trait` - Unsafe trait definitions  
- `unsafe impl` - Unsafe trait implementations
- `extern "C"` - Foreign function interfaces

## Output formats

### Human (default)
```
╔══════════════════════════════════════╗
║     cargo-trust Security Report      ║
╚══════════════════════════════════════╝

► src/ffi.rs
  ├─ unsafe function at 42:1
  │  unsafe fn raw_pointer_deref() -> i32 {
  ├─ unsafe block at 67:5
  │  unsafe { *ptr.add(offset) }
```

### Compact
```
src/ffi.rs:42:1: unsafe function
src/ffi.rs:67:5: unsafe block
```

### JSON
```json
{
  "project_root": ".",
  "files_with_unsafe": {
    "src/ffi.rs": [
      {
        "line": 42,
        "column": 1,
        "kind": "Function",
        "context": "unsafe fn raw_pointer_deref() -> i32 {"
      }
    ]
  },
  "total_unsafe_count": 1,
  "errors": []
}
```

## Integration

Perfect for CI/CD pipelines, pre-commit hooks, and security audits:

```bash
# Fail CI if unsafe code is found
cargo trust && echo "No unsafe code detected" || exit 1

# Generate audit reports
cargo trust --format json > unsafe-audit.json
```

## Architecture

- **Zero-copy parsing** - Efficient line-by-line analysis
- **Walkdir traversal** - Recursive filesystem scanning with smart filtering
- **Trait-based formatters** - Extensible output system
- **Serde serialization** - Structured data for tooling integration

## Development

```bash
# Build
cargo build --release

# Test
cargo test

# Install locally
cargo install --path .
```

## License

MIT OR Apache-2.0