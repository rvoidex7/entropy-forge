# Build Instructions

## Prerequisites

- Rust 1.70+ (install from https://rustup.rs/)
- C compiler (for dependencies)

## Quick Start

### 1. Clone and Build

```bash
cd entropy-weaver
cargo build --release
```

### 2. Run the GUI

```bash
cargo run --release
```

The GUI will open with three tabs:
- **Use**: Stream cipher with visualization
- **Test**: Quality metrics and NIST tests
- **Benchmark**: Performance measurement

### 3. Run Examples

```bash
# Basic usage example
cargo run --release --example basic_usage

# Custom entropy source example
cargo run --release --example custom_source

# Quality check with NIST tests
cargo run --release --example quality_check
```

## Development

### Run Tests

```bash
cargo test
```

### Run with Debug Info

```bash
cargo run
```

### Build Documentation

```bash
cargo doc --open
```

## Project Structure

```
entropy-weaver/
├── Cargo.toml           # Dependencies and metadata
├── src/
│   ├── lib.rs          # Library root
│   ├── main.rs         # GUI binary
│   ├── entropy/        # EntropySource trait
│   ├── crypto/         # Stream cipher
│   ├── quality/        # Metrics and NIST tests
│   ├── bench/          # Performance tests
│   └── viz/            # GUI (egui)
├── examples/           # Usage examples
└── README.md          # Documentation
```

## Features

The project has the following features:

- `default`: Includes GUI
- `gui`: GUI visualization (egui)

To build without GUI:

```bash
cargo build --release --no-default-features
```

## Troubleshooting

### Linux: Missing libraries

If you get linking errors on Linux, install development packages:

```bash
# Ubuntu/Debian
sudo apt-get install libgtk-3-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev

# Fedora
sudo dnf install gtk3-devel
```

### Windows: MSVC or GNU

Use either:
- MSVC toolchain (default, requires Visual Studio)
- GNU toolchain: `rustup default stable-gnu`

### macOS: Works out of the box

No additional dependencies needed.

## Performance Tips

For maximum performance:

```bash
# Use release mode
cargo run --release

# For benchmarking, use:
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

## Next Steps

1. Open the GUI and explore the three tabs
2. Try the examples to understand the API
3. Implement your own `EntropySource`
4. Run quality tests on your source

Happy coding! 🦀
