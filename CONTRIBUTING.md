# Contributing to Entropy Forge

Thank you for your interest in contributing to Entropy Forge! This document provides guidelines and instructions for contributing.

## Code of Conduct

Please be respectful and constructive in all interactions. We welcome contributors of all backgrounds and experience levels.

## How to Contribute

### Reporting Bugs

1. Check if the bug has already been reported in [Issues](https://github.com/rvoidex7/entropy-forge/issues)
2. If not, create a new issue using the Bug Report template
3. Include as much detail as possible

### Suggesting Features

1. Check existing issues for similar suggestions
2. Create a new issue using the Feature Request template
3. Describe the use case and potential implementation

### Submitting Code

1. **Fork** the repository
2. **Clone** your fork locally
3. **Create a branch** for your changes:
   ```bash
   git checkout -b feature/your-feature-name
   ```
4. **Make your changes** following the code style guidelines
5. **Test your changes**:
   ```bash
   cargo test
   cargo clippy
   cargo fmt --check
   ```
6. **Commit** with clear, descriptive messages
7. **Push** to your fork
8. **Open a Pull Request**

## Development Setup

### Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- Git

### Building

```bash
git clone https://github.com/rvoidex7/entropy-forge.git
cd entropy-forge
cargo build
```

### Running Tests

```bash
cargo test
```

### Running the Application

```bash
cargo run --release
```

## Code Style Guidelines

### Rust Style

- Follow the official [Rust Style Guide](https://doc.rust-lang.org/style-guide/)
- Use `cargo fmt` before committing
- Ensure `cargo clippy` passes without warnings
- Write documentation comments for public APIs

### Commit Messages

- Use present tense ("Add feature" not "Added feature")
- Use imperative mood ("Move cursor to..." not "Moves cursor to...")
- Limit the first line to 72 characters
- Reference issues when applicable

**Examples:**
```
feat: add step-by-step XOR visualization
fix: correct entropy calculation for edge cases
docs: update README with new screenshots
refactor: simplify cipher state management
```

### Code Organization

- Keep functions small and focused
- Add comments for complex logic
- Write tests for new functionality
- Update documentation as needed

## Project Structure

```
entropy-forge/
├── src/
│   ├── entropy/     # Entropy source trait and implementations
│   ├── crypto/      # Cryptographic operations
│   ├── quality/     # Quality metrics and NIST tests
│   ├── bench/       # Performance benchmarking
│   ├── learn/       # Educational visualizations
│   └── viz/         # GUI components
├── examples/        # Usage examples
└── tests/           # Integration tests
```

## Getting Help

- Open an issue for questions
- Check existing documentation
- Look at existing code for examples

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing! 🦀
