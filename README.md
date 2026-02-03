# zk-rust-api

<div align="center">

<img width="512" height="512" alt="shui" src="https://github.com/user-attachments/assets/32b62642-a248-4036-bfbe-4bc982f1d756" />

**A high-performance zero-knowledge proof API built with Halo2**

[![Crates.io](https://img.shields.io/crates/v/zk-rust-api.svg)](https://crates.io/crates/zk-rust-api)
[![Documentation](https://docs.rs/zk-rust-api/badge.svg)](https://docs.rs/zk-rust-api)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://github.com/yourusername/zk-rust-api/workflows/CI/badge.svg)](https://github.com/yourusername/zk-rust-api/actions)
[![codecov](https://codecov.io/gh/yourusername/zk-rust-api/branch/main/graph/badge.svg)](https://codecov.io/gh/yourusername/zk-rust-api)
[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org)

[Documentation](https://docs.rs/zk-rust-api) | [Examples](./examples) | [Contributing](./CONTRIBUTING.md)

</div>

## Features

- Zero-knowledge proof generation and verification using Halo2
- Type-safe API with comprehensive error handling
- High-performance implementation with optimized release builds
- Comprehensive logging and tracing support
- Well-documented with examples and API documentation

## Quick Start

### Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
zk-rust-api = "0.1.0"
```

### Basic Usage

```rust
use zk_rust_api::*;

fn main() {
    // Your ZK proof implementation here
    println!("Hello, ZK world!");
}
```

For more examples, see the [examples directory](./examples).

## Building from Source

### Prerequisites

- Rust 1.70 or later
- Cargo

### Build

```bash
# Clone the repository
git clone https://github.com/yourusername/zk-rust-api.git
cd zk-rust-api

# Build in release mode
cargo build --release

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run
```

## Development

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### Benchmarking

```bash
cargo bench
```

### Code Quality

```bash
# Format code
cargo fmt

# Run linter
cargo clippy -- -D warnings

# Check for security vulnerabilities
cargo audit
```

## Documentation

Generate and open documentation locally:

```bash
cargo doc --open
```

## Project Structure

```
zk-rust-api/
├── src/           # Source code
├── examples/      # Usage examples
├── benches/       # Benchmarks
├── docs/          # Additional documentation
├── tests/         # Integration tests
└── target/        # Build artifacts (gitignored)
```

## Performance

This library is optimized for performance with:

- LTO (Link Time Optimization) enabled
- Single codegen unit for better optimization
- Stripped binaries for smaller size
- Optimized dependency compilation

## Security

For security concerns, please see our [Security Policy](./SECURITY.md).

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

## Code of Conduct

This project adheres to the Contributor Covenant [Code of Conduct](./CODE_OF_CONDUCT.md).

## License

Licensed under the MIT License. See [LICENSE](./LICENSE) for details.

## Acknowledgments

- Built with [Halo2](https://github.com/zcash/halo2) - A zero-knowledge proof system
- Inspired by the Rust cryptography community

## Changelog

See [CHANGELOG.md](./CHANGELOG.md) for version history and changes.

---
