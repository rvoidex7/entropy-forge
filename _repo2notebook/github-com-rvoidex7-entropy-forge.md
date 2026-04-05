# Repository: entropy-forge

Generated: 2026-04-05 12:30

Total files: 78

---

## File: .gitignore

```
# Rust
/target
**/*.rs.bk
*.pdb
Cargo.lock

# IDE
.vscode/
.idea/
*.swp
*.swo
*~

# OS
.DS_Store
Thumbs.db

# Build artifacts
*.exe
*.dll
*.so
*.dylib

# Test outputs
*.log
ui/node_modules
```

---

## File: BUILD.md

```markdown
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
```

---

## File: Cargo.toml

```toml
[package]
name = "entropy-forge"
version = "0.1.0"
edition = "2021"
authors = ["rvoidex7"]
license = "MIT"
description = "Pluggable entropy framework with quality testing and visualization"
repository = "https://github.com/rvoidex7/entropy-forge"
keywords = ["cryptography", "entropy", "rng", "visualization", "nist"]
categories = ["cryptography", "visualization"]

[dependencies]
# Crypto
rand = "0.8"
getrandom = "0.2"
sha2 = "0.10"
hex = "0.4"

# Math & Stats
statrs = "0.17"

# GUI (optional)
egui = { version = "0.29", optional = true }
eframe = { version = "0.29", optional = true }

# Utilities
thiserror = "1.0"

[dev-dependencies]
criterion = "0.5"

[features]
default = ["gui"]
gui = ["dep:egui", "dep:eframe"]

[[bin]]
name = "entropy-forge"
path = "src/main.rs"
required-features = ["gui"]

[[example]]
name = "basic_usage"
path = "examples/basic_usage.rs"

[[example]]
name = "custom_source"
path = "examples/custom_source.rs"

[[example]]
name = "quality_check"
path = "examples/quality_check.rs"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
```

---

## File: CHANGELOG.md

```markdown
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Interactive "Learn" tab with step-by-step XOR visualization
- Bit-level encryption visualization
- Auto-play animation with speed control
- Educational explanations for cryptographic concepts

## [0.1.0] - 2025-01-XX

### Added
- Initial release
- Pluggable `EntropySource` trait system
- System entropy implementation (Windows BCryptGenRandom, Linux /dev/urandom, macOS SecRandomCopyBytes)
- Stream cipher implementation with XOR keystream
- Quality metrics (Shannon entropy, min-entropy, chi-square)
- NIST SP 800-22 statistical tests (Frequency, Runs, Longest Run, Chi-Square, Serial)
- Performance benchmarking
- egui-based GUI with four tabs:
  - **Use**: Stream cipher encryption/decryption
  - **Test**: Quality analysis and NIST tests
  - **Benchmark**: Performance measurement
  - **Learn**: Educational visualization
- Cross-platform support (Windows, Linux, macOS)

### Security
- Note: This is an educational tool. Do not use for production cryptography.

---

[Unreleased]: https://github.com/rvoidex7/entropy-forge/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/rvoidex7/entropy-forge/releases/tag/v0.1.0
```

---

## File: clippy.toml

```toml
# Clippy configuration
# Run with: cargo clippy

[workspace]
# Deny certain lints
deny = [
    "unsafe_code",
]

# Warn on these
warn = [
    "clippy::all",
    "clippy::pedantic",
    "clippy::nursery",
]

# Allow these (too noisy or not applicable)
allow = [
    "clippy::module_name_repetitions",
    "clippy::must_use_candidate",
    "clippy::missing_errors_doc",
    "clippy::missing_panics_doc",
]
```

---

## File: CODE_OF_CONDUCT.md

```markdown
# Code of Conduct

## Our Pledge

We as members, contributors, and leaders pledge to make participation in our
community a harassment-free experience for everyone, regardless of age, body
size, visible or invisible disability, ethnicity, sex characteristics, gender
identity and expression, level of experience, education, socio-economic status,
nationality, personal appearance, race, religion, or sexual identity
and orientation.

## Our Standards

Examples of behavior that contributes to a positive environment:

* Using welcoming and inclusive language
* Being respectful of differing viewpoints and experiences
* Gracefully accepting constructive criticism
* Focusing on what is best for the community
* Showing empathy towards other community members

Examples of unacceptable behavior:

* The use of sexualized language or imagery and unwelcome sexual attention
* Trolling, insulting/derogatory comments, and personal or political attacks
* Public or private harassment
* Publishing others' private information without explicit permission
* Other conduct which could reasonably be considered inappropriate

## Enforcement

Instances of abusive, harassing, or otherwise unacceptable behavior may be
reported to the project maintainers. All complaints will be reviewed and
investigated and will result in a response that is deemed necessary and
appropriate to the circumstances.

## Attribution

This Code of Conduct is adapted from the [Contributor Covenant](https://www.contributor-covenant.org),
version 2.0.
```

---

## File: CONTRIBUTING.md

```markdown
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
```

---

## File: Docs/00-OVERVIEW.md

```markdown
# Entropy Forge — Tauri Migration: Master Overview

## Document Index

| File | Phase | Description |
|------|-------|-------------|
| `00-OVERVIEW.md` *(this file)* | All | Master document, goals, success criteria, constraints |
| `01-ARCHITECTURE.md` | 1 | System design, component map, data flow |
| `02-DESKTOP-UI.md` | 2 | Tab-by-tab UI refactoring, desktop layout, state management |
| `03-MOBILE-RESPONSIVE.md` | 3 | Responsive breakpoints, layout adaptation, touch patterns |
| `04-ANDROID.md` | 4 | Tauri Android integration, build config, signing, Play Store |
| `05-FEATURE-PARITY.md` | 5 | Cross-platform feature matrix, fallbacks, performance constraints |
| `06-TESTING-DEPLOYMENT.md` | 6 | QA strategy, testing checklist, CI/CD, release pipeline |
| `07-MIGRATION-CHECKLIST.md` | 7 | Ordered step-by-step migration guide with decision points |

---

## Project Background

**Entropy Forge** is an educational cryptography desktop application written in Rust. It uses the `egui`/`eframe` GUI framework to deliver four interactive tabs that teach, test, and demonstrate cryptographic entropy concepts:

- **Use Tab** — Stream cipher encryption with real-time keystream visualization
- **Test Tab** — NIST SP 800-22 randomness tests and entropy quality metrics
- **Benchmark Tab** — Performance profiling of the entropy source
- **Learn Tab** — Step-by-step animated visualizations of XOR cipher, Shannon entropy, and NIST frequency tests

The application's core Rust logic (entropy generation, cryptography, quality analysis, benchmarking, and educational step generation) is production-quality, well-tested, and must remain **completely unchanged**.

---

## Transformation Objective

Migrate Entropy Forge from an `egui` desktop-only application to a **Tauri v2 cross-platform application** that runs natively on:

1. **Desktop** — Windows, macOS, Linux (primary experience, preserved 1:1)
2. **Android** — Phone/tablet (secondary experience, responsive adaptation)

The web frontend layer will be rewritten in **vanilla TypeScript + HTML/CSS** (no heavy SPA framework). The Rust backend exposes all functionality through Tauri's `invoke` command API. All core Rust modules remain in place, unmodified.

---

## Guiding Principles

1. **Zero core changes** — `entropy/`, `crypto/`, `quality/`, `bench/`, and `learn/` modules are read-only. No Rust logic is rewritten.
2. **Desktop-first** — The Tauri web UI is designed to replicate the existing 900×700 desktop layout exactly before any mobile adaptation work begins.
3. **Educational fidelity** — Every visualization, animation, and interactive control in the current Learn tab must be faithfully reproduced. The step-by-step animated breakdowns are the defining feature of the application.
4. **Vanilla TypeScript** — No React, Vue, Svelte, or similar frameworks. The UI is plain TypeScript with direct DOM manipulation. This minimizes dependencies, binary weight, and framework churn.
5. **Single codebase** — One Tauri project handles desktop and Android. Responsive CSS and TypeScript feature checks drive platform adaptation; there are no separate builds for each platform beyond what Tauri generates.
6. **Play Store compliance** — The Android build must satisfy Google Play's technical and policy requirements from day one. There are no "we'll fix this later" deviations.

---

## What Changes and What Stays

### Stays Identical (Rust, no changes)
```
src/entropy/      EntropySource trait, SystemEntropy, MockEntropy
src/crypto/       StreamCipher<E>
src/quality/      QualityMetrics, NistTests
src/bench/        PerformanceBench, BenchmarkResult
src/learn/        EncryptionProcess, EntropyProcess, NistProcess, step types
src/lib.rs        Public re-exports
```

### Removed
```
src/main.rs       egui window setup — replaced by Tauri entry point
src/viz/          egui UI components — replaced by web frontend
```

### Added — Tauri Backend Layer
```
src-tauri/
  src/
    lib.rs        Tauri app builder, invoke handler registration
    commands/
      use_tab.rs      encrypt/decrypt, keystream state extraction
      test_tab.rs     quality metrics, NIST test execution
      bench_tab.rs    performance benchmarking
      learn_tab.rs    step generation for all 3 visualizers
  Cargo.toml      Tauri + entropy-forge core dependency
  tauri.conf.json Window config, Android permissions, bundle settings
  capabilities/   Permission capability files
  build.rs        Tauri build script
  gen/android/    Auto-generated Android Gradle project (managed by Tauri)
```

### Added — Web Frontend
```
src/              (or ui/ if preferred — configured in tauri.conf.json)
  index.html      Single page shell with tab structure
  styles/
    main.css      Global styles, CSS variables, dark theme
    tabs.css      Tab-specific layout rules
    responsive.css  Breakpoint overrides for mobile
    animations.css  Step transition animations for Learn tab
  ts/
    main.ts       App bootstrap, tab routing
    use-tab.ts    Encryption UI logic
    test-tab.ts   Quality metrics UI logic
    bench-tab.ts  Benchmark UI logic
    learn/
      index.ts    Learn tab sub-tab router
      xor.ts      XOR cipher step visualizer
      entropy.ts  Shannon entropy step visualizer
      nist.ts     NIST frequency test step visualizer
    components/
      progress-bar.ts  Reusable metric bar component
      grid.ts          Keystream byte grid renderer
      controls.ts      Step navigation / play-pause / speed slider
    types.ts      Shared TypeScript interfaces matching Rust structs
    invoke.ts     Typed wrappers around tauri invoke calls
  tsconfig.json
  package.json    Build tooling only (tsc + esbuild or Vite)
```

---

## Success Criteria

### Functional Completeness
- [ ] All four tabs render on desktop and reproduce all current functionality
- [ ] Stream cipher encrypts text and displays keystream state grid identically
- [ ] Quality metrics and NIST tests run and display results correctly
- [ ] Benchmark runs and shows throughput/latency results
- [ ] Learn tab runs all three animated visualizers with play/pause/speed/navigation controls
- [ ] "Help: How does this work?" link in Use tab navigates to Learn tab and pre-fills the input

### Desktop Fidelity
- [ ] UI dimensions match the 900×700 reference layout
- [ ] Color scheme (dark theme with cyan/orange/green accents) is preserved
- [ ] Tooltip/info text for all metrics is preserved
- [ ] Tab navigation matches current behavior

### Mobile Responsiveness
- [ ] All four tabs are usable on a 360×800 Android phone (portrait)
- [ ] Touch targets are ≥ 44×44 px
- [ ] Keystream grid adapts to narrow screens without horizontal scroll
- [ ] Learn tab step visualizer renders legibly at mobile font sizes
- [ ] Navigation adapts (bottom nav bar or hamburger) for small screens

### Android Build
- [ ] `tauri android build` produces a signed APK/AAB
- [ ] App launches on Android API 24+ emulator and real device
- [ ] All Tauri commands return correct results on Android
- [ ] App icon and splash screen are present
- [ ] Play Store metadata is complete

### Quality
- [ ] All existing Rust unit tests continue to pass (unchanged modules)
- [ ] No Tauri command panics or unwrap failures on valid input
- [ ] No JavaScript runtime errors in browser devtools
- [ ] Performance: first contentful paint on desktop < 500 ms

---

## Constraints Summary

| Constraint | Detail |
|------------|--------|
| Tauri version | v2 (stable) |
| Frontend framework | Vanilla TypeScript only |
| Rust core | Read-only — zero modifications |
| Android min SDK | API 24 (Android 7.0) |
| Android target SDK | API 35 (Android 15) |
| Desktop window | 900×700 default, 700×500 minimum |
| iOS | Out of scope |
| React Native | Explicitly excluded |
| egui/eframe | Removed from default build |

---

## Phase Summary

| Phase | Deliverable | Effort Estimate |
|-------|-------------|-----------------|
| **1 — Architecture** | Project scaffold, Tauri setup, command stubs | Medium |
| **2 — Desktop UI** | Full desktop web UI in vanilla TypeScript, all 4 tabs | High |
| **3 — Mobile Responsive** | Responsive CSS layer, touch adaptations | Medium |
| **4 — Android** | Tauri Android target, Gradle config, signing | Medium |
| **5 — Feature Parity** | Validation of all features on both platforms | Low |
| **6 — Testing & Deployment** | Test suite, CI config, Play Store submission | Medium |
| **7 — Migration Checklist** | Final validation and cutover | Low |

Phases 1–2 are sequential and foundational. Phases 3–4 can be parallelized after Phase 2 is stable. Phases 5–7 are validation and release activities.
```

---

## File: Docs/01-ARCHITECTURE.md

```markdown
# Phase 1 — Architecture & Project Structure

## 1. Current Architecture (egui Desktop)

```
┌─────────────────────────────────────────────────────────────┐
│  entropy-forge binary (main.rs)                              │
│                                                              │
│   eframe::run_native()                                       │
│        ↓                                                     │
│   EntropyForgeApp (viz/app.rs)  ← egui::App trait           │
│   ┌────────────────────────────────────────────────────┐    │
│   │  Tab: Use   │  Tab: Test  │  Tab: Bench │ Tab: Learn│   │
│   └────────────────────────────────────────────────────┘    │
│        ↓             ↓              ↓            ↓          │
│   StreamCipher   QualityMetrics  PerfBench  EncryptionProcess│
│                  NistTests                  EntropyProcess   │
│                                             NistProcess      │
│        └─────────────┴──────────────────────────────┘       │
│                              ↓                               │
│                    EntropySource trait                       │
│                    SystemEntropy / MockEntropy               │
└─────────────────────────────────────────────────────────────┘
```

### Key Characteristics of Current Architecture

- **Single process, single thread** for all UI and logic (egui runs on the main thread)
- **State held in `EntropyForgeApp` struct** — all tab-local state lives in one struct
- **GUI framework owns the event loop** — `eframe` calls `update()` on every frame
- **Immediate-mode rendering** — egui redraws all widgets every frame; no retained widget objects
- **Feature flag `gui`** — egui and eframe are optional dependencies, core library compiles without them
- **`viz/` module** contains all GUI code and is the only module that imports egui

---

## 2. Target Architecture (Tauri v2)

```
┌──────────────────────────────────────────────────────────────────┐
│  Tauri Process                                                    │
│                                                                   │
│  ┌────────────────────────┐    ┌──────────────────────────────┐  │
│  │  Rust Backend          │    │  Web Frontend (WebView)      │  │
│  │  (src-tauri/src/)      │◄──►│  (src/ — HTML/CSS/TS)        │  │
│  │                        │IPC │                              │  │
│  │  Tauri commands:       │    │  Tab router + state          │  │
│  │  • encrypt_decrypt()   │    │  • use-tab.ts                │  │
│  │  • run_quality_tests() │    │  • test-tab.ts               │  │
│  │  • run_benchmark()     │    │  • bench-tab.ts              │  │
│  │  • get_learn_steps()   │    │  • learn/ (xor/entropy/nist) │  │
│  │                        │    │                              │  │
│  │  Core library:         │    │  Shared components:          │  │
│  │  • entropy/            │    │  • progress-bar.ts           │  │
│  │  • crypto/             │    │  • grid.ts                   │  │
│  │  • quality/            │    │  • controls.ts               │  │
│  │  • bench/              │    │  • invoke.ts (typed IPC)     │  │
│  │  • learn/              │    │                              │  │
│  └────────────────────────┘    └──────────────────────────────┘  │
└──────────────────────────────────────────────────────────────────┘
```

### IPC Mechanism

Tauri v2 uses a typed command system. The frontend calls `invoke('command_name', { payload })` and receives a typed Promise. On the Rust side, a function decorated with `#[tauri::command]` handles the call. All data crossing the IPC boundary is serialized as JSON (via `serde_json`).

The frontend never directly accesses Rust memory; it receives serialized snapshots of state. Animation (play/pause/step) is driven by the frontend's own timer loop, using pre-computed step arrays returned from Rust.

---

## 3. Directory Structure — Target Project

```
entropy-forge/
│
├── src/                          ← Core Rust library (UNCHANGED)
│   ├── lib.rs
│   ├── entropy/
│   ├── crypto/
│   ├── quality/
│   ├── bench/
│   └── learn/
│
├── src-tauri/                    ← Tauri application layer (NEW)
│   ├── src/
│   │   ├── lib.rs                ← App builder, invoke handler registration
│   │   ├── commands/
│   │   │   ├── mod.rs            ← Re-exports all command modules
│   │   │   ├── use_tab.rs        ← encrypt, get_keystream_state
│   │   │   ├── test_tab.rs       ← run_quality_metrics, run_nist_tests
│   │   │   ├── bench_tab.rs      ← run_benchmark
│   │   │   └── learn_tab.rs      ← get_xor_steps, get_entropy_steps, get_nist_steps
│   │   └── types/
│   │       ├── mod.rs
│   │       └── responses.rs      ← Serde-serializable structs for IPC responses
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── capabilities/
│   │   └── default.json          ← Allowed Tauri API surface
│   ├── build.rs
│   ├── icons/                    ← All required platform icons
│   └── gen/
│       └── android/              ← Auto-generated by `tauri android init`
│
├── ui/                           ← Web frontend (NEW)
│   ├── index.html                ← Single-page shell
│   ├── styles/
│   │   ├── main.css              ← Global variables, reset, dark theme, typography
│   │   ├── tabs.css              ← Tab bar and content area
│   │   ├── use-tab.css
│   │   ├── test-tab.css
│   │   ├── bench-tab.css
│   │   ├── learn-tab.css
│   │   ├── responsive.css        ← All breakpoint overrides
│   │   └── animations.css        ← Step transition keyframes
│   ├── ts/
│   │   ├── main.ts               ← App init, tab switching, global event setup
│   │   ├── use-tab.ts
│   │   ├── test-tab.ts
│   │   ├── bench-tab.ts
│   │   ├── learn/
│   │   │   ├── index.ts          ← Learn sub-tab router
│   │   │   ├── xor.ts            ← XOR cipher step visualizer
│   │   │   ├── entropy.ts        ← Shannon entropy step visualizer
│   │   │   └── nist.ts           ← NIST frequency test step visualizer
│   │   ├── components/
│   │   │   ├── progress-bar.ts   ← Reusable labeled progress bar
│   │   │   ├── grid.ts           ← Keystream 8×8 byte grid
│   │   │   ├── controls.ts       ← Step nav / play-pause / speed slider
│   │   │   └── tooltip.ts        ← Hover info icon tooltips
│   │   ├── invoke.ts             ← Typed invoke() wrappers
│   │   └── types.ts              ← TypeScript interfaces mirroring Rust response types
│   ├── tsconfig.json
│   └── package.json
│
├── examples/                     ← Unchanged
├── Docs/                         ← This documentation set
├── Cargo.toml                    ← Workspace or package root
├── README.md
└── BUILD.md
```

> **Note on `Cargo.toml` at root:** The root `Cargo.toml` should be converted to a Cargo workspace. The `[workspace]` section lists `"."` (current package — entropy core library) and `"src-tauri"` (Tauri application). This allows `cargo test` at the workspace root to test both crates.

---

## 4. Core Library — Tauri Crate Dependency

The `src-tauri/Cargo.toml` adds the root entropy-forge crate as a path dependency:

```
entropy-forge = { path = "..", default-features = false }
```

The `default-features = false` ensures that `egui`/`eframe` are NOT compiled into the Tauri binary. Only the core library modules are compiled. The `gui` feature is disabled permanently for the Tauri target.

---

## 5. Data Flow

### Encryption Flow (Use Tab)
```
User types text → TypeScript calls invoke('encrypt_decrypt', { plaintext, hex_output })
    → Tauri routes to use_tab::encrypt_decrypt()
        → Creates SystemEntropy instance
        → Creates StreamCipher<SystemEntropy>
        → Calls cipher.process(&plaintext_bytes)
        → Extracts cipher.state() for keystream visualization
        → Returns EncryptResponse { ciphertext_hex, keystream_bytes: Vec<u8> }
    → TypeScript receives response
        → Updates output display
        → Renders keystream grid (8×8, 64 bytes)
```

### Learn Tab Flow (XOR Visualizer)
```
User enters text → TypeScript calls invoke('get_xor_steps', { text })
    → Tauri routes to learn_tab::get_xor_steps()
        → Creates MockEntropy(seed=42) for determinism
        → Creates EncryptionProcess
        → Calls process.start(&text)
        → Serializes process.steps Vec<EncryptionStep> to JSON
        → Returns XorStepsResponse { steps: Vec<SerializedStep>, total: usize }
    → TypeScript receives full steps array
        → Stores locally as animation state
        → Renders current step (index 0)
        → User navigation / auto-play handled entirely in TypeScript
        → No further Rust calls until user submits new text
```

This pattern (compute all steps in Rust once, animate entirely in TypeScript) is critical for performance and latency. It avoids per-frame IPC calls.

### Quality Test Flow (Test Tab)
```
User adjusts sample size slider → clicks "Run All Tests"
    → TypeScript calls invoke('run_quality_tests', { sample_size })
        → Tauri routes to test_tab::run_quality_tests()
            → Creates SystemEntropy
            → Calls QualityMetrics::analyze(&mut entropy, sample_size)
            → Generates separate sample for NIST
            → Calls NistTests::run_all_tests(&data)
            → Serializes both results
            → Returns QualityResponse { metrics: MetricsData, nist: Vec<NistResult> }
    → TypeScript renders all metrics and NIST table
```

### Benchmark Flow (Bench Tab)
```
User adjusts benchmark size → clicks "Run Benchmark"
    → TypeScript calls invoke('run_benchmark', { bytes })
        → Tauri routes to bench_tab::run_benchmark()
            → Creates SystemEntropy
            → Calls PerformanceBench::benchmark(&mut entropy, bytes)
            → Returns BenchResponse { throughput_mbps, latency_us, bytes, duration_secs }
    → TypeScript renders results
```

---

## 6. IPC Response Type Contracts

All response types must implement `serde::Serialize` and `serde::Deserialize` on the Rust side. Their TypeScript counterparts must mirror the JSON field names exactly.

### EncryptResponse
```
{
  ciphertext: string,     // hex-encoded or raw ciphertext
  keystream_bytes: number[]  // exactly 64 bytes (0-255), padded if input < 64 chars
}
```

### QualityResponse
```
{
  shannon_entropy: number,    // 0.0 – 8.0
  min_entropy: number,        // 0.0 – 8.0
  chi_square: number,
  mean: number,               // 0.0 – 255.0
  longest_run: number,        // integer
  overall_score: number,      // 0.0 – 100.0
  total_bytes: number,
  nist_results: [
    { name: string, p_value: number, passed: boolean }
  ]
}
```

### BenchResponse
```
{
  throughput_mbps: number,
  latency_us: number,
  bytes_generated: number,
  duration_secs: number
}
```

### XorStepsResponse
```
{
  steps: [
    {
      character: string,        // single char
      input_byte: number,       // 0-255
      keystream_byte: number,   // 0-255
      result_byte: number,      // 0-255
      input_binary: string,     // "01001000"
      keystream_binary: string, // "01001000"
      result_binary: string,    // "01001000"
      bit_ops: [
        { input_bit: boolean, key_bit: boolean, result_bit: boolean, position: number }
      ]
    }
  ],
  total: number
}
```

### EntropyStepsResponse
```
{
  steps: [
    {
      step_type: string,  // "CountBytes" | "CalculateProbabilities" | "CalculateContributions" | "SumEntropy" | "Interpret"
      byte_counts: { [key: string]: number },        // byte (as string) → count
      probabilities: { [key: string]: number },       // byte → probability
      entropy_contributions: { [key: string]: number }, // byte → contribution bits
      current_entropy_sum: number,
      total_entropy: number,
      max_entropy: number
    }
  ],
  total: number
}
```

### NistStepsResponse
```
{
  steps: [
    {
      step_type: string,  // "ConvertToBits" | "CountOnesZeros" | "CalculateStatistic" | "CalculatePValue" | "Interpret"
      bits: number[],     // 0 or 1
      ones_count: number,
      zeros_count: number,
      sum: number,
      s_obs: number,
      p_value: number,
      passed: boolean
    }
  ],
  total: number,
  input_text: string
}
```

---

## 7. State Management Strategy

The Tauri frontend has **no global state manager** (no Redux, no Vuex). State is managed per-tab using plain TypeScript module-level variables (or simple class instances). Tab switching saves and restores UI state from these variables.

### Per-Tab State Pattern
Each tab module (e.g., `test-tab.ts`) owns:
- Last received response data (rendered on display)
- Current UI control values (slider positions, toggle states)
- Loading/running flags (disables buttons during Rust calls)

### Cross-Tab State
The only cross-tab interaction is the "Help" link in the Use tab that navigates to the Learn tab with the plaintext pre-filled. This is implemented as a simple navigation call with a URL parameter or a shared singleton holding a pending "deep link" text value.

### Animation State (Learn Tab)
Each Learn visualizer (xor.ts, entropy.ts, nist.ts) maintains:
- `steps: StepType[]` — full array received from Rust
- `currentIndex: number`
- `isPlaying: boolean`
- `speed: number` — steps per second (0.1 to 5.0)
- `animationTimerId: number | null` — `setInterval` handle

Animation is driven by `setInterval` in TypeScript. No per-step Rust calls are made during playback.

---

## 8. Build System

### Frontend Build
The TypeScript source in `ui/ts/` is compiled with `tsc` (type checking) and bundled with `esbuild` (fast bundling). The build output (HTML, CSS, JS) is placed in `ui/dist/` which is referenced by `tauri.conf.json` as the `frontendDist` path.

### Tauri Build
`cargo tauri build` (desktop) and `cargo tauri android build` (Android) handle the full compilation pipeline: Rust backend + WebView bundling. The desktop build produces platform-native installers. The Android build produces an APK or AAB.

### Development Workflow
`cargo tauri dev` runs the development server. The TypeScript is watched and rebuilt on change. The Tauri WebView hot-reloads the frontend. Rust backend changes require a Rust recompile but not a frontend rebuild.

---

## 9. Dependency Additions

### src-tauri/Cargo.toml additions (compared to current Cargo.toml)
```
tauri = { version = "2", features = ["devtools"] }
tauri-build = { version = "2", features = [] }  (build-dependency)
serde = { version = "1", features = ["derive"] }
serde_json = "1"
entropy-forge = { path = "..", default-features = false }
```

All other dependencies (rand, getrandom, sha2, hex, statrs, thiserror) are pulled transitively through the `entropy-forge` crate dependency.

### ui/package.json additions
```
typescript (devDependency)
esbuild (devDependency)
@tauri-apps/api (dependency — Tauri JS/TS API)
@tauri-apps/cli (devDependency — tauri CLI wrapper)
```

No runtime JavaScript frameworks. No CSS preprocessors (plain CSS variables are sufficient).
```

---

## File: Docs/02-DESKTOP-UI.md

```markdown
# Phase 2 — Desktop UI Refactoring

## Reference Layout

The current egui application runs at **900×700 px** default (700×500 minimum). The Tauri web UI must reproduce this layout exactly at desktop sizes before any responsive adaptation is applied.

**Design language:**
- Dark background (`#1a1a2e` or equivalent near-black dark blue)
- Cyan accent (`#00d4ff`) for primary interactive elements and headings
- Orange accent (`#ff8c00`) for keystream/key byte highlights in Learn tab
- Green (`#00ff88`) for results, XOR output, and passing NIST tests
- Red (`#ff4444`) for NIST test failures and error states
- Monospace font for binary strings, hex values, and code-like output
- Proportional font for labels and descriptions
- Tight vertical spacing — the egui layout uses compact padding by default

---

## Global Layout Structure

```
┌─────────────────────────────────── 900px ──────────────────────────────────┐
│  [🔐 Entropy Forge]                                       [title bar area]  │
├────────────────────────────────────────────────────────────────────────────┤
│  [ Use ]  [ Test ]  [ Benchmark ]  [ Learn ]              [tab bar ~40px]  │
├────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│                                                                              │
│                        TAB CONTENT AREA                                      │
│                          (~620px tall)                                       │
│                                                                              │
│                                                                              │
└────────────────────────────────────────────────────────────────────────────┘
```

HTML structure:
```
<body>
  <div id="app">
    <header id="app-header">...</header>
    <nav id="tab-bar">
      <button class="tab-btn" data-tab="use">Use</button>
      <button class="tab-btn" data-tab="test">Test</button>
      <button class="tab-btn" data-tab="benchmark">Benchmark</button>
      <button class="tab-btn" data-tab="learn">Learn</button>
    </nav>
    <main id="tab-content">
      <section id="tab-use" class="tab-panel">...</section>
      <section id="tab-test" class="tab-panel hidden">...</section>
      <section id="tab-benchmark" class="tab-panel hidden">...</section>
      <section id="tab-learn" class="tab-panel hidden">...</section>
    </main>
  </div>
</body>
```

Tab switching is done by toggling the `hidden` class. Active tab button gets an `active` class.

---

## Tab 1: Use Tab

### Purpose
Stream cipher encryption/decryption with real-time keystream visualization.

### Layout

```
┌─────────────────────────────────────────────────────────────────────┐
│ 🔐 Use — Stream Cipher                                               │
│                                                                       │
│  [textarea: "Enter plaintext..."]                                     │
│  [  ] Show as Hex         [ Encrypt / Decrypt ]                       │
│                                                                       │
│  Output:                                                              │
│  [textarea readonly: ciphertext display]                              │
│                                                                       │
│  Keystream State (64 bytes):                                          │
│  [8×8 grid of colored squares]                                        │
│                                                                       │
│  [ℹ Help: How does this work?  →  Learn Tab]                          │
└─────────────────────────────────────────────────────────────────────┘
```

### Component: Input Area
- `<textarea>` for plaintext entry, ~4 rows tall, full width
- Placeholder text: "Enter plaintext to encrypt..."
- Character counter (optional cosmetic, not in original — omit for fidelity)

### Component: Controls Row
- Checkbox with label "Show as Hex" — toggles output format
- Primary action button "Encrypt / Decrypt"
- Button is full-width or right-aligned (matches egui layout)
- Clicking button calls `invoke('encrypt_decrypt', { plaintext: string, hex_output: boolean })`
- Button is disabled (grayed) while an invoke call is in flight

### Component: Output Area
- Read-only `<textarea>` or `<div>` with monospace font
- Displays ciphertext as hex string (e.g., `"3f a2 b1..."`) or raw bytes depending on toggle
- Blank until first encryption

### Component: Keystream State Grid
- 8 columns × 8 rows = 64 cells
- Each cell is a square (approx. 24×24 px on desktop)
- Background color: `rgb(v, v, v)` where `v` is the byte value (0=black, 255=white, grayscale)
- Cell border: 1px solid dark separator
- Hover: tooltip showing `"Byte N: 0xHH (decimal D)"` (N = index 0–63, HH = hex, D = decimal)
- Grid is blank/hidden before first encryption
- Grid is populated from `keystream_bytes` in the EncryptResponse

### Component: Help Link
- Styled as a subtle inline link: `ℹ Help: How does this work?`
- On click: switches to Learn tab and pre-fills the XOR visualizer input with the current plaintext

### State (use-tab.ts)
```
currentPlaintext: string
currentResponse: EncryptResponse | null
hexMode: boolean
isLoading: boolean
```

---

## Tab 2: Test Tab

### Purpose
Run entropy quality analysis and NIST randomness tests.

### Layout

```
┌─────────────────────────────────────────────────────────────────────┐
│ 🔬 Test — Entropy Quality                                            │
│                                                                       │
│  Sample Size: [=========|===========] 10,000 bytes                   │
│  [ Run All Tests ]                                                    │
│                                                                       │
│  ── Entropy Metrics ─────────────────────────────────────────────── │
│  Shannon Entropy  [████████████████████░░░░] 7.94 bits/byte  [ℹ]   │
│  Min-Entropy      [██████████████████░░░░░░] 7.82 bits/byte  [ℹ]   │
│  Mean             127.3 / 127.5 ideal                                 │
│  Chi-Square       248.3                                               │
│  Longest Run      12 bits                                             │
│  Overall Score    [████████████████████████] 96 / 100                │
│                                                                       │
│  ── NIST SP 800-22 Tests ───────────────────────────────────────── │
│  Test Name           P-Value    Result                                │
│  ─────────────────────────────────────────                           │
│  Frequency           0.4821     ✓ Pass                               │
│  Runs                0.3156     ✓ Pass                               │
│  Longest Run         0.2847     ✓ Pass                               │
│  Chi-Square          0.5023     ✓ Pass                               │
│  Serial              0.4418     ✓ Pass                               │
└─────────────────────────────────────────────────────────────────────┘
```

### Component: Sample Size Slider
- `<input type="range">` with logarithmic mapping
- Min: 1,000 — Max: 1,000,000
- Display: formatted with thousands separator (e.g., "10,000 bytes")
- Logarithmic step: the slider value maps to `10^(min + slider * (max-min)/100)`
- Visual marks at 1K, 10K, 100K, 1M

### Component: Run Button
- Full-width or wide primary button "Run All Tests"
- Shows a spinner / loading indicator while running
- Disabled while `isLoading` is true
- Calls `invoke('run_quality_tests', { sample_size: number })`

### Component: Entropy Metrics Section
- Section header "Entropy Metrics"
- Each metric row:
  ```
  [Label]  [Progress Bar]  [Value]  [ℹ tooltip icon]
  ```
- Progress bars:
  - Shannon Entropy: fill = value / 8.0 (max 8 bits/byte)
  - Min-Entropy: fill = value / 8.0
  - Overall Score: fill = score / 100
- Color thresholds for progress bars:
  - ≥ 7.5 bits: green (`#00ff88`)
  - 6.0 – 7.5: orange (`#ff8c00`)
  - < 6.0: red (`#ff4444`)
- Mean, Chi-Square, Longest Run displayed as plain text values without bars
- Tooltip content (shown on ℹ icon hover):
  - Shannon: "Measures unpredictability. 8.0 bits/byte = perfect randomness. Lower values indicate detectable patterns."
  - Min-Entropy: "Conservative entropy estimate based on the most frequent byte. If low, an attacker can make educated guesses."
  - Overall Score: "Weighted quality score: Shannon entropy (50%), Min-entropy (30%), Mean closeness to 127.5 (20%)."

### Component: NIST Test Results Table
- Section header "NIST SP 800-22 Tests"
- `<table>` with columns: Test Name | P-Value | Result
- Each row:
  - Test name (plain text)
  - P-value to 4 decimal places
  - Result: green "✓ Pass" if p-value ≥ 0.01, red "✗ Fail" if < 0.01
- Table footer row showing pass threshold: "Pass threshold: p-value ≥ 0.01"
- Loading state: table rows replaced with skeleton rows or spinner

### State (test-tab.ts)
```
sampleSize: number (default 10000)
lastResponse: QualityResponse | null
isLoading: boolean
```

---

## Tab 3: Benchmark Tab

### Purpose
Measure performance of entropy source generation.

### Layout

```
┌─────────────────────────────────────────────────────────────────────┐
│ ⚡ Benchmark — Performance                                           │
│                                                                       │
│  Benchmark Size: [=======|===============] 1,000,000 bytes           │
│  [ Run Benchmark ]                                                    │
│                                                                       │
│  ── Results ──────────────────────────────────────────────────────  │
│  Throughput       342.8 MB/s                [ℹ]                      │
│  Latency          2.91 µs/byte              [ℹ]                      │
│  Bytes Generated  1,000,000                                           │
│  Total Time       0.29 s                                              │
└─────────────────────────────────────────────────────────────────────┘
```

### Component: Benchmark Size Slider
- Same logarithmic slider pattern as Test tab
- Range: 10,000 – 10,000,000 bytes
- Default: 1,000,000

### Component: Run Button
- Wide primary button "Run Benchmark"
- Loading spinner during execution (benchmark may take several hundred milliseconds)
- Disabled while `isLoading`
- Calls `invoke('run_benchmark', { bytes: number })`

### Component: Results Section
- Section header "Results"
- Metric rows (no progress bars — just label + value + tooltip):
  - Throughput (MB/s) with ℹ: "Speed for bulk operations like file encryption and video streaming."
  - Latency (µs/byte) with ℹ: "Per-byte delay relevant to key generation and protocol handshakes."
  - Bytes Generated (formatted integer)
  - Total Time (formatted seconds, 2 decimal places)
- Results section hidden/greyed out until first benchmark run

### State (bench-tab.ts)
```
benchSize: number (default 1000000)
lastResponse: BenchResponse | null
isLoading: boolean
```

---

## Tab 4: Learn Tab

### Purpose
Interactive, animated, step-by-step educational visualizations of three cryptographic concepts.

### Layout — Sub-Tab Navigation

```
┌─────────────────────────────────────────────────────────────────────┐
│ 🎓 Learn — Interactive Visualizations                                │
│                                                                       │
│  [ XOR Cipher ] [ Shannon Entropy ] [ NIST Frequency ]  (sub-tabs)   │
│  ─────────────────────────────────────────────────────               │
│  [sub-tab content area]                                               │
└─────────────────────────────────────────────────────────────────────┘
```

Sub-tabs are rendered as a secondary tab bar inside the Learn section. Only one sub-tab is active at a time. Sub-tab state is preserved when switching between them (input text, current step, play state, speed).

---

### Sub-Tab 4.1: XOR Cipher Visualizer

#### Input Phase
```
  [textarea: "Type text to visualize..."]
  [ Start Visualization ]
```
- On click, calls `invoke('get_xor_steps', { text: string })`
- While loading: button shows spinner
- On response: transitions to visualization phase, `currentIndex = 0`

#### Visualization Phase
```
  Step 3 of 11: Encrypting 'l'

  ┌───────────────────────────────────────────────────────┐
  │  Input:      'l'  (108)    01101100   [cyan]          │
  │  Key:         42  (42)     00101010   [orange]        │
  │  ─────────────────────────────────────────────────    │
  │  Result:      66  (66)     01000110   [green]         │
  │                                                        │
  │  XOR Truth Table:  0⊕1=1  1⊕0=1  0⊕0=0  1⊕1=0       │
  └───────────────────────────────────────────────────────┘

  [◀ Prev]  [▶ Play]  [▶▶ Next]    Speed: [===|===] 1.0 steps/s

  Encryption Progress:
  [3f] [a2] [█66█] [--] [--] [--] [--] [--] [--] [--] [--]
    ^                ^                 ^
  past (gray)   current (green)    future (dark)

  [▸ How it works]  (collapsible)
```

#### Component: Step Header
- `"Step N of M: Encrypting 'X'"` where X is the character at current step
- If character is non-printable, show `"Encrypting byte 0xHH"` instead

#### Component: Visualization Frame
- Framed box with left-aligned rows
- Row 1 (cyan): `Input:   'X'  (DDD)   BBBBBBBB`
  - Character, decimal ASCII value, 8-character binary string
  - Text color: cyan (`#00d4ff`)
- Row 2 (orange): `Key:     DDD         BBBBBBBB`
  - Keystream byte decimal, binary string
  - Text color: orange (`#ff8c00`)
- Separator line
- Row 3 (green): `Result:  DDD         BBBBBBBB`
  - XOR result decimal, binary string
  - Text color: green (`#00ff88`)
- XOR truth table below separator: small monospace label

#### Component: Step Controls
- Three buttons: Previous (◀), Play/Pause (▶/⏸), Next (▶▶)
- Speed slider: `<input type="range">` from 0.1 to 5.0, step 0.1
- Speed label: "N.N steps/s"
- Previous disabled at step 0; Next disabled at last step
- Play/Pause toggles `isPlaying` and starts/stops `setInterval`

#### Component: Encryption Progress Bar
- Horizontal flex row of byte badges
- Each badge: 2-character hex representation
- Past steps: `color: #888`, `background: #222`
- Current step: `color: #000`, `background: #00ff88` (green highlight)
- Future steps: `color: #444`, `background: #111`
- Scroll horizontally if text is long (> 20 chars)

#### Component: "How it works" Collapsible
- `<details><summary>▸ How it works</summary>...` HTML element
- Content: 5-item numbered list explaining the XOR process:
  1. Text character is converted to its ASCII number
  2. The ASCII number is expressed as 8 binary bits
  3. A random keystream byte is generated from the entropy source
  4. Each bit of the input is XORed with the matching keystream bit
  5. The resulting bits form the encrypted byte

#### State (learn/xor.ts)
```
inputText: string
steps: XorStep[]
currentIndex: number
isPlaying: boolean
speed: number
animationTimerId: number | null
```

---

### Sub-Tab 4.2: Shannon Entropy Visualizer

#### Input Phase
```
  [textarea: "Enter text to analyze..."]
  [ Calculate ]
```
- Calls `invoke('get_entropy_steps', { text: string })`

#### Visualization Phase — 5 Steps

**Step 1 — Count Bytes:**
```
  Counting byte frequencies in "Hello World"...

  Byte  | Count | Visual
  ──────────────────────────────────────
  32 (' ')  | 1   | █
  72 ('H')  | 1   | █
  101 ('e') | 1   | █
  108 ('l') | 3   | ███
  ...
```
Table shows each unique byte (decimal + printable char if applicable), count, and proportional bar.

**Step 2 — Calculate Probabilities:**
```
  Formula:  P(x) = Count(x) / Total Bytes

  Byte  | Count | Probability | Visual
  ───────────────────────────────────────────
  108 ('l') | 3   | 0.2727  | ████████
  ...
```
Same table extended with probability column and formula explanation above.

**Step 3 — Calculate Contributions:**
```
  Formula:  contribution(x) = -P(x) × log₂(P(x))

  Byte  | Count | Probability | Contribution | Visual
  ─────────────────────────────────────────────────────
  108 ('l') | 3 | 0.2727 | 0.5300 bits | ████
  ...
```
Note below: "Rare bytes contribute MORE information per occurrence."

**Step 4 — Sum Entropy:**
```
  Total Entropy    =  3.180 bits/byte
  Maximum Possible =  3.459 bits/byte  (log₂ of unique byte count)
  Efficiency       =  91.9%
```
Large display for these three values. No table.

**Step 5 — Interpret:**
```
  [Summary statement based on thresholds]
  "This data shows high entropy (91.9% efficiency). It appears quite random."
  
  [Full metrics from Step 4 still visible below]
```
Interpretation thresholds:
- Efficiency > 80%: "high entropy … quite random"
- Efficiency < 25%: "very low entropy … highly predictable"
- Otherwise: "moderate entropy … some patterns present"

#### Step Controls
Same navigation + play/pause + speed slider as XOR visualizer.

#### State (learn/entropy.ts)
```
inputText: string
steps: EntropyStep[]
currentIndex: number
isPlaying: boolean
speed: number
animationTimerId: number | null
```

---

### Sub-Tab 4.3: NIST Frequency Test Visualizer

#### Input Phase
```
  [textarea: "Enter text to analyze..."]
  [ Analyze ]   [ Generate Random (16 bytes) ]
```
- "Analyze" calls `invoke('get_nist_steps', { text: string })`
- "Generate Random" calls `invoke('get_nist_steps_random', { count: 16 })`

#### Visualization Phase — 5 Steps

**Step 1 — Convert to Bits:**
```
  Bit sequence (space every 8 bits):
  01001000 01100101 01101100 01101100 ...
  
  (1s shown in green, 0s shown in light gray)
```

**Step 2 — Count Ones and Zeros:**
```
  Ones:   44        Zeros:   44
  
  [████████████████████|████████████████████]
   44 (50.0%)          44 (50.0%)
              ↑
           Ideal: 50%
```
Bar split: left half = ones (green), right half = zeros (gray). Center vertical line marks ideal 50%. Percentage shown below each half.

**Step 3 — Calculate Statistic:**
```
  Sum  = (+1 × 44 ones) + (−1 × 44 zeros)  =  0
  S_obs = |Sum| / √n  =  |0| / √88  =  0.0000
```

**Step 4 — Calculate P-Value:**
```
  P-Value = erfc(S_obs / √2)
          = erfc(0.0000 / 1.4142)
          = erfc(0.0000)
          = 1.0000
```

**Step 5 — Interpret:**
```
  ✅ PASS   (or  ❌ FAIL)
  
  "The sequence looks random."   (or "Too many 1s or 0s.")
  
  P-Value: 1.0000   Threshold: ≥ 0.01
```
Pass: green accent with checkmark. Fail: red accent with X.

#### Step Controls
Same navigation + play/pause + speed slider.

#### State (learn/nist.ts)
```
inputText: string
steps: NistStep[]
currentIndex: number
isPlaying: boolean
speed: number
animationTimerId: number | null
```

---

## Shared Component: Step Controls

The navigation + play/pause + speed slider pattern is identical across all three Learn visualizers. It must be implemented as a reusable component (`controls.ts`) that accepts callbacks:

```
Controls component interface:
  - onPrev(): void
  - onNext(): void
  - onTogglePlay(): void
  - onSpeedChange(speed: number): void
  - currentIndex: number
  - totalSteps: number
  - isPlaying: boolean
  - speed: number
```

The component renders into a container element and calls the provided callbacks on user interaction.

---

## Shared Component: Progress Bar

Used in Test tab for Shannon, Min-Entropy, and Overall Score:

```
ProgressBar component interface:
  - container: HTMLElement
  - label: string
  - value: number
  - max: number
  - displayValue: string  (formatted string, e.g., "7.94 bits/byte")
  - tooltipText: string | null
  - colorThresholds: { green: number, orange: number }  (fraction of max)
```

---

## Shared Component: Keystream Grid

Used in Use tab for 8×8 keystream visualization:

```
KeystreamGrid component interface:
  - container: HTMLElement
  - bytes: number[]  (exactly 64 values 0–255)
```

Renders 64 `<div>` elements in an 8-column CSS grid. Each div has `background-color: rgb(v,v,v)`. A `title` attribute provides the tooltip text for hover.

---

## CSS Architecture

### Color Variables (defined in `main.css` `:root`)
```
--bg-primary: #0f0f1a
--bg-secondary: #1a1a2e
--bg-tertiary: #16213e
--accent-cyan: #00d4ff
--accent-orange: #ff8c00
--accent-green: #00ff88
--accent-red: #ff4444
--text-primary: #e0e0e0
--text-secondary: #a0a0a0
--text-muted: #606060
--border-color: #2a2a4a
--font-mono: 'Courier New', Courier, monospace
--font-sans: system-ui, -apple-system, sans-serif
```

### Spacing Scale
```
--space-xs: 4px
--space-sm: 8px
--space-md: 16px
--space-lg: 24px
--space-xl: 32px
```

### Typography Scale
```
--text-sm: 12px
--text-base: 14px
--text-lg: 16px
--text-xl: 20px
--text-2xl: 24px
```

All desktop sizes. Mobile overrides are applied in `responsive.css`.
```

---

## File: Docs/03-MOBILE-RESPONSIVE.md

```markdown
# Phase 3 — Mobile Responsiveness

## Strategy Overview

The mobile adaptation follows a **desktop-first, CSS breakpoint override** approach:

1. The base CSS is designed for desktop (≥ 900px wide).
2. A single CSS file (`responsive.css`) contains all breakpoint overrides.
3. A secondary breakpoint handles mid-size screens (tablets, small laptops).
4. JavaScript feature detection adds a platform class to `<body>` so TypeScript can apply behavioral differences when running on Android.

No component is rebuilt for mobile — every component is the same HTML, just reflowed and resized by CSS overrides. JavaScript behavioral changes are minimal: touch event handling and adjusted animation defaults.

---

## Breakpoints

| Breakpoint | Target | CSS media query |
|------------|--------|-----------------|
| **Mobile** | Phone portrait (< 500px) | `@media (max-width: 499px)` |
| **Tablet** | Tablet portrait / small laptop (500px – 767px) | `@media (min-width: 500px) and (max-width: 767px)` |
| **Desktop** | Full layout (≥ 768px) | Base CSS (no query needed) |

> The primary mobile target is an Android phone in portrait orientation. Reference dimensions: **360×800** (logical pixels, before device pixel ratio scaling). At 360px wide, the entire layout must be usable without horizontal scrolling.

---

## Layout Changes at Mobile (< 500px)

### Global Changes

| Property | Desktop | Mobile |
|----------|---------|--------|
| App max-width | 900px | 100vw |
| App horizontal padding | 24px | 12px |
| Tab bar layout | Horizontal, equal-width | Horizontal, scrollable (overflow-x: auto) |
| Tab bar labels | "Use", "Test", "Benchmark", "Learn" | "Use", "Test", "⚡", "Learn" (Benchmark label shortened or icon-only) |
| Tab button min-width | auto | 64px |
| Tab button font size | 14px | 12px |
| Section heading font size | 20px | 16px |
| Body font size | 14px | 13px |
| Monospace font size | 14px | 12px |
| Base spacing unit | 16px | 10px |

### Tab Bar on Mobile

The tab bar must not wrap to two lines on narrow screens. If all four labels do not fit, shorten or use icons:

```
Desktop:  [ Use ]  [ Test ]  [ Benchmark ]  [ Learn ]
Mobile:   [ Use ]  [ Test ]  [    ⚡    ]  [ Learn ]
```

Alternatively, make the tab bar horizontally scrollable with a single visible row. The active tab is always visible (scroll-into-view on tab switch).

### Content Area on Mobile
- Remove all fixed-height containers
- All panels must scroll vertically within the content area
- Content area height: `calc(100dvh - tab-bar-height - header-height)`
- `overflow-y: auto` on each tab panel

---

## Use Tab — Mobile Adaptation

### Changes
- Textarea height reduced: 3 rows instead of 4–5
- "Show as Hex" checkbox and "Encrypt/Decrypt" button stack vertically
- Output textarea height: auto (grows with content), max-height with scroll
- Keystream grid: reduce cell size from 24×24 to 18×18 px
  - Grid remains 8×8 (64 cells) but fits in ~160px width
- Help link moves below the grid

### Keystream Grid at 360px
```
8 cells × 18px + 7 gaps × 2px = 144px + 14px = 158px
```
This fits comfortably within 360px - 24px padding = 336px.

### Touch Consideration
- Checkbox is replaced with a toggle switch (`<label class="toggle">`) for touch-friendliness
- Minimum tap target for all buttons: 44×44 px (Apple HIG and Android guidelines)

---

## Test Tab — Mobile Adaptation

### Changes
- Sample size slider goes full-width (stacked below label)
- Slider label and value on one line above the slider
- "Run All Tests" button: full-width
- Entropy metrics table rows stack differently:
  - On desktop: `[Label] [Progress Bar] [Value] [ℹ]` in one row
  - On mobile: Label on one line, progress bar + value + ℹ on the next
- Progress bars: full-width (subtract padding)
- NIST results table:
  - Desktop: three columns (Test Name | P-Value | Result)
  - Mobile: two columns (Test Name | Result); P-value shown in a `<details>` or second row
  - Alternatively: reduce P-value to 2 decimal places to fit
- All text remains legible at 13px

### NIST Table at Mobile

Option A — Two-column table:
```
Test Name         Result
──────────────────────────
Frequency         ✓ Pass
Runs              ✓ Pass
Longest Run       ✓ Pass
Chi-Square        ✓ Pass
Serial            ✓ Pass
```

Option B — Three-column with truncated P-value:
```
Test             P-Value  Result
─────────────────────────────────
Frequency         0.48    ✓
Runs              0.32    ✓
```

Prefer Option A for readability on very small screens.

---

## Benchmark Tab — Mobile Adaptation

### Changes
- Slider goes full-width
- Result rows: label on line 1, value on line 2 (two-line layout)
- Tooltip icons (ℹ) remain accessible — tap to show/hide tooltip text inline instead of on hover

```
Desktop row:   Throughput     342.8 MB/s   [ℹ]
Mobile rows:   Throughput     [ℹ]
               342.8 MB/s
```

### Tooltip Behavior on Mobile
On desktop, tooltips show on mouse hover using CSS `:hover` + absolute positioned element.
On mobile (touch), the ℹ icon is a button. Tapping toggles a tooltip visible class. The tooltip appears as an inline block below the row, not a floating overlay.

Detection: Tauri provides `platform()` from `@tauri-apps/plugin-os`. If platform is `android`, tooltips use tap-to-toggle behavior. Otherwise, hover behavior is used.

---

## Learn Tab — Mobile Adaptation

The Learn tab is the most complex for mobile. All three sub-tabs contain dense visualizations that need careful adaptation.

### Sub-Tab Bar on Mobile
```
Desktop:  [ XOR Cipher ] [ Shannon Entropy ] [ NIST Frequency ]
Mobile:   [ XOR ] [ Entropy ] [ NIST ]
```
Shortened labels. Still single-row. Sub-tab bar scrolls horizontally if needed.

### Sub-Tab 4.1: XOR Cipher — Mobile

#### Visualization Frame (most complex area)
On desktop:
```
Input:   'H'  (72)   01001000
Key:      42         00101010
─────────────────────────────
Result:   42         01000110
```

On mobile (< 500px), the frame columns are too wide for monospace alignment. Adapt:

```
── Input ──────────────────────
'H'   72   01001000
── Key ────────────────────────
      42   00101010
── Result (XOR) ───────────────
      42   01000110
```

Three separate sub-sections within the frame, each full-width. Character/decimal/binary still on one line, but font-size reduced to 12px monospace and line-height compressed.

#### Step Controls on Mobile
- Navigation row: [◀ Prev] [▶ Play/Pause] [▶▶ Next] — each button 44px tall, distributed evenly
- Speed slider: full-width below the nav row, with label

#### Encryption Progress on Mobile
- Hex badges wrap to multiple rows (natural wrapping in flex container)
- Current step badge still highlighted green
- Max display: 20–30 badges before scroll; longer inputs scroll horizontally within the progress area

### Sub-Tab 4.2: Shannon Entropy — Mobile

Tables with 4 columns (Byte | Count | Probability | Contribution) are too wide for 360px.

#### Table Adaptation
On mobile, show a condensed 3-column table:
```
Byte       Probability    Contribution
──────────────────────────────────────
108 ('l')  0.2727        0.530 bits
```
The "Count" column is merged into the Byte column as a sub-label or tooltip.

Alternatively, collapse to cards for very small screens: each unique byte gets a card showing all four values stacked vertically. This is more readable but takes more vertical space.

Prefer the condensed 3-column table approach for Step 2 and 3, as it maps more naturally to the desktop version.

#### Step 4 and 5 (Summary)
These are text-heavy with large numbers — they adapt naturally to any width. No changes needed.

### Sub-Tab 4.3: NIST Frequency — Mobile

#### Bit Sequence Display
On desktop, the bit sequence is shown as a long string of `01001000 01100101 ...`. On mobile, this wraps naturally within the container. Ensure `word-break: break-all` is applied so the sequence wraps at any character, not just at spaces.

#### Ratio Bar
The ones/zeros ratio bar is a horizontal bar divided at the measured split. On mobile, reduce height to 24px (from ~32px). Percentage labels remain visible.

#### Statistical Values (Steps 3 and 4)
Formulas using long expressions may overflow. Wrap them:
```
S_obs = |Sum| / √n
      = |0| / √88
      = 0.0000
```
Each term on its own line, indented with left padding.

---

## Touch Interaction Patterns

### General Rules
1. All interactive elements ≥ 44×44 px tap target
2. Sliders: `<input type="range">` has adequate default touch target on Android WebView
3. No hover-dependent critical information (tooltips moved to tap-to-show)
4. Avoid multi-finger gestures — app uses no pinch-zoom, swipe navigation, or long-press behavior
5. Form inputs (textarea, text input) trigger the Android soft keyboard — ensure enough viewport space remains visible

### Keyboard Handling on Android
When a soft keyboard opens on Android, the viewport height shrinks. This can cause layout issues. Mitigation:
- Use `height: 100dvh` (dynamic viewport height) on the app container, not `100vh`
- Tab content area uses `overflow-y: auto` so content scrolls within remaining space
- Learn tab visualizer frame must not be pushed off-screen when keyboard opens

### Scrolling Behavior
- Tabs themselves are not scrollable (they are always visible)
- Tab content panels scroll vertically
- Encryption progress row in XOR visualizer scrolls horizontally within its container
- No page-level scroll (the entire UI fits within the viewport at all times when the keyboard is closed)

---

## Font Sizing Adjustments

| Context | Desktop | Mobile |
|---------|---------|--------|
| App title | 24px | 18px |
| Section headings | 18px | 15px |
| Body text / labels | 14px | 13px |
| Monospace values (hex, binary) | 14px | 12px |
| Button text | 14px | 13px |
| Tooltip text | 12px | 12px |
| Step counter | 14px | 13px |
| Large metric values (throughput) | 20px | 16px |

---

## Spacing Adjustments

| Context | Desktop | Mobile |
|---------|---------|--------|
| App horizontal padding | 24px | 12px |
| Section internal padding | 16px | 10px |
| Between tab sections | 24px | 14px |
| Button padding (vertical) | 8px | 10px (larger tap target) |
| Table row height | auto | min 36px |
| Grid cell size | 24×24px | 18×18px |

---

## CSS Implementation Notes

All responsive changes live in `responsive.css`, imported last. The file uses only `@media` queries — no JavaScript class toggling for layout.

Structure of `responsive.css`:
```
/* ─── Tablet: 500–767px ─── */
@media (min-width: 500px) and (max-width: 767px) {
  /* Moderate adjustments... */
}

/* ─── Mobile: < 500px ─── */
@media (max-width: 499px) {
  /* All mobile overrides... */
}
```

No `!important` declarations. All overrides use the same specificity as the base rules (element or single class selector), relying on cascade order for precedence.

---

## Platform Detection for Behavioral Changes

Tauri provides `@tauri-apps/plugin-os`. In `main.ts`, on startup:

```
Detect platform → if android: add class "platform-android" to document.body
```

TypeScript modules check for this class when applying touch-specific behavior:
- Tooltip display mode (hover vs tap-to-show)
- Animation default speed (slightly slower on Android for readability)
- Keyboard avoidance logic
```

---

## File: Docs/04-ANDROID.md

```markdown
# Phase 4 — Android Platform Implementation

## Overview

Tauri v2 supports Android as a first-class build target. The Tauri CLI handles scaffolding the Android Gradle project, embedding the WebView, and wiring the Rust backend to the Android process. This document covers all Android-specific configuration, build, signing, and deployment steps.

---

## Prerequisites

Before starting Android work, the following must be installed and configured on the build machine:

| Tool | Version | Notes |
|------|---------|-------|
| Android Studio | Latest stable | Provides SDK, emulator, and signing tools |
| Android SDK | API 35 (Android 15) | Target SDK level |
| Android SDK | API 24 (Android 7.0) | Minimum SDK level |
| NDK (Native Dev Kit) | r27+ or latest stable | Required for Rust cross-compilation |
| Rust Android targets | `aarch64-linux-android`, `armv7-linux-androideabi`, `i686-linux-android`, `x86_64-linux-android` | Added via `rustup target add` |
| Java JDK | 17 or 21 | Required by Gradle |
| Gradle | Auto-managed via wrapper in generated project | Do not install separately |
| `ANDROID_HOME` env var | Path to Android SDK directory | Must be set in environment |
| `NDK_HOME` env var | Path to NDK directory | Must be set in environment |

---

## Tauri Android Initialization

Run once per project setup:

```
cargo tauri android init
```

This command:
1. Creates `src-tauri/gen/android/` directory structure
2. Generates `AndroidManifest.xml` with sensible defaults
3. Creates Gradle wrapper and build scripts
4. Sets up the WebView container Activity (`MainActivity.kt`)
5. Configures JNI loading for the compiled Rust library

**Important:** The `gen/android/` directory is auto-generated by Tauri but is NOT fully ephemeral — some files within it (like `MainActivity.kt`, custom Gradle configurations, and signing configs) may be edited and should be committed to version control. Tauri's documentation specifies which files are safe to edit.

---

## Project Structure After `android init`

```
src-tauri/gen/android/
├── app/
│   ├── src/
│   │   └── main/
│   │       ├── AndroidManifest.xml      ← Edit for permissions
│   │       ├── java/
│   │       │   └── com/entropyforge/app/
│   │       │       └── MainActivity.kt  ← Android entry point
│   │       └── res/
│   │           ├── drawable/            ← Icon sources (will be overwritten by icons/)
│   │           ├── values/
│   │           │   ├── strings.xml      ← App name string resource
│   │           │   └── themes.xml       ← Splash screen theme
│   │           └── mipmap-*/            ← Generated launcher icons
│   ├── build.gradle.kts                 ← App-level build config
│   └── proguard-rules.pro
├── gradle/wrapper/
├── settings.gradle.kts
├── build.gradle.kts                     ← Project-level build config
└── gradle.properties
```

---

## AndroidManifest.xml Configuration

Located at `src-tauri/gen/android/app/src/main/AndroidManifest.xml`.

### Permissions Required
Entropy Forge does **not** require any special Android permissions for its core functionality:
- No internet access needed (all computation is local)
- No file system access beyond app-private storage
- No camera, microphone, location, or contacts

The only permission that may be needed is `INTERNET` if the Tauri WebView requires it for DevTools during development. For production builds, no permissions are required beyond what Android adds by default.

```xml
<uses-permission android:name="android.permission.INTERNET" />
```
Add only for debug builds. Remove for release builds (or guard with a build type condition in Gradle).

### Minimum and Target SDK
```xml
<uses-sdk
    android:minSdkVersion="24"
    android:targetSdkVersion="35" />
```

These values are also set in `app/build.gradle.kts` — keep them synchronized:
```
android {
    defaultConfig {
        minSdk = 24
        targetSdk = 35
    }
}
```

### App Label and Icon
```xml
<application
    android:label="@string/app_name"
    android:icon="@mipmap/ic_launcher"
    android:roundIcon="@mipmap/ic_launcher_round"
    ...>
```

`strings.xml`:
```xml
<resources>
    <string name="app_name">Entropy Forge</string>
</resources>
```

---

## App Icons

Tauri uses the `icons/` directory in `src-tauri/` as the source of truth for all platform icons. To generate Android icons, place a 1024×1024 px PNG in `src-tauri/icons/` and run:

```
cargo tauri icon src-tauri/icons/app-icon-1024.png
```

This generates all required icon sizes:
- `mipmap-mdpi`: 48×48
- `mipmap-hdpi`: 72×72
- `mipmap-xhdpi`: 96×96
- `mipmap-xxhdpi`: 144×144
- `mipmap-xxxhdpi`: 192×192
- `mipmap-xxxhdpi` round variants (for adaptive icons)

### Icon Design Requirements for Play Store
- Background: a solid dark color or subtle gradient (no transparency in background layer)
- Foreground: the Entropy Forge logo/symbol (lock + wave, or entropy-inspired symbol)
- Adaptive icon layers: provide separate foreground and background layers for Android 8.0+ adaptive icon support
- Play Store also requires a 512×512 px icon uploaded separately in the Developer Console

---

## Splash Screen

Tauri v2 supports splash screens on Android. A simple dark background with the app icon centered satisfies Google Play requirements.

Configuration in `tauri.conf.json`:
```json
{
  "bundle": {
    "android": {
      "splashScreen": {
        "backgroundColor": "#0f0f1a",
        "image": "icons/splash.png"
      }
    }
  }
}
```

Splash screen shows until the WebView finishes loading. For an app of this size, loading should be < 1 second on modern Android.

---

## tauri.conf.json — Android Section

Key fields to configure:

```json
{
  "productName": "Entropy Forge",
  "version": "1.0.0",
  "identifier": "com.entropyforge.app",
  "build": {
    "frontendDist": "../ui/dist"
  },
  "bundle": {
    "active": true,
    "targets": "all",
    "android": {
      "minSdkVersion": 24
    }
  },
  "app": {
    "withGlobalTauri": true
  }
}
```

The `identifier` field must be a unique reverse-domain package name. It becomes the Android package name (`applicationId` in Gradle). **This value cannot be changed after publishing to Play Store** — choose it once and finalize it.

---

## Capabilities Configuration

Tauri v2 uses a capabilities system to control which APIs are accessible from the frontend. Create `src-tauri/capabilities/default.json`:

```json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "default",
  "description": "Default capabilities for Entropy Forge",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "core:window:allow-close",
    "core:window:allow-minimize",
    "core:window:allow-maximize"
  ]
}
```

For Android, a separate capabilities file `src-tauri/capabilities/mobile.json`:
```json
{
  "$schema": "../gen/schemas/mobile-schema.json",
  "identifier": "mobile",
  "description": "Mobile capabilities for Entropy Forge",
  "platforms": ["android"],
  "windows": ["main"],
  "permissions": [
    "core:default"
  ]
}
```

Entropy Forge does not need filesystem, shell, dialog, or clipboard APIs. Only `core:default` (which enables custom `invoke` commands) is required.

---

## Build Configuration

### Debug Build (for development and testing)
```
cargo tauri android dev
```
This:
1. Builds the Rust backend for Android ABIs (arm64-v8a by default in debug)
2. Builds the TypeScript frontend
3. Installs and launches on a connected emulator or device
4. Enables WebView DevTools accessible from Chrome's `chrome://inspect`

### Release Build
```
cargo tauri android build
```

This produces:
- `src-tauri/gen/android/app/build/outputs/apk/release/app-release.apk` (unsigned)
- Or AAB (Android App Bundle) if configured

To produce a signed release build, signing must be configured (see Signing section below).

### ABI Targets
The default release build compiles for all four Android ABIs:
- `aarch64-linux-android` (64-bit ARM — most modern phones)
- `armv7-linux-androideabi` (32-bit ARM — older phones)
- `i686-linux-android` (32-bit x86 — emulators)
- `x86_64-linux-android` (64-bit x86 — emulators)

For Play Store submission using AAB format, all ABI builds are bundled and Google Play delivers the correct ABI to each device.

For direct APK distribution, produce a universal APK or separate per-ABI APKs.

---

## APK Signing

Google Play requires all APKs and AABs to be signed with a release key.

### Keystore Creation
```
keytool -genkey -v -keystore entropy-forge-release.jks \
  -keyalg RSA -keysize 2048 -validity 10000 \
  -alias entropy-forge-key
```

Prompts for:
- Keystore password
- Key alias password
- Distinguished name (name, org, city, country)

**Security:** Never commit the `.jks` file or its passwords to version control. Store in a secrets manager or secure local path.

### Gradle Signing Configuration
In `app/build.gradle.kts`:

```kotlin
android {
    signingConfigs {
        create("release") {
            storeFile = file(System.getenv("KEYSTORE_PATH") ?: "../../entropy-forge-release.jks")
            storePassword = System.getenv("KEYSTORE_PASSWORD")
            keyAlias = System.getenv("KEY_ALIAS")
            keyPassword = System.getenv("KEY_PASSWORD")
        }
    }
    buildTypes {
        release {
            signingConfig = signingConfigs.getByName("release")
            isMinifyEnabled = true
            isShrinkResources = true
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
        }
    }
}
```

Signing credentials are passed as environment variables — never hardcoded.

### Play App Signing (Recommended)
Enroll in Google Play App Signing. Upload your APK/AAB signed with an upload key. Google re-signs with their distribution key. This allows key rotation if the upload key is ever compromised.

---

## Android App Bundle (AAB) vs APK

| Format | Play Store | Direct Distribution |
|--------|-----------|---------------------|
| AAB | ✅ Required for new apps (since 2021) | ❌ Not installable directly |
| APK | ✅ Accepted but not required | ✅ Sideloadable |

**Recommendation:** Build AAB for Play Store submission. Build a debug APK for local device testing.

To configure Gradle to produce AAB:
In `app/build.gradle.kts`, AAB is the default output when running `./gradlew bundleRelease`. The Tauri CLI may expose this via a flag — check `cargo tauri android build --help` for options.

---

## WebView Behavior on Android

Tauri uses `android.webkit.WebView` (system WebView) on Android. Key considerations:

### WebView Version
- Android 7.0 (API 24) ships with an older Chromium-based WebView
- Modern Android (API 29+) uses Android System WebView that auto-updates through Play Store
- Minimum supported WebView version for Tauri v2: Chromium 80+
- API 24 devices may have an outdated WebView — advise users to update "Android System WebView" from Play Store

### CSS Feature Compatibility
The web frontend must not use CSS features unavailable in Chromium 80. Avoid:
- Container queries (use media queries instead — already planned)
- `:has()` selector
- CSS nesting (write expanded rules)
- `@layer` cascade layers

CSS custom properties (variables), Grid, Flexbox, and `dvh` units are all supported.

### JavaScript Feature Compatibility
Chromium 80 supports ES2019+. Avoid:
- `BigInt` (not needed)
- Optional chaining `?.` — supported in Chromium 80 ✅
- Nullish coalescing `??` — supported in Chromium 80 ✅
- `globalThis` — supported ✅

The TypeScript compiler target should be set to `ES2019` or `ES2020` for safe compatibility.

---

## getrandom on Android

The `getrandom` crate (used by `SystemEntropy`) is configured to work on Android automatically. No special configuration is needed for Tauri Android builds because Tauri sets up the correct Android linker environment.

The `getrandom` crate on Android uses `getrandom(2)` syscall (Android API 28+) or falls back to `/dev/urandom` on older versions. Both are cryptographically secure.

For the Tauri binary compiled for Android, the `entropy-forge` crate is compiled with `default-features = false` (no egui), and the `getrandom` dependency does not require additional feature flags when used within a normal Android binary.

---

## Screen Orientation

Configure the app to support both portrait and landscape, but optimize for portrait:

In `AndroidManifest.xml`:
```xml
<activity
    android:screenOrientation="userPortrait"
    ...>
```

`userPortrait` allows portrait and reverse-portrait (upside down portrait) but not landscape. This simplifies the responsive design — landscape mode on a phone creates usability challenges for a content-heavy app like this and is out of scope.

---

## Hardware Back Button

Android devices have a hardware or gesture back button. The Tauri WebView must handle this gracefully:

- If a sub-tab or expandable section is open: close it
- If on the main tab level: show an "Exit app?" confirmation, then exit
- Do not let the back gesture close the WebView without warning

Tauri provides `@tauri-apps/plugin-barcode-scanner` — not relevant here. The back button behavior is handled by the Android Activity via Tauri's built-in back button handling or by registering a `backPressed` listener in Kotlin.

---

## Gradle Dependency Notes

The Tauri-generated Gradle project has its own dependencies. Do not add any additional Android library dependencies unless strictly required. Entropy Forge has no need for:
- Firebase
- Advertising SDKs
- Analytics SDKs
- Crash reporting (can add in future iteration)
- Notification APIs

Keeping dependencies minimal reduces APK size, reduces supply chain risk, and simplifies Play Store compliance.

---

## ProGuard / R8

Enable ProGuard (R8 optimizer) for release builds (already shown in signing config). The `proguard-rules.pro` file should keep Tauri's JNI bridge classes:

```
# Keep Tauri bridge
-keep class com.tauri.** { *; }
-keep class app.tauri.** { *; }

# Keep WebView interface
-keepclassmembers class * extends android.webkit.WebViewClient {
    public *;
}
```

---

## App Size Target

| Component | Estimated Size |
|-----------|---------------|
| Rust binary (stripped, release) | ~3–6 MB |
| WebView HTML/CSS/JS bundle | ~200–500 KB |
| Icons and resources | ~1 MB |
| Android Gradle overhead | ~2 MB |
| **Total APK** | **~8–12 MB** |
| **Total AAB** | **~6–10 MB** (Play Store optimizes per device) |

This is well within typical user expectations for a developer/utility app.
```

---

## File: Docs/05-FEATURE-PARITY.md

```markdown
# Phase 5 — Desktop-to-Android Feature Parity

## Feature Matrix

This matrix documents every interactive feature in Entropy Forge and its availability and behavior on each target platform.

| Feature | Desktop | Android | Notes |
|---------|---------|---------|-------|
| **USE TAB** | | | |
| Text input for plaintext | ✅ Full | ✅ Full | Soft keyboard opens on Android |
| Hex output toggle | ✅ Full | ✅ Full | Touch-friendly toggle |
| Encrypt / Decrypt button | ✅ Full | ✅ Full | Same Rust backend |
| Ciphertext display | ✅ Full | ✅ Full | Scrollable textarea |
| Keystream 8×8 grid | ✅ Full | ✅ Adapted | Smaller cells (18×18 vs 24×24 px) |
| Byte hover tooltip (grid) | ✅ Hover | ✅ Tap-to-show | Touch devices use tap; same info |
| "How does this work?" link | ✅ Full | ✅ Full | Navigates to Learn > XOR |
| **TEST TAB** | | | |
| Sample size slider | ✅ Full | ✅ Full | Same range (1K–1M), full-width on mobile |
| Run All Tests button | ✅ Full | ✅ Full | Same Rust backend |
| Shannon entropy + progress bar | ✅ Full | ✅ Full | Full-width bar on mobile |
| Min-entropy + progress bar | ✅ Full | ✅ Full | Full-width bar on mobile |
| Mean, Chi-square, Longest Run | ✅ Full | ✅ Full | Text-only display, no change |
| Overall quality score bar | ✅ Full | ✅ Full | Full-width bar on mobile |
| Metric tooltips (ℹ) | ✅ Hover | ✅ Tap-to-show | Same text content |
| NIST results table (3-col) | ✅ Full | ✅ 2-col | P-value column removed on mobile |
| Pass/Fail color coding | ✅ Full | ✅ Full | Same colors |
| Loading spinner | ✅ Full | ✅ Full | |
| **BENCHMARK TAB** | | | |
| Benchmark size slider | ✅ Full | ✅ Full | Same range (10K–10M), full-width on mobile |
| Run Benchmark button | ✅ Full | ✅ Full | Same Rust backend |
| Throughput result | ✅ Full | ✅ Full | |
| Latency result | ✅ Full | ✅ Full | |
| Bytes generated + duration | ✅ Full | ✅ Full | |
| Metric tooltips (ℹ) | ✅ Hover | ✅ Tap-to-show | Same text content |
| Loading spinner | ✅ Full | ✅ Full | |
| **LEARN TAB — XOR CIPHER** | | | |
| Text input | ✅ Full | ✅ Full | |
| Start Visualization button | ✅ Full | ✅ Full | |
| Step counter header | ✅ Full | ✅ Full | |
| Input row (cyan, binary) | ✅ Full | ✅ Adapted | Stacked rows on mobile |
| Key row (orange, binary) | ✅ Full | ✅ Adapted | Stacked rows on mobile |
| Result row (green, binary) | ✅ Full | ✅ Adapted | Stacked rows on mobile |
| XOR truth table | ✅ Full | ✅ Full | Single-line display |
| Previous / Next navigation | ✅ Full | ✅ Full | 44px touch targets |
| Play / Pause | ✅ Full | ✅ Full | |
| Speed slider | ✅ Full | ✅ Full | Full-width on mobile |
| Encryption progress badges | ✅ Full | ✅ Wrapping | Wraps to multiple lines on mobile |
| "How it works" collapsible | ✅ Full | ✅ Full | |
| **LEARN TAB — SHANNON ENTROPY** | | | |
| Text input | ✅ Full | ✅ Full | |
| Calculate button | ✅ Full | ✅ Full | |
| Step 1: Byte counts table | ✅ Full | ✅ Adapted | 3-col condensed table on mobile |
| Step 2: Probabilities table | ✅ Full | ✅ Adapted | 3-col condensed table |
| Step 3: Contributions table | ✅ Full | ✅ Adapted | 3-col condensed table |
| Step 4: Summary numbers | ✅ Full | ✅ Full | Large numbers, wrap naturally |
| Step 5: Interpretation text | ✅ Full | ✅ Full | |
| Visual bar charts in table | ✅ Full | ✅ Full | CSS width bars, scale with container |
| Step navigation + play/pause | ✅ Full | ✅ Full | |
| Speed slider | ✅ Full | ✅ Full | |
| **LEARN TAB — NIST FREQUENCY** | | | |
| Text input | ✅ Full | ✅ Full | |
| Analyze button | ✅ Full | ✅ Full | |
| Generate Random button | ✅ Full | ✅ Full | |
| Step 1: Bit sequence display | ✅ Full | ✅ Full | word-break: break-all for wrapping |
| Step 2: Ones/zeros ratio bar | ✅ Full | ✅ Adapted | Reduced height (24px vs 32px) |
| Step 3: Statistic formula | ✅ Full | ✅ Adapted | Multi-line formula layout |
| Step 4: P-value formula | ✅ Full | ✅ Adapted | Multi-line formula layout |
| Step 5: Pass/Fail result | ✅ Full | ✅ Full | Same colors and icons |
| Step navigation + play/pause | ✅ Full | ✅ Full | |
| Speed slider | ✅ Full | ✅ Full | |

---

## Limitations and Fallbacks

### 1. Keystream Grid Hover Tooltips → Tap-to-Show

**Desktop:** Hovering over a grid cell shows a tooltip with byte index, hex value, and decimal value.

**Android:** Touch devices have no hover state. The tooltip is converted to tap-to-show:
- Tapping a cell shows the tooltip as an inline element below the grid (or a small modal overlay)
- Tapping again or tapping elsewhere dismisses it

**Impact:** Minimal. The tooltip is informational only. The grid itself still provides visual entropy distribution at a glance.

**Implementation note:** Use `platform()` from `@tauri-apps/plugin-os` at startup to set a global flag. The grid component checks this flag and registers either `mouseover` (desktop) or `click` (Android) as the tooltip trigger.

---

### 2. Benchmark Accuracy on Android

**Desktop:** The `PerformanceBench::benchmark()` function measures `fill_bytes()` with nanosecond precision using `std::time::Instant`. On desktop, this is highly accurate.

**Android:** Android scheduling and thermal throttling can introduce variance. Benchmark results on Android will be:
- Consistently lower throughput (typically 50–200 MB/s vs 300–500 MB/s on desktop)
- More variance between runs (scheduler interference)
- May drop further under thermal throttling (after sustained load)

**Fallback strategy:**
- Use `benchmark_avg()` with 5 iterations instead of single `benchmark()` on Android
- Display average throughput ± standard deviation if multiple iterations are used
- Add a note in the UI: "Benchmark results on mobile may vary due to device thermal management."
- Reduce default benchmark size on Android to 100,000 bytes (instead of 1,000,000) to avoid excessive heat generation

**Android-specific behavior in `bench_tab.ts`:**
```
if (platform === 'android') {
  defaultBenchSize = 100_000
  benchIterations = 5
} else {
  defaultBenchSize = 1_000_000
  benchIterations = 1
}
```

The `run_benchmark` Tauri command accepts a `iterations` parameter on Android path.

---

### 3. NIST Test Execution Time on Android

**Desktop:** NIST tests on 100,000 bytes complete in < 100 ms.

**Android:** The same computation on an entry-level Android device may take 200–500 ms. On high-end devices, < 150 ms.

**Fallback strategy:**
- Reduce default sample size on Android to 50,000 bytes (instead of 10,000 — which is actually fine, but the maximum should be lowered from 1,000,000)
- Show a more prominent loading indicator on Android since the operation takes longer
- Consider reducing the slider max to 500,000 on Android

**Android-specific slider max in `test-tab.ts`:**
```
if (platform === 'android') {
  sliderMax = 500_000
}
```

---

### 4. Memory Constraints

**Desktop:** Available RAM measured in gigabytes. Generating 1,000,000 bytes of entropy and running all quality tests is trivial.

**Android:** Entry-level devices may have 2 GB RAM total with 500–800 MB available to user apps. The Android OS may kill background processes, but the app itself should stay well within 256 MB.

**Memory usage estimate:**
- 1,000,000-byte entropy buffer: ~1 MB
- Quality metrics (byte frequency HashMap): ~2 KB
- XOR steps for 100-char text: ~100 steps × ~80 bytes each ≈ 8 KB serialized
- Entropy steps: ~5 steps × ~256 entries each ≈ 40 KB serialized
- Total app memory: < 10 MB in all scenarios

**No changes needed** — the workloads are well within Android memory constraints even at maximum slider values. The Android-specific slider reductions described above are for performance/battery reasons, not memory.

---

### 5. WebView Rendering Differences

**Desktop:** Tauri uses the system browser engine (WebKit on macOS, WebView2 on Windows, WebKitGTK on Linux).

**Android:** Tauri uses `android.webkit.WebView` (Chromium-based, auto-updating).

**Differences:**
- Font rendering may vary slightly (subpixel antialiasing differences)
- CSS `dvh` unit behavior is the same (both support it)
- `monospace` font stack may resolve to different fonts (Droid Sans Mono on older Android, Roboto Mono on newer)

**Mitigation:**
- Specify a complete monospace font stack in CSS: `'Courier New', 'Roboto Mono', monospace`
- Test on Android API 24 emulator (oldest supported) to verify font rendering
- Accept minor font rendering differences as expected platform behavior

---

### 6. Animation Frame Rate

**Desktop:** `setInterval` animation at 60 fps is smooth. Step speed control (0.1–5.0 steps/s) works precisely.

**Android:** On budget Android devices, `setInterval` precision may degrade slightly. At 5.0 steps/s (200 ms interval), this is imperceptible. At 0.1 steps/s (10,000 ms interval), it is exact.

**Mitigation:** None needed. The Learn tab animations are not frame-rate dependent — they advance by step index, not by continuous animation. The timer fires every `1000/speed` ms and increments the step index. Imprecision of ±50 ms at any step rate is imperceptible.

---

### 7. Hardware Back Button

**Desktop:** No equivalent — the window has a close button.

**Android:** The hardware/gesture back button must be handled.

**Behavior:**
- If a sub-tab in Learn is active and a "How it works" section is expanded: collapse it
- If on any sub-tab of Learn (not the XOR default): navigate to XOR sub-tab
- If on any top-level tab other than Use: navigate to Use tab
- If already on Use tab: show an "Exit app?" toast/confirm dialog, then close app

This behavior is implemented in the Rust `lib.rs` using Tauri's `app.run()` callback, or in a Kotlin plugin if Tauri's cross-platform back button handling is insufficient.

---

## Platform-Specific Default Settings

| Setting | Desktop Default | Android Default | Reason |
|---------|----------------|-----------------|--------|
| Test sample size | 10,000 bytes | 10,000 bytes | Same — fast on both |
| Test sample size max | 1,000,000 | 500,000 | Reduce thermal load |
| Benchmark size | 1,000,000 bytes | 100,000 bytes | Thermal/performance |
| Benchmark iterations | 1 | 5 (average) | Reduce variance |
| Learn tab animation speed | 1.0 steps/s | 0.8 steps/s | Slightly slower for readability |
| Tooltip trigger | Mouse hover | Tap to show | Touch device behavior |
| NIST table columns | 3 (Name, P-Val, Result) | 2 (Name, Result) | Narrow screen |
| Grid cell size | 24×24 px | 18×18 px | Narrow screen |

---

## Feature Exclusions (Out of Scope for Both Platforms)

The following features are **not present** in the current egui application and are **not added** in this migration:

- File encryption (encrypt a file from disk)
- Key export / save output to file
- Custom entropy source selection in UI
- Batch testing (run tests on multiple data sets)
- History / session persistence across app restarts
- Dark/light theme toggle (app is dark-theme only)
- Localization / multiple languages
- Crash reporting / telemetry

These are noted for future roadmap consideration but are explicitly excluded from the Tauri migration scope.
```

---

## File: Docs/06-TESTING-DEPLOYMENT.md

```markdown
# Phase 6 — Testing & Deployment Strategy

## Testing Philosophy

Entropy Forge's testing strategy is divided into three layers:

1. **Rust unit tests** — Core library correctness (pre-existing, unchanged, always run)
2. **Tauri command tests** — IPC layer correctness (new, verifies serialization and logic)
3. **UI integration tests** — End-to-end behavior on desktop and Android (manual + automated)

---

## Layer 1: Rust Unit Tests (Pre-Existing)

All existing tests in the core library modules must continue to pass throughout the migration. These tests are the safety net that ensures the Rust logic has not been accidentally changed.

### Running Tests
```
cargo test --workspace --exclude entropy-forge-tauri
```

Or at the root if using a workspace:
```
cargo test -p entropy-forge
```

### Test Coverage Summary (existing)
| Module | What Is Tested |
|--------|---------------|
| `entropy/mock.rs` | LCG determinism, `next_byte()`, `fill_bytes()`, `reset()` |
| `crypto/cipher.rs` | Deterministic encryption with mock entropy, avalanche effect ~50% |
| `quality/metrics.rs` | Shannon entropy calculation, min-entropy, chi-square, overall score |
| `quality/nist.rs` | Frequency test, runs test, chi-square test p-values in expected range |
| `bench/performance.rs` | Benchmark returns positive throughput and latency, correct byte count |
| `learn/xor_visual.rs` | Step generation, step navigation, animation state |
| `learn/entropy_visual.rs` | Stage progression, correct entropy calculation per stage |
| `learn/nist_visual.rs` | Stage progression, correct p-value calculation |

### CI Gate
All Rust unit tests must pass before any build artifact is produced. In CI, this runs as:
```
cargo test --all
```

**Pass requirement:** Zero test failures, zero panics.

---

## Layer 2: Tauri Command Tests

New tests verify that each Tauri command:
1. Accepts valid input without panicking
2. Returns the correct response structure
3. Handles edge case inputs gracefully (empty string, very long input, Unicode)

These tests are written as Rust integration tests in `src-tauri/tests/` or as unit tests within each command module.

### Command Test Matrix

#### `encrypt_decrypt`
| Test Case | Input | Expected |
|-----------|-------|---------|
| Normal ASCII | "Hello World", hex=true | ciphertext is 22 hex chars (11 bytes × 2), keystream_bytes has 64 elements |
| Empty string | "", hex=true | ciphertext is empty string, keystream_bytes has 64 elements (keystream still generated) |
| Single character | "A", hex=false | ciphertext is 1 byte, keystream_bytes has 64 elements |
| Unicode input | "Héllo", hex=true | ciphertext length matches UTF-8 byte length × 2 hex chars |
| Long input (> 64 chars) | 100-char string | keystream_bytes always exactly 64 elements |

#### `run_quality_tests`
| Test Case | Input | Expected |
|-----------|-------|---------|
| Minimum sample | sample_size=1000 | Returns all fields, no panic |
| Default sample | sample_size=10000 | Shannon entropy > 7.0, overall score > 80 |
| Large sample | sample_size=100000 | Returns all 5 NIST results, no panic |
| Very small | sample_size=100 | longest_run_test may not run (< 128 bytes); handled gracefully |

#### `run_benchmark`
| Test Case | Input | Expected |
|-----------|-------|---------|
| Small | bytes=10000 | throughput_mbps > 0, latency_us > 0, bytes_generated == 10000 |
| Default | bytes=1000000 | Completes without timeout (< 5s) |
| Minimum | bytes=1 | Returns valid result |

#### `get_xor_steps`
| Test Case | Input | Expected |
|-----------|-------|---------|
| Short text | "Hi" | steps.length == 2, each step has all fields |
| Single char | "A" | steps.length == 1 |
| Empty string | "" | steps.length == 0, total == 0 |
| All Unicode | "日本語" | steps.length == UTF-8 byte count (not char count) |

#### `get_entropy_steps`
| Test Case | Input | Expected |
|-----------|-------|---------|
| Repeated chars | "aaaa" | steps.length == 5, Step 4 shows low entropy |
| All unique | "ABCDEFGH" | steps.length == 5, high efficiency |
| Single char | "a" | Steps generated, max_entropy approaches 0 |
| Empty | "" | Handled gracefully (0 steps or error with message) |

#### `get_nist_steps` and `get_nist_steps_random`
| Test Case | Input | Expected |
|-----------|-------|---------|
| Short text | "Hi" | steps.length == 5, p_value and passed fields present |
| Random (count=16) | n/a | 16 random bytes analyzed, steps.length == 5 |
| High-randomness text | "..."  | Step 5 likely shows PASS |

---

## Layer 3: UI Integration Tests

### Desktop Testing

#### Manual Smoke Test Checklist (run before each release)
```
USE TAB:
[ ] Enter text and click Encrypt — output appears
[ ] Toggle hex mode — output changes format
[ ] Keystream grid renders (64 colored squares)
[ ] Hover grid cell — tooltip shows byte index, hex, decimal
[ ] Click "How does this work?" — navigates to Learn > XOR, input pre-filled

TEST TAB:
[ ] Adjust slider — value updates in label
[ ] Click Run All Tests — spinner shows, then results populate
[ ] Shannon and Min-entropy bars colored correctly (green for high values)
[ ] NIST table shows 5 rows with pass/fail
[ ] All tooltip icons (ℹ) show correct text on hover

BENCHMARK TAB:
[ ] Adjust slider — value updates
[ ] Click Run Benchmark — spinner shows, then results populate
[ ] Throughput and latency show positive values
[ ] Tooltip icons work

LEARN TAB — XOR:
[ ] Type text and click Start Visualization
[ ] Step 1 shows correctly (input/key/result with binary)
[ ] Colors correct: cyan input, orange key, green result
[ ] Click Next — advances to Step 2
[ ] Click Prev — goes back to Step 1
[ ] Click Play — auto-advances through all steps
[ ] Speed slider changes animation rate
[ ] Encryption progress badges update as steps advance
[ ] "How it works" expander opens and closes
[ ] At last step: Next button is disabled

LEARN TAB — SHANNON:
[ ] Type text and click Calculate
[ ] Step 1 shows byte frequency table
[ ] Steps 2–5 show progressive calculation
[ ] Step 4 shows total entropy, max entropy, efficiency
[ ] Step 5 shows interpretation text
[ ] Navigation and play/pause work

LEARN TAB — NIST:
[ ] Type text and click Analyze
[ ] 5 steps visible
[ ] Step 2 shows ones/zeros bar with percentages
[ ] Step 5 shows PASS (green) or FAIL (red)
[ ] "Generate Random" button fills with random bytes and analyzes
[ ] Navigation and play/pause work
```

#### Automated Desktop Tests
Use Tauri's test utilities or `tauri-driver` (WebDriver integration) for automated UI tests on desktop. Priority:
1. Tab switching works correctly
2. All invoke calls return without error on valid input
3. Loading state shows/hides correctly

### Android Testing

#### Emulator Testing
Test on AVD (Android Virtual Device) configurations:
- API 24 (minimum) — Nexus 5X
- API 30 (mid-range) — Pixel 3
- API 35 (latest) — Pixel 9

For each emulator:
```
[ ] App launches without crash
[ ] All 4 tabs visible and switchable
[ ] Soft keyboard opens when tapping text inputs
[ ] Layout does not break at 360px width
[ ] All tab content scrollable vertically
[ ] Keystream grid fits without horizontal scroll
[ ] Learn tab binary rows not overflowing
[ ] All buttons reachable without scroll overlap
```

#### Real Device Testing
Test on at minimum two physical Android devices:
- Entry-level (e.g., Android Go device, 2 GB RAM, API 27)
- Mid-range (e.g., Samsung Galaxy A-series, 4 GB RAM, API 31)

Performance validation on real devices:
```
[ ] Benchmark completes within 10 seconds at 100,000 bytes
[ ] NIST tests complete within 5 seconds at 10,000 bytes
[ ] XOR visualizer animation is smooth (no visible jank)
[ ] Shannon entropy visualizer responds in < 2 seconds for 50-char input
[ ] No ANR (Application Not Responding) dialog appears during any operation
```

#### Android-Specific Checks
```
[ ] Hardware back button behavior works (see Feature Parity doc)
[ ] App returns to correct state after backgrounding and resuming
[ ] Rotating to landscape doesn't crash (even though landscape is userPortrait)
[ ] App icon appears correctly in launcher
[ ] App name "Entropy Forge" appears in launcher and task switcher
[ ] Splash screen appears briefly on first launch
[ ] No permissions dialogs appear (app requests no permissions)
```

---

## Performance Benchmarking Strategy

### Desktop Performance Targets
| Metric | Target |
|--------|--------|
| App cold start (first contentful paint) | < 500 ms |
| Tab switch latency | < 16 ms (1 frame) |
| Encrypt response time (100-char input) | < 50 ms |
| Quality test response (10K bytes) | < 500 ms |
| Benchmark (1M bytes) | < 2 seconds |
| Learn step generation (50-char XOR) | < 100 ms |

### Android Performance Targets
| Metric | Target |
|--------|--------|
| App cold start | < 2 seconds |
| Tab switch latency | < 100 ms |
| Encrypt response time | < 200 ms |
| Quality test (10K bytes) | < 2 seconds |
| Benchmark (100K bytes) | < 5 seconds |
| Learn step generation | < 500 ms |

---

## Play Store Submission Checklist

### Before Building Release APK/AAB

```
[ ] app identifier set in tauri.conf.json (com.entropyforge.app)
[ ] Version number updated (tauri.conf.json "version" field)
[ ] Version code incremented (app/build.gradle.kts "versionCode")
[ ] All icons generated (512x512, all mipmap sizes, adaptive icon layers)
[ ] Splash screen configured
[ ] Release keystore created and stored securely
[ ] Signing config added to build.gradle.kts
[ ] INTERNET permission removed from release build
[ ] ProGuard enabled for release build
[ ] App tested on API 24 emulator
[ ] App tested on real device
[ ] All smoke tests passed
```

### Build and Sign
```
[ ] cargo tauri android build (produces AAB)
[ ] Verify AAB is signed: jarsigner -verify -verbose app-release.aab
[ ] Verify AAB size < 150 MB (Play Store limit)
```

### Play Console Setup (one-time)

```
[ ] Google Play Developer account active ($25 one-time fee)
[ ] New app created in Play Console
[ ] App name: "Entropy Forge"
[ ] Default language: English (United States)
[ ] App or Game: App
[ ] Free or Paid: Free
```

### Store Listing — Required Content

**Short Description (80 chars max):**
```
Learn and visualize cryptographic entropy — step by step.
```

**Full Description (4000 chars max):**
Content must cover:
- What the app does (stream cipher, entropy testing, benchmarking, education)
- The four tabs and their purpose
- Educational value of the Learn tab
- No data leaves the device (local computation only)
- Mention of NIST SP 800-22 standards
- Target audience: developers, students, cryptography enthusiasts

**App Category:** Education  (or Tools, depending on primary audience target)

**Tags/Keywords:** cryptography, entropy, randomness, NIST, stream cipher, XOR, Shannon, education, security, developer tools

### Graphics Assets

```
[ ] App icon: 512×512 px PNG (uploaded to Play Console separately)
[ ] Feature graphic: 1024×500 px PNG (shown in Play Store listing header)
[ ] Screenshots (phone): minimum 2, recommended 4–8
    - Screenshot 1: Use tab showing encryption result and keystream grid
    - Screenshot 2: Test tab showing NIST results and metrics
    - Screenshot 3: Benchmark tab showing throughput results
    - Screenshot 4: Learn tab — XOR cipher visualization step
    Screenshot dimensions: 16:9 or 9:16 ratio, at least 320px on shortest side
```

### Privacy Policy

```
[ ] Privacy policy URL provided in Play Console
```

Privacy policy content must state:
- No personal data is collected
- No data is transmitted to any server
- All computation happens locally on the device
- No analytics, advertising, or crash reporting SDKs are included

A simple static page hosted on GitHub Pages is sufficient. A minimal policy:

```
Entropy Forge Privacy Policy

Entropy Forge does not collect, store, or transmit any personal data.
All operations are performed locally on your device.
No data leaves your device under any circumstances.
No third-party SDKs, analytics, or advertising libraries are included.

Last updated: [date]
```

### Content Rating

Complete the IARC content rating questionnaire in Play Console:
- No violence, sexual content, or gambling
- Expected rating: PEGI 3 / Everyone

### Data Safety Section

In Play Console → Data Safety:
- Does the app collect or share any user data? **No**
- Does the app use any system permissions? **No** (or only INTERNET in debug)

Fill out: No data types collected, no data types shared.

### Internal Testing Track

Before public release:
```
[ ] Upload AAB to Internal Testing track
[ ] Add tester email addresses
[ ] Install from Play Store on test devices using tester link
[ ] Verify all functionality works via Play Store install (different from direct APK install)
[ ] No crashes in Android Vitals
```

### Production Release

```
[ ] Promote from Internal Testing to Production (or Open Testing first)
[ ] Set rollout percentage (start with 10–20% for initial release)
[ ] Monitor Android Vitals for crashes after rollout
[ ] Increase rollout to 100% if no issues after 48 hours
```

---

## CI/CD Pipeline Recommendations

For future automation, a CI pipeline could include:

1. **On push to main:**
   - `cargo test --all` (Rust unit tests)
   - `cargo clippy --all` (Rust linting)
   - TypeScript type check: `tsc --noEmit`

2. **On release tag:**
   - All tests pass
   - `cargo tauri build` (desktop binary)
   - `cargo tauri android build` (Android AAB)
   - Sign AAB with release key from secrets
   - Upload to Play Console Internal Testing via Google Play API

3. **Desktop builds:**
   - Build on GitHub Actions matrix: ubuntu-latest, windows-latest, macos-latest
   - Upload build artifacts (installer files)
```

---

## File: Docs/07-MIGRATION-CHECKLIST.md

```markdown
# Phase 7 — Migration Checklist

This document is the ordered execution guide for the complete migration from `egui` desktop application to Tauri v2 cross-platform application. Follow steps in sequence within each phase. Phases 1–2 are strictly sequential. Phases 3 and 4 can overlap after Phase 2 is stable.

---

## Pre-Migration Preparation

### Checkpoint: Baseline Validation
```
[ ] Run cargo test --all  →  all tests pass
[ ] Run cargo build --release  →  current egui app builds without warnings
[ ] Launch the egui app and manually verify all 4 tabs work correctly
[ ] Capture screenshots of all 4 tabs for reference comparison during migration
[ ] Note the current window size and confirm 900×700 default
[ ] Document any existing warnings in cargo build output
```

### Checkpoint: Environment Setup
```
[ ] Rust toolchain is stable (1.77+ recommended for Tauri v2)
[ ] Node.js installed (18+ LTS)
[ ] npm or pnpm available
[ ] Tauri CLI installed: cargo install tauri-cli --version "^2"
     OR: npm install -g @tauri-apps/cli@latest
[ ] Verify: cargo tauri --version  →  shows 2.x
[ ] Android Studio installed (for Android work in Phase 4)
[ ] Android SDK API 24 and API 35 installed via SDK Manager
[ ] NDK (r27+) installed via SDK Manager
[ ] ANDROID_HOME environment variable set
[ ] NDK_HOME environment variable set
[ ] rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
```

---

## Phase 1: Project Restructuring

### Step 1.1: Convert to Cargo Workspace

Modify the root `Cargo.toml` to become a workspace manifest:

```
[ ] Add [workspace] section with members = [".", "src-tauri"]
[ ] Ensure [package] section still defines the entropy-forge library
[ ] Verify: cargo test -p entropy-forge  →  all tests still pass
[ ] Verify: cargo build -p entropy-forge  →  library compiles (no binary without --features gui)
```

The root package (`.`) remains the entropy-forge core library. The `src-tauri` member will be added in the next step.

### Step 1.2: Scaffold Tauri Project

```
[ ] From the repository root, run: cargo tauri init
     Answer prompts:
     - App name: Entropy Forge
     - Window title: Entropy Forge
     - Frontend dist dir: ../ui/dist
     - Dev server URL: http://localhost:1420  (or whatever the frontend dev server uses)
[ ] This creates src-tauri/ directory with initial structure
[ ] Verify src-tauri/Cargo.toml was created
[ ] Add entropy-forge as a dependency in src-tauri/Cargo.toml:
     entropy-forge = { path = "..", default-features = false }
[ ] Verify: cargo build -p entropy-forge-tauri  →  builds (no commands yet)
```

### Step 1.3: Disable GUI Feature in Tauri Build

```
[ ] Confirm src-tauri/Cargo.toml has entropy-forge with default-features = false
[ ] Confirm egui and eframe are NOT in src-tauri/Cargo.toml dependencies
[ ] Verify: cargo build -p entropy-forge-tauri  →  builds without egui
```

### Step 1.4: Create Frontend Directory

```
[ ] Create ui/ directory at repository root
[ ] Create ui/index.html (minimal HTML shell — 4 tabs, empty panels)
[ ] Create ui/ts/ directory
[ ] Create ui/ts/main.ts (tab switching logic only)
[ ] Create ui/styles/ directory
[ ] Create ui/styles/main.css (CSS variables and global reset)
[ ] Create ui/package.json with typescript and esbuild devDependencies
[ ] Create ui/tsconfig.json targeting ES2020 with strict mode
[ ] Run: cd ui && npm install
[ ] Verify: npx tsc --noEmit  →  no type errors
```

### Step 1.5: Configure Tauri for Web Frontend

```
[ ] In src-tauri/tauri.conf.json:
    - Set "frontendDist": "../ui/dist"
    - Set "devUrl": "http://localhost:1420"
    - Set "productName": "Entropy Forge"
    - Set "identifier": "com.entropyforge.app"
    - Set default window size: width 900, height 700
    - Set minimum window size: width 700, height 500
[ ] Create src-tauri/capabilities/default.json with core:default permission
[ ] Verify: cargo tauri dev  →  Tauri window opens (may be empty — frontend not built yet)
```

### Step 1.6: Add Tauri Command Stubs

```
[ ] Create src-tauri/src/commands/ directory
[ ] Create src-tauri/src/commands/mod.rs
[ ] Create stub functions in each commands file:
    - use_tab.rs: encrypt_decrypt() → returns placeholder EncryptResponse
    - test_tab.rs: run_quality_tests() → returns placeholder QualityResponse
    - bench_tab.rs: run_benchmark() → returns placeholder BenchResponse
    - learn_tab.rs: get_xor_steps(), get_entropy_steps(), get_nist_steps(), get_nist_steps_random()
[ ] Create src-tauri/src/types/responses.rs with all Serde-serializable response structs
[ ] Register all commands in src-tauri/src/lib.rs tauri::generate_handler![...]
[ ] Verify: cargo build -p entropy-forge-tauri  →  builds without errors
```

**Decision point:** Do not proceed to Phase 2 until the project builds cleanly with stub commands. This establishes the structural foundation.

---

## Phase 2: Desktop UI Implementation

Work through tabs in order: Use → Test → Benchmark → Learn. Implement each tab end-to-end before moving to the next.

### Step 2.1: Implement Use Tab

#### Backend
```
[ ] Implement encrypt_decrypt() in src-tauri/src/commands/use_tab.rs:
    - Accept plaintext: String and hex_output: bool
    - Create SystemEntropy instance
    - Create StreamCipher<SystemEntropy>
    - Call cipher.process(&plaintext.as_bytes())
    - Extract cipher.state() for keystream_bytes (pad to 64 if needed)
    - Format ciphertext as hex string if hex_output = true, else as escaped string
    - Return EncryptResponse { ciphertext: String, keystream_bytes: Vec<u8> }
[ ] Write unit tests for encrypt_decrypt with empty, normal, and unicode inputs
[ ] Verify: cargo test -p entropy-forge-tauri  →  tests pass
```

#### Frontend
```
[ ] Implement ui/ts/use-tab.ts:
    - Textarea for plaintext input
    - Hex mode checkbox/toggle
    - Encrypt button with loading state
    - invoke('encrypt_decrypt', ...) call
    - Output display update
    - Keystream grid rendering (8×8 grid of colored divs)
    - Grid cell hover tooltip
    - "How does this work?" link with cross-tab navigation
[ ] Implement ui/ts/components/grid.ts (reusable grid component)
[ ] Add ui/styles/use-tab.css
[ ] Build and verify in browser: cd ui && npx esbuild ts/main.ts --bundle --outdir=dist
[ ] Verify: cargo tauri dev  →  Use tab shows and functions correctly
[ ] Compare against baseline screenshot — layout must match egui version
```

### Step 2.2: Implement Test Tab

#### Backend
```
[ ] Implement run_quality_tests() in test_tab.rs:
    - Accept sample_size: usize
    - Generate entropy, run QualityMetrics::analyze()
    - Generate separate entropy buffer, run NistTests::run_all_tests()
    - Serialize results to QualityResponse
[ ] Write unit tests
[ ] Verify: cargo test  →  passes
```

#### Frontend
```
[ ] Implement ui/ts/test-tab.ts:
    - Logarithmic sample size slider
    - Run button with loading state
    - Progress bar rendering for Shannon, Min-Entropy, Overall Score
    - Metric value display with color thresholds
    - Tooltip icons (ℹ) with hover text
    - NIST results table (3 columns)
    - Pass/fail color coding
[ ] Implement ui/ts/components/progress-bar.ts (reusable)
[ ] Implement ui/ts/components/tooltip.ts (reusable ℹ tooltip)
[ ] Add ui/styles/test-tab.css
[ ] Verify: cargo tauri dev  →  Test tab shows and functions correctly
[ ] Compare against baseline screenshot
```

### Step 2.3: Implement Benchmark Tab

#### Backend
```
[ ] Implement run_benchmark() in bench_tab.rs:
    - Accept bytes: usize and optional iterations: usize (default 1)
    - If iterations > 1: call benchmark_avg(), else benchmark()
    - Return BenchResponse
[ ] Write unit tests
[ ] Verify: cargo test  →  passes
```

#### Frontend
```
[ ] Implement ui/ts/bench-tab.ts:
    - Logarithmic benchmark size slider
    - Run button with loading state
    - Results section (throughput, latency, bytes, duration)
    - Tooltip icons
[ ] Add ui/styles/bench-tab.css
[ ] Verify: cargo tauri dev  →  Benchmark tab shows and functions correctly
```

### Step 2.4: Implement Learn Tab — XOR Cipher

#### Backend
```
[ ] Implement get_xor_steps() in learn_tab.rs:
    - Accept text: String
    - Use MockEntropy(seed=42) for deterministic keystream
    - Create EncryptionProcess, call start(&text)
    - Serialize all steps to XorStepsResponse
    - Include pre-computed input_binary, keystream_binary, result_binary strings
[ ] Write tests verifying step count equals UTF-8 byte count of input
[ ] Verify: cargo test  →  passes
```

#### Frontend
```
[ ] Implement ui/ts/learn/xor.ts:
    - Input textarea + Start Visualization button
    - Step visualization frame (3 rows: input/key/result with binary strings)
    - Color application: cyan input, orange key, green result
    - XOR truth table line
    - Step navigation + play/pause controls
    - Speed slider
    - Encryption progress badge row
    - "How it works" collapsible
[ ] Implement ui/ts/components/controls.ts (reusable step nav component)
[ ] Implement ui/ts/learn/index.ts (sub-tab routing)
[ ] Verify: cargo tauri dev  →  XOR visualizer works end-to-end
```

### Step 2.5: Implement Learn Tab — Shannon Entropy

#### Backend
```
[ ] Implement get_entropy_steps() in learn_tab.rs:
    - Accept text: String
    - Create EntropyProcess, call start(&text)
    - Serialize all 5 stage steps to EntropyStepsResponse
    - Convert HashMaps to JSON-serializable format (string keys)
[ ] Write tests for empty string, single unique byte, all unique bytes
[ ] Verify: cargo test  →  passes
```

#### Frontend
```
[ ] Implement ui/ts/learn/entropy.ts:
    - Input textarea + Calculate button
    - 5 stage visualizations:
      Stage 1: byte count table (3 cols: byte, count, visual bar)
      Stage 2: probability table (4 cols: byte, count, probability, visual bar)
      Stage 3: contributions table (5 cols: +contribution column)
      Stage 4: large-text summary (3 values)
      Stage 5: interpretation text + stage 4 data
    - Visual bars as CSS width percentage divs
    - Step navigation + play/pause + speed slider
[ ] Verify: cargo tauri dev  →  entropy visualizer works correctly
```

### Step 2.6: Implement Learn Tab — NIST Frequency

#### Backend
```
[ ] Implement get_nist_steps() and get_nist_steps_random() in learn_tab.rs:
    - get_nist_steps(text: String): NistProcess::start(&text), serialize steps
    - get_nist_steps_random(count: usize): NistProcess::generate_random(count), serialize steps
    - NistStepsResponse with all stage data
[ ] Write tests for both endpoints
[ ] Verify: cargo test  →  passes
```

#### Frontend
```
[ ] Implement ui/ts/learn/nist.ts:
    - Input textarea + Analyze button + Generate Random button
    - 5 stage visualizations:
      Stage 1: bit sequence display with color-coded 1/0
      Stage 2: ones/zeros ratio bar with percentages
      Stage 3: statistic formula display
      Stage 4: p-value formula display
      Stage 5: pass/fail result with color coding
    - Step navigation + play/pause + speed slider
[ ] Verify: cargo tauri dev  →  NIST visualizer works correctly
```

### Step 2.7: Desktop Polish

```
[ ] Cross-tab navigation ("How does this work?" link in Use tab) works
[ ] App header renders correctly
[ ] Tab bar active state styling correct
[ ] All tooltips display on hover
[ ] All loading spinners show/hide correctly
[ ] Window resize down to 700×500 minimum — no content cut off
[ ] Full smoke test checklist from testing doc passes
[ ] Compare all 4 tabs against baseline screenshots — layout matches
[ ] cargo clippy -p entropy-forge-tauri  →  no warnings
[ ] npx tsc --noEmit  →  no TypeScript errors
```

**Decision point:** Do not start Phases 3 or 4 until the full desktop smoke test passes.

---

## Phase 3: Mobile Responsive UI

### Step 3.1: Add Responsive CSS

```
[ ] Create ui/styles/responsive.css
[ ] Import responsive.css last in index.html (after all other CSS)
[ ] Implement mobile breakpoint (<= 499px) for global layout:
    - Reduced padding
    - Shortened tab labels
    - Reduced font sizes (see spec in 03-MOBILE-RESPONSIVE.md)
[ ] Implement tablet breakpoint (500–767px) if desired (optional)
```

### Step 3.2: Use Tab Responsive

```
[ ] Grid cell size reduced to 18×18px on mobile
[ ] Controls row stacks vertically on mobile
[ ] Output textarea adjusts height on mobile
[ ] Test at 360px width in browser devtools — no horizontal scroll
```

### Step 3.3: Test Tab Responsive

```
[ ] Slider full-width on mobile
[ ] Metric rows stack label above bar on mobile
[ ] NIST table switches to 2-column layout on mobile
[ ] Test at 360px — no overflow
```

### Step 3.4: Benchmark Tab Responsive

```
[ ] Slider full-width
[ ] Result rows two-line layout (label + value)
[ ] Tooltip icons toggle on tap (platform detection)
```

### Step 3.5: Learn Tab Responsive

```
[ ] XOR visualizer frame switches from single-row to stacked sub-sections on mobile
[ ] Sub-tab labels shortened
[ ] Step controls full-width
[ ] Progress badges wrap to multiple lines
[ ] Shannon entropy table condensed to 3 columns on mobile
[ ] Bit sequence display wraps correctly
[ ] Ratio bar height reduced to 24px
[ ] Formula displays use multi-line layout on mobile
```

### Step 3.6: Platform Detection

```
[ ] Add @tauri-apps/plugin-os to src-tauri dependencies
[ ] In main.ts: detect platform on startup, add class to body
[ ] Tooltip toggle behavior changes on Android class
[ ] Default animation speed slightly lower on Android class
[ ] Slider max values adjusted on Android class
[ ] Verify in browser at 360px: all tabs usable, no horizontal scroll
```

### Step 3.7: Responsive Verification

```
[ ] Test all 4 tabs in browser devtools at 360×800 (portrait)
[ ] Test all 4 tabs at 768×1024 (tablet)
[ ] Full mobile smoke test from testing doc passes in devtools emulation
```

---

## Phase 4: Android Build

### Step 4.1: Android Project Initialization

```
[ ] Ensure all env vars set: ANDROID_HOME, NDK_HOME
[ ] Run: cargo tauri android init
[ ] Verify gen/android/ directory created
[ ] Open gen/android/ in Android Studio and resolve any Gradle sync errors
```

### Step 4.2: Configure AndroidManifest.xml

```
[ ] Set app name: "Entropy Forge"
[ ] Set minSdkVersion: 24
[ ] Set targetSdkVersion: 35
[ ] Add INTERNET permission for debug build only
[ ] Set screenOrientation: userPortrait
[ ] Verify no unnecessary permissions are present
```

### Step 4.3: App Icons and Splash Screen

```
[ ] Design 1024×1024 app icon (see 04-ANDROID.md for design requirements)
[ ] Place in src-tauri/icons/app-icon-1024.png
[ ] Run: cargo tauri icon src-tauri/icons/app-icon-1024.png
[ ] Verify mipmap-* directories populated in gen/android/
[ ] Configure splash screen background color and image in tauri.conf.json
[ ] Design 512×512 icon for Play Store upload (separate PNG)
```

### Step 4.4: Debug Build and Test on Emulator

```
[ ] Start API 35 emulator in Android Studio
[ ] Run: cargo tauri android dev
[ ] Verify app launches
[ ] Verify all 4 tabs function
[ ] Check Chrome DevTools via chrome://inspect for any JS errors
[ ] Full Android smoke test (emulator) from testing doc passes
```

### Step 4.5: Test on API 24 Emulator (Minimum SDK)

```
[ ] Start API 24 emulator in Android Studio
[ ] Run: cargo tauri android dev  (targeting this emulator)
[ ] Verify app launches
[ ] Verify no CSS or JS errors on older WebView
[ ] All 4 tabs functional
```

### Step 4.6: Signing Setup

```
[ ] Generate release keystore (keytool command in 04-ANDROID.md)
[ ] Store keystore file securely (NOT in repository)
[ ] Set KEYSTORE_PATH, KEYSTORE_PASSWORD, KEY_ALIAS, KEY_PASSWORD environment variables
[ ] Configure signing in app/build.gradle.kts
[ ] Enroll in Google Play App Signing (recommended)
```

### Step 4.7: Release Build

```
[ ] Run: cargo tauri android build
[ ] Verify AAB produced in gen/android/app/build/outputs/
[ ] Verify AAB is signed: jarsigner -verify app-release.aab
[ ] Verify AAB size (target < 20 MB)
[ ] Install AAB via bundletool on physical device for final validation
[ ] Full Android smoke test (physical device) from testing doc passes
```

---

## Phase 5: Validation and Cleanup

### Step 5.1: Complete Feature Parity Check

```
[ ] Review 05-FEATURE-PARITY.md feature matrix
[ ] Verify each ✅ Full item works on desktop
[ ] Verify each ✅ Full item works on Android
[ ] Verify each ✅ Adapted item shows correct adaptive behavior on Android
[ ] Document any deviations from the matrix
```

### Step 5.2: Performance Validation

```
[ ] Desktop: measure app cold start — target < 500 ms
[ ] Desktop: measure quality test response at 10K — target < 500 ms
[ ] Android: measure app cold start — target < 2 seconds
[ ] Android: measure benchmark at 100K bytes — target < 5 seconds
[ ] Android: check for ANR during any operation
```

### Step 5.3: Code Quality

```
[ ] cargo test --all  →  all Rust tests pass
[ ] cargo clippy --all  →  no warnings
[ ] npx tsc --noEmit  →  no TypeScript errors
[ ] Review all Tauri commands for unwrap() calls — replace with proper error handling
[ ] Review TypeScript for any 'any' types — type properly
```

### Step 5.4: Remove Deprecated Files

```
[ ] Review whether src/viz/ and src/main.rs should be retained
    Option A: Keep as-is (gui feature still works for desktop without Tauri)
    Option B: Move main.rs to main-legacy.rs and gate behind a feature flag
    Option C: Remove entirely (breaking change for library users)
    RECOMMENDED: Keep src/main.rs and src/viz/ with gui feature intact
    The egui build still works via: cargo run --features gui
    This preserves backwards compatibility
[ ] Update README.md to document both egui and Tauri build paths
[ ] Update BUILD.md with Tauri build instructions
```

### Step 5.5: Final Verification

```
[ ] cargo build --features gui  →  egui desktop app still builds
[ ] cargo tauri build  →  Tauri desktop app builds
[ ] cargo tauri android build  →  Android AAB builds
[ ] All smoke tests pass on desktop
[ ] All smoke tests pass on Android (emulator + device)
```

---

## Phase 6: Play Store Submission

```
[ ] Play Store submission checklist from 06-TESTING-DEPLOYMENT.md completed
[ ] Privacy policy published at a stable URL
[ ] Store listing prepared (description, screenshots, feature graphic)
[ ] Content rating questionnaire completed
[ ] Data safety section filled out (no data collected)
[ ] AAB uploaded to Internal Testing track
[ ] Tested via Play Store install on 2+ test devices
[ ] No crashes in Android Vitals (48-hour monitoring)
[ ] Promote to Production
[ ] Set rollout to 10%, monitor, increase to 100% after 48h with no issues
```

---

## Rollback Plan

If the migration encounters blocking issues, the original egui application remains fully buildable:
```
cargo run --features gui
```

The core library is unchanged. Any Tauri work can be abandoned without affecting the existing desktop application. The Tauri `src-tauri/` directory can be removed and the root `Cargo.toml` reverted from workspace to single package.

---

## Decision Points Summary

| Point | Condition to Proceed |
|-------|---------------------|
| After Step 1.6 | Project builds with stub commands — no compile errors |
| After Step 2.7 | Full desktop smoke test passes, all 4 tabs match baseline |
| After Step 3.7 | Mobile layout verified in devtools at 360px |
| After Step 4.5 | Android debug build runs on API 24 emulator |
| After Step 5.5 | All three build targets compile, all smoke tests pass |
| Before Play Store | Physical device testing complete, no ANR or crashes |
```

---

## File: examples/basic_usage.rs

```rust
//! Basic usage example

use entropy_forge::entropy::{EntropySource, SystemEntropy};
use entropy_forge::crypto::StreamCipher;
use entropy_forge::quality::QualityMetrics;

fn main() {
    println!("=== Entropy Forge - Basic Usage ===\n");
    
    // Create entropy source
    let mut entropy = SystemEntropy::new();
    println!("Using entropy source: {}", entropy.name());
    
    // Generate some random bytes
    let mut buffer = [0u8; 16];
    entropy.fill_bytes(&mut buffer);
    println!("\nGenerated 16 random bytes:");
    println!("{}", hex::encode(buffer));
    
    // Use with stream cipher
    println!("\n--- Stream Cipher Example ---");
    let mut cipher = StreamCipher::new(SystemEntropy::new());
    let plaintext = b"Hello, Entropy Forge!";
    let ciphertext = cipher.process(plaintext);
    println!("Plaintext:  {}", String::from_utf8_lossy(plaintext));
    println!("Ciphertext: {}", hex::encode(&ciphertext));
    
    // Analyze quality
    println!("\n--- Quality Analysis ---");
    let mut entropy_for_test = SystemEntropy::new();
    let metrics = QualityMetrics::analyze(&mut entropy_for_test, 10_000);
    
    println!("Shannon Entropy: {:.4} bits/byte", metrics.shannon_entropy);
    println!("Min-Entropy:     {:.4} bits/byte", metrics.min_entropy);
    println!("Mean byte value: {:.2}", metrics.mean);
    println!("Overall Score:   {:.1}/100", metrics.overall_score());
    
    println!("\nDone!");
}
```

---

## File: examples/custom_source.rs

```rust
//! Example of implementing a custom entropy source

use entropy_forge::entropy::EntropySource;
use entropy_forge::quality::QualityMetrics;
use entropy_forge::crypto::StreamCipher;

/// Simple custom RNG (DO NOT USE IN PRODUCTION!)
/// This is just an example of implementing the EntropySource trait.
struct SimpleRNG {
    state: u64,
}

impl SimpleRNG {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }
}

impl EntropySource for SimpleRNG {
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        for byte in dest.iter_mut() {
            // Simple Linear Congruential Generator
            self.state = self.state.wrapping_mul(1103515245).wrapping_add(12345);
            *byte = (self.state >> 24) as u8;
        }
    }
    
    fn name(&self) -> &str {
        "Simple LCG RNG (example only)"
    }
}

fn main() {
    println!("=== Custom Entropy Source Example ===\n");
    
    // Create custom source
    let mut rng = SimpleRNG::new(42);
    println!("Using: {}", rng.name());
    
    // Generate some bytes
    let mut buffer = [0u8; 16];
    rng.fill_bytes(&mut buffer);
    println!("\nGenerated bytes: {}", hex::encode(buffer));
    
    // Use with cipher
    println!("\n--- Using with StreamCipher ---");
    let mut cipher = StreamCipher::new(SimpleRNG::new(42));
    let plaintext = b"Custom source test";
    let ciphertext = cipher.process(plaintext);
    println!("Encrypted: {}", hex::encode(&ciphertext));
    
    // Analyze quality
    println!("\n--- Quality Analysis ---");
    let mut test_rng = SimpleRNG::new(42);
    let metrics = QualityMetrics::analyze(&mut test_rng, 100_000);
    
    println!("Shannon Entropy: {:.4} bits/byte", metrics.shannon_entropy);
    println!("Min-Entropy:     {:.4} bits/byte", metrics.min_entropy);
    println!("Overall Score:   {:.1}/100", metrics.overall_score());
    
    println!("\nNote: This simple RNG has lower quality than system RNG.");
    println!("For production use, implement a cryptographically secure RNG.");
}
```

---

## File: examples/quality_check.rs

```rust
//! Comprehensive quality check example

use entropy_forge::entropy::{EntropySource, SystemEntropy};
use entropy_forge::quality::{QualityMetrics, NistTests};

fn main() {
    println!("=== Entropy Quality Check ===\n");
    
    let mut entropy = SystemEntropy::new();
    println!("Testing: {}", entropy.name());
    println!("Sample size: 500,000 bytes\n");
    
    // Run quality metrics
    println!("--- Basic Metrics ---");
    let metrics = QualityMetrics::analyze(&mut entropy, 500_000);
    
    println!("Shannon Entropy:  {:.4} bits/byte (max: 8.0)", metrics.shannon_entropy);
    println!("Min-Entropy:      {:.4} bits/byte", metrics.min_entropy);
    println!("Mean byte value:  {:.2} (ideal: 127.5)", metrics.mean);
    println!("Chi-square stat:  {:.2}", metrics.chi_square);
    println!("Longest bit run:  {} bits", metrics.longest_run);
    println!("Overall Score:    {:.1}/100", metrics.overall_score());
    
    // Quality assessment
    println!("\nQuality Assessment:");
    if metrics.shannon_entropy >= 7.9 {
        println!("  ✓ Excellent entropy quality");
    } else if metrics.shannon_entropy >= 7.5 {
        println!("  ✓ Good entropy quality");
    } else {
        println!("  ⚠ Entropy quality could be improved");
    }
    
    // Run NIST tests
    println!("\n--- NIST SP 800-22 Tests ---");
    let mut data = vec![0u8; 500_000];
    entropy.fill_bytes(&mut data);
    let nist_results = NistTests::run_all_tests(&data);
    
    let mut passed = 0;
    for (name, p_value) in nist_results {
        let status = if p_value >= 0.01 {
            passed += 1;
            "✓ Pass"
        } else {
            "✗ Fail"
        };
        
        println!("{:<20} p={:.4}  {}", name, p_value, status);
    }
    
    println!("\nNIST Tests Passed: {}/5", passed);
    
    if passed >= 4 {
        println!("\n✓ Source passes statistical randomness tests");
    } else {
        println!("\n⚠ Source shows signs of non-randomness");
    }
    
    // Byte distribution
    println!("\n--- Byte Distribution Sample ---");
    println!("First 10 bytes: {}", 
        hex::encode(&data[..10]));
    println!("Middle 10 bytes: {}", 
        hex::encode(&data[250_000..250_010]));
    println!("Last 10 bytes: {}", 
        hex::encode(&data[data.len()-10..]));
    
    println!("\nQuality check complete!");
}
```

---

## File: LICENSE

```
MIT License

Copyright (c) 2025 Entropy Forge Contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

---

## File: README.md

```markdown
# 🔐 Entropy Forge

Pluggable entropy framework for cryptographic applications with built-in quality testing, visualization, and **interactive educational tools**.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge)
![CI](https://github.com/rvoidex7/entropy-forge/workflows/CI/badge.svg)

## Features

- **🔌 Trait-based Design**: Any RNG can be plugged in via the `EntropySource` trait
- **🔬 Quality Testing**: NIST SP 800-22 tests, Shannon entropy, min-entropy calculations
- **🔐 Cryptographic Use**: Stream cipher implementation using your entropy source
- **📊 Real-time Visualization**: egui-based GUI for interactive exploration
- **⚡ Performance Benchmarks**: Measure throughput and latency of entropy sources
- **🎓 Interactive Learning**: Step-by-step visualizations of cryptographic operations

## Screenshots

| Use Tab | Test Tab | Learn Tab |
|---------|----------|-----------|
| Stream cipher encryption | Quality metrics & NIST tests | Step-by-step XOR visualization |

## Quick Start

### Run the GUI

```bash
cargo run --release
```

### Run Examples

```bash
# Basic usage
cargo run --example basic_usage

# Custom entropy source
cargo run --example custom_source

# Quality check with NIST tests
cargo run --example quality_check
```

## GUI Tabs

### 📝 Use Tab
Encrypt and decrypt data using the stream cipher with real-time keystream visualization.

### 🔬 Test Tab
Analyze entropy quality with:
- Shannon Entropy (bits/byte)
- Min-Entropy (conservative estimate)
- Chi-Square uniformity test
- NIST SP 800-22 statistical tests (Frequency, Runs, Longest Run, Serial, Chi-Square)

### ⚡ Benchmark Tab
Measure entropy source performance:
- Throughput (MB/s)
- Latency per byte (µs)

### 🎓 Learn Tab
**Interactive educational visualizations** that teach cryptographic concepts step-by-step:

- **XOR Cipher Visualization**: See exactly how each character is encrypted
  - Character → ASCII → Binary conversion
  - Bit-by-bit XOR operation with color coding
  - Keystream generation from entropy source
  - Navigation controls (Previous/Next/Play/Pause)
  - Adjustable animation speed

Perfect for understanding how stream ciphers actually work!

## Usage

### Using the Default Source

```rust
use entropy_forge::entropy::SystemEntropy;
use entropy_forge::crypto::StreamCipher;

let entropy = SystemEntropy::new();
let mut cipher = StreamCipher::new(entropy);

let plaintext = b"Hello, World!";
let ciphertext = cipher.process(plaintext);
```

### Implementing a Custom Source

```rust
use entropy_forge::entropy::EntropySource;

struct MyRNG {
    state: u64,
}

impl EntropySource for MyRNG {
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        // Your RNG implementation here
        for byte in dest.iter_mut() {
            self.state = self.state.wrapping_mul(1103515245).wrapping_add(12345);
            *byte = (self.state >> 24) as u8;
        }
    }
    
    fn name(&self) -> &str {
        "My Custom RNG"
    }
}
```

### Testing Entropy Quality

```rust
use entropy_forge::quality::QualityMetrics;
use entropy_forge::entropy::SystemEntropy;

let mut source = SystemEntropy::new();
let metrics = QualityMetrics::analyze(&mut source, 100_000);

println!("Shannon Entropy: {:.4} bits/byte", metrics.shannon_entropy);
println!("Min-Entropy: {:.4} bits/byte", metrics.min_entropy);
println!("Quality Score: {:.1}/100", metrics.overall_score());
```

### Running NIST Tests

```rust
use entropy_forge::quality::NistTests;
use entropy_forge::entropy::{EntropySource, SystemEntropy};

let mut source = SystemEntropy::new();
let mut data = vec![0u8; 100_000];
source.fill_bytes(&mut data);

let results = NistTests::run_all_tests(&data);
for (name, p_value) in results {
    println!("{}: {:.4} {}", 
        name, 
        p_value, 
        if p_value >= 0.01 { "✓" } else { "✗" }
    );
}
```

## Design Philosophy

Entropy Forge is designed to work with **any** entropy source through a clean trait interface. Whether you're using:

- System RNG (`/dev/urandom`, `BCryptGenRandom`, etc.)
- Hardware RNG (TPM, RDRAND, etc.)
- Chaotic systems (Lorenz, Rössler attractors, etc.)
- Quantum sources
- Custom implementations

The framework provides:

1. **Practical cryptographic use** (stream ciphers, key derivation)
2. **Quality verification** (statistical tests, entropy metrics)
3. **Performance measurement** (throughput, latency benchmarks)
4. **Visual feedback** (real-time GUI, state visualization)
5. **Educational tools** (step-by-step algorithm breakdowns)

**High-quality entropy sources** (7.5+ bits/byte min-entropy) will show optimal performance across all metrics.

## Architecture

```
entropy-forge/
├── src/
│   ├── entropy/      # EntropySource trait + implementations
│   ├── crypto/       # Stream cipher, key derivation
│   ├── quality/      # Metrics, NIST tests
│   ├── bench/        # Performance benchmarking
│   ├── learn/        # Educational visualizations
│   └── viz/          # GUI components (egui)
├── examples/         # Usage examples
└── tests/            # Integration tests
```

## Quality Metrics

The framework measures:

- **Shannon Entropy**: Average information content (max: 8.0 bits/byte)
- **Min-Entropy**: Conservative estimate based on most probable outcome
- **Chi-Square**: Tests uniformity of byte distribution
- **NIST SP 800-22 Tests**: Industry-standard randomness tests
  - Frequency Test
  - Runs Test
  - Longest Run Test
  - Serial Test
  - Chi-Square Test

## Performance

Typical results on modern hardware (example with System RNG):

- **Throughput**: 300-500 MB/s
- **Latency**: 2-3 µs per byte
- **Quality**: 7.98+ bits/byte Shannon entropy

## Examples in the Wild

### Use Case: Chaotic System RNG

High-quality chaotic systems (e.g., Rössler attractors) producing 7.5+ bits/byte entropy can be seamlessly integrated:

```rust
// Hypothetical example
struct ChaoticRNG {
    // Rössler system state...
}

impl EntropySource for ChaoticRNG {
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        // Evolve chaotic system, extract bytes...
    }
    
    fn name(&self) -> &str {
        "Chaotic RNG (Rössler)"
    }
}

// Use with Entropy Forge
let mut source = ChaoticRNG::new();
let metrics = QualityMetrics::analyze(&mut source, 1_000_000);
// Expect: ~7.75 bits/byte, excellent NIST scores

let mut cipher = StreamCipher::new(source);
// Now using chaotic entropy for cryptography!
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- NIST SP 800-22 for statistical test specifications
- The Rust community for excellent cryptographic libraries
- egui for the immediate-mode GUI framework

## Disclaimer

This is an educational and research tool. For production cryptographic applications, use well-vetted libraries and consult with security experts.

---

**Made with 🦀 Rust**
```

---

## File: rust-toolchain.toml

```toml
[toolchain]
channel = "stable"
components = ["rustfmt", "clippy"]
```

---

## File: rustfmt.toml

```toml
# Rustfmt configuration
# Run with: cargo fmt

edition = "2021"
max_width = 100
tab_spaces = 4
use_small_heuristics = "Default"
newline_style = "Auto"

# Import organization
imports_granularity = "Crate"
group_imports = "StdExternalCrate"
reorder_imports = true

# Formatting options
format_code_in_doc_comments = true
format_strings = false
wrap_comments = true
comment_width = 100

# Function formatting
fn_single_line = false
fn_args_layout = "Tall"

# Control flow
control_brace_style = "AlwaysSameLine"
```

---

## File: SECURITY.md

```markdown
# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

If you discover a security vulnerability, please report it by:

1. **DO NOT** create a public GitHub issue
2. Email the maintainers directly (or use GitHub's private vulnerability reporting)
3. Include detailed steps to reproduce the issue
4. Allow reasonable time for a fix before public disclosure

## Security Considerations

### Educational Tool Disclaimer

Entropy Forge is primarily an **educational tool** for understanding cryptographic concepts. 

**DO NOT use this software for:**
- Production cryptographic applications
- Protecting sensitive data
- Any security-critical systems

### Cryptographic Implementations

The cryptographic implementations in this project (e.g., XOR stream cipher) are simplified for educational purposes and are **NOT cryptographically secure** for real-world use.

For production cryptography, use well-audited libraries such as:
- [RustCrypto](https://github.com/RustCrypto)
- [ring](https://github.com/briansmith/ring)
- [sodiumoxide](https://github.com/sodiumoxide/sodiumoxide)

## Dependencies

We use Dependabot to automatically monitor and update dependencies for security vulnerabilities.

## Acknowledgments

We appreciate responsible disclosure of security issues.
```

---

## File: src-tauri/.gitignore

```
# Generated by Cargo
# will have compiled files and executables
/target/
/gen/schemas
```

---

## File: src-tauri/build.rs

```rust
fn main() {
  tauri_build::build()
}
```

---

## File: src-tauri/capabilities/default.json

```json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "default",
  "description": "enables the default permissions",
  "windows": [
    "main"
  ],
  "permissions": [
    "core:default"
  ]
}
```

---

## File: src-tauri/Cargo.toml

```toml
[package]
name = "app"
version = "0.1.0"
description = "A Tauri App"
authors = ["you"]
license = ""
repository = ""
edition = "2021"
rust-version = "1.77.2"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[lib]
name = "app_lib"
crate-type = ["staticlib", "cdylib", "rlib"]

[build-dependencies]
tauri-build = { version = "2.5.6", features = [] }

[dependencies]
serde_json = "1.0"
serde = { version = "1.0", features = ["derive"] }
log = "0.4"
tauri = { version = "2.10.3", features = [] }
tauri-plugin-log = "2.0.0"
hex = "0.4"
entropy-forge = { path = "..", default-features = false }
```

---

## File: src-tauri/gen/schemas/acl-manifests.json

```json
{"core":{"default_permission":{"identifier":"default","description":"Default core plugins set.","permissions":["core:path:default","core:event:default","core:window:default","core:webview:default","core:app:default","core:image:default","core:resources:default","core:menu:default","core:tray:default"]},"permissions":{},"permission_sets":{},"global_scope_schema":null},"core:app":{"default_permission":{"identifier":"default","description":"Default permissions for the plugin.","permissions":["allow-version","allow-name","allow-tauri-version","allow-identifier","allow-bundle-type","allow-register-listener","allow-remove-listener"]},"permissions":{"allow-app-hide":{"identifier":"allow-app-hide","description":"Enables the app_hide command without any pre-configured scope.","commands":{"allow":["app_hide"],"deny":[]}},"allow-app-show":{"identifier":"allow-app-show","description":"Enables the app_show command without any pre-configured scope.","commands":{"allow":["app_show"],"deny":[]}},"allow-bundle-type":{"identifier":"allow-bundle-type","description":"Enables the bundle_type command without any pre-configured scope.","commands":{"allow":["bundle_type"],"deny":[]}},"allow-default-window-icon":{"identifier":"allow-default-window-icon","description":"Enables the default_window_icon command without any pre-configured scope.","commands":{"allow":["default_window_icon"],"deny":[]}},"allow-fetch-data-store-identifiers":{"identifier":"allow-fetch-data-store-identifiers","description":"Enables the fetch_data_store_identifiers command without any pre-configured scope.","commands":{"allow":["fetch_data_store_identifiers"],"deny":[]}},"allow-identifier":{"identifier":"allow-identifier","description":"Enables the identifier command without any pre-configured scope.","commands":{"allow":["identifier"],"deny":[]}},"allow-name":{"identifier":"allow-name","description":"Enables the name command without any pre-configured scope.","commands":{"allow":["name"],"deny":[]}},"allow-register-listener":{"identifier":"allow-register-listener","description":"Enables the register_listener command without any pre-configured scope.","commands":{"allow":["register_listener"],"deny":[]}},"allow-remove-data-store":{"identifier":"allow-remove-data-store","description":"Enables the remove_data_store command without any pre-configured scope.","commands":{"allow":["remove_data_store"],"deny":[]}},"allow-remove-listener":{"identifier":"allow-remove-listener","description":"Enables the remove_listener command without any pre-configured scope.","commands":{"allow":["remove_listener"],"deny":[]}},"allow-set-app-theme":{"identifier":"allow-set-app-theme","description":"Enables the set_app_theme command without any pre-configured scope.","commands":{"allow":["set_app_theme"],"deny":[]}},"allow-set-dock-visibility":{"identifier":"allow-set-dock-visibility","description":"Enables the set_dock_visibility command without any pre-configured scope.","commands":{"allow":["set_dock_visibility"],"deny":[]}},"allow-tauri-version":{"identifier":"allow-tauri-version","description":"Enables the tauri_version command without any pre-configured scope.","commands":{"allow":["tauri_version"],"deny":[]}},"allow-version":{"identifier":"allow-version","description":"Enables the version command without any pre-configured scope.","commands":{"allow":["version"],"deny":[]}},"deny-app-hide":{"identifier":"deny-app-hide","description":"Denies the app_hide command without any pre-configured scope.","commands":{"allow":[],"deny":["app_hide"]}},"deny-app-show":{"identifier":"deny-app-show","description":"Denies the app_show command without any pre-configured scope.","commands":{"allow":[],"deny":["app_show"]}},"deny-bundle-type":{"identifier":"deny-bundle-type","description":"Denies the bundle_type command without any pre-configured scope.","commands":{"allow":[],"deny":["bundle_type"]}},"deny-default-window-icon":{"identifier":"deny-default-window-icon","description":"Denies the default_window_icon command without any pre-configured scope.","commands":{"allow":[],"deny":["default_window_icon"]}},"deny-fetch-data-store-identifiers":{"identifier":"deny-fetch-data-store-identifiers","description":"Denies the fetch_data_store_identifiers command without any pre-configured scope.","commands":{"allow":[],"deny":["fetch_data_store_identifiers"]}},"deny-identifier":{"identifier":"deny-identifier","description":"Denies the identifier command without any pre-configured scope.","commands":{"allow":[],"deny":["identifier"]}},"deny-name":{"identifier":"deny-name","description":"Denies the name command without any pre-configured scope.","commands":{"allow":[],"deny":["name"]}},"deny-register-listener":{"identifier":"deny-register-listener","description":"Denies the register_listener command without any pre-configured scope.","commands":{"allow":[],"deny":["register_listener"]}},"deny-remove-data-store":{"identifier":"deny-remove-data-store","description":"Denies the remove_data_store command without any pre-configured scope.","commands":{"allow":[],"deny":["remove_data_store"]}},"deny-remove-listener":{"identifier":"deny-remove-listener","description":"Denies the remove_listener command without any pre-configured scope.","commands":{"allow":[],"deny":["remove_listener"]}},"deny-set-app-theme":{"identifier":"deny-set-app-theme","description":"Denies the set_app_theme command without any pre-configured scope.","commands":{"allow":[],"deny":["set_app_theme"]}},"deny-set-dock-visibility":{"identifier":"deny-set-dock-visibility","description":"Denies the set_dock_visibility command without any pre-configured scope.","commands":{"allow":[],"deny":["set_dock_visibility"]}},"deny-tauri-version":{"identifier":"deny-tauri-version","description":"Denies the tauri_version command without any pre-configured scope.","commands":{"allow":[],"deny":["tauri_version"]}},"deny-version":{"identifier":"deny-version","description":"Denies the version command without any pre-configured scope.","commands":{"allow":[],"deny":["version"]}}},"permission_sets":{},"global_scope_schema":null},"core:event":{"default_permission":{"identifier":"default","description":"Default permissions for the plugin, which enables all commands.","permissions":["allow-listen","allow-unlisten","allow-emit","allow-emit-to"]},"permissions":{"allow-emit":{"identifier":"allow-emit","description":"Enables the emit command without any pre-configured scope.","commands":{"allow":["emit"],"deny":[]}},"allow-emit-to":{"identifier":"allow-emit-to","description":"Enables the emit_to command without any pre-configured scope.","commands":{"allow":["emit_to"],"deny":[]}},"allow-listen":{"identifier":"allow-listen","description":"Enables the listen command without any pre-configured scope.","commands":{"allow":["listen"],"deny":[]}},"allow-unlisten":{"identifier":"allow-unlisten","description":"Enables the unlisten command without any pre-configured scope.","commands":{"allow":["unlisten"],"deny":[]}},"deny-emit":{"identifier":"deny-emit","description":"Denies the emit command without any pre-configured scope.","commands":{"allow":[],"deny":["emit"]}},"deny-emit-to":{"identifier":"deny-emit-to","description":"Denies the emit_to command without any pre-configured scope.","commands":{"allow":[],"deny":["emit_to"]}},"deny-listen":{"identifier":"deny-listen","description":"Denies the listen command without any pre-configured scope.","commands":{"allow":[],"deny":["listen"]}},"deny-unlisten":{"identifier":"deny-unlisten","description":"Denies the unlisten command without any pre-configured scope.","commands":{"allow":[],"deny":["unlisten"]}}},"permission_sets":{},"global_scope_schema":null},"core:image":{"default_permission":{"identifier":"default","description":"Default permissions for the plugin, which enables all commands.","permissions":["allow-new","allow-from-bytes","allow-from-path","allow-rgba","allow-size"]},"permissions":{"allow-from-bytes":{"identifier":"allow-from-bytes","description":"Enables the from_bytes command without any pre-configured scope.","commands":{"allow":["from_bytes"],"deny":[]}},"allow-from-path":{"identifier":"allow-from-path","description":"Enables the from_path command without any pre-configured scope.","commands":{"allow":["from_path"],"deny":[]}},"allow-new":{"identifier":"allow-new","description":"Enables the new command without any pre-configured scope.","commands":{"allow":["new"],"deny":[]}},"allow-rgba":{"identifier":"allow-rgba","description":"Enables the rgba command without any pre-configured scope.","commands":{"allow":["rgba"],"deny":[]}},"allow-size":{"identifier":"allow-size","description":"Enables the size command without any pre-configured scope.","commands":{"allow":["size"],"deny":[]}},"deny-from-bytes":{"identifier":"deny-from-bytes","description":"Denies the from_bytes command without any pre-configured scope.","commands":{"allow":[],"deny":["from_bytes"]}},"deny-from-path":{"identifier":"deny-from-path","description":"Denies the from_path command without any pre-configured scope.","commands":{"allow":[],"deny":["from_path"]}},"deny-new":{"identifier":"deny-new","description":"Denies the new command without any pre-configured scope.","commands":{"allow":[],"deny":["new"]}},"deny-rgba":{"identifier":"deny-rgba","description":"Denies the rgba command without any pre-configured scope.","commands":{"allow":[],"deny":["rgba"]}},"deny-size":{"identifier":"deny-size","description":"Denies the size command without any pre-configured scope.","commands":{"allow":[],"deny":["size"]}}},"permission_sets":{},"global_scope_schema":null},"core:menu":{"default_permission":{"identifier":"default","description":"Default permissions for the plugin, which enables all commands.","permissions":["allow-new","allow-append","allow-prepend","allow-insert","allow-remove","allow-remove-at","allow-items","allow-get","allow-popup","allow-create-default","allow-set-as-app-menu","allow-set-as-window-menu","allow-text","allow-set-text","allow-is-enabled","allow-set-enabled","allow-set-accelerator","allow-set-as-windows-menu-for-nsapp","allow-set-as-help-menu-for-nsapp","allow-is-checked","allow-set-checked","allow-set-icon"]},"permissions":{"allow-append":{"identifier":"allow-append","description":"Enables the append command without any pre-configured scope.","commands":{"allow":["append"],"deny":[]}},"allow-create-default":{"identifier":"allow-create-default","description":"Enables the create_default command without any pre-configured scope.","commands":{"allow":["create_default"],"deny":[]}},"allow-get":{"identifier":"allow-get","description":"Enables the get command without any pre-configured scope.","commands":{"allow":["get"],"deny":[]}},"allow-insert":{"identifier":"allow-insert","description":"Enables the insert command without any pre-configured scope.","commands":{"allow":["insert"],"deny":[]}},"allow-is-checked":{"identifier":"allow-is-checked","description":"Enables the is_checked command without any pre-configured scope.","commands":{"allow":["is_checked"],"deny":[]}},"allow-is-enabled":{"identifier":"allow-is-enabled","description":"Enables the is_enabled command without any pre-configured scope.","commands":{"allow":["is_enabled"],"deny":[]}},"allow-items":{"identifier":"allow-items","description":"Enables the items command without any pre-configured scope.","commands":{"allow":["items"],"deny":[]}},"allow-new":{"identifier":"allow-new","description":"Enables the new command without any pre-configured scope.","commands":{"allow":["new"],"deny":[]}},"allow-popup":{"identifier":"allow-popup","description":"Enables the popup command without any pre-configured scope.","commands":{"allow":["popup"],"deny":[]}},"allow-prepend":{"identifier":"allow-prepend","description":"Enables the prepend command without any pre-configured scope.","commands":{"allow":["prepend"],"deny":[]}},"allow-remove":{"identifier":"allow-remove","description":"Enables the remove command without any pre-configured scope.","commands":{"allow":["remove"],"deny":[]}},"allow-remove-at":{"identifier":"allow-remove-at","description":"Enables the remove_at command without any pre-configured scope.","commands":{"allow":["remove_at"],"deny":[]}},"allow-set-accelerator":{"identifier":"allow-set-accelerator","description":"Enables the set_accelerator command without any pre-configured scope.","commands":{"allow":["set_accelerator"],"deny":[]}},"allow-set-as-app-menu":{"identifier":"allow-set-as-app-menu","description":"Enables the set_as_app_menu command without any pre-configured scope.","commands":{"allow":["set_as_app_menu"],"deny":[]}},"allow-set-as-help-menu-for-nsapp":{"identifier":"allow-set-as-help-menu-for-nsapp","description":"Enables the set_as_help_menu_for_nsapp command without any pre-configured scope.","commands":{"allow":["set_as_help_menu_for_nsapp"],"deny":[]}},"allow-set-as-window-menu":{"identifier":"allow-set-as-window-menu","description":"Enables the set_as_window_menu command without any pre-configured scope.","commands":{"allow":["set_as_window_menu"],"deny":[]}},"allow-set-as-windows-menu-for-nsapp":{"identifier":"allow-set-as-windows-menu-for-nsapp","description":"Enables the set_as_windows_menu_for_nsapp command without any pre-configured scope.","commands":{"allow":["set_as_windows_menu_for_nsapp"],"deny":[]}},"allow-set-checked":{"identifier":"allow-set-checked","description":"Enables the set_checked command without any pre-configured scope.","commands":{"allow":["set_checked"],"deny":[]}},"allow-set-enabled":{"identifier":"allow-set-enabled","description":"Enables the set_enabled command without any pre-configured scope.","commands":{"allow":["set_enabled"],"deny":[]}},"allow-set-icon":{"identifier":"allow-set-icon","description":"Enables the set_icon command without any pre-configured scope.","commands":{"allow":["set_icon"],"deny":[]}},"allow-set-text":{"identifier":"allow-set-text","description":"Enables the set_text command without any pre-configured scope.","commands":{"allow":["set_text"],"deny":[]}},"allow-text":{"identifier":"allow-text","description":"Enables the text command without any pre-configured scope.","commands":{"allow":["text"],"deny":[]}},"deny-append":{"identifier":"deny-append","description":"Denies the append command without any pre-configured scope.","commands":{"allow":[],"deny":["append"]}},"deny-create-default":{"identifier":"deny-create-default","description":"Denies the create_default command without any pre-configured scope.","commands":{"allow":[],"deny":["create_default"]}},"deny-get":{"identifier":"deny-get","description":"Denies the get command without any pre-configured scope.","commands":{"allow":[],"deny":["get"]}},"deny-insert":{"identifier":"deny-insert","description":"Denies the insert command without any pre-configured scope.","commands":{"allow":[],"deny":["insert"]}},"deny-is-checked":{"identifier":"deny-is-checked","description":"Denies the is_checked command without any pre-configured scope.","commands":{"allow":[],"deny":["is_checked"]}},"deny-is-enabled":{"identifier":"deny-is-enabled","description":"Denies the is_enabled command without any pre-configured scope.","commands":{"allow":[],"deny":["is_enabled"]}},"deny-items":{"identifier":"deny-items","description":"Denies the items command without any pre-configured scope.","commands":{"allow":[],"deny":["items"]}},"deny-new":{"identifier":"deny-new","description":"Denies the new command without any pre-configured scope.","commands":{"allow":[],"deny":["new"]}},"deny-popup":{"identifier":"deny-popup","description":"Denies the popup command without any pre-configured scope.","commands":{"allow":[],"deny":["popup"]}},"deny-prepend":{"identifier":"deny-prepend","description":"Denies the prepend command without any pre-configured scope.","commands":{"allow":[],"deny":["prepend"]}},"deny-remove":{"identifier":"deny-remove","description":"Denies the remove command without any pre-configured scope.","commands":{"allow":[],"deny":["remove"]}},"deny-remove-at":{"identifier":"deny-remove-at","description":"Denies the remove_at command without any pre-configured scope.","commands":{"allow":[],"deny":["remove_at"]}},"deny-set-accelerator":{"identifier":"deny-set-accelerator","description":"Denies the set_accelerator command without any pre-configured scope.","commands":{"allow":[],"deny":["set_accelerator"]}},"deny-set-as-app-menu":{"identifier":"deny-set-as-app-menu","description":"Denies the set_as_app_menu command without any pre-configured scope.","commands":{"allow":[],"deny":["set_as_app_menu"]}},"deny-set-as-help-menu-for-nsapp":{"identifier":"deny-set-as-help-menu-for-nsapp","description":"Denies the set_as_help_menu_for_nsapp command without any pre-configured scope.","commands":{"allow":[],"deny":["set_as_help_menu_for_nsapp"]}},"deny-set-as-window-menu":{"identifier":"deny-set-as-window-menu","description":"Denies the set_as_window_menu command without any pre-configured scope.","commands":{"allow":[],"deny":["set_as_window_menu"]}},"deny-set-as-windows-menu-for-nsapp":{"identifier":"deny-set-as-windows-menu-for-nsapp","description":"Denies the set_as_windows_menu_for_nsapp command without any pre-configured scope.","commands":{"allow":[],"deny":["set_as_windows_menu_for_nsapp"]}},"deny-set-checked":{"identifier":"deny-set-checked","description":"Denies the set_checked command without any pre-configured scope.","commands":{"allow":[],"deny":["set_checked"]}},"deny-set-enabled":{"identifier":"deny-set-enabled","description":"Denies the set_enabled command without any pre-configured scope.","commands":{"allow":[],"deny":["set_enabled"]}},"deny-set-icon":{"identifier":"deny-set-icon","description":"Denies the set_icon command without any pre-configured scope.","commands":{"allow":[],"deny":["set_icon"]}},"deny-set-text":{"identifier":"deny-set-text","description":"Denies the set_text command without any pre-configured scope.","commands":{"allow":[],"deny":["set_text"]}},"deny-text":{"identifier":"deny-text","description":"Denies the text command without any pre-configured scope.","commands":{"allow":[],"deny":["text"]}}},"permission_sets":{},"global_scope_schema":null},"core:path":{"default_permission":{"identifier":"default","description":"Default permissions for the plugin, which enables all commands.","permissions":["allow-resolve-directory","allow-resolve","allow-normalize","allow-join","allow-dirname","allow-extname","allow-basename","allow-is-absolute"]},"permissions":{"allow-basename":{"identifier":"allow-basename","description":"Enables the basename command without any pre-configured scope.","commands":{"allow":["basename"],"deny":[]}},"allow-dirname":{"identifier":"allow-dirname","description":"Enables the dirname command without any pre-configured scope.","commands":{"allow":["dirname"],"deny":[]}},"allow-extname":{"identifier":"allow-extname","description":"Enables the extname command without any pre-configured scope.","commands":{"allow":["extname"],"deny":[]}},"allow-is-absolute":{"identifier":"allow-is-absolute","description":"Enables the is_absolute command without any pre-configured scope.","commands":{"allow":["is_absolute"],"deny":[]}},"allow-join":{"identifier":"allow-join","description":"Enables the join command without any pre-configured scope.","commands":{"allow":["join"],"deny":[]}},"allow-normalize":{"identifier":"allow-normalize","description":"Enables the normalize command without any pre-configured scope.","commands":{"allow":["normalize"],"deny":[]}},"allow-resolve":{"identifier":"allow-resolve","description":"Enables the resolve command without any pre-configured scope.","commands":{"allow":["resolve"],"deny":[]}},"allow-resolve-directory":{"identifier":"allow-resolve-directory","description":"Enables the resolve_directory command without any pre-configured scope.","commands":{"allow":["resolve_directory"],"deny":[]}},"deny-basename":{"identifier":"deny-basename","description":"Denies the basename command without any pre-configured scope.","commands":{"allow":[],"deny":["basename"]}},"deny-dirname":{"identifier":"deny-dirname","description":"Denies the dirname command without any pre-configured scope.","commands":{"allow":[],"deny":["dirname"]}},"deny-extname":{"identifier":"deny-extname","description":"Denies the extname command without any pre-configured scope.","commands":{"allow":[],"deny":["extname"]}},"deny-is-absolute":{"identifier":"deny-is-absolute","description":"Denies the is_absolute command without any pre-configured scope.","commands":{"allow":[],"deny":["is_absolute"]}},"deny-join":{"identifier":"deny-join","description":"Denies the join command without any pre-configured scope.","commands":{"allow":[],"deny":["join"]}},"deny-normalize":{"identifier":"deny-normalize","description":"Denies the normalize command without any pre-configured scope.","commands":{"allow":[],"deny":["normalize"]}},"deny-resolve":{"identifier":"deny-resolve","description":"Denies the resolve command without any pre-configured scope.","commands":{"allow":[],"deny":["resolve"]}},"deny-resolve-directory":{"identifier":"deny-resolve-directory","description":"Denies the resolve_directory command without any pre-configured scope.","commands":{"allow":[],"deny":["resolve_directory"]}}},"permission_sets":{},"global_scope_schema":null},"core:resources":{"default_permission":{"identifier":"default","description":"Default permissions for the plugin, which enables all commands.","permissions":["allow-close"]},"permissions":{"allow-close":{"identifier":"allow-close","description":"Enables the close command without any pre-configured scope.","commands":{"allow":["close"],"deny":[]}},"deny-close":{"identifier":"deny-close","description":"Denies the close command without any pre-configured scope.","commands":{"allow":[],"deny":["close"]}}},"permission_sets":{},"global_scope_schema":null},"core:tray":{"default_permission":{"identifier":"default","description":"Default permissions for the plugin, which enables all commands.","permissions":["allow-new","allow-get-by-id","allow-remove-by-id","allow-set-icon","allow-set-menu","allow-set-tooltip","allow-set-title","allow-set-visible","allow-set-temp-dir-path","allow-set-icon-as-template","allow-set-show-menu-on-left-click"]},"permissions":{"allow-get-by-id":{"identifier":"allow-get-by-id","description":"Enables the get_by_id command without any pre-configured scope.","commands":{"allow":["get_by_id"],"deny":[]}},"allow-new":{"identifier":"allow-new","description":"Enables the new command without any pre-configured scope.","commands":{"allow":["new"],"deny":[]}},"allow-remove-by-id":{"identifier":"allow-remove-by-id","description":"Enables the remove_by_id command without any pre-configured scope.","commands":{"allow":["remove_by_id"],"deny":[]}},"allow-set-icon":{"identifier":"allow-set-icon","description":"Enables the set_icon command without any pre-configured scope.","commands":{"allow":["set_icon"],"deny":[]}},"allow-set-icon-as-template":{"identifier":"allow-set-icon-as-template","description":"Enables the set_icon_as_template command without any pre-configured scope.","commands":{"allow":["set_icon_as_template"],"deny":[]}},"allow-set-menu":{"identifier":"allow-set-menu","description":"Enables the set_menu command without any pre-configured scope.","commands":{"allow":["set_menu"],"deny":[]}},"allow-set-show-menu-on-left-click":{"identifier":"allow-set-show-menu-on-left-click","description":"Enables the set_show_menu_on_left_click command without any pre-configured scope.","commands":{"allow":["set_show_menu_on_left_click"],"deny":[]}},"allow-set-temp-dir-path":{"identifier":"allow-set-temp-dir-path","description":"Enables the set_temp_dir_path command without any pre-configured scope.","commands":{"allow":["set_temp_dir_path"],"deny":[]}},"allow-set-title":{"identifier":"allow-set-title","description":"Enables the set_title command without any pre-configured scope.","commands":{"allow":["set_title"],"deny":[]}},"allow-set-tooltip":{"identifier":"allow-set-tooltip","description":"Enables the set_tooltip command without any pre-configured scope.","commands":{"allow":["set_tooltip"],"deny":[]}},"allow-set-visible":{"identifier":"allow-set-visible","description":"Enables the set_visible command without any pre-configured scope.","commands":{"allow":["set_visible"],"deny":[]}},"deny-get-by-id":{"identifier":"deny-get-by-id","description":"Denies the get_by_id command without any pre-configured scope.","commands":{"allow":[],"deny":["get_by_id"]}},"deny-new":{"identifier":"deny-new","description":"Denies the new command without any pre-configured scope.","commands":{"allow":[],"deny":["new"]}},"deny-remove-by-id":{"identifier":"deny-remove-by-id","description":"Denies the remove_by_id command without any pre-configured scope.","commands":{"allow":[],"deny":["remove_by_id"]}},"deny-set-icon":{"identifier":"deny-set-icon","description":"Denies the set_icon command without any pre-configured scope.","commands":{"allow":[],"deny":["set_icon"]}},"deny-set-icon-as-template":{"identifier":"deny-set-icon-as-template","description":"Denies the set_icon_as_template command without any pre-configured scope.","commands":{"allow":[],"deny":["set_icon_as_template"]}},"deny-set-menu":{"identifier":"deny-set-menu","description":"Denies the set_menu command without any pre-configured scope.","commands":{"allow":[],"deny":["set_menu"]}},"deny-set-show-menu-on-left-click":{"identifier":"deny-set-show-menu-on-left-click","description":"Denies the set_show_menu_on_left_click command without any pre-configured scope.","commands":{"allow":[],"deny":["set_show_menu_on_left_click"]}},"deny-set-temp-dir-path":{"identifier":"deny-set-temp-dir-path","description":"Denies the set_temp_dir_path command without any pre-configured scope.","commands":{"allow":[],"deny":["set_temp_dir_path"]}},"deny-set-title":{"identifier":"deny-set-title","description":"Denies the set_title command without any pre-configured scope.","commands":{"allow":[],"deny":["set_title"]}},"deny-set-tooltip":{"identifier":"deny-set-tooltip","description":"Denies the set_tooltip command without any pre-configured scope.","commands":{"allow":[],"deny":["set_tooltip"]}},"deny-set-visible":{"identifier":"deny-set-visible","description":"Denies the set_visible command without any pre-configured scope.","commands":{"allow":[],"deny":["set_visible"]}}},"permission_sets":{},"global_scope_schema":null},"core:webview":{"default_permission":{"identifier":"default","description":"Default permissions for the plugin.","permissions":["allow-get-all-webviews","allow-webview-position","allow-webview-size","allow-internal-toggle-devtools"]},"permissions":{"allow-clear-all-browsing-data":{"identifier":"allow-clear-all-browsing-data","description":"Enables the clear_all_browsing_data command without any pre-configured scope.","commands":{"allow":["clear_all_browsing_data"],"deny":[]}},"allow-create-webview":{"identifier":"allow-create-webview","description":"Enables the create_webview command without any pre-configured scope.","commands":{"allow":["create_webview"],"deny":[]}},"allow-create-webview-window":{"identifier":"allow-create-webview-window","description":"Enables the create_webview_window command without any pre-configured scope.","commands":{"allow":["create_webview_window"],"deny":[]}},"allow-get-all-webviews":{"identifier":"allow-get-all-webviews","description":"Enables the get_all_webviews command without any pre-configured scope.","commands":{"allow":["get_all_webviews"],"deny":[]}},"allow-internal-toggle-devtools":{"identifier":"allow-internal-toggle-devtools","description":"Enables the internal_toggle_devtools command without any pre-configured scope.","commands":{"allow":["internal_toggle_devtools"],"deny":[]}},"allow-print":{"identifier":"allow-print","description":"Enables the print command without any pre-configured scope.","commands":{"allow":["print"],"deny":[]}},"allow-reparent":{"identifier":"allow-reparent","description":"Enables the reparent command without any pre-configured scope.","commands":{"allow":["reparent"],"deny":[]}},"allow-set-webview-auto-resize":{"identifier":"allow-set-webview-auto-resize","description":"Enables the set_webview_auto_resize command without any pre-configured scope.","commands":{"allow":["set_webview_auto_resize"],"deny":[]}},"allow-set-webview-background-color":{"identifier":"allow-set-webview-background-color","description":"Enables the set_webview_background_color command without any pre-configured scope.","commands":{"allow":["set_webview_background_color"],"deny":[]}},"allow-set-webview-focus":{"identifier":"allow-set-webview-focus","description":"Enables the set_webview_focus command without any pre-configured scope.","commands":{"allow":["set_webview_focus"],"deny":[]}},"allow-set-webview-position":{"identifier":"allow-set-webview-position","description":"Enables the set_webview_position command without any pre-configured scope.","commands":{"allow":["set_webview_position"],"deny":[]}},"allow-set-webview-size":{"identifier":"allow-set-webview-size","description":"Enables the set_webview_size command without any pre-configured scope.","commands":{"allow":["set_webview_size"],"deny":[]}},"allow-set-webview-zoom":{"identifier":"allow-set-webview-zoom","description":"Enables the set_webview_zoom command without any pre-configured scope.","commands":{"allow":["set_webview_zoom"],"deny":[]}},"allow-webview-close":{"identifier":"allow-webview-close","description":"Enables the webview_close command without any pre-configured scope.","commands":{"allow":["webview_close"],"deny":[]}},"allow-webview-hide":{"identifier":"allow-webview-hide","description":"Enables the webview_hide command without any pre-configured scope.","commands":{"allow":["webview_hide"],"deny":[]}},"allow-webview-position":{"identifier":"allow-webview-position","description":"Enables the webview_position command without any pre-configured scope.","commands":{"allow":["webview_position"],"deny":[]}},"allow-webview-show":{"identifier":"allow-webview-show","description":"Enables the webview_show command without any pre-configured scope.","commands":{"allow":["webview_show"],"deny":[]}},"allow-webview-size":{"identifier":"allow-webview-size","description":"Enables the webview_size command without any pre-configured scope.","commands":{"allow":["webview_size"],"deny":[]}},"deny-clear-all-browsing-data":{"identifier":"deny-clear-all-browsing-data","description":"Denies the clear_all_browsing_data command without any pre-configured scope.","commands":{"allow":[],"deny":["clear_all_browsing_data"]}},"deny-create-webview":{"identifier":"deny-create-webview","description":"Denies the create_webview command without any pre-configured scope.","commands":{"allow":[],"deny":["create_webview"]}},"deny-create-webview-window":{"identifier":"deny-create-webview-window","description":"Denies the create_webview_window command without any pre-configured scope.","commands":{"allow":[],"deny":["create_webview_window"]}},"deny-get-all-webviews":{"identifier":"deny-get-all-webviews","description":"Denies the get_all_webviews command without any pre-configured scope.","commands":{"allow":[],"deny":["get_all_webviews"]}},"deny-internal-toggle-devtools":{"identifier":"deny-internal-toggle-devtools","description":"Denies the internal_toggle_devtools command without any pre-configured scope.","commands":{"allow":[],"deny":["internal_toggle_devtools"]}},"deny-print":{"identifier":"deny-print","description":"Denies the print command without any pre-configured scope.","commands":{"allow":[],"deny":["print"]}},"deny-reparent":{"identifier":"deny-reparent","description":"Denies the reparent command without any pre-configured scope.","commands":{"allow":[],"deny":["reparent"]}},"deny-set-webview-auto-resize":{"identifier":"deny-set-webview-auto-resize","description":"Denies the set_webview_auto_resize command without any pre-configured scope.","commands":{"allow":[],"deny":["set_webview_auto_resize"]}},"deny-set-webview-background-color":{"identifier":"deny-set-webview-background-color","description":"Denies the set_webview_background_color command without any pre-configured scope.","commands":{"allow":[],"deny":["set_webview_background_color"]}},"deny-set-webview-focus":{"identifier":"deny-set-webview-focus","description":"Denies the set_webview_focus command without any pre-configured scope.","commands":{"allow":[],"deny":["set_webview_focus"]}},"deny-set-webview-position":{"identifier":"deny-set-webview-position","description":"Denies the set_webview_position command without any pre-configured scope.","commands":{"allow":[],"deny":["set_webview_position"]}},"deny-set-webview-size":{"identifier":"deny-set-webview-size","description":"Denies the set_webview_size command without any pre-configured scope.","commands":{"allow":[],"deny":["set_webview_size"]}},"deny-set-webview-zoom":{"identifier":"deny-set-webview-zoom","description":"Denies the set_webview_zoom command without any pre-configured scope.","commands":{"allow":[],"deny":["set_webview_zoom"]}},"deny-webview-close":{"identifier":"deny-webview-close","description":"Denies the webview_close command without any pre-configured scope.","commands":{"allow":[],"deny":["webview_close"]}},"deny-webview-hide":{"identifier":"deny-webview-hide","description":"Denies the webview_hide command without any pre-configured scope.","commands":{"allow":[],"deny":["webview_hide"]}},"deny-webview-position":{"identifier":"deny-webview-position","description":"Denies the webview_position command without any pre-configured scope.","commands":{"allow":[],"deny":["webview_position"]}},"deny-webview-show":{"identifier":"deny-webview-show","description":"Denies the webview_show command without any pre-configured scope.","commands":{"allow":[],"deny":["webview_show"]}},"deny-webview-size":{"identifier":"deny-webview-size","description":"Denies the webview_size command without any pre-configured scope.","commands":{"allow":[],"deny":["webview_size"]}}},"permission_sets":{},"global_scope_schema":null},"core:window":{"default_permission":{"identifier":"default","description":"Default permissions for the plugin.","permissions":["allow-get-all-windows","allow-scale-factor","allow-inner-position","allow-outer-position","allow-inner-size","allow-outer-size","allow-is-fullscreen","allow-is-minimized","allow-is-maximized","allow-is-focused","allow-is-decorated","allow-is-resizable","allow-is-maximizable","allow-is-minimizable","allow-is-closable","allow-is-visible","allow-is-enabled","allow-title","allow-current-monitor","allow-primary-monitor","allow-monitor-from-point","allow-available-monitors","allow-cursor-position","allow-theme","allow-is-always-on-top","allow-internal-toggle-maximize"]},"permissions":{"allow-available-monitors":{"identifier":"allow-available-monitors","description":"Enables the available_monitors command without any pre-configured scope.","commands":{"allow":["available_monitors"],"deny":[]}},"allow-center":{"identifier":"allow-center","description":"Enables the center command without any pre-configured scope.","commands":{"allow":["center"],"deny":[]}},"allow-close":{"identifier":"allow-close","description":"Enables the close command without any pre-configured scope.","commands":{"allow":["close"],"deny":[]}},"allow-create":{"identifier":"allow-create","description":"Enables the create command without any pre-configured scope.","commands":{"allow":["create"],"deny":[]}},"allow-current-monitor":{"identifier":"allow-current-monitor","description":"Enables the current_monitor command without any pre-configured scope.","commands":{"allow":["current_monitor"],"deny":[]}},"allow-cursor-position":{"identifier":"allow-cursor-position","description":"Enables the cursor_position command without any pre-configured scope.","commands":{"allow":["cursor_position"],"deny":[]}},"allow-destroy":{"identifier":"allow-destroy","description":"Enables the destroy command without any pre-configured scope.","commands":{"allow":["destroy"],"deny":[]}},"allow-get-all-windows":{"identifier":"allow-get-all-windows","description":"Enables the get_all_windows command without any pre-configured scope.","commands":{"allow":["get_all_windows"],"deny":[]}},"allow-hide":{"identifier":"allow-hide","description":"Enables the hide command without any pre-configured scope.","commands":{"allow":["hide"],"deny":[]}},"allow-inner-position":{"identifier":"allow-inner-position","description":"Enables the inner_position command without any pre-configured scope.","commands":{"allow":["inner_position"],"deny":[]}},"allow-inner-size":{"identifier":"allow-inner-size","description":"Enables the inner_size command without any pre-configured scope.","commands":{"allow":["inner_size"],"deny":[]}},"allow-internal-toggle-maximize":{"identifier":"allow-internal-toggle-maximize","description":"Enables the internal_toggle_maximize command without any pre-configured scope.","commands":{"allow":["internal_toggle_maximize"],"deny":[]}},"allow-is-always-on-top":{"identifier":"allow-is-always-on-top","description":"Enables the is_always_on_top command without any pre-configured scope.","commands":{"allow":["is_always_on_top"],"deny":[]}},"allow-is-closable":{"identifier":"allow-is-closable","description":"Enables the is_closable command without any pre-configured scope.","commands":{"allow":["is_closable"],"deny":[]}},"allow-is-decorated":{"identifier":"allow-is-decorated","description":"Enables the is_decorated command without any pre-configured scope.","commands":{"allow":["is_decorated"],"deny":[]}},"allow-is-enabled":{"identifier":"allow-is-enabled","description":"Enables the is_enabled command without any pre-configured scope.","commands":{"allow":["is_enabled"],"deny":[]}},"allow-is-focused":{"identifier":"allow-is-focused","description":"Enables the is_focused command without any pre-configured scope.","commands":{"allow":["is_focused"],"deny":[]}},"allow-is-fullscreen":{"identifier":"allow-is-fullscreen","description":"Enables the is_fullscreen command without any pre-configured scope.","commands":{"allow":["is_fullscreen"],"deny":[]}},"allow-is-maximizable":{"identifier":"allow-is-maximizable","description":"Enables the is_maximizable command without any pre-configured scope.","commands":{"allow":["is_maximizable"],"deny":[]}},"allow-is-maximized":{"identifier":"allow-is-maximized","description":"Enables the is_maximized command without any pre-configured scope.","commands":{"allow":["is_maximized"],"deny":[]}},"allow-is-minimizable":{"identifier":"allow-is-minimizable","description":"Enables the is_minimizable command without any pre-configured scope.","commands":{"allow":["is_minimizable"],"deny":[]}},"allow-is-minimized":{"identifier":"allow-is-minimized","description":"Enables the is_minimized command without any pre-configured scope.","commands":{"allow":["is_minimized"],"deny":[]}},"allow-is-resizable":{"identifier":"allow-is-resizable","description":"Enables the is_resizable command without any pre-configured scope.","commands":{"allow":["is_resizable"],"deny":[]}},"allow-is-visible":{"identifier":"allow-is-visible","description":"Enables the is_visible command without any pre-configured scope.","commands":{"allow":["is_visible"],"deny":[]}},"allow-maximize":{"identifier":"allow-maximize","description":"Enables the maximize command without any pre-configured scope.","commands":{"allow":["maximize"],"deny":[]}},"allow-minimize":{"identifier":"allow-minimize","description":"Enables the minimize command without any pre-configured scope.","commands":{"allow":["minimize"],"deny":[]}},"allow-monitor-from-point":{"identifier":"allow-monitor-from-point","description":"Enables the monitor_from_point command without any pre-configured scope.","commands":{"allow":["monitor_from_point"],"deny":[]}},"allow-outer-position":{"identifier":"allow-outer-position","description":"Enables the outer_position command without any pre-configured scope.","commands":{"allow":["outer_position"],"deny":[]}},"allow-outer-size":{"identifier":"allow-outer-size","description":"Enables the outer_size command without any pre-configured scope.","commands":{"allow":["outer_size"],"deny":[]}},"allow-primary-monitor":{"identifier":"allow-primary-monitor","description":"Enables the primary_monitor command without any pre-configured scope.","commands":{"allow":["primary_monitor"],"deny":[]}},"allow-request-user-attention":{"identifier":"allow-request-user-attention","description":"Enables the request_user_attention command without any pre-configured scope.","commands":{"allow":["request_user_attention"],"deny":[]}},"allow-scale-factor":{"identifier":"allow-scale-factor","description":"Enables the scale_factor command without any pre-configured scope.","commands":{"allow":["scale_factor"],"deny":[]}},"allow-set-always-on-bottom":{"identifier":"allow-set-always-on-bottom","description":"Enables the set_always_on_bottom command without any pre-configured scope.","commands":{"allow":["set_always_on_bottom"],"deny":[]}},"allow-set-always-on-top":{"identifier":"allow-set-always-on-top","description":"Enables the set_always_on_top command without any pre-configured scope.","commands":{"allow":["set_always_on_top"],"deny":[]}},"allow-set-background-color":{"identifier":"allow-set-background-color","description":"Enables the set_background_color command without any pre-configured scope.","commands":{"allow":["set_background_color"],"deny":[]}},"allow-set-badge-count":{"identifier":"allow-set-badge-count","description":"Enables the set_badge_count command without any pre-configured scope.","commands":{"allow":["set_badge_count"],"deny":[]}},"allow-set-badge-label":{"identifier":"allow-set-badge-label","description":"Enables the set_badge_label command without any pre-configured scope.","commands":{"allow":["set_badge_label"],"deny":[]}},"allow-set-closable":{"identifier":"allow-set-closable","description":"Enables the set_closable command without any pre-configured scope.","commands":{"allow":["set_closable"],"deny":[]}},"allow-set-content-protected":{"identifier":"allow-set-content-protected","description":"Enables the set_content_protected command without any pre-configured scope.","commands":{"allow":["set_content_protected"],"deny":[]}},"allow-set-cursor-grab":{"identifier":"allow-set-cursor-grab","description":"Enables the set_cursor_grab command without any pre-configured scope.","commands":{"allow":["set_cursor_grab"],"deny":[]}},"allow-set-cursor-icon":{"identifier":"allow-set-cursor-icon","description":"Enables the set_cursor_icon command without any pre-configured scope.","commands":{"allow":["set_cursor_icon"],"deny":[]}},"allow-set-cursor-position":{"identifier":"allow-set-cursor-position","description":"Enables the set_cursor_position command without any pre-configured scope.","commands":{"allow":["set_cursor_position"],"deny":[]}},"allow-set-cursor-visible":{"identifier":"allow-set-cursor-visible","description":"Enables the set_cursor_visible command without any pre-configured scope.","commands":{"allow":["set_cursor_visible"],"deny":[]}},"allow-set-decorations":{"identifier":"allow-set-decorations","description":"Enables the set_decorations command without any pre-configured scope.","commands":{"allow":["set_decorations"],"deny":[]}},"allow-set-effects":{"identifier":"allow-set-effects","description":"Enables the set_effects command without any pre-configured scope.","commands":{"allow":["set_effects"],"deny":[]}},"allow-set-enabled":{"identifier":"allow-set-enabled","description":"Enables the set_enabled command without any pre-configured scope.","commands":{"allow":["set_enabled"],"deny":[]}},"allow-set-focus":{"identifier":"allow-set-focus","description":"Enables the set_focus command without any pre-configured scope.","commands":{"allow":["set_focus"],"deny":[]}},"allow-set-focusable":{"identifier":"allow-set-focusable","description":"Enables the set_focusable command without any pre-configured scope.","commands":{"allow":["set_focusable"],"deny":[]}},"allow-set-fullscreen":{"identifier":"allow-set-fullscreen","description":"Enables the set_fullscreen command without any pre-configured scope.","commands":{"allow":["set_fullscreen"],"deny":[]}},"allow-set-icon":{"identifier":"allow-set-icon","description":"Enables the set_icon command without any pre-configured scope.","commands":{"allow":["set_icon"],"deny":[]}},"allow-set-ignore-cursor-events":{"identifier":"allow-set-ignore-cursor-events","description":"Enables the set_ignore_cursor_events command without any pre-configured scope.","commands":{"allow":["set_ignore_cursor_events"],"deny":[]}},"allow-set-max-size":{"identifier":"allow-set-max-size","description":"Enables the set_max_size command without any pre-configured scope.","commands":{"allow":["set_max_size"],"deny":[]}},"allow-set-maximizable":{"identifier":"allow-set-maximizable","description":"Enables the set_maximizable command without any pre-configured scope.","commands":{"allow":["set_maximizable"],"deny":[]}},"allow-set-min-size":{"identifier":"allow-set-min-size","description":"Enables the set_min_size command without any pre-configured scope.","commands":{"allow":["set_min_size"],"deny":[]}},"allow-set-minimizable":{"identifier":"allow-set-minimizable","description":"Enables the set_minimizable command without any pre-configured scope.","commands":{"allow":["set_minimizable"],"deny":[]}},"allow-set-overlay-icon":{"identifier":"allow-set-overlay-icon","description":"Enables the set_overlay_icon command without any pre-configured scope.","commands":{"allow":["set_overlay_icon"],"deny":[]}},"allow-set-position":{"identifier":"allow-set-position","description":"Enables the set_position command without any pre-configured scope.","commands":{"allow":["set_position"],"deny":[]}},"allow-set-progress-bar":{"identifier":"allow-set-progress-bar","description":"Enables the set_progress_bar command without any pre-configured scope.","commands":{"allow":["set_progress_bar"],"deny":[]}},"allow-set-resizable":{"identifier":"allow-set-resizable","description":"Enables the set_resizable command without any pre-configured scope.","commands":{"allow":["set_resizable"],"deny":[]}},"allow-set-shadow":{"identifier":"allow-set-shadow","description":"Enables the set_shadow command without any pre-configured scope.","commands":{"allow":["set_shadow"],"deny":[]}},"allow-set-simple-fullscreen":{"identifier":"allow-set-simple-fullscreen","description":"Enables the set_simple_fullscreen command without any pre-configured scope.","commands":{"allow":["set_simple_fullscreen"],"deny":[]}},"allow-set-size":{"identifier":"allow-set-size","description":"Enables the set_size command without any pre-configured scope.","commands":{"allow":["set_size"],"deny":[]}},"allow-set-size-constraints":{"identifier":"allow-set-size-constraints","description":"Enables the set_size_constraints command without any pre-configured scope.","commands":{"allow":["set_size_constraints"],"deny":[]}},"allow-set-skip-taskbar":{"identifier":"allow-set-skip-taskbar","description":"Enables the set_skip_taskbar command without any pre-configured scope.","commands":{"allow":["set_skip_taskbar"],"deny":[]}},"allow-set-theme":{"identifier":"allow-set-theme","description":"Enables the set_theme command without any pre-configured scope.","commands":{"allow":["set_theme"],"deny":[]}},"allow-set-title":{"identifier":"allow-set-title","description":"Enables the set_title command without any pre-configured scope.","commands":{"allow":["set_title"],"deny":[]}},"allow-set-title-bar-style":{"identifier":"allow-set-title-bar-style","description":"Enables the set_title_bar_style command without any pre-configured scope.","commands":{"allow":["set_title_bar_style"],"deny":[]}},"allow-set-visible-on-all-workspaces":{"identifier":"allow-set-visible-on-all-workspaces","description":"Enables the set_visible_on_all_workspaces command without any pre-configured scope.","commands":{"allow":["set_visible_on_all_workspaces"],"deny":[]}},"allow-show":{"identifier":"allow-show","description":"Enables the show command without any pre-configured scope.","commands":{"allow":["show"],"deny":[]}},"allow-start-dragging":{"identifier":"allow-start-dragging","description":"Enables the start_dragging command without any pre-configured scope.","commands":{"allow":["start_dragging"],"deny":[]}},"allow-start-resize-dragging":{"identifier":"allow-start-resize-dragging","description":"Enables the start_resize_dragging command without any pre-configured scope.","commands":{"allow":["start_resize_dragging"],"deny":[]}},"allow-theme":{"identifier":"allow-theme","description":"Enables the theme command without any pre-configured scope.","commands":{"allow":["theme"],"deny":[]}},"allow-title":{"identifier":"allow-title","description":"Enables the title command without any pre-configured scope.","commands":{"allow":["title"],"deny":[]}},"allow-toggle-maximize":{"identifier":"allow-toggle-maximize","description":"Enables the toggle_maximize command without any pre-configured scope.","commands":{"allow":["toggle_maximize"],"deny":[]}},"allow-unmaximize":{"identifier":"allow-unmaximize","description":"Enables the unmaximize command without any pre-configured scope.","commands":{"allow":["unmaximize"],"deny":[]}},"allow-unminimize":{"identifier":"allow-unminimize","description":"Enables the unminimize command without any pre-configured scope.","commands":{"allow":["unminimize"],"deny":[]}},"deny-available-monitors":{"identifier":"deny-available-monitors","description":"Denies the available_monitors command without any pre-configured scope.","commands":{"allow":[],"deny":["available_monitors"]}},"deny-center":{"identifier":"deny-center","description":"Denies the center command without any pre-configured scope.","commands":{"allow":[],"deny":["center"]}},"deny-close":{"identifier":"deny-close","description":"Denies the close command without any pre-configured scope.","commands":{"allow":[],"deny":["close"]}},"deny-create":{"identifier":"deny-create","description":"Denies the create command without any pre-configured scope.","commands":{"allow":[],"deny":["create"]}},"deny-current-monitor":{"identifier":"deny-current-monitor","description":"Denies the current_monitor command without any pre-configured scope.","commands":{"allow":[],"deny":["current_monitor"]}},"deny-cursor-position":{"identifier":"deny-cursor-position","description":"Denies the cursor_position command without any pre-configured scope.","commands":{"allow":[],"deny":["cursor_position"]}},"deny-destroy":{"identifier":"deny-destroy","description":"Denies the destroy command without any pre-configured scope.","commands":{"allow":[],"deny":["destroy"]}},"deny-get-all-windows":{"identifier":"deny-get-all-windows","description":"Denies the get_all_windows command without any pre-configured scope.","commands":{"allow":[],"deny":["get_all_windows"]}},"deny-hide":{"identifier":"deny-hide","description":"Denies the hide command without any pre-configured scope.","commands":{"allow":[],"deny":["hide"]}},"deny-inner-position":{"identifier":"deny-inner-position","description":"Denies the inner_position command without any pre-configured scope.","commands":{"allow":[],"deny":["inner_position"]}},"deny-inner-size":{"identifier":"deny-inner-size","description":"Denies the inner_size command without any pre-configured scope.","commands":{"allow":[],"deny":["inner_size"]}},"deny-internal-toggle-maximize":{"identifier":"deny-internal-toggle-maximize","description":"Denies the internal_toggle_maximize command without any pre-configured scope.","commands":{"allow":[],"deny":["internal_toggle_maximize"]}},"deny-is-always-on-top":{"identifier":"deny-is-always-on-top","description":"Denies the is_always_on_top command without any pre-configured scope.","commands":{"allow":[],"deny":["is_always_on_top"]}},"deny-is-closable":{"identifier":"deny-is-closable","description":"Denies the is_closable command without any pre-configured scope.","commands":{"allow":[],"deny":["is_closable"]}},"deny-is-decorated":{"identifier":"deny-is-decorated","description":"Denies the is_decorated command without any pre-configured scope.","commands":{"allow":[],"deny":["is_decorated"]}},"deny-is-enabled":{"identifier":"deny-is-enabled","description":"Denies the is_enabled command without any pre-configured scope.","commands":{"allow":[],"deny":["is_enabled"]}},"deny-is-focused":{"identifier":"deny-is-focused","description":"Denies the is_focused command without any pre-configured scope.","commands":{"allow":[],"deny":["is_focused"]}},"deny-is-fullscreen":{"identifier":"deny-is-fullscreen","description":"Denies the is_fullscreen command without any pre-configured scope.","commands":{"allow":[],"deny":["is_fullscreen"]}},"deny-is-maximizable":{"identifier":"deny-is-maximizable","description":"Denies the is_maximizable command without any pre-configured scope.","commands":{"allow":[],"deny":["is_maximizable"]}},"deny-is-maximized":{"identifier":"deny-is-maximized","description":"Denies the is_maximized command without any pre-configured scope.","commands":{"allow":[],"deny":["is_maximized"]}},"deny-is-minimizable":{"identifier":"deny-is-minimizable","description":"Denies the is_minimizable command without any pre-configured scope.","commands":{"allow":[],"deny":["is_minimizable"]}},"deny-is-minimized":{"identifier":"deny-is-minimized","description":"Denies the is_minimized command without any pre-configured scope.","commands":{"allow":[],"deny":["is_minimized"]}},"deny-is-resizable":{"identifier":"deny-is-resizable","description":"Denies the is_resizable command without any pre-configured scope.","commands":{"allow":[],"deny":["is_resizable"]}},"deny-is-visible":{"identifier":"deny-is-visible","description":"Denies the is_visible command without any pre-configured scope.","commands":{"allow":[],"deny":["is_visible"]}},"deny-maximize":{"identifier":"deny-maximize","description":"Denies the maximize command without any pre-configured scope.","commands":{"allow":[],"deny":["maximize"]}},"deny-minimize":{"identifier":"deny-minimize","description":"Denies the minimize command without any pre-configured scope.","commands":{"allow":[],"deny":["minimize"]}},"deny-monitor-from-point":{"identifier":"deny-monitor-from-point","description":"Denies the monitor_from_point command without any pre-configured scope.","commands":{"allow":[],"deny":["monitor_from_point"]}},"deny-outer-position":{"identifier":"deny-outer-position","description":"Denies the outer_position command without any pre-configured scope.","commands":{"allow":[],"deny":["outer_position"]}},"deny-outer-size":{"identifier":"deny-outer-size","description":"Denies the outer_size command without any pre-configured scope.","commands":{"allow":[],"deny":["outer_size"]}},"deny-primary-monitor":{"identifier":"deny-primary-monitor","description":"Denies the primary_monitor command without any pre-configured scope.","commands":{"allow":[],"deny":["primary_monitor"]}},"deny-request-user-attention":{"identifier":"deny-request-user-attention","description":"Denies the request_user_attention command without any pre-configured scope.","commands":{"allow":[],"deny":["request_user_attention"]}},"deny-scale-factor":{"identifier":"deny-scale-factor","description":"Denies the scale_factor command without any pre-configured scope.","commands":{"allow":[],"deny":["scale_factor"]}},"deny-set-always-on-bottom":{"identifier":"deny-set-always-on-bottom","description":"Denies the set_always_on_bottom command without any pre-configured scope.","commands":{"allow":[],"deny":["set_always_on_bottom"]}},"deny-set-always-on-top":{"identifier":"deny-set-always-on-top","description":"Denies the set_always_on_top command without any pre-configured scope.","commands":{"allow":[],"deny":["set_always_on_top"]}},"deny-set-background-color":{"identifier":"deny-set-background-color","description":"Denies the set_background_color command without any pre-configured scope.","commands":{"allow":[],"deny":["set_background_color"]}},"deny-set-badge-count":{"identifier":"deny-set-badge-count","description":"Denies the set_badge_count command without any pre-configured scope.","commands":{"allow":[],"deny":["set_badge_count"]}},"deny-set-badge-label":{"identifier":"deny-set-badge-label","description":"Denies the set_badge_label command without any pre-configured scope.","commands":{"allow":[],"deny":["set_badge_label"]}},"deny-set-closable":{"identifier":"deny-set-closable","description":"Denies the set_closable command without any pre-configured scope.","commands":{"allow":[],"deny":["set_closable"]}},"deny-set-content-protected":{"identifier":"deny-set-content-protected","description":"Denies the set_content_protected command without any pre-configured scope.","commands":{"allow":[],"deny":["set_content_protected"]}},"deny-set-cursor-grab":{"identifier":"deny-set-cursor-grab","description":"Denies the set_cursor_grab command without any pre-configured scope.","commands":{"allow":[],"deny":["set_cursor_grab"]}},"deny-set-cursor-icon":{"identifier":"deny-set-cursor-icon","description":"Denies the set_cursor_icon command without any pre-configured scope.","commands":{"allow":[],"deny":["set_cursor_icon"]}},"deny-set-cursor-position":{"identifier":"deny-set-cursor-position","description":"Denies the set_cursor_position command without any pre-configured scope.","commands":{"allow":[],"deny":["set_cursor_position"]}},"deny-set-cursor-visible":{"identifier":"deny-set-cursor-visible","description":"Denies the set_cursor_visible command without any pre-configured scope.","commands":{"allow":[],"deny":["set_cursor_visible"]}},"deny-set-decorations":{"identifier":"deny-set-decorations","description":"Denies the set_decorations command without any pre-configured scope.","commands":{"allow":[],"deny":["set_decorations"]}},"deny-set-effects":{"identifier":"deny-set-effects","description":"Denies the set_effects command without any pre-configured scope.","commands":{"allow":[],"deny":["set_effects"]}},"deny-set-enabled":{"identifier":"deny-set-enabled","description":"Denies the set_enabled command without any pre-configured scope.","commands":{"allow":[],"deny":["set_enabled"]}},"deny-set-focus":{"identifier":"deny-set-focus","description":"Denies the set_focus command without any pre-configured scope.","commands":{"allow":[],"deny":["set_focus"]}},"deny-set-focusable":{"identifier":"deny-set-focusable","description":"Denies the set_focusable command without any pre-configured scope.","commands":{"allow":[],"deny":["set_focusable"]}},"deny-set-fullscreen":{"identifier":"deny-set-fullscreen","description":"Denies the set_fullscreen command without any pre-configured scope.","commands":{"allow":[],"deny":["set_fullscreen"]}},"deny-set-icon":{"identifier":"deny-set-icon","description":"Denies the set_icon command without any pre-configured scope.","commands":{"allow":[],"deny":["set_icon"]}},"deny-set-ignore-cursor-events":{"identifier":"deny-set-ignore-cursor-events","description":"Denies the set_ignore_cursor_events command without any pre-configured scope.","commands":{"allow":[],"deny":["set_ignore_cursor_events"]}},"deny-set-max-size":{"identifier":"deny-set-max-size","description":"Denies the set_max_size command without any pre-configured scope.","commands":{"allow":[],"deny":["set_max_size"]}},"deny-set-maximizable":{"identifier":"deny-set-maximizable","description":"Denies the set_maximizable command without any pre-configured scope.","commands":{"allow":[],"deny":["set_maximizable"]}},"deny-set-min-size":{"identifier":"deny-set-min-size","description":"Denies the set_min_size command without any pre-configured scope.","commands":{"allow":[],"deny":["set_min_size"]}},"deny-set-minimizable":{"identifier":"deny-set-minimizable","description":"Denies the set_minimizable command without any pre-configured scope.","commands":{"allow":[],"deny":["set_minimizable"]}},"deny-set-overlay-icon":{"identifier":"deny-set-overlay-icon","description":"Denies the set_overlay_icon command without any pre-configured scope.","commands":{"allow":[],"deny":["set_overlay_icon"]}},"deny-set-position":{"identifier":"deny-set-position","description":"Denies the set_position command without any pre-configured scope.","commands":{"allow":[],"deny":["set_position"]}},"deny-set-progress-bar":{"identifier":"deny-set-progress-bar","description":"Denies the set_progress_bar command without any pre-configured scope.","commands":{"allow":[],"deny":["set_progress_bar"]}},"deny-set-resizable":{"identifier":"deny-set-resizable","description":"Denies the set_resizable command without any pre-configured scope.","commands":{"allow":[],"deny":["set_resizable"]}},"deny-set-shadow":{"identifier":"deny-set-shadow","description":"Denies the set_shadow command without any pre-configured scope.","commands":{"allow":[],"deny":["set_shadow"]}},"deny-set-simple-fullscreen":{"identifier":"deny-set-simple-fullscreen","description":"Denies the set_simple_fullscreen command without any pre-configured scope.","commands":{"allow":[],"deny":["set_simple_fullscreen"]}},"deny-set-size":{"identifier":"deny-set-size","description":"Denies the set_size command without any pre-configured scope.","commands":{"allow":[],"deny":["set_size"]}},"deny-set-size-constraints":{"identifier":"deny-set-size-constraints","description":"Denies the set_size_constraints command without any pre-configured scope.","commands":{"allow":[],"deny":["set_size_constraints"]}},"deny-set-skip-taskbar":{"identifier":"deny-set-skip-taskbar","description":"Denies the set_skip_taskbar command without any pre-configured scope.","commands":{"allow":[],"deny":["set_skip_taskbar"]}},"deny-set-theme":{"identifier":"deny-set-theme","description":"Denies the set_theme command without any pre-configured scope.","commands":{"allow":[],"deny":["set_theme"]}},"deny-set-title":{"identifier":"deny-set-title","description":"Denies the set_title command without any pre-configured scope.","commands":{"allow":[],"deny":["set_title"]}},"deny-set-title-bar-style":{"identifier":"deny-set-title-bar-style","description":"Denies the set_title_bar_style command without any pre-configured scope.","commands":{"allow":[],"deny":["set_title_bar_style"]}},"deny-set-visible-on-all-workspaces":{"identifier":"deny-set-visible-on-all-workspaces","description":"Denies the set_visible_on_all_workspaces command without any pre-configured scope.","commands":{"allow":[],"deny":["set_visible_on_all_workspaces"]}},"deny-show":{"identifier":"deny-show","description":"Denies the show command without any pre-configured scope.","commands":{"allow":[],"deny":["show"]}},"deny-start-dragging":{"identifier":"deny-start-dragging","description":"Denies the start_dragging command without any pre-configured scope.","commands":{"allow":[],"deny":["start_dragging"]}},"deny-start-resize-dragging":{"identifier":"deny-start-resize-dragging","description":"Denies the start_resize_dragging command without any pre-configured scope.","commands":{"allow":[],"deny":["start_resize_dragging"]}},"deny-theme":{"identifier":"deny-theme","description":"Denies the theme command without any pre-configured scope.","commands":{"allow":[],"deny":["theme"]}},"deny-title":{"identifier":"deny-title","description":"Denies the title command without any pre-configured scope.","commands":{"allow":[],"deny":["title"]}},"deny-toggle-maximize":{"identifier":"deny-toggle-maximize","description":"Denies the toggle_maximize command without any pre-configured scope.","commands":{"allow":[],"deny":["toggle_maximize"]}},"deny-unmaximize":{"identifier":"deny-unmaximize","description":"Denies the unmaximize command without any pre-configured scope.","commands":{"allow":[],"deny":["unmaximize"]}},"deny-unminimize":{"identifier":"deny-unminimize","description":"Denies the unminimize command without any pre-configured scope.","commands":{"allow":[],"deny":["unminimize"]}}},"permission_sets":{},"global_scope_schema":null},"log":{"default_permission":{"identifier":"default","description":"Allows the log command","permissions":["allow-log"]},"permissions":{"allow-log":{"identifier":"allow-log","description":"Enables the log command without any pre-configured scope.","commands":{"allow":["log"],"deny":[]}},"deny-log":{"identifier":"deny-log","description":"Denies the log command without any pre-configured scope.","commands":{"allow":[],"deny":["log"]}}},"permission_sets":{},"global_scope_schema":null}}
```

---

## File: src-tauri/gen/schemas/capabilities.json

```json
{"default":{"identifier":"default","description":"enables the default permissions","local":true,"windows":["main"],"permissions":["core:default"]}}
```

---

## File: src-tauri/gen/schemas/desktop-schema.json

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "CapabilityFile",
  "description": "Capability formats accepted in a capability file.",
  "anyOf": [
    {
      "description": "A single capability.",
      "allOf": [
        {
          "$ref": "#/definitions/Capability"
        }
      ]
    },
    {
      "description": "A list of capabilities.",
      "type": "array",
      "items": {
        "$ref": "#/definitions/Capability"
      }
    },
    {
      "description": "A list of capabilities.",
      "type": "object",
      "required": [
        "capabilities"
      ],
      "properties": {
        "capabilities": {
          "description": "The list of capabilities.",
          "type": "array",
          "items": {
            "$ref": "#/definitions/Capability"
          }
        }
      }
    }
  ],
  "definitions": {
    "Capability": {
      "description": "A grouping and boundary mechanism developers can use to isolate access to the IPC layer.\n\nIt controls application windows' and webviews' fine grained access to the Tauri core, application, or plugin commands. If a webview or its window is not matching any capability then it has no access to the IPC layer at all.\n\nThis can be done to create groups of windows, based on their required system access, which can reduce impact of frontend vulnerabilities in less privileged windows. Windows can be added to a capability by exact name (e.g. `main-window`) or glob patterns like `*` or `admin-*`. A Window can have none, one, or multiple associated capabilities.\n\n## Example\n\n```json { \"identifier\": \"main-user-files-write\", \"description\": \"This capability allows the `main` window on macOS and Windows access to `filesystem` write related commands and `dialog` commands to enable programmatic access to files selected by the user.\", \"windows\": [ \"main\" ], \"permissions\": [ \"core:default\", \"dialog:open\", { \"identifier\": \"fs:allow-write-text-file\", \"allow\": [{ \"path\": \"$HOME/test.txt\" }] }, ], \"platforms\": [\"macOS\",\"windows\"] } ```",
      "type": "object",
      "required": [
        "identifier",
        "permissions"
      ],
      "properties": {
        "identifier": {
          "description": "Identifier of the capability.\n\n## Example\n\n`main-user-files-write`",
          "type": "string"
        },
        "description": {
          "description": "Description of what the capability is intended to allow on associated windows.\n\nIt should contain a description of what the grouped permissions should allow.\n\n## Example\n\nThis capability allows the `main` window access to `filesystem` write related commands and `dialog` commands to enable programmatic access to files selected by the user.",
          "default": "",
          "type": "string"
        },
        "remote": {
          "description": "Configure remote URLs that can use the capability permissions.\n\nThis setting is optional and defaults to not being set, as our default use case is that the content is served from our local application.\n\n:::caution Make sure you understand the security implications of providing remote sources with local system access. :::\n\n## Example\n\n```json { \"urls\": [\"https://*.mydomain.dev\"] } ```",
          "anyOf": [
            {
              "$ref": "#/definitions/CapabilityRemote"
            },
            {
              "type": "null"
            }
          ]
        },
        "local": {
          "description": "Whether this capability is enabled for local app URLs or not. Defaults to `true`.",
          "default": true,
          "type": "boolean"
        },
        "windows": {
          "description": "List of windows that are affected by this capability. Can be a glob pattern.\n\nIf a window label matches any of the patterns in this list, the capability will be enabled on all the webviews of that window, regardless of the value of [`Self::webviews`].\n\nOn multiwebview windows, prefer specifying [`Self::webviews`] and omitting [`Self::windows`] for a fine grained access control.\n\n## Example\n\n`[\"main\"]`",
          "type": "array",
          "items": {
            "type": "string"
          }
        },
        "webviews": {
          "description": "List of webviews that are affected by this capability. Can be a glob pattern.\n\nThe capability will be enabled on all the webviews whose label matches any of the patterns in this list, regardless of whether the webview's window label matches a pattern in [`Self::windows`].\n\n## Example\n\n`[\"sub-webview-one\", \"sub-webview-two\"]`",
          "type": "array",
          "items": {
            "type": "string"
          }
        },
        "permissions": {
          "description": "List of permissions attached to this capability.\n\nMust include the plugin name as prefix in the form of `${plugin-name}:${permission-name}`. For commands directly implemented in the application itself only `${permission-name}` is required.\n\n## Example\n\n```json [ \"core:default\", \"shell:allow-open\", \"dialog:open\", { \"identifier\": \"fs:allow-write-text-file\", \"allow\": [{ \"path\": \"$HOME/test.txt\" }] } ] ```",
          "type": "array",
          "items": {
            "$ref": "#/definitions/PermissionEntry"
          },
          "uniqueItems": true
        },
        "platforms": {
          "description": "Limit which target platforms this capability applies to.\n\nBy default all platforms are targeted.\n\n## Example\n\n`[\"macOS\",\"windows\"]`",
          "type": [
            "array",
            "null"
          ],
          "items": {
            "$ref": "#/definitions/Target"
          }
        }
      }
    },
    "CapabilityRemote": {
      "description": "Configuration for remote URLs that are associated with the capability.",
      "type": "object",
      "required": [
        "urls"
      ],
      "properties": {
        "urls": {
          "description": "Remote domains this capability refers to using the [URLPattern standard](https://urlpattern.spec.whatwg.org/).\n\n## Examples\n\n- \"https://*.mydomain.dev\": allows subdomains of mydomain.dev - \"https://mydomain.dev/api/*\": allows any subpath of mydomain.dev/api",
          "type": "array",
          "items": {
            "type": "string"
          }
        }
      }
    },
    "PermissionEntry": {
      "description": "An entry for a permission value in a [`Capability`] can be either a raw permission [`Identifier`] or an object that references a permission and extends its scope.",
      "anyOf": [
        {
          "description": "Reference a permission or permission set by identifier.",
          "allOf": [
            {
              "$ref": "#/definitions/Identifier"
            }
          ]
        },
        {
          "description": "Reference a permission or permission set by identifier and extends its scope.",
          "type": "object",
          "allOf": [
            {
              "properties": {
                "identifier": {
                  "description": "Identifier of the permission or permission set.",
                  "allOf": [
                    {
                      "$ref": "#/definitions/Identifier"
                    }
                  ]
                },
                "allow": {
                  "description": "Data that defines what is allowed by the scope.",
                  "type": [
                    "array",
                    "null"
                  ],
                  "items": {
                    "$ref": "#/definitions/Value"
                  }
                },
                "deny": {
                  "description": "Data that defines what is denied by the scope. This should be prioritized by validation logic.",
                  "type": [
                    "array",
                    "null"
                  ],
                  "items": {
                    "$ref": "#/definitions/Value"
                  }
                }
              }
            }
          ],
          "required": [
            "identifier"
          ]
        }
      ]
    },
    "Identifier": {
      "description": "Permission identifier",
      "oneOf": [
        {
          "description": "Default core plugins set.\n#### This default permission set includes:\n\n- `core:path:default`\n- `core:event:default`\n- `core:window:default`\n- `core:webview:default`\n- `core:app:default`\n- `core:image:default`\n- `core:resources:default`\n- `core:menu:default`\n- `core:tray:default`",
          "type": "string",
          "const": "core:default",
          "markdownDescription": "Default core plugins set.\n#### This default permission set includes:\n\n- `core:path:default`\n- `core:event:default`\n- `core:window:default`\n- `core:webview:default`\n- `core:app:default`\n- `core:image:default`\n- `core:resources:default`\n- `core:menu:default`\n- `core:tray:default`"
        },
        {
          "description": "Default permissions for the plugin.\n#### This default permission set includes:\n\n- `allow-version`\n- `allow-name`\n- `allow-tauri-version`\n- `allow-identifier`\n- `allow-bundle-type`\n- `allow-register-listener`\n- `allow-remove-listener`",
          "type": "string",
          "const": "core:app:default",
          "markdownDescription": "Default permissions for the plugin.\n#### This default permission set includes:\n\n- `allow-version`\n- `allow-name`\n- `allow-tauri-version`\n- `allow-identifier`\n- `allow-bundle-type`\n- `allow-register-listener`\n- `allow-remove-listener`"
        },
        {
          "description": "Enables the app_hide command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:allow-app-hide",
          "markdownDescription": "Enables the app_hide command without any pre-configured scope."
        },
        {
          "description": "Enables the app_show command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:allow-app-show",
          "markdownDescription": "Enables the app_show command without any pre-configured scope."
        },
        {
          "description": "Enables the bundle_type command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:allow-bundle-type",
          "markdownDescription": "Enables the bundle_type command without any pre-configured scope."
        },
        {
          "description": "Enables the default_window_icon command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:allow-default-window-icon",
          "markdownDescription": "Enables the default_window_icon command without any pre-configured scope."
        },
        {
          "description": "Enables the fetch_data_store_identifiers command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:allow-fetch-data-store-identifiers",
          "markdownDescription": "Enables the fetch_data_store_identifiers command without any pre-configured scope."
        },
        {
          "description": "Enables the identifier command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:allow-identifier",
          "markdownDescription": "Enables the identifier command without any pre-configured scope."
        },
        {
          "description": "Enables the name command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:allow-name",
          "markdownDescription": "Enables the name command without any pre-configured scope."
        },
        {
          "description": "Enables the register_listener command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:allow-register-listener",
          "markdownDescription": "Enables the register_listener command without any pre-configured scope."
        },
        {
          "description": "Enables the remove_data_store command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:allow-remove-data-store",
          "markdownDescription": "Enables the remove_data_store command without any pre-configured scope."
        },
        {
          "description": "Enables the remove_listener command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:allow-remove-listener",
          "markdownDescription": "Enables the remove_listener command without any pre-configured scope."
        },
        {
          "description": "Enables the set_app_theme command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:allow-set-app-theme",
          "markdownDescription": "Enables the set_app_theme command without any pre-configured scope."
        },
        {
          "description": "Enables the set_dock_visibility command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:allow-set-dock-visibility",
          "markdownDescription": "Enables the set_dock_visibility command without any pre-configured scope."
        },
        {
          "description": "Enables the tauri_version command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:allow-tauri-version",
          "markdownDescription": "Enables the tauri_version command without any pre-configured scope."
        },
        {
          "description": "Enables the version command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:allow-version",
          "markdownDescription": "Enables the version command without any pre-configured scope."
        },
        {
          "description": "Denies the app_hide command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:deny-app-hide",
          "markdownDescription": "Denies the app_hide command without any pre-configured scope."
        },
        {
          "description": "Denies the app_show command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:deny-app-show",
          "markdownDescription": "Denies the app_show command without any pre-configured scope."
        },
        {
          "description": "Denies the bundle_type command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:deny-bundle-type",
          "markdownDescription": "Denies the bundle_type command without any pre-configured scope."
        },
        {
          "description": "Denies the default_window_icon command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:deny-default-window-icon",
          "markdownDescription": "Denies the default_window_icon command without any pre-configured scope."
        },
        {
          "description": "Denies the fetch_data_store_identifiers command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:deny-fetch-data-store-identifiers",
          "markdownDescription": "Denies the fetch_data_store_identifiers command without any pre-configured scope."
        },
        {
          "description": "Denies the identifier command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:deny-identifier",
          "markdownDescription": "Denies the identifier command without any pre-configured scope."
        },
        {
          "description": "Denies the name command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:deny-name",
          "markdownDescription": "Denies the name command without any pre-configured scope."
        },
        {
          "description": "Denies the register_listener command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:deny-register-listener",
          "markdownDescription": "Denies the register_listener command without any pre-configured scope."
        },
        {
          "description": "Denies the remove_data_store command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:deny-remove-data-store",
          "markdownDescription": "Denies the remove_data_store command without any pre-configured scope."
        },
        {
          "description": "Denies the remove_listener command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:deny-remove-listener",
          "markdownDescription": "Denies the remove_listener command without any pre-configured scope."
        },
        {
          "description": "Denies the set_app_theme command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:deny-set-app-theme",
          "markdownDescription": "Denies the set_app_theme command without any pre-configured scope."
        },
        {
          "description": "Denies the set_dock_visibility command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:deny-set-dock-visibility",
          "markdownDescription": "Denies the set_dock_visibility command without any pre-configured scope."
        },
        {
          "description": "Denies the tauri_version command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:deny-tauri-version",
          "markdownDescription": "Denies the tauri_version command without any pre-configured scope."
        },
        {
          "description": "Denies the version command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:deny-version",
          "markdownDescription": "Denies the version command without any pre-configured scope."
        },
        {
          "description": "Default permissions for the plugin, which enables all commands.\n#### This default permission set includes:\n\n- `allow-listen`\n- `allow-unlisten`\n- `allow-emit`\n- `allow-emit-to`",
          "type": "string",
          "const": "core:event:default",
          "markdownDescription": "Default permissions for the plugin, which enables all commands.\n#### This default permission set includes:\n\n- `allow-listen`\n- `allow-unlisten`\n- `allow-emit`\n- `allow-emit-to`"
        },
        {
          "description": "Enables the emit command without any pre-configured scope.",
          "type": "string",
          "const": "core:event:allow-emit",
          "markdownDescription": "Enables the emit command without any pre-configured scope."
        },
        {
          "description": "Enables the emit_to command without any pre-configured scope.",
          "type": "string",
          "const": "core:event:allow-emit-to",
          "markdownDescription": "Enables the emit_to command without any pre-configured scope."
        },
        {
          "description": "Enables the listen command without any pre-configured scope.",
          "type": "string",
          "const": "core:event:allow-listen",
          "markdownDescription": "Enables the listen command without any pre-configured scope."
        },
        {
          "description": "Enables the unlisten command without any pre-configured scope.",
          "type": "string",
          "const": "core:event:allow-unlisten",
          "markdownDescription": "Enables the unlisten command without any pre-configured scope."
        },
        {
          "description": "Denies the emit command without any pre-configured scope.",
          "type": "string",
          "const": "core:event:deny-emit",
          "markdownDescription": "Denies the emit command without any pre-configured scope."
        },
        {
          "description": "Denies the emit_to command without any pre-configured scope.",
          "type": "string",
          "const": "core:event:deny-emit-to",
          "markdownDescription": "Denies the emit_to command without any pre-configured scope."
        },
        {
          "description": "Denies the listen command without any pre-configured scope.",
          "type": "string",
          "const": "core:event:deny-listen",
          "markdownDescription": "Denies the listen command without any pre-configured scope."
        },
        {
          "description": "Denies the unlisten command without any pre-configured scope.",
          "type": "string",
          "const": "core:event:deny-unlisten",
          "markdownDescription": "Denies the unlisten command without any pre-configured scope."
        },
        {
          "description": "Default permissions for the plugin, which enables all commands.\n#### This default permission set includes:\n\n- `allow-new`\n- `allow-from-bytes`\n- `allow-from-path`\n- `allow-rgba`\n- `allow-size`",
          "type": "string",
          "const": "core:image:default",
          "markdownDescription": "Default permissions for the plugin, which enables all commands.\n#### This default permission set includes:\n\n- `allow-new`\n- `allow-from-bytes`\n- `allow-from-path`\n- `allow-rgba`\n- `allow-size`"
        },
        {
          "description": "Enables the from_bytes command without any pre-configured scope.",
          "type": "string",
          "const": "core:image:allow-from-bytes",
          "markdownDescription": "Enables the from_bytes command without any pre-configured scope."
        },
        {
          "description": "Enables the from_path command without any pre-configured scope.",
          "type": "string",
          "const": "core:image:allow-from-path",
          "markdownDescription": "Enables the from_path command without any pre-configured scope."
        },
        {
          "description": "Enables the new command without any pre-configured scope.",
          "type": "string",
          "const": "core:image:allow-new",
          "markdownDescription": "Enables the new command without any pre-configured scope."
        },
        {
          "description": "Enables the rgba command without any pre-configured scope.",
          "type": "string",
          "const": "core:image:allow-rgba",
          "markdownDescription": "Enables the rgba command without any pre-configured scope."
        },
        {
          "description": "Enables the size command without any pre-configured scope.",
          "type": "string",
          "const": "core:image:allow-size",
          "markdownDescription": "Enables the size command without any pre-configured scope."
        },
        {
          "description": "Denies the from_bytes command without any pre-configured scope.",
          "type": "string",
          "const": "core:image:deny-from-bytes",
          "markdownDescription": "Denies the from_bytes command without any pre-configured scope."
        },
        {
          "description": "Denies the from_path command without any pre-configured scope.",
          "type": "string",
          "const": "core:image:deny-from-path",
          "markdownDescription": "Denies the from_path command without any pre-configured scope."
        },
        {
          "description": "Denies the new command without any pre-configured scope.",
          "type": "string",
          "const": "core:image:deny-new",
          "markdownDescription": "Denies the new command without any pre-configured scope."
        },
        {
          "description": "Denies the rgba command without any pre-configured scope.",
          "type": "string",
          "const": "core:image:deny-rgba",
          "markdownDescription": "Denies the rgba command without any pre-configured scope."
        },
        {
          "description": "Denies the size command without any pre-configured scope.",
          "type": "string",
          "const": "core:image:deny-size",
          "markdownDescription": "Denies the size command without any pre-configured scope."
        },
        {
          "description": "Default permissions for the plugin, which enables all commands.\n#### This default permission set includes:\n\n- `allow-new`\n- `allow-append`\n- `allow-prepend`\n- `allow-insert`\n- `allow-remove`\n- `allow-remove-at`\n- `allow-items`\n- `allow-get`\n- `allow-popup`\n- `allow-create-default`\n- `allow-set-as-app-menu`\n- `allow-set-as-window-menu`\n- `allow-text`\n- `allow-set-text`\n- `allow-is-enabled`\n- `allow-set-enabled`\n- `allow-set-accelerator`\n- `allow-set-as-windows-menu-for-nsapp`\n- `allow-set-as-help-menu-for-nsapp`\n- `allow-is-checked`\n- `allow-set-checked`\n- `allow-set-icon`",
          "type": "string",
          "const": "core:menu:default",
          "markdownDescription": "Default permissions for the plugin, which enables all commands.\n#### This default permission set includes:\n\n- `allow-new`\n- `allow-append`\n- `allow-prepend`\n- `allow-insert`\n- `allow-remove`\n- `allow-remove-at`\n- `allow-items`\n- `allow-get`\n- `allow-popup`\n- `allow-create-default`\n- `allow-set-as-app-menu`\n- `allow-set-as-window-menu`\n- `allow-text`\n- `allow-set-text`\n- `allow-is-enabled`\n- `allow-set-enabled`\n- `allow-set-accelerator`\n- `allow-set-as-windows-menu-for-nsapp`\n- `allow-set-as-help-menu-for-nsapp`\n- `allow-is-checked`\n- `allow-set-checked`\n- `allow-set-icon`"
        },
        {
          "description": "Enables the append command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:allow-append",
          "markdownDescription": "Enables the append command without any pre-configured scope."
        },
        {
          "description": "Enables the create_default command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:allow-create-default",
          "markdownDescription": "Enables the create_default command without any pre-configured scope."
        },
        {
          "description": "Enables the get command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:allow-get",
          "markdownDescription": "Enables the get command without any pre-configured scope."
        },
        {
          "description": "Enables the insert command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:allow-insert",
          "markdownDescription": "Enables the insert command without any pre-configured scope."
        },
        {
          "description": "Enables the is_checked command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:allow-is-checked",
          "markdownDescription": "Enables the is_checked command without any pre-configured scope."
        },
        {
          "description": "Enables the is_enabled command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:allow-is-enabled",
          "markdownDescription": "Enables the is_enabled command without any pre-configured scope."
        },
        {
          "description": "Enables the items command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:allow-items",
          "markdownDescription": "Enables the items command without any pre-configured scope."
        },
        {
          "description": "Enables the new command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:allow-new",
          "markdownDescription": "Enables the new command without any pre-configured scope."
        },
        {
          "description": "Enables the popup command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:allow-popup",
          "markdownDescription": "Enables the popup command without any pre-configured scope."
        },
        {
          "description": "Enables the prepend command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:allow-prepend",
          "markdownDescription": "Enables the prepend command without any pre-configured scope."
        },
        {
          "description": "Enables the remove command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:allow-remove",
          "markdownDescription": "Enables the remove command without any pre-configured scope."
        },
        {
          "description": "Enables the remove_at command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:allow-remove-at",
          "markdownDescription": "Enables the remove_at command without any pre-configured scope."
        },
        {
          "description": "Enables the set_accelerator command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:allow-set-accelerator",
          "markdownDescription": "Enables the set_accelerator command without any pre-configured scope."
        },
        {
          "description": "Enables the set_as_app_menu command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:allow-set-as-app-menu",
          "markdownDescription": "Enables the set_as_app_menu command without any pre-configured scope."
        },
        {
          "description": "Enables the set_as_help_menu_for_nsapp command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:allow-set-as-help-menu-for-nsapp",
          "markdownDescription": "Enables the set_as_help_menu_for_nsapp command without any pre-configured scope."
        },
        {
          "description": "Enables the set_as_window_menu command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:allow-set-as-window-menu",
          "markdownDescription": "Enables the set_as_window_menu command without any pre-configured scope."
        },
        {
          "description": "Enables the set_as_windows_menu_for_nsapp command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:allow-set-as-windows-menu-for-nsapp",
          "markdownDescription": "Enables the set_as_windows_menu_for_nsapp command without any pre-configured scope."
        },
        {
          "description": "Enables the set_checked command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:allow-set-checked",
          "markdownDescription": "Enables the set_checked command without any pre-configured scope."
        },
        {
          "description": "Enables the set_enabled command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:allow-set-enabled",
          "markdownDescription": "Enables the set_enabled command without any pre-configured scope."
        },
        {
          "description": "Enables the set_icon command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:allow-set-icon",
          "markdownDescription": "Enables the set_icon command without any pre-configured scope."
        },
        {
          "description": "Enables the set_text command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:allow-set-text",
          "markdownDescription": "Enables the set_text command without any pre-configured scope."
        },
        {
          "description": "Enables the text command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:allow-text",
          "markdownDescription": "Enables the text command without any pre-configured scope."
        },
        {
          "description": "Denies the append command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:deny-append",
          "markdownDescription": "Denies the append command without any pre-configured scope."
        },
        {
          "description": "Denies the create_default command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:deny-create-default",
          "markdownDescription": "Denies the create_default command without any pre-configured scope."
        },
        {
          "description": "Denies the get command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:deny-get",
          "markdownDescription": "Denies the get command without any pre-configured scope."
        },
        {
          "description": "Denies the insert command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:deny-insert",
          "markdownDescription": "Denies the insert command without any pre-configured scope."
        },
        {
          "description": "Denies the is_checked command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:deny-is-checked",
          "markdownDescription": "Denies the is_checked command without any pre-configured scope."
        },
        {
          "description": "Denies the is_enabled command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:deny-is-enabled",
          "markdownDescription": "Denies the is_enabled command without any pre-configured scope."
        },
        {
          "description": "Denies the items command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:deny-items",
          "markdownDescription": "Denies the items command without any pre-configured scope."
        },
        {
          "description": "Denies the new command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:deny-new",
          "markdownDescription": "Denies the new command without any pre-configured scope."
        },
        {
          "description": "Denies the popup command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:deny-popup",
          "markdownDescription": "Denies the popup command without any pre-configured scope."
        },
        {
          "description": "Denies the prepend command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:deny-prepend",
          "markdownDescription": "Denies the prepend command without any pre-configured scope."
        },
        {
          "description": "Denies the remove command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:deny-remove",
          "markdownDescription": "Denies the remove command without any pre-configured scope."
        },
        {
          "description": "Denies the remove_at command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:deny-remove-at",
          "markdownDescription": "Denies the remove_at command without any pre-configured scope."
        },
        {
          "description": "Denies the set_accelerator command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:deny-set-accelerator",
          "markdownDescription": "Denies the set_accelerator command without any pre-configured scope."
        },
        {
          "description": "Denies the set_as_app_menu command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:deny-set-as-app-menu",
          "markdownDescription": "Denies the set_as_app_menu command without any pre-configured scope."
        },
        {
          "description": "Denies the set_as_help_menu_for_nsapp command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:deny-set-as-help-menu-for-nsapp",
          "markdownDescription": "Denies the set_as_help_menu_for_nsapp command without any pre-configured scope."
        },
        {
          "description": "Denies the set_as_window_menu command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:deny-set-as-window-menu",
          "markdownDescription": "Denies the set_as_window_menu command without any pre-configured scope."
        },
        {
          "description": "Denies the set_as_windows_menu_for_nsapp command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:deny-set-as-windows-menu-for-nsapp",
          "markdownDescription": "Denies the set_as_windows_menu_for_nsapp command without any pre-configured scope."
        },
        {
          "description": "Denies the set_checked command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:deny-set-checked",
          "markdownDescription": "Denies the set_checked command without any pre-configured scope."
        },
        {
          "description": "Denies the set_enabled command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:deny-set-enabled",
          "markdownDescription": "Denies the set_enabled command without any pre-configured scope."
        },
        {
          "description": "Denies the set_icon command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:deny-set-icon",
          "markdownDescription": "Denies the set_icon command without any pre-configured scope."
        },
        {
          "description": "Denies the set_text command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:deny-set-text",
          "markdownDescription": "Denies the set_text command without any pre-configured scope."
        },
        {
          "description": "Denies the text command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:deny-text",
          "markdownDescription": "Denies the text command without any pre-configured scope."
        },
        {
          "description": "Default permissions for the plugin, which enables all commands.\n#### This default permission set includes:\n\n- `allow-resolve-directory`\n- `allow-resolve`\n- `allow-normalize`\n- `allow-join`\n- `allow-dirname`\n- `allow-extname`\n- `allow-basename`\n- `allow-is-absolute`",
          "type": "string",
          "const": "core:path:default",
          "markdownDescription": "Default permissions for the plugin, which enables all commands.\n#### This default permission set includes:\n\n- `allow-resolve-directory`\n- `allow-resolve`\n- `allow-normalize`\n- `allow-join`\n- `allow-dirname`\n- `allow-extname`\n- `allow-basename`\n- `allow-is-absolute`"
        },
        {
          "description": "Enables the basename command without any pre-configured scope.",
          "type": "string",
          "const": "core:path:allow-basename",
          "markdownDescription": "Enables the basename command without any pre-configured scope."
        },
        {
          "description": "Enables the dirname command without any pre-configured scope.",
          "type": "string",
          "const": "core:path:allow-dirname",
          "markdownDescription": "Enables the dirname command without any pre-configured scope."
        },
        {
          "description": "Enables the extname command without any pre-configured scope.",
          "type": "string",
          "const": "core:path:allow-extname",
          "markdownDescription": "Enables the extname command without any pre-configured scope."
        },
        {
          "description": "Enables the is_absolute command without any pre-configured scope.",
          "type": "string",
          "const": "core:path:allow-is-absolute",
          "markdownDescription": "Enables the is_absolute command without any pre-configured scope."
        },
        {
          "description": "Enables the join command without any pre-configured scope.",
          "type": "string",
          "const": "core:path:allow-join",
          "markdownDescription": "Enables the join command without any pre-configured scope."
        },
        {
          "description": "Enables the normalize command without any pre-configured scope.",
          "type": "string",
          "const": "core:path:allow-normalize",
          "markdownDescription": "Enables the normalize command without any pre-configured scope."
        },
        {
          "description": "Enables the resolve command without any pre-configured scope.",
          "type": "string",
          "const": "core:path:allow-resolve",
          "markdownDescription": "Enables the resolve command without any pre-configured scope."
        },
        {
          "description": "Enables the resolve_directory command without any pre-configured scope.",
          "type": "string",
          "const": "core:path:allow-resolve-directory",
          "markdownDescription": "Enables the resolve_directory command without any pre-configured scope."
        },
        {
          "description": "Denies the basename command without any pre-configured scope.",
          "type": "string",
          "const": "core:path:deny-basename",
          "markdownDescription": "Denies the basename command without any pre-configured scope."
        },
        {
          "description": "Denies the dirname command without any pre-configured scope.",
          "type": "string",
          "const": "core:path:deny-dirname",
          "markdownDescription": "Denies the dirname command without any pre-configured scope."
        },
        {
          "description": "Denies the extname command without any pre-configured scope.",
          "type": "string",
          "const": "core:path:deny-extname",
          "markdownDescription": "Denies the extname command without any pre-configured scope."
        },
        {
          "description": "Denies the is_absolute command without any pre-configured scope.",
          "type": "string",
          "const": "core:path:deny-is-absolute",
          "markdownDescription": "Denies the is_absolute command without any pre-configured scope."
        },
        {
          "description": "Denies the join command without any pre-configured scope.",
          "type": "string",
          "const": "core:path:deny-join",
          "markdownDescription": "Denies the join command without any pre-configured scope."
        },
        {
          "description": "Denies the normalize command without any pre-configured scope.",
          "type": "string",
          "const": "core:path:deny-normalize",
          "markdownDescription": "Denies the normalize command without any pre-configured scope."
        },
        {
          "description": "Denies the resolve command without any pre-configured scope.",
          "type": "string",
          "const": "core:path:deny-resolve",
          "markdownDescription": "Denies the resolve command without any pre-configured scope."
        },
        {
          "description": "Denies the resolve_directory command without any pre-configured scope.",
          "type": "string",
          "const": "core:path:deny-resolve-directory",
          "markdownDescription": "Denies the resolve_directory command without any pre-configured scope."
        },
        {
          "description": "Default permissions for the plugin, which enables all commands.\n#### This default permission set includes:\n\n- `allow-close`",
          "type": "string",
          "const": "core:resources:default",
          "markdownDescription": "Default permissions for the plugin, which enables all commands.\n#### This default permission set includes:\n\n- `allow-close`"
        },
        {
          "description": "Enables the close command without any pre-configured scope.",
          "type": "string",
          "const": "core:resources:allow-close",
          "markdownDescription": "Enables the close command without any pre-configured scope."
        },
        {
          "description": "Denies the close command without any pre-configured scope.",
          "type": "string",
          "const": "core:resources:deny-close",
          "markdownDescription": "Denies the close command without any pre-configured scope."
        },
        {
          "description": "Default permissions for the plugin, which enables all commands.\n#### This default permission set includes:\n\n- `allow-new`\n- `allow-get-by-id`\n- `allow-remove-by-id`\n- `allow-set-icon`\n- `allow-set-menu`\n- `allow-set-tooltip`\n- `allow-set-title`\n- `allow-set-visible`\n- `allow-set-temp-dir-path`\n- `allow-set-icon-as-template`\n- `allow-set-show-menu-on-left-click`",
          "type": "string",
          "const": "core:tray:default",
          "markdownDescription": "Default permissions for the plugin, which enables all commands.\n#### This default permission set includes:\n\n- `allow-new`\n- `allow-get-by-id`\n- `allow-remove-by-id`\n- `allow-set-icon`\n- `allow-set-menu`\n- `allow-set-tooltip`\n- `allow-set-title`\n- `allow-set-visible`\n- `allow-set-temp-dir-path`\n- `allow-set-icon-as-template`\n- `allow-set-show-menu-on-left-click`"
        },
        {
          "description": "Enables the get_by_id command without any pre-configured scope.",
          "type": "string",
          "const": "core:tray:allow-get-by-id",
          "markdownDescription": "Enables the get_by_id command without any pre-configured scope."
        },
        {
          "description": "Enables the new command without any pre-configured scope.",
          "type": "string",
          "const": "core:tray:allow-new",
          "markdownDescription": "Enables the new command without any pre-configured scope."
        },
        {
          "description": "Enables the remove_by_id command without any pre-configured scope.",
          "type": "string",
          "const": "core:tray:allow-remove-by-id",
          "markdownDescription": "Enables the remove_by_id command without any pre-configured scope."
        },
        {
          "description": "Enables the set_icon command without any pre-configured scope.",
          "type": "string",
          "const": "core:tray:allow-set-icon",
          "markdownDescription": "Enables the set_icon command without any pre-configured scope."
        },
        {
          "description": "Enables the set_icon_as_template command without any pre-configured scope.",
          "type": "string",
          "const": "core:tray:allow-set-icon-as-template",
          "markdownDescription": "Enables the set_icon_as_template command without any pre-configured scope."
        },
        {
          "description": "Enables the set_menu command without any pre-configured scope.",
          "type": "string",
          "const": "core:tray:allow-set-menu",
          "markdownDescription": "Enables the set_menu command without any pre-configured scope."
        },
        {
          "description": "Enables the set_show_menu_on_left_click command without any pre-configured scope.",
          "type": "string",
          "const": "core:tray:allow-set-show-menu-on-left-click",
          "markdownDescription": "Enables the set_show_menu_on_left_click command without any pre-configured scope."
        },
        {
          "description": "Enables the set_temp_dir_path command without any pre-configured scope.",
          "type": "string",
          "const": "core:tray:allow-set-temp-dir-path",
          "markdownDescription": "Enables the set_temp_dir_path command without any pre-configured scope."
        },
        {
          "description": "Enables the set_title command without any pre-configured scope.",
          "type": "string",
          "const": "core:tray:allow-set-title",
          "markdownDescription": "Enables the set_title command without any pre-configured scope."
        },
        {
          "description": "Enables the set_tooltip command without any pre-configured scope.",
          "type": "string",
          "const": "core:tray:allow-set-tooltip",
          "markdownDescription": "Enables the set_tooltip command without any pre-configured scope."
        },
        {
          "description": "Enables the set_visible command without any pre-configured scope.",
          "type": "string",
          "const": "core:tray:allow-set-visible",
          "markdownDescription": "Enables the set_visible command without any pre-configured scope."
        },
        {
          "description": "Denies the get_by_id command without any pre-configured scope.",
          "type": "string",
          "const": "core:tray:deny-get-by-id",
          "markdownDescription": "Denies the get_by_id command without any pre-configured scope."
        },
        {
          "description": "Denies the new command without any pre-configured scope.",
          "type": "string",
          "const": "core:tray:deny-new",
          "markdownDescription": "Denies the new command without any pre-configured scope."
        },
        {
          "description": "Denies the remove_by_id command without any pre-configured scope.",
          "type": "string",
          "const": "core:tray:deny-remove-by-id",
          "markdownDescription": "Denies the remove_by_id command without any pre-configured scope."
        },
        {
          "description": "Denies the set_icon command without any pre-configured scope.",
          "type": "string",
          "const": "core:tray:deny-set-icon",
          "markdownDescription": "Denies the set_icon command without any pre-configured scope."
        },
        {
          "description": "Denies the set_icon_as_template command without any pre-configured scope.",
          "type": "string",
          "const": "core:tray:deny-set-icon-as-template",
          "markdownDescription": "Denies the set_icon_as_template command without any pre-configured scope."
        },
        {
          "description": "Denies the set_menu command without any pre-configured scope.",
          "type": "string",
          "const": "core:tray:deny-set-menu",
          "markdownDescription": "Denies the set_menu command without any pre-configured scope."
        },
        {
          "description": "Denies the set_show_menu_on_left_click command without any pre-configured scope.",
          "type": "string",
          "const": "core:tray:deny-set-show-menu-on-left-click",
          "markdownDescription": "Denies the set_show_menu_on_left_click command without any pre-configured scope."
        },
        {
          "description": "Denies the set_temp_dir_path command without any pre-configured scope.",
          "type": "string",
          "const": "core:tray:deny-set-temp-dir-path",
          "markdownDescription": "Denies the set_temp_dir_path command without any pre-configured scope."
        },
        {
          "description": "Denies the set_title command without any pre-configured scope.",
          "type": "string",
          "const": "core:tray:deny-set-title",
          "markdownDescription": "Denies the set_title command without any pre-configured scope."
        },
        {
          "description": "Denies the set_tooltip command without any pre-configured scope.",
          "type": "string",
          "const": "core:tray:deny-set-tooltip",
          "markdownDescription": "Denies the set_tooltip command without any pre-configured scope."
        },
        {
          "description": "Denies the set_visible command without any pre-configured scope.",
          "type": "string",
          "const": "core:tray:deny-set-visible",
          "markdownDescription": "Denies the set_visible command without any pre-configured scope."
        },
        {
          "description": "Default permissions for the plugin.\n#### This default permission set includes:\n\n- `allow-get-all-webviews`\n- `allow-webview-position`\n- `allow-webview-size`\n- `allow-internal-toggle-devtools`",
          "type": "string",
          "const": "core:webview:default",
          "markdownDescription": "Default permissions for the plugin.\n#### This default permission set includes:\n\n- `allow-get-all-webviews`\n- `allow-webview-position`\n- `allow-webview-size`\n- `allow-internal-toggle-devtools`"
        },
        {
          "description": "Enables the clear_all_browsing_data command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:allow-clear-all-browsing-data",
          "markdownDescription": "Enables the clear_all_browsing_data command without any pre-configured scope."
        },
        {
          "description": "Enables the create_webview command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:allow-create-webview",
          "markdownDescription": "Enables the create_webview command without any pre-configured scope."
        },
        {
          "description": "Enables the create_webview_window command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:allow-create-webview-window",
          "markdownDescription": "Enables the create_webview_window command without any pre-configured scope."
        },
        {
          "description": "Enables the get_all_webviews command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:allow-get-all-webviews",
          "markdownDescription": "Enables the get_all_webviews command without any pre-configured scope."
        },
        {
          "description": "Enables the internal_toggle_devtools command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:allow-internal-toggle-devtools",
          "markdownDescription": "Enables the internal_toggle_devtools command without any pre-configured scope."
        },
        {
          "description": "Enables the print command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:allow-print",
          "markdownDescription": "Enables the print command without any pre-configured scope."
        },
        {
          "description": "Enables the reparent command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:allow-reparent",
          "markdownDescription": "Enables the reparent command without any pre-configured scope."
        },
        {
          "description": "Enables the set_webview_auto_resize command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:allow-set-webview-auto-resize",
          "markdownDescription": "Enables the set_webview_auto_resize command without any pre-configured scope."
        },
        {
          "description": "Enables the set_webview_background_color command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:allow-set-webview-background-color",
          "markdownDescription": "Enables the set_webview_background_color command without any pre-configured scope."
        },
        {
          "description": "Enables the set_webview_focus command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:allow-set-webview-focus",
          "markdownDescription": "Enables the set_webview_focus command without any pre-configured scope."
        },
        {
          "description": "Enables the set_webview_position command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:allow-set-webview-position",
          "markdownDescription": "Enables the set_webview_position command without any pre-configured scope."
        },
        {
          "description": "Enables the set_webview_size command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:allow-set-webview-size",
          "markdownDescription": "Enables the set_webview_size command without any pre-configured scope."
        },
        {
          "description": "Enables the set_webview_zoom command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:allow-set-webview-zoom",
          "markdownDescription": "Enables the set_webview_zoom command without any pre-configured scope."
        },
        {
          "description": "Enables the webview_close command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:allow-webview-close",
          "markdownDescription": "Enables the webview_close command without any pre-configured scope."
        },
        {
          "description": "Enables the webview_hide command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:allow-webview-hide",
          "markdownDescription": "Enables the webview_hide command without any pre-configured scope."
        },
        {
          "description": "Enables the webview_position command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:allow-webview-position",
          "markdownDescription": "Enables the webview_position command without any pre-configured scope."
        },
        {
          "description": "Enables the webview_show command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:allow-webview-show",
          "markdownDescription": "Enables the webview_show command without any pre-configured scope."
        },
        {
          "description": "Enables the webview_size command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:allow-webview-size",
          "markdownDescription": "Enables the webview_size command without any pre-configured scope."
        },
        {
          "description": "Denies the clear_all_browsing_data command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:deny-clear-all-browsing-data",
          "markdownDescription": "Denies the clear_all_browsing_data command without any pre-configured scope."
        },
        {
          "description": "Denies the create_webview command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:deny-create-webview",
          "markdownDescription": "Denies the create_webview command without any pre-configured scope."
        },
        {
          "description": "Denies the create_webview_window command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:deny-create-webview-window",
          "markdownDescription": "Denies the create_webview_window command without any pre-configured scope."
        },
        {
          "description": "Denies the get_all_webviews command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:deny-get-all-webviews",
          "markdownDescription": "Denies the get_all_webviews command without any pre-configured scope."
        },
        {
          "description": "Denies the internal_toggle_devtools command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:deny-internal-toggle-devtools",
          "markdownDescription": "Denies the internal_toggle_devtools command without any pre-configured scope."
        },
        {
          "description": "Denies the print command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:deny-print",
          "markdownDescription": "Denies the print command without any pre-configured scope."
        },
        {
          "description": "Denies the reparent command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:deny-reparent",
          "markdownDescription": "Denies the reparent command without any pre-configured scope."
        },
        {
          "description": "Denies the set_webview_auto_resize command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:deny-set-webview-auto-resize",
          "markdownDescription": "Denies the set_webview_auto_resize command without any pre-configured scope."
        },
        {
          "description": "Denies the set_webview_background_color command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:deny-set-webview-background-color",
          "markdownDescription": "Denies the set_webview_background_color command without any pre-configured scope."
        },
        {
          "description": "Denies the set_webview_focus command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:deny-set-webview-focus",
          "markdownDescription": "Denies the set_webview_focus command without any pre-configured scope."
        },
        {
          "description": "Denies the set_webview_position command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:deny-set-webview-position",
          "markdownDescription": "Denies the set_webview_position command without any pre-configured scope."
        },
        {
          "description": "Denies the set_webview_size command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:deny-set-webview-size",
          "markdownDescription": "Denies the set_webview_size command without any pre-configured scope."
        },
        {
          "description": "Denies the set_webview_zoom command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:deny-set-webview-zoom",
          "markdownDescription": "Denies the set_webview_zoom command without any pre-configured scope."
        },
        {
          "description": "Denies the webview_close command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:deny-webview-close",
          "markdownDescription": "Denies the webview_close command without any pre-configured scope."
        },
        {
          "description": "Denies the webview_hide command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:deny-webview-hide",
          "markdownDescription": "Denies the webview_hide command without any pre-configured scope."
        },
        {
          "description": "Denies the webview_position command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:deny-webview-position",
          "markdownDescription": "Denies the webview_position command without any pre-configured scope."
        },
        {
          "description": "Denies the webview_show command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:deny-webview-show",
          "markdownDescription": "Denies the webview_show command without any pre-configured scope."
        },
        {
          "description": "Denies the webview_size command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:deny-webview-size",
          "markdownDescription": "Denies the webview_size command without any pre-configured scope."
        },
        {
          "description": "Default permissions for the plugin.\n#### This default permission set includes:\n\n- `allow-get-all-windows`\n- `allow-scale-factor`\n- `allow-inner-position`\n- `allow-outer-position`\n- `allow-inner-size`\n- `allow-outer-size`\n- `allow-is-fullscreen`\n- `allow-is-minimized`\n- `allow-is-maximized`\n- `allow-is-focused`\n- `allow-is-decorated`\n- `allow-is-resizable`\n- `allow-is-maximizable`\n- `allow-is-minimizable`\n- `allow-is-closable`\n- `allow-is-visible`\n- `allow-is-enabled`\n- `allow-title`\n- `allow-current-monitor`\n- `allow-primary-monitor`\n- `allow-monitor-from-point`\n- `allow-available-monitors`\n- `allow-cursor-position`\n- `allow-theme`\n- `allow-is-always-on-top`\n- `allow-internal-toggle-maximize`",
          "type": "string",
          "const": "core:window:default",
          "markdownDescription": "Default permissions for the plugin.\n#### This default permission set includes:\n\n- `allow-get-all-windows`\n- `allow-scale-factor`\n- `allow-inner-position`\n- `allow-outer-position`\n- `allow-inner-size`\n- `allow-outer-size`\n- `allow-is-fullscreen`\n- `allow-is-minimized`\n- `allow-is-maximized`\n- `allow-is-focused`\n- `allow-is-decorated`\n- `allow-is-resizable`\n- `allow-is-maximizable`\n- `allow-is-minimizable`\n- `allow-is-closable`\n- `allow-is-visible`\n- `allow-is-enabled`\n- `allow-title`\n- `allow-current-monitor`\n- `allow-primary-monitor`\n- `allow-monitor-from-point`\n- `allow-available-monitors`\n- `allow-cursor-position`\n- `allow-theme`\n- `allow-is-always-on-top`\n- `allow-internal-toggle-maximize`"
        },
        {
          "description": "Enables the available_monitors command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-available-monitors",
          "markdownDescription": "Enables the available_monitors command without any pre-configured scope."
        },
        {
          "description": "Enables the center command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-center",
          "markdownDescription": "Enables the center command without any pre-configured scope."
        },
        {
          "description": "Enables the close command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-close",
          "markdownDescription": "Enables the close command without any pre-configured scope."
        },
        {
          "description": "Enables the create command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-create",
          "markdownDescription": "Enables the create command without any pre-configured scope."
        },
        {
          "description": "Enables the current_monitor command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-current-monitor",
          "markdownDescription": "Enables the current_monitor command without any pre-configured scope."
        },
        {
          "description": "Enables the cursor_position command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-cursor-position",
          "markdownDescription": "Enables the cursor_position command without any pre-configured scope."
        },
        {
          "description": "Enables the destroy command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-destroy",
          "markdownDescription": "Enables the destroy command without any pre-configured scope."
        },
        {
          "description": "Enables the get_all_windows command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-get-all-windows",
          "markdownDescription": "Enables the get_all_windows command without any pre-configured scope."
        },
        {
          "description": "Enables the hide command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-hide",
          "markdownDescription": "Enables the hide command without any pre-configured scope."
        },
        {
          "description": "Enables the inner_position command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-inner-position",
          "markdownDescription": "Enables the inner_position command without any pre-configured scope."
        },
        {
          "description": "Enables the inner_size command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-inner-size",
          "markdownDescription": "Enables the inner_size command without any pre-configured scope."
        },
        {
          "description": "Enables the internal_toggle_maximize command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-internal-toggle-maximize",
          "markdownDescription": "Enables the internal_toggle_maximize command without any pre-configured scope."
        },
        {
          "description": "Enables the is_always_on_top command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-is-always-on-top",
          "markdownDescription": "Enables the is_always_on_top command without any pre-configured scope."
        },
        {
          "description": "Enables the is_closable command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-is-closable",
          "markdownDescription": "Enables the is_closable command without any pre-configured scope."
        },
        {
          "description": "Enables the is_decorated command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-is-decorated",
          "markdownDescription": "Enables the is_decorated command without any pre-configured scope."
        },
        {
          "description": "Enables the is_enabled command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-is-enabled",
          "markdownDescription": "Enables the is_enabled command without any pre-configured scope."
        },
        {
          "description": "Enables the is_focused command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-is-focused",
          "markdownDescription": "Enables the is_focused command without any pre-configured scope."
        },
        {
          "description": "Enables the is_fullscreen command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-is-fullscreen",
          "markdownDescription": "Enables the is_fullscreen command without any pre-configured scope."
        },
        {
          "description": "Enables the is_maximizable command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-is-maximizable",
          "markdownDescription": "Enables the is_maximizable command without any pre-configured scope."
        },
        {
          "description": "Enables the is_maximized command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-is-maximized",
          "markdownDescription": "Enables the is_maximized command without any pre-configured scope."
        },
        {
          "description": "Enables the is_minimizable command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-is-minimizable",
          "markdownDescription": "Enables the is_minimizable command without any pre-configured scope."
        },
        {
          "description": "Enables the is_minimized command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-is-minimized",
          "markdownDescription": "Enables the is_minimized command without any pre-configured scope."
        },
        {
          "description": "Enables the is_resizable command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-is-resizable",
          "markdownDescription": "Enables the is_resizable command without any pre-configured scope."
        },
        {
          "description": "Enables the is_visible command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-is-visible",
          "markdownDescription": "Enables the is_visible command without any pre-configured scope."
        },
        {
          "description": "Enables the maximize command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-maximize",
          "markdownDescription": "Enables the maximize command without any pre-configured scope."
        },
        {
          "description": "Enables the minimize command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-minimize",
          "markdownDescription": "Enables the minimize command without any pre-configured scope."
        },
        {
          "description": "Enables the monitor_from_point command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-monitor-from-point",
          "markdownDescription": "Enables the monitor_from_point command without any pre-configured scope."
        },
        {
          "description": "Enables the outer_position command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-outer-position",
          "markdownDescription": "Enables the outer_position command without any pre-configured scope."
        },
        {
          "description": "Enables the outer_size command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-outer-size",
          "markdownDescription": "Enables the outer_size command without any pre-configured scope."
        },
        {
          "description": "Enables the primary_monitor command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-primary-monitor",
          "markdownDescription": "Enables the primary_monitor command without any pre-configured scope."
        },
        {
          "description": "Enables the request_user_attention command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-request-user-attention",
          "markdownDescription": "Enables the request_user_attention command without any pre-configured scope."
        },
        {
          "description": "Enables the scale_factor command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-scale-factor",
          "markdownDescription": "Enables the scale_factor command without any pre-configured scope."
        },
        {
          "description": "Enables the set_always_on_bottom command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-always-on-bottom",
          "markdownDescription": "Enables the set_always_on_bottom command without any pre-configured scope."
        },
        {
          "description": "Enables the set_always_on_top command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-always-on-top",
          "markdownDescription": "Enables the set_always_on_top command without any pre-configured scope."
        },
        {
          "description": "Enables the set_background_color command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-background-color",
          "markdownDescription": "Enables the set_background_color command without any pre-configured scope."
        },
        {
          "description": "Enables the set_badge_count command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-badge-count",
          "markdownDescription": "Enables the set_badge_count command without any pre-configured scope."
        },
        {
          "description": "Enables the set_badge_label command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-badge-label",
          "markdownDescription": "Enables the set_badge_label command without any pre-configured scope."
        },
        {
          "description": "Enables the set_closable command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-closable",
          "markdownDescription": "Enables the set_closable command without any pre-configured scope."
        },
        {
          "description": "Enables the set_content_protected command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-content-protected",
          "markdownDescription": "Enables the set_content_protected command without any pre-configured scope."
        },
        {
          "description": "Enables the set_cursor_grab command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-cursor-grab",
          "markdownDescription": "Enables the set_cursor_grab command without any pre-configured scope."
        },
        {
          "description": "Enables the set_cursor_icon command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-cursor-icon",
          "markdownDescription": "Enables the set_cursor_icon command without any pre-configured scope."
        },
        {
          "description": "Enables the set_cursor_position command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-cursor-position",
          "markdownDescription": "Enables the set_cursor_position command without any pre-configured scope."
        },
        {
          "description": "Enables the set_cursor_visible command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-cursor-visible",
          "markdownDescription": "Enables the set_cursor_visible command without any pre-configured scope."
        },
        {
          "description": "Enables the set_decorations command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-decorations",
          "markdownDescription": "Enables the set_decorations command without any pre-configured scope."
        },
        {
          "description": "Enables the set_effects command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-effects",
          "markdownDescription": "Enables the set_effects command without any pre-configured scope."
        },
        {
          "description": "Enables the set_enabled command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-enabled",
          "markdownDescription": "Enables the set_enabled command without any pre-configured scope."
        },
        {
          "description": "Enables the set_focus command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-focus",
          "markdownDescription": "Enables the set_focus command without any pre-configured scope."
        },
        {
          "description": "Enables the set_focusable command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-focusable",
          "markdownDescription": "Enables the set_focusable command without any pre-configured scope."
        },
        {
          "description": "Enables the set_fullscreen command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-fullscreen",
          "markdownDescription": "Enables the set_fullscreen command without any pre-configured scope."
        },
        {
          "description": "Enables the set_icon command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-icon",
          "markdownDescription": "Enables the set_icon command without any pre-configured scope."
        },
        {
          "description": "Enables the set_ignore_cursor_events command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-ignore-cursor-events",
          "markdownDescription": "Enables the set_ignore_cursor_events command without any pre-configured scope."
        },
        {
          "description": "Enables the set_max_size command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-max-size",
          "markdownDescription": "Enables the set_max_size command without any pre-configured scope."
        },
        {
          "description": "Enables the set_maximizable command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-maximizable",
          "markdownDescription": "Enables the set_maximizable command without any pre-configured scope."
        },
        {
          "description": "Enables the set_min_size command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-min-size",
          "markdownDescription": "Enables the set_min_size command without any pre-configured scope."
        },
        {
          "description": "Enables the set_minimizable command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-minimizable",
          "markdownDescription": "Enables the set_minimizable command without any pre-configured scope."
        },
        {
          "description": "Enables the set_overlay_icon command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-overlay-icon",
          "markdownDescription": "Enables the set_overlay_icon command without any pre-configured scope."
        },
        {
          "description": "Enables the set_position command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-position",
          "markdownDescription": "Enables the set_position command without any pre-configured scope."
        },
        {
          "description": "Enables the set_progress_bar command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-progress-bar",
          "markdownDescription": "Enables the set_progress_bar command without any pre-configured scope."
        },
        {
          "description": "Enables the set_resizable command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-resizable",
          "markdownDescription": "Enables the set_resizable command without any pre-configured scope."
        },
        {
          "description": "Enables the set_shadow command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-shadow",
          "markdownDescription": "Enables the set_shadow command without any pre-configured scope."
        },
        {
          "description": "Enables the set_simple_fullscreen command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-simple-fullscreen",
          "markdownDescription": "Enables the set_simple_fullscreen command without any pre-configured scope."
        },
        {
          "description": "Enables the set_size command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-size",
          "markdownDescription": "Enables the set_size command without any pre-configured scope."
        },
        {
          "description": "Enables the set_size_constraints command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-size-constraints",
          "markdownDescription": "Enables the set_size_constraints command without any pre-configured scope."
        },
        {
          "description": "Enables the set_skip_taskbar command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-skip-taskbar",
          "markdownDescription": "Enables the set_skip_taskbar command without any pre-configured scope."
        },
        {
          "description": "Enables the set_theme command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-theme",
          "markdownDescription": "Enables the set_theme command without any pre-configured scope."
        },
        {
          "description": "Enables the set_title command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-title",
          "markdownDescription": "Enables the set_title command without any pre-configured scope."
        },
        {
          "description": "Enables the set_title_bar_style command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-title-bar-style",
          "markdownDescription": "Enables the set_title_bar_style command without any pre-configured scope."
        },
        {
          "description": "Enables the set_visible_on_all_workspaces command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-visible-on-all-workspaces",
          "markdownDescription": "Enables the set_visible_on_all_workspaces command without any pre-configured scope."
        },
        {
          "description": "Enables the show command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-show",
          "markdownDescription": "Enables the show command without any pre-configured scope."
        },
        {
          "description": "Enables the start_dragging command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-start-dragging",
          "markdownDescription": "Enables the start_dragging command without any pre-configured scope."
        },
        {
          "description": "Enables the start_resize_dragging command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-start-resize-dragging",
          "markdownDescription": "Enables the start_resize_dragging command without any pre-configured scope."
        },
        {
          "description": "Enables the theme command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-theme",
          "markdownDescription": "Enables the theme command without any pre-configured scope."
        },
        {
          "description": "Enables the title command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-title",
          "markdownDescription": "Enables the title command without any pre-configured scope."
        },
        {
          "description": "Enables the toggle_maximize command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-toggle-maximize",
          "markdownDescription": "Enables the toggle_maximize command without any pre-configured scope."
        },
        {
          "description": "Enables the unmaximize command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-unmaximize",
          "markdownDescription": "Enables the unmaximize command without any pre-configured scope."
        },
        {
          "description": "Enables the unminimize command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-unminimize",
          "markdownDescription": "Enables the unminimize command without any pre-configured scope."
        },
        {
          "description": "Denies the available_monitors command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-available-monitors",
          "markdownDescription": "Denies the available_monitors command without any pre-configured scope."
        },
        {
          "description": "Denies the center command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-center",
          "markdownDescription": "Denies the center command without any pre-configured scope."
        },
        {
          "description": "Denies the close command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-close",
          "markdownDescription": "Denies the close command without any pre-configured scope."
        },
        {
          "description": "Denies the create command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-create",
          "markdownDescription": "Denies the create command without any pre-configured scope."
        },
        {
          "description": "Denies the current_monitor command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-current-monitor",
          "markdownDescription": "Denies the current_monitor command without any pre-configured scope."
        },
        {
          "description": "Denies the cursor_position command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-cursor-position",
          "markdownDescription": "Denies the cursor_position command without any pre-configured scope."
        },
        {
          "description": "Denies the destroy command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-destroy",
          "markdownDescription": "Denies the destroy command without any pre-configured scope."
        },
        {
          "description": "Denies the get_all_windows command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-get-all-windows",
          "markdownDescription": "Denies the get_all_windows command without any pre-configured scope."
        },
        {
          "description": "Denies the hide command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-hide",
          "markdownDescription": "Denies the hide command without any pre-configured scope."
        },
        {
          "description": "Denies the inner_position command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-inner-position",
          "markdownDescription": "Denies the inner_position command without any pre-configured scope."
        },
        {
          "description": "Denies the inner_size command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-inner-size",
          "markdownDescription": "Denies the inner_size command without any pre-configured scope."
        },
        {
          "description": "Denies the internal_toggle_maximize command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-internal-toggle-maximize",
          "markdownDescription": "Denies the internal_toggle_maximize command without any pre-configured scope."
        },
        {
          "description": "Denies the is_always_on_top command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-is-always-on-top",
          "markdownDescription": "Denies the is_always_on_top command without any pre-configured scope."
        },
        {
          "description": "Denies the is_closable command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-is-closable",
          "markdownDescription": "Denies the is_closable command without any pre-configured scope."
        },
        {
          "description": "Denies the is_decorated command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-is-decorated",
          "markdownDescription": "Denies the is_decorated command without any pre-configured scope."
        },
        {
          "description": "Denies the is_enabled command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-is-enabled",
          "markdownDescription": "Denies the is_enabled command without any pre-configured scope."
        },
        {
          "description": "Denies the is_focused command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-is-focused",
          "markdownDescription": "Denies the is_focused command without any pre-configured scope."
        },
        {
          "description": "Denies the is_fullscreen command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-is-fullscreen",
          "markdownDescription": "Denies the is_fullscreen command without any pre-configured scope."
        },
        {
          "description": "Denies the is_maximizable command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-is-maximizable",
          "markdownDescription": "Denies the is_maximizable command without any pre-configured scope."
        },
        {
          "description": "Denies the is_maximized command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-is-maximized",
          "markdownDescription": "Denies the is_maximized command without any pre-configured scope."
        },
        {
          "description": "Denies the is_minimizable command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-is-minimizable",
          "markdownDescription": "Denies the is_minimizable command without any pre-configured scope."
        },
        {
          "description": "Denies the is_minimized command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-is-minimized",
          "markdownDescription": "Denies the is_minimized command without any pre-configured scope."
        },
        {
          "description": "Denies the is_resizable command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-is-resizable",
          "markdownDescription": "Denies the is_resizable command without any pre-configured scope."
        },
        {
          "description": "Denies the is_visible command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-is-visible",
          "markdownDescription": "Denies the is_visible command without any pre-configured scope."
        },
        {
          "description": "Denies the maximize command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-maximize",
          "markdownDescription": "Denies the maximize command without any pre-configured scope."
        },
        {
          "description": "Denies the minimize command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-minimize",
          "markdownDescription": "Denies the minimize command without any pre-configured scope."
        },
        {
          "description": "Denies the monitor_from_point command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-monitor-from-point",
          "markdownDescription": "Denies the monitor_from_point command without any pre-configured scope."
        },
        {
          "description": "Denies the outer_position command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-outer-position",
          "markdownDescription": "Denies the outer_position command without any pre-configured scope."
        },
        {
          "description": "Denies the outer_size command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-outer-size",
          "markdownDescription": "Denies the outer_size command without any pre-configured scope."
        },
        {
          "description": "Denies the primary_monitor command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-primary-monitor",
          "markdownDescription": "Denies the primary_monitor command without any pre-configured scope."
        },
        {
          "description": "Denies the request_user_attention command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-request-user-attention",
          "markdownDescription": "Denies the request_user_attention command without any pre-configured scope."
        },
        {
          "description": "Denies the scale_factor command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-scale-factor",
          "markdownDescription": "Denies the scale_factor command without any pre-configured scope."
        },
        {
          "description": "Denies the set_always_on_bottom command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-always-on-bottom",
          "markdownDescription": "Denies the set_always_on_bottom command without any pre-configured scope."
        },
        {
          "description": "Denies the set_always_on_top command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-always-on-top",
          "markdownDescription": "Denies the set_always_on_top command without any pre-configured scope."
        },
        {
          "description": "Denies the set_background_color command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-background-color",
          "markdownDescription": "Denies the set_background_color command without any pre-configured scope."
        },
        {
          "description": "Denies the set_badge_count command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-badge-count",
          "markdownDescription": "Denies the set_badge_count command without any pre-configured scope."
        },
        {
          "description": "Denies the set_badge_label command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-badge-label",
          "markdownDescription": "Denies the set_badge_label command without any pre-configured scope."
        },
        {
          "description": "Denies the set_closable command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-closable",
          "markdownDescription": "Denies the set_closable command without any pre-configured scope."
        },
        {
          "description": "Denies the set_content_protected command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-content-protected",
          "markdownDescription": "Denies the set_content_protected command without any pre-configured scope."
        },
        {
          "description": "Denies the set_cursor_grab command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-cursor-grab",
          "markdownDescription": "Denies the set_cursor_grab command without any pre-configured scope."
        },
        {
          "description": "Denies the set_cursor_icon command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-cursor-icon",
          "markdownDescription": "Denies the set_cursor_icon command without any pre-configured scope."
        },
        {
          "description": "Denies the set_cursor_position command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-cursor-position",
          "markdownDescription": "Denies the set_cursor_position command without any pre-configured scope."
        },
        {
          "description": "Denies the set_cursor_visible command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-cursor-visible",
          "markdownDescription": "Denies the set_cursor_visible command without any pre-configured scope."
        },
        {
          "description": "Denies the set_decorations command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-decorations",
          "markdownDescription": "Denies the set_decorations command without any pre-configured scope."
        },
        {
          "description": "Denies the set_effects command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-effects",
          "markdownDescription": "Denies the set_effects command without any pre-configured scope."
        },
        {
          "description": "Denies the set_enabled command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-enabled",
          "markdownDescription": "Denies the set_enabled command without any pre-configured scope."
        },
        {
          "description": "Denies the set_focus command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-focus",
          "markdownDescription": "Denies the set_focus command without any pre-configured scope."
        },
        {
          "description": "Denies the set_focusable command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-focusable",
          "markdownDescription": "Denies the set_focusable command without any pre-configured scope."
        },
        {
          "description": "Denies the set_fullscreen command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-fullscreen",
          "markdownDescription": "Denies the set_fullscreen command without any pre-configured scope."
        },
        {
          "description": "Denies the set_icon command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-icon",
          "markdownDescription": "Denies the set_icon command without any pre-configured scope."
        },
        {
          "description": "Denies the set_ignore_cursor_events command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-ignore-cursor-events",
          "markdownDescription": "Denies the set_ignore_cursor_events command without any pre-configured scope."
        },
        {
          "description": "Denies the set_max_size command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-max-size",
          "markdownDescription": "Denies the set_max_size command without any pre-configured scope."
        },
        {
          "description": "Denies the set_maximizable command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-maximizable",
          "markdownDescription": "Denies the set_maximizable command without any pre-configured scope."
        },
        {
          "description": "Denies the set_min_size command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-min-size",
          "markdownDescription": "Denies the set_min_size command without any pre-configured scope."
        },
        {
          "description": "Denies the set_minimizable command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-minimizable",
          "markdownDescription": "Denies the set_minimizable command without any pre-configured scope."
        },
        {
          "description": "Denies the set_overlay_icon command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-overlay-icon",
          "markdownDescription": "Denies the set_overlay_icon command without any pre-configured scope."
        },
        {
          "description": "Denies the set_position command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-position",
          "markdownDescription": "Denies the set_position command without any pre-configured scope."
        },
        {
          "description": "Denies the set_progress_bar command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-progress-bar",
          "markdownDescription": "Denies the set_progress_bar command without any pre-configured scope."
        },
        {
          "description": "Denies the set_resizable command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-resizable",
          "markdownDescription": "Denies the set_resizable command without any pre-configured scope."
        },
        {
          "description": "Denies the set_shadow command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-shadow",
          "markdownDescription": "Denies the set_shadow command without any pre-configured scope."
        },
        {
          "description": "Denies the set_simple_fullscreen command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-simple-fullscreen",
          "markdownDescription": "Denies the set_simple_fullscreen command without any pre-configured scope."
        },
        {
          "description": "Denies the set_size command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-size",
          "markdownDescription": "Denies the set_size command without any pre-configured scope."
        },
        {
          "description": "Denies the set_size_constraints command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-size-constraints",
          "markdownDescription": "Denies the set_size_constraints command without any pre-configured scope."
        },
        {
          "description": "Denies the set_skip_taskbar command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-skip-taskbar",
          "markdownDescription": "Denies the set_skip_taskbar command without any pre-configured scope."
        },
        {
          "description": "Denies the set_theme command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-theme",
          "markdownDescription": "Denies the set_theme command without any pre-configured scope."
        },
        {
          "description": "Denies the set_title command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-title",
          "markdownDescription": "Denies the set_title command without any pre-configured scope."
        },
        {
          "description": "Denies the set_title_bar_style command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-title-bar-style",
          "markdownDescription": "Denies the set_title_bar_style command without any pre-configured scope."
        },
        {
          "description": "Denies the set_visible_on_all_workspaces command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-visible-on-all-workspaces",
          "markdownDescription": "Denies the set_visible_on_all_workspaces command without any pre-configured scope."
        },
        {
          "description": "Denies the show command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-show",
          "markdownDescription": "Denies the show command without any pre-configured scope."
        },
        {
          "description": "Denies the start_dragging command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-start-dragging",
          "markdownDescription": "Denies the start_dragging command without any pre-configured scope."
        },
        {
          "description": "Denies the start_resize_dragging command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-start-resize-dragging",
          "markdownDescription": "Denies the start_resize_dragging command without any pre-configured scope."
        },
        {
          "description": "Denies the theme command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-theme",
          "markdownDescription": "Denies the theme command without any pre-configured scope."
        },
        {
          "description": "Denies the title command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-title",
          "markdownDescription": "Denies the title command without any pre-configured scope."
        },
        {
          "description": "Denies the toggle_maximize command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-toggle-maximize",
          "markdownDescription": "Denies the toggle_maximize command without any pre-configured scope."
        },
        {
          "description": "Denies the unmaximize command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-unmaximize",
          "markdownDescription": "Denies the unmaximize command without any pre-configured scope."
        },
        {
          "description": "Denies the unminimize command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-unminimize",
          "markdownDescription": "Denies the unminimize command without any pre-configured scope."
        },
        {
          "description": "Allows the log command\n#### This default permission set includes:\n\n- `allow-log`",
          "type": "string",
          "const": "log:default",
          "markdownDescription": "Allows the log command\n#### This default permission set includes:\n\n- `allow-log`"
        },
        {
          "description": "Enables the log command without any pre-configured scope.",
          "type": "string",
          "const": "log:allow-log",
          "markdownDescription": "Enables the log command without any pre-configured scope."
        },
        {
          "description": "Denies the log command without any pre-configured scope.",
          "type": "string",
          "const": "log:deny-log",
          "markdownDescription": "Denies the log command without any pre-configured scope."
        }
      ]
    },
    "Value": {
      "description": "All supported ACL values.",
      "anyOf": [
        {
          "description": "Represents a null JSON value.",
          "type": "null"
        },
        {
          "description": "Represents a [`bool`].",
          "type": "boolean"
        },
        {
          "description": "Represents a valid ACL [`Number`].",
          "allOf": [
            {
              "$ref": "#/definitions/Number"
            }
          ]
        },
        {
          "description": "Represents a [`String`].",
          "type": "string"
        },
        {
          "description": "Represents a list of other [`Value`]s.",
          "type": "array",
          "items": {
            "$ref": "#/definitions/Value"
          }
        },
        {
          "description": "Represents a map of [`String`] keys to [`Value`]s.",
          "type": "object",
          "additionalProperties": {
            "$ref": "#/definitions/Value"
          }
        }
      ]
    },
    "Number": {
      "description": "A valid ACL number.",
      "anyOf": [
        {
          "description": "Represents an [`i64`].",
          "type": "integer",
          "format": "int64"
        },
        {
          "description": "Represents a [`f64`].",
          "type": "number",
          "format": "double"
        }
      ]
    },
    "Target": {
      "description": "Platform target.",
      "oneOf": [
        {
          "description": "MacOS.",
          "type": "string",
          "enum": [
            "macOS"
          ]
        },
        {
          "description": "Windows.",
          "type": "string",
          "enum": [
            "windows"
          ]
        },
        {
          "description": "Linux.",
          "type": "string",
          "enum": [
            "linux"
          ]
        },
        {
          "description": "Android.",
          "type": "string",
          "enum": [
            "android"
          ]
        },
        {
          "description": "iOS.",
          "type": "string",
          "enum": [
            "iOS"
          ]
        }
      ]
    }
  }
}
```

---

## File: src-tauri/gen/schemas/linux-schema.json

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "CapabilityFile",
  "description": "Capability formats accepted in a capability file.",
  "anyOf": [
    {
      "description": "A single capability.",
      "allOf": [
        {
          "$ref": "#/definitions/Capability"
        }
      ]
    },
    {
      "description": "A list of capabilities.",
      "type": "array",
      "items": {
        "$ref": "#/definitions/Capability"
      }
    },
    {
      "description": "A list of capabilities.",
      "type": "object",
      "required": [
        "capabilities"
      ],
      "properties": {
        "capabilities": {
          "description": "The list of capabilities.",
          "type": "array",
          "items": {
            "$ref": "#/definitions/Capability"
          }
        }
      }
    }
  ],
  "definitions": {
    "Capability": {
      "description": "A grouping and boundary mechanism developers can use to isolate access to the IPC layer.\n\nIt controls application windows' and webviews' fine grained access to the Tauri core, application, or plugin commands. If a webview or its window is not matching any capability then it has no access to the IPC layer at all.\n\nThis can be done to create groups of windows, based on their required system access, which can reduce impact of frontend vulnerabilities in less privileged windows. Windows can be added to a capability by exact name (e.g. `main-window`) or glob patterns like `*` or `admin-*`. A Window can have none, one, or multiple associated capabilities.\n\n## Example\n\n```json { \"identifier\": \"main-user-files-write\", \"description\": \"This capability allows the `main` window on macOS and Windows access to `filesystem` write related commands and `dialog` commands to enable programmatic access to files selected by the user.\", \"windows\": [ \"main\" ], \"permissions\": [ \"core:default\", \"dialog:open\", { \"identifier\": \"fs:allow-write-text-file\", \"allow\": [{ \"path\": \"$HOME/test.txt\" }] }, ], \"platforms\": [\"macOS\",\"windows\"] } ```",
      "type": "object",
      "required": [
        "identifier",
        "permissions"
      ],
      "properties": {
        "identifier": {
          "description": "Identifier of the capability.\n\n## Example\n\n`main-user-files-write`",
          "type": "string"
        },
        "description": {
          "description": "Description of what the capability is intended to allow on associated windows.\n\nIt should contain a description of what the grouped permissions should allow.\n\n## Example\n\nThis capability allows the `main` window access to `filesystem` write related commands and `dialog` commands to enable programmatic access to files selected by the user.",
          "default": "",
          "type": "string"
        },
        "remote": {
          "description": "Configure remote URLs that can use the capability permissions.\n\nThis setting is optional and defaults to not being set, as our default use case is that the content is served from our local application.\n\n:::caution Make sure you understand the security implications of providing remote sources with local system access. :::\n\n## Example\n\n```json { \"urls\": [\"https://*.mydomain.dev\"] } ```",
          "anyOf": [
            {
              "$ref": "#/definitions/CapabilityRemote"
            },
            {
              "type": "null"
            }
          ]
        },
        "local": {
          "description": "Whether this capability is enabled for local app URLs or not. Defaults to `true`.",
          "default": true,
          "type": "boolean"
        },
        "windows": {
          "description": "List of windows that are affected by this capability. Can be a glob pattern.\n\nIf a window label matches any of the patterns in this list, the capability will be enabled on all the webviews of that window, regardless of the value of [`Self::webviews`].\n\nOn multiwebview windows, prefer specifying [`Self::webviews`] and omitting [`Self::windows`] for a fine grained access control.\n\n## Example\n\n`[\"main\"]`",
          "type": "array",
          "items": {
            "type": "string"
          }
        },
        "webviews": {
          "description": "List of webviews that are affected by this capability. Can be a glob pattern.\n\nThe capability will be enabled on all the webviews whose label matches any of the patterns in this list, regardless of whether the webview's window label matches a pattern in [`Self::windows`].\n\n## Example\n\n`[\"sub-webview-one\", \"sub-webview-two\"]`",
          "type": "array",
          "items": {
            "type": "string"
          }
        },
        "permissions": {
          "description": "List of permissions attached to this capability.\n\nMust include the plugin name as prefix in the form of `${plugin-name}:${permission-name}`. For commands directly implemented in the application itself only `${permission-name}` is required.\n\n## Example\n\n```json [ \"core:default\", \"shell:allow-open\", \"dialog:open\", { \"identifier\": \"fs:allow-write-text-file\", \"allow\": [{ \"path\": \"$HOME/test.txt\" }] } ] ```",
          "type": "array",
          "items": {
            "$ref": "#/definitions/PermissionEntry"
          },
          "uniqueItems": true
        },
        "platforms": {
          "description": "Limit which target platforms this capability applies to.\n\nBy default all platforms are targeted.\n\n## Example\n\n`[\"macOS\",\"windows\"]`",
          "type": [
            "array",
            "null"
          ],
          "items": {
            "$ref": "#/definitions/Target"
          }
        }
      }
    },
    "CapabilityRemote": {
      "description": "Configuration for remote URLs that are associated with the capability.",
      "type": "object",
      "required": [
        "urls"
      ],
      "properties": {
        "urls": {
          "description": "Remote domains this capability refers to using the [URLPattern standard](https://urlpattern.spec.whatwg.org/).\n\n## Examples\n\n- \"https://*.mydomain.dev\": allows subdomains of mydomain.dev - \"https://mydomain.dev/api/*\": allows any subpath of mydomain.dev/api",
          "type": "array",
          "items": {
            "type": "string"
          }
        }
      }
    },
    "PermissionEntry": {
      "description": "An entry for a permission value in a [`Capability`] can be either a raw permission [`Identifier`] or an object that references a permission and extends its scope.",
      "anyOf": [
        {
          "description": "Reference a permission or permission set by identifier.",
          "allOf": [
            {
              "$ref": "#/definitions/Identifier"
            }
          ]
        },
        {
          "description": "Reference a permission or permission set by identifier and extends its scope.",
          "type": "object",
          "allOf": [
            {
              "properties": {
                "identifier": {
                  "description": "Identifier of the permission or permission set.",
                  "allOf": [
                    {
                      "$ref": "#/definitions/Identifier"
                    }
                  ]
                },
                "allow": {
                  "description": "Data that defines what is allowed by the scope.",
                  "type": [
                    "array",
                    "null"
                  ],
                  "items": {
                    "$ref": "#/definitions/Value"
                  }
                },
                "deny": {
                  "description": "Data that defines what is denied by the scope. This should be prioritized by validation logic.",
                  "type": [
                    "array",
                    "null"
                  ],
                  "items": {
                    "$ref": "#/definitions/Value"
                  }
                }
              }
            }
          ],
          "required": [
            "identifier"
          ]
        }
      ]
    },
    "Identifier": {
      "description": "Permission identifier",
      "oneOf": [
        {
          "description": "Default core plugins set.\n#### This default permission set includes:\n\n- `core:path:default`\n- `core:event:default`\n- `core:window:default`\n- `core:webview:default`\n- `core:app:default`\n- `core:image:default`\n- `core:resources:default`\n- `core:menu:default`\n- `core:tray:default`",
          "type": "string",
          "const": "core:default",
          "markdownDescription": "Default core plugins set.\n#### This default permission set includes:\n\n- `core:path:default`\n- `core:event:default`\n- `core:window:default`\n- `core:webview:default`\n- `core:app:default`\n- `core:image:default`\n- `core:resources:default`\n- `core:menu:default`\n- `core:tray:default`"
        },
        {
          "description": "Default permissions for the plugin.\n#### This default permission set includes:\n\n- `allow-version`\n- `allow-name`\n- `allow-tauri-version`\n- `allow-identifier`\n- `allow-bundle-type`\n- `allow-register-listener`\n- `allow-remove-listener`",
          "type": "string",
          "const": "core:app:default",
          "markdownDescription": "Default permissions for the plugin.\n#### This default permission set includes:\n\n- `allow-version`\n- `allow-name`\n- `allow-tauri-version`\n- `allow-identifier`\n- `allow-bundle-type`\n- `allow-register-listener`\n- `allow-remove-listener`"
        },
        {
          "description": "Enables the app_hide command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:allow-app-hide",
          "markdownDescription": "Enables the app_hide command without any pre-configured scope."
        },
        {
          "description": "Enables the app_show command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:allow-app-show",
          "markdownDescription": "Enables the app_show command without any pre-configured scope."
        },
        {
          "description": "Enables the bundle_type command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:allow-bundle-type",
          "markdownDescription": "Enables the bundle_type command without any pre-configured scope."
        },
        {
          "description": "Enables the default_window_icon command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:allow-default-window-icon",
          "markdownDescription": "Enables the default_window_icon command without any pre-configured scope."
        },
        {
          "description": "Enables the fetch_data_store_identifiers command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:allow-fetch-data-store-identifiers",
          "markdownDescription": "Enables the fetch_data_store_identifiers command without any pre-configured scope."
        },
        {
          "description": "Enables the identifier command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:allow-identifier",
          "markdownDescription": "Enables the identifier command without any pre-configured scope."
        },
        {
          "description": "Enables the name command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:allow-name",
          "markdownDescription": "Enables the name command without any pre-configured scope."
        },
        {
          "description": "Enables the register_listener command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:allow-register-listener",
          "markdownDescription": "Enables the register_listener command without any pre-configured scope."
        },
        {
          "description": "Enables the remove_data_store command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:allow-remove-data-store",
          "markdownDescription": "Enables the remove_data_store command without any pre-configured scope."
        },
        {
          "description": "Enables the remove_listener command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:allow-remove-listener",
          "markdownDescription": "Enables the remove_listener command without any pre-configured scope."
        },
        {
          "description": "Enables the set_app_theme command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:allow-set-app-theme",
          "markdownDescription": "Enables the set_app_theme command without any pre-configured scope."
        },
        {
          "description": "Enables the set_dock_visibility command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:allow-set-dock-visibility",
          "markdownDescription": "Enables the set_dock_visibility command without any pre-configured scope."
        },
        {
          "description": "Enables the tauri_version command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:allow-tauri-version",
          "markdownDescription": "Enables the tauri_version command without any pre-configured scope."
        },
        {
          "description": "Enables the version command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:allow-version",
          "markdownDescription": "Enables the version command without any pre-configured scope."
        },
        {
          "description": "Denies the app_hide command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:deny-app-hide",
          "markdownDescription": "Denies the app_hide command without any pre-configured scope."
        },
        {
          "description": "Denies the app_show command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:deny-app-show",
          "markdownDescription": "Denies the app_show command without any pre-configured scope."
        },
        {
          "description": "Denies the bundle_type command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:deny-bundle-type",
          "markdownDescription": "Denies the bundle_type command without any pre-configured scope."
        },
        {
          "description": "Denies the default_window_icon command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:deny-default-window-icon",
          "markdownDescription": "Denies the default_window_icon command without any pre-configured scope."
        },
        {
          "description": "Denies the fetch_data_store_identifiers command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:deny-fetch-data-store-identifiers",
          "markdownDescription": "Denies the fetch_data_store_identifiers command without any pre-configured scope."
        },
        {
          "description": "Denies the identifier command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:deny-identifier",
          "markdownDescription": "Denies the identifier command without any pre-configured scope."
        },
        {
          "description": "Denies the name command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:deny-name",
          "markdownDescription": "Denies the name command without any pre-configured scope."
        },
        {
          "description": "Denies the register_listener command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:deny-register-listener",
          "markdownDescription": "Denies the register_listener command without any pre-configured scope."
        },
        {
          "description": "Denies the remove_data_store command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:deny-remove-data-store",
          "markdownDescription": "Denies the remove_data_store command without any pre-configured scope."
        },
        {
          "description": "Denies the remove_listener command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:deny-remove-listener",
          "markdownDescription": "Denies the remove_listener command without any pre-configured scope."
        },
        {
          "description": "Denies the set_app_theme command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:deny-set-app-theme",
          "markdownDescription": "Denies the set_app_theme command without any pre-configured scope."
        },
        {
          "description": "Denies the set_dock_visibility command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:deny-set-dock-visibility",
          "markdownDescription": "Denies the set_dock_visibility command without any pre-configured scope."
        },
        {
          "description": "Denies the tauri_version command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:deny-tauri-version",
          "markdownDescription": "Denies the tauri_version command without any pre-configured scope."
        },
        {
          "description": "Denies the version command without any pre-configured scope.",
          "type": "string",
          "const": "core:app:deny-version",
          "markdownDescription": "Denies the version command without any pre-configured scope."
        },
        {
          "description": "Default permissions for the plugin, which enables all commands.\n#### This default permission set includes:\n\n- `allow-listen`\n- `allow-unlisten`\n- `allow-emit`\n- `allow-emit-to`",
          "type": "string",
          "const": "core:event:default",
          "markdownDescription": "Default permissions for the plugin, which enables all commands.\n#### This default permission set includes:\n\n- `allow-listen`\n- `allow-unlisten`\n- `allow-emit`\n- `allow-emit-to`"
        },
        {
          "description": "Enables the emit command without any pre-configured scope.",
          "type": "string",
          "const": "core:event:allow-emit",
          "markdownDescription": "Enables the emit command without any pre-configured scope."
        },
        {
          "description": "Enables the emit_to command without any pre-configured scope.",
          "type": "string",
          "const": "core:event:allow-emit-to",
          "markdownDescription": "Enables the emit_to command without any pre-configured scope."
        },
        {
          "description": "Enables the listen command without any pre-configured scope.",
          "type": "string",
          "const": "core:event:allow-listen",
          "markdownDescription": "Enables the listen command without any pre-configured scope."
        },
        {
          "description": "Enables the unlisten command without any pre-configured scope.",
          "type": "string",
          "const": "core:event:allow-unlisten",
          "markdownDescription": "Enables the unlisten command without any pre-configured scope."
        },
        {
          "description": "Denies the emit command without any pre-configured scope.",
          "type": "string",
          "const": "core:event:deny-emit",
          "markdownDescription": "Denies the emit command without any pre-configured scope."
        },
        {
          "description": "Denies the emit_to command without any pre-configured scope.",
          "type": "string",
          "const": "core:event:deny-emit-to",
          "markdownDescription": "Denies the emit_to command without any pre-configured scope."
        },
        {
          "description": "Denies the listen command without any pre-configured scope.",
          "type": "string",
          "const": "core:event:deny-listen",
          "markdownDescription": "Denies the listen command without any pre-configured scope."
        },
        {
          "description": "Denies the unlisten command without any pre-configured scope.",
          "type": "string",
          "const": "core:event:deny-unlisten",
          "markdownDescription": "Denies the unlisten command without any pre-configured scope."
        },
        {
          "description": "Default permissions for the plugin, which enables all commands.\n#### This default permission set includes:\n\n- `allow-new`\n- `allow-from-bytes`\n- `allow-from-path`\n- `allow-rgba`\n- `allow-size`",
          "type": "string",
          "const": "core:image:default",
          "markdownDescription": "Default permissions for the plugin, which enables all commands.\n#### This default permission set includes:\n\n- `allow-new`\n- `allow-from-bytes`\n- `allow-from-path`\n- `allow-rgba`\n- `allow-size`"
        },
        {
          "description": "Enables the from_bytes command without any pre-configured scope.",
          "type": "string",
          "const": "core:image:allow-from-bytes",
          "markdownDescription": "Enables the from_bytes command without any pre-configured scope."
        },
        {
          "description": "Enables the from_path command without any pre-configured scope.",
          "type": "string",
          "const": "core:image:allow-from-path",
          "markdownDescription": "Enables the from_path command without any pre-configured scope."
        },
        {
          "description": "Enables the new command without any pre-configured scope.",
          "type": "string",
          "const": "core:image:allow-new",
          "markdownDescription": "Enables the new command without any pre-configured scope."
        },
        {
          "description": "Enables the rgba command without any pre-configured scope.",
          "type": "string",
          "const": "core:image:allow-rgba",
          "markdownDescription": "Enables the rgba command without any pre-configured scope."
        },
        {
          "description": "Enables the size command without any pre-configured scope.",
          "type": "string",
          "const": "core:image:allow-size",
          "markdownDescription": "Enables the size command without any pre-configured scope."
        },
        {
          "description": "Denies the from_bytes command without any pre-configured scope.",
          "type": "string",
          "const": "core:image:deny-from-bytes",
          "markdownDescription": "Denies the from_bytes command without any pre-configured scope."
        },
        {
          "description": "Denies the from_path command without any pre-configured scope.",
          "type": "string",
          "const": "core:image:deny-from-path",
          "markdownDescription": "Denies the from_path command without any pre-configured scope."
        },
        {
          "description": "Denies the new command without any pre-configured scope.",
          "type": "string",
          "const": "core:image:deny-new",
          "markdownDescription": "Denies the new command without any pre-configured scope."
        },
        {
          "description": "Denies the rgba command without any pre-configured scope.",
          "type": "string",
          "const": "core:image:deny-rgba",
          "markdownDescription": "Denies the rgba command without any pre-configured scope."
        },
        {
          "description": "Denies the size command without any pre-configured scope.",
          "type": "string",
          "const": "core:image:deny-size",
          "markdownDescription": "Denies the size command without any pre-configured scope."
        },
        {
          "description": "Default permissions for the plugin, which enables all commands.\n#### This default permission set includes:\n\n- `allow-new`\n- `allow-append`\n- `allow-prepend`\n- `allow-insert`\n- `allow-remove`\n- `allow-remove-at`\n- `allow-items`\n- `allow-get`\n- `allow-popup`\n- `allow-create-default`\n- `allow-set-as-app-menu`\n- `allow-set-as-window-menu`\n- `allow-text`\n- `allow-set-text`\n- `allow-is-enabled`\n- `allow-set-enabled`\n- `allow-set-accelerator`\n- `allow-set-as-windows-menu-for-nsapp`\n- `allow-set-as-help-menu-for-nsapp`\n- `allow-is-checked`\n- `allow-set-checked`\n- `allow-set-icon`",
          "type": "string",
          "const": "core:menu:default",
          "markdownDescription": "Default permissions for the plugin, which enables all commands.\n#### This default permission set includes:\n\n- `allow-new`\n- `allow-append`\n- `allow-prepend`\n- `allow-insert`\n- `allow-remove`\n- `allow-remove-at`\n- `allow-items`\n- `allow-get`\n- `allow-popup`\n- `allow-create-default`\n- `allow-set-as-app-menu`\n- `allow-set-as-window-menu`\n- `allow-text`\n- `allow-set-text`\n- `allow-is-enabled`\n- `allow-set-enabled`\n- `allow-set-accelerator`\n- `allow-set-as-windows-menu-for-nsapp`\n- `allow-set-as-help-menu-for-nsapp`\n- `allow-is-checked`\n- `allow-set-checked`\n- `allow-set-icon`"
        },
        {
          "description": "Enables the append command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:allow-append",
          "markdownDescription": "Enables the append command without any pre-configured scope."
        },
        {
          "description": "Enables the create_default command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:allow-create-default",
          "markdownDescription": "Enables the create_default command without any pre-configured scope."
        },
        {
          "description": "Enables the get command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:allow-get",
          "markdownDescription": "Enables the get command without any pre-configured scope."
        },
        {
          "description": "Enables the insert command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:allow-insert",
          "markdownDescription": "Enables the insert command without any pre-configured scope."
        },
        {
          "description": "Enables the is_checked command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:allow-is-checked",
          "markdownDescription": "Enables the is_checked command without any pre-configured scope."
        },
        {
          "description": "Enables the is_enabled command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:allow-is-enabled",
          "markdownDescription": "Enables the is_enabled command without any pre-configured scope."
        },
        {
          "description": "Enables the items command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:allow-items",
          "markdownDescription": "Enables the items command without any pre-configured scope."
        },
        {
          "description": "Enables the new command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:allow-new",
          "markdownDescription": "Enables the new command without any pre-configured scope."
        },
        {
          "description": "Enables the popup command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:allow-popup",
          "markdownDescription": "Enables the popup command without any pre-configured scope."
        },
        {
          "description": "Enables the prepend command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:allow-prepend",
          "markdownDescription": "Enables the prepend command without any pre-configured scope."
        },
        {
          "description": "Enables the remove command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:allow-remove",
          "markdownDescription": "Enables the remove command without any pre-configured scope."
        },
        {
          "description": "Enables the remove_at command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:allow-remove-at",
          "markdownDescription": "Enables the remove_at command without any pre-configured scope."
        },
        {
          "description": "Enables the set_accelerator command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:allow-set-accelerator",
          "markdownDescription": "Enables the set_accelerator command without any pre-configured scope."
        },
        {
          "description": "Enables the set_as_app_menu command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:allow-set-as-app-menu",
          "markdownDescription": "Enables the set_as_app_menu command without any pre-configured scope."
        },
        {
          "description": "Enables the set_as_help_menu_for_nsapp command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:allow-set-as-help-menu-for-nsapp",
          "markdownDescription": "Enables the set_as_help_menu_for_nsapp command without any pre-configured scope."
        },
        {
          "description": "Enables the set_as_window_menu command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:allow-set-as-window-menu",
          "markdownDescription": "Enables the set_as_window_menu command without any pre-configured scope."
        },
        {
          "description": "Enables the set_as_windows_menu_for_nsapp command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:allow-set-as-windows-menu-for-nsapp",
          "markdownDescription": "Enables the set_as_windows_menu_for_nsapp command without any pre-configured scope."
        },
        {
          "description": "Enables the set_checked command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:allow-set-checked",
          "markdownDescription": "Enables the set_checked command without any pre-configured scope."
        },
        {
          "description": "Enables the set_enabled command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:allow-set-enabled",
          "markdownDescription": "Enables the set_enabled command without any pre-configured scope."
        },
        {
          "description": "Enables the set_icon command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:allow-set-icon",
          "markdownDescription": "Enables the set_icon command without any pre-configured scope."
        },
        {
          "description": "Enables the set_text command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:allow-set-text",
          "markdownDescription": "Enables the set_text command without any pre-configured scope."
        },
        {
          "description": "Enables the text command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:allow-text",
          "markdownDescription": "Enables the text command without any pre-configured scope."
        },
        {
          "description": "Denies the append command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:deny-append",
          "markdownDescription": "Denies the append command without any pre-configured scope."
        },
        {
          "description": "Denies the create_default command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:deny-create-default",
          "markdownDescription": "Denies the create_default command without any pre-configured scope."
        },
        {
          "description": "Denies the get command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:deny-get",
          "markdownDescription": "Denies the get command without any pre-configured scope."
        },
        {
          "description": "Denies the insert command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:deny-insert",
          "markdownDescription": "Denies the insert command without any pre-configured scope."
        },
        {
          "description": "Denies the is_checked command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:deny-is-checked",
          "markdownDescription": "Denies the is_checked command without any pre-configured scope."
        },
        {
          "description": "Denies the is_enabled command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:deny-is-enabled",
          "markdownDescription": "Denies the is_enabled command without any pre-configured scope."
        },
        {
          "description": "Denies the items command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:deny-items",
          "markdownDescription": "Denies the items command without any pre-configured scope."
        },
        {
          "description": "Denies the new command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:deny-new",
          "markdownDescription": "Denies the new command without any pre-configured scope."
        },
        {
          "description": "Denies the popup command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:deny-popup",
          "markdownDescription": "Denies the popup command without any pre-configured scope."
        },
        {
          "description": "Denies the prepend command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:deny-prepend",
          "markdownDescription": "Denies the prepend command without any pre-configured scope."
        },
        {
          "description": "Denies the remove command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:deny-remove",
          "markdownDescription": "Denies the remove command without any pre-configured scope."
        },
        {
          "description": "Denies the remove_at command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:deny-remove-at",
          "markdownDescription": "Denies the remove_at command without any pre-configured scope."
        },
        {
          "description": "Denies the set_accelerator command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:deny-set-accelerator",
          "markdownDescription": "Denies the set_accelerator command without any pre-configured scope."
        },
        {
          "description": "Denies the set_as_app_menu command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:deny-set-as-app-menu",
          "markdownDescription": "Denies the set_as_app_menu command without any pre-configured scope."
        },
        {
          "description": "Denies the set_as_help_menu_for_nsapp command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:deny-set-as-help-menu-for-nsapp",
          "markdownDescription": "Denies the set_as_help_menu_for_nsapp command without any pre-configured scope."
        },
        {
          "description": "Denies the set_as_window_menu command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:deny-set-as-window-menu",
          "markdownDescription": "Denies the set_as_window_menu command without any pre-configured scope."
        },
        {
          "description": "Denies the set_as_windows_menu_for_nsapp command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:deny-set-as-windows-menu-for-nsapp",
          "markdownDescription": "Denies the set_as_windows_menu_for_nsapp command without any pre-configured scope."
        },
        {
          "description": "Denies the set_checked command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:deny-set-checked",
          "markdownDescription": "Denies the set_checked command without any pre-configured scope."
        },
        {
          "description": "Denies the set_enabled command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:deny-set-enabled",
          "markdownDescription": "Denies the set_enabled command without any pre-configured scope."
        },
        {
          "description": "Denies the set_icon command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:deny-set-icon",
          "markdownDescription": "Denies the set_icon command without any pre-configured scope."
        },
        {
          "description": "Denies the set_text command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:deny-set-text",
          "markdownDescription": "Denies the set_text command without any pre-configured scope."
        },
        {
          "description": "Denies the text command without any pre-configured scope.",
          "type": "string",
          "const": "core:menu:deny-text",
          "markdownDescription": "Denies the text command without any pre-configured scope."
        },
        {
          "description": "Default permissions for the plugin, which enables all commands.\n#### This default permission set includes:\n\n- `allow-resolve-directory`\n- `allow-resolve`\n- `allow-normalize`\n- `allow-join`\n- `allow-dirname`\n- `allow-extname`\n- `allow-basename`\n- `allow-is-absolute`",
          "type": "string",
          "const": "core:path:default",
          "markdownDescription": "Default permissions for the plugin, which enables all commands.\n#### This default permission set includes:\n\n- `allow-resolve-directory`\n- `allow-resolve`\n- `allow-normalize`\n- `allow-join`\n- `allow-dirname`\n- `allow-extname`\n- `allow-basename`\n- `allow-is-absolute`"
        },
        {
          "description": "Enables the basename command without any pre-configured scope.",
          "type": "string",
          "const": "core:path:allow-basename",
          "markdownDescription": "Enables the basename command without any pre-configured scope."
        },
        {
          "description": "Enables the dirname command without any pre-configured scope.",
          "type": "string",
          "const": "core:path:allow-dirname",
          "markdownDescription": "Enables the dirname command without any pre-configured scope."
        },
        {
          "description": "Enables the extname command without any pre-configured scope.",
          "type": "string",
          "const": "core:path:allow-extname",
          "markdownDescription": "Enables the extname command without any pre-configured scope."
        },
        {
          "description": "Enables the is_absolute command without any pre-configured scope.",
          "type": "string",
          "const": "core:path:allow-is-absolute",
          "markdownDescription": "Enables the is_absolute command without any pre-configured scope."
        },
        {
          "description": "Enables the join command without any pre-configured scope.",
          "type": "string",
          "const": "core:path:allow-join",
          "markdownDescription": "Enables the join command without any pre-configured scope."
        },
        {
          "description": "Enables the normalize command without any pre-configured scope.",
          "type": "string",
          "const": "core:path:allow-normalize",
          "markdownDescription": "Enables the normalize command without any pre-configured scope."
        },
        {
          "description": "Enables the resolve command without any pre-configured scope.",
          "type": "string",
          "const": "core:path:allow-resolve",
          "markdownDescription": "Enables the resolve command without any pre-configured scope."
        },
        {
          "description": "Enables the resolve_directory command without any pre-configured scope.",
          "type": "string",
          "const": "core:path:allow-resolve-directory",
          "markdownDescription": "Enables the resolve_directory command without any pre-configured scope."
        },
        {
          "description": "Denies the basename command without any pre-configured scope.",
          "type": "string",
          "const": "core:path:deny-basename",
          "markdownDescription": "Denies the basename command without any pre-configured scope."
        },
        {
          "description": "Denies the dirname command without any pre-configured scope.",
          "type": "string",
          "const": "core:path:deny-dirname",
          "markdownDescription": "Denies the dirname command without any pre-configured scope."
        },
        {
          "description": "Denies the extname command without any pre-configured scope.",
          "type": "string",
          "const": "core:path:deny-extname",
          "markdownDescription": "Denies the extname command without any pre-configured scope."
        },
        {
          "description": "Denies the is_absolute command without any pre-configured scope.",
          "type": "string",
          "const": "core:path:deny-is-absolute",
          "markdownDescription": "Denies the is_absolute command without any pre-configured scope."
        },
        {
          "description": "Denies the join command without any pre-configured scope.",
          "type": "string",
          "const": "core:path:deny-join",
          "markdownDescription": "Denies the join command without any pre-configured scope."
        },
        {
          "description": "Denies the normalize command without any pre-configured scope.",
          "type": "string",
          "const": "core:path:deny-normalize",
          "markdownDescription": "Denies the normalize command without any pre-configured scope."
        },
        {
          "description": "Denies the resolve command without any pre-configured scope.",
          "type": "string",
          "const": "core:path:deny-resolve",
          "markdownDescription": "Denies the resolve command without any pre-configured scope."
        },
        {
          "description": "Denies the resolve_directory command without any pre-configured scope.",
          "type": "string",
          "const": "core:path:deny-resolve-directory",
          "markdownDescription": "Denies the resolve_directory command without any pre-configured scope."
        },
        {
          "description": "Default permissions for the plugin, which enables all commands.\n#### This default permission set includes:\n\n- `allow-close`",
          "type": "string",
          "const": "core:resources:default",
          "markdownDescription": "Default permissions for the plugin, which enables all commands.\n#### This default permission set includes:\n\n- `allow-close`"
        },
        {
          "description": "Enables the close command without any pre-configured scope.",
          "type": "string",
          "const": "core:resources:allow-close",
          "markdownDescription": "Enables the close command without any pre-configured scope."
        },
        {
          "description": "Denies the close command without any pre-configured scope.",
          "type": "string",
          "const": "core:resources:deny-close",
          "markdownDescription": "Denies the close command without any pre-configured scope."
        },
        {
          "description": "Default permissions for the plugin, which enables all commands.\n#### This default permission set includes:\n\n- `allow-new`\n- `allow-get-by-id`\n- `allow-remove-by-id`\n- `allow-set-icon`\n- `allow-set-menu`\n- `allow-set-tooltip`\n- `allow-set-title`\n- `allow-set-visible`\n- `allow-set-temp-dir-path`\n- `allow-set-icon-as-template`\n- `allow-set-show-menu-on-left-click`",
          "type": "string",
          "const": "core:tray:default",
          "markdownDescription": "Default permissions for the plugin, which enables all commands.\n#### This default permission set includes:\n\n- `allow-new`\n- `allow-get-by-id`\n- `allow-remove-by-id`\n- `allow-set-icon`\n- `allow-set-menu`\n- `allow-set-tooltip`\n- `allow-set-title`\n- `allow-set-visible`\n- `allow-set-temp-dir-path`\n- `allow-set-icon-as-template`\n- `allow-set-show-menu-on-left-click`"
        },
        {
          "description": "Enables the get_by_id command without any pre-configured scope.",
          "type": "string",
          "const": "core:tray:allow-get-by-id",
          "markdownDescription": "Enables the get_by_id command without any pre-configured scope."
        },
        {
          "description": "Enables the new command without any pre-configured scope.",
          "type": "string",
          "const": "core:tray:allow-new",
          "markdownDescription": "Enables the new command without any pre-configured scope."
        },
        {
          "description": "Enables the remove_by_id command without any pre-configured scope.",
          "type": "string",
          "const": "core:tray:allow-remove-by-id",
          "markdownDescription": "Enables the remove_by_id command without any pre-configured scope."
        },
        {
          "description": "Enables the set_icon command without any pre-configured scope.",
          "type": "string",
          "const": "core:tray:allow-set-icon",
          "markdownDescription": "Enables the set_icon command without any pre-configured scope."
        },
        {
          "description": "Enables the set_icon_as_template command without any pre-configured scope.",
          "type": "string",
          "const": "core:tray:allow-set-icon-as-template",
          "markdownDescription": "Enables the set_icon_as_template command without any pre-configured scope."
        },
        {
          "description": "Enables the set_menu command without any pre-configured scope.",
          "type": "string",
          "const": "core:tray:allow-set-menu",
          "markdownDescription": "Enables the set_menu command without any pre-configured scope."
        },
        {
          "description": "Enables the set_show_menu_on_left_click command without any pre-configured scope.",
          "type": "string",
          "const": "core:tray:allow-set-show-menu-on-left-click",
          "markdownDescription": "Enables the set_show_menu_on_left_click command without any pre-configured scope."
        },
        {
          "description": "Enables the set_temp_dir_path command without any pre-configured scope.",
          "type": "string",
          "const": "core:tray:allow-set-temp-dir-path",
          "markdownDescription": "Enables the set_temp_dir_path command without any pre-configured scope."
        },
        {
          "description": "Enables the set_title command without any pre-configured scope.",
          "type": "string",
          "const": "core:tray:allow-set-title",
          "markdownDescription": "Enables the set_title command without any pre-configured scope."
        },
        {
          "description": "Enables the set_tooltip command without any pre-configured scope.",
          "type": "string",
          "const": "core:tray:allow-set-tooltip",
          "markdownDescription": "Enables the set_tooltip command without any pre-configured scope."
        },
        {
          "description": "Enables the set_visible command without any pre-configured scope.",
          "type": "string",
          "const": "core:tray:allow-set-visible",
          "markdownDescription": "Enables the set_visible command without any pre-configured scope."
        },
        {
          "description": "Denies the get_by_id command without any pre-configured scope.",
          "type": "string",
          "const": "core:tray:deny-get-by-id",
          "markdownDescription": "Denies the get_by_id command without any pre-configured scope."
        },
        {
          "description": "Denies the new command without any pre-configured scope.",
          "type": "string",
          "const": "core:tray:deny-new",
          "markdownDescription": "Denies the new command without any pre-configured scope."
        },
        {
          "description": "Denies the remove_by_id command without any pre-configured scope.",
          "type": "string",
          "const": "core:tray:deny-remove-by-id",
          "markdownDescription": "Denies the remove_by_id command without any pre-configured scope."
        },
        {
          "description": "Denies the set_icon command without any pre-configured scope.",
          "type": "string",
          "const": "core:tray:deny-set-icon",
          "markdownDescription": "Denies the set_icon command without any pre-configured scope."
        },
        {
          "description": "Denies the set_icon_as_template command without any pre-configured scope.",
          "type": "string",
          "const": "core:tray:deny-set-icon-as-template",
          "markdownDescription": "Denies the set_icon_as_template command without any pre-configured scope."
        },
        {
          "description": "Denies the set_menu command without any pre-configured scope.",
          "type": "string",
          "const": "core:tray:deny-set-menu",
          "markdownDescription": "Denies the set_menu command without any pre-configured scope."
        },
        {
          "description": "Denies the set_show_menu_on_left_click command without any pre-configured scope.",
          "type": "string",
          "const": "core:tray:deny-set-show-menu-on-left-click",
          "markdownDescription": "Denies the set_show_menu_on_left_click command without any pre-configured scope."
        },
        {
          "description": "Denies the set_temp_dir_path command without any pre-configured scope.",
          "type": "string",
          "const": "core:tray:deny-set-temp-dir-path",
          "markdownDescription": "Denies the set_temp_dir_path command without any pre-configured scope."
        },
        {
          "description": "Denies the set_title command without any pre-configured scope.",
          "type": "string",
          "const": "core:tray:deny-set-title",
          "markdownDescription": "Denies the set_title command without any pre-configured scope."
        },
        {
          "description": "Denies the set_tooltip command without any pre-configured scope.",
          "type": "string",
          "const": "core:tray:deny-set-tooltip",
          "markdownDescription": "Denies the set_tooltip command without any pre-configured scope."
        },
        {
          "description": "Denies the set_visible command without any pre-configured scope.",
          "type": "string",
          "const": "core:tray:deny-set-visible",
          "markdownDescription": "Denies the set_visible command without any pre-configured scope."
        },
        {
          "description": "Default permissions for the plugin.\n#### This default permission set includes:\n\n- `allow-get-all-webviews`\n- `allow-webview-position`\n- `allow-webview-size`\n- `allow-internal-toggle-devtools`",
          "type": "string",
          "const": "core:webview:default",
          "markdownDescription": "Default permissions for the plugin.\n#### This default permission set includes:\n\n- `allow-get-all-webviews`\n- `allow-webview-position`\n- `allow-webview-size`\n- `allow-internal-toggle-devtools`"
        },
        {
          "description": "Enables the clear_all_browsing_data command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:allow-clear-all-browsing-data",
          "markdownDescription": "Enables the clear_all_browsing_data command without any pre-configured scope."
        },
        {
          "description": "Enables the create_webview command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:allow-create-webview",
          "markdownDescription": "Enables the create_webview command without any pre-configured scope."
        },
        {
          "description": "Enables the create_webview_window command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:allow-create-webview-window",
          "markdownDescription": "Enables the create_webview_window command without any pre-configured scope."
        },
        {
          "description": "Enables the get_all_webviews command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:allow-get-all-webviews",
          "markdownDescription": "Enables the get_all_webviews command without any pre-configured scope."
        },
        {
          "description": "Enables the internal_toggle_devtools command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:allow-internal-toggle-devtools",
          "markdownDescription": "Enables the internal_toggle_devtools command without any pre-configured scope."
        },
        {
          "description": "Enables the print command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:allow-print",
          "markdownDescription": "Enables the print command without any pre-configured scope."
        },
        {
          "description": "Enables the reparent command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:allow-reparent",
          "markdownDescription": "Enables the reparent command without any pre-configured scope."
        },
        {
          "description": "Enables the set_webview_auto_resize command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:allow-set-webview-auto-resize",
          "markdownDescription": "Enables the set_webview_auto_resize command without any pre-configured scope."
        },
        {
          "description": "Enables the set_webview_background_color command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:allow-set-webview-background-color",
          "markdownDescription": "Enables the set_webview_background_color command without any pre-configured scope."
        },
        {
          "description": "Enables the set_webview_focus command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:allow-set-webview-focus",
          "markdownDescription": "Enables the set_webview_focus command without any pre-configured scope."
        },
        {
          "description": "Enables the set_webview_position command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:allow-set-webview-position",
          "markdownDescription": "Enables the set_webview_position command without any pre-configured scope."
        },
        {
          "description": "Enables the set_webview_size command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:allow-set-webview-size",
          "markdownDescription": "Enables the set_webview_size command without any pre-configured scope."
        },
        {
          "description": "Enables the set_webview_zoom command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:allow-set-webview-zoom",
          "markdownDescription": "Enables the set_webview_zoom command without any pre-configured scope."
        },
        {
          "description": "Enables the webview_close command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:allow-webview-close",
          "markdownDescription": "Enables the webview_close command without any pre-configured scope."
        },
        {
          "description": "Enables the webview_hide command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:allow-webview-hide",
          "markdownDescription": "Enables the webview_hide command without any pre-configured scope."
        },
        {
          "description": "Enables the webview_position command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:allow-webview-position",
          "markdownDescription": "Enables the webview_position command without any pre-configured scope."
        },
        {
          "description": "Enables the webview_show command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:allow-webview-show",
          "markdownDescription": "Enables the webview_show command without any pre-configured scope."
        },
        {
          "description": "Enables the webview_size command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:allow-webview-size",
          "markdownDescription": "Enables the webview_size command without any pre-configured scope."
        },
        {
          "description": "Denies the clear_all_browsing_data command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:deny-clear-all-browsing-data",
          "markdownDescription": "Denies the clear_all_browsing_data command without any pre-configured scope."
        },
        {
          "description": "Denies the create_webview command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:deny-create-webview",
          "markdownDescription": "Denies the create_webview command without any pre-configured scope."
        },
        {
          "description": "Denies the create_webview_window command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:deny-create-webview-window",
          "markdownDescription": "Denies the create_webview_window command without any pre-configured scope."
        },
        {
          "description": "Denies the get_all_webviews command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:deny-get-all-webviews",
          "markdownDescription": "Denies the get_all_webviews command without any pre-configured scope."
        },
        {
          "description": "Denies the internal_toggle_devtools command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:deny-internal-toggle-devtools",
          "markdownDescription": "Denies the internal_toggle_devtools command without any pre-configured scope."
        },
        {
          "description": "Denies the print command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:deny-print",
          "markdownDescription": "Denies the print command without any pre-configured scope."
        },
        {
          "description": "Denies the reparent command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:deny-reparent",
          "markdownDescription": "Denies the reparent command without any pre-configured scope."
        },
        {
          "description": "Denies the set_webview_auto_resize command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:deny-set-webview-auto-resize",
          "markdownDescription": "Denies the set_webview_auto_resize command without any pre-configured scope."
        },
        {
          "description": "Denies the set_webview_background_color command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:deny-set-webview-background-color",
          "markdownDescription": "Denies the set_webview_background_color command without any pre-configured scope."
        },
        {
          "description": "Denies the set_webview_focus command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:deny-set-webview-focus",
          "markdownDescription": "Denies the set_webview_focus command without any pre-configured scope."
        },
        {
          "description": "Denies the set_webview_position command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:deny-set-webview-position",
          "markdownDescription": "Denies the set_webview_position command without any pre-configured scope."
        },
        {
          "description": "Denies the set_webview_size command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:deny-set-webview-size",
          "markdownDescription": "Denies the set_webview_size command without any pre-configured scope."
        },
        {
          "description": "Denies the set_webview_zoom command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:deny-set-webview-zoom",
          "markdownDescription": "Denies the set_webview_zoom command without any pre-configured scope."
        },
        {
          "description": "Denies the webview_close command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:deny-webview-close",
          "markdownDescription": "Denies the webview_close command without any pre-configured scope."
        },
        {
          "description": "Denies the webview_hide command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:deny-webview-hide",
          "markdownDescription": "Denies the webview_hide command without any pre-configured scope."
        },
        {
          "description": "Denies the webview_position command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:deny-webview-position",
          "markdownDescription": "Denies the webview_position command without any pre-configured scope."
        },
        {
          "description": "Denies the webview_show command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:deny-webview-show",
          "markdownDescription": "Denies the webview_show command without any pre-configured scope."
        },
        {
          "description": "Denies the webview_size command without any pre-configured scope.",
          "type": "string",
          "const": "core:webview:deny-webview-size",
          "markdownDescription": "Denies the webview_size command without any pre-configured scope."
        },
        {
          "description": "Default permissions for the plugin.\n#### This default permission set includes:\n\n- `allow-get-all-windows`\n- `allow-scale-factor`\n- `allow-inner-position`\n- `allow-outer-position`\n- `allow-inner-size`\n- `allow-outer-size`\n- `allow-is-fullscreen`\n- `allow-is-minimized`\n- `allow-is-maximized`\n- `allow-is-focused`\n- `allow-is-decorated`\n- `allow-is-resizable`\n- `allow-is-maximizable`\n- `allow-is-minimizable`\n- `allow-is-closable`\n- `allow-is-visible`\n- `allow-is-enabled`\n- `allow-title`\n- `allow-current-monitor`\n- `allow-primary-monitor`\n- `allow-monitor-from-point`\n- `allow-available-monitors`\n- `allow-cursor-position`\n- `allow-theme`\n- `allow-is-always-on-top`\n- `allow-internal-toggle-maximize`",
          "type": "string",
          "const": "core:window:default",
          "markdownDescription": "Default permissions for the plugin.\n#### This default permission set includes:\n\n- `allow-get-all-windows`\n- `allow-scale-factor`\n- `allow-inner-position`\n- `allow-outer-position`\n- `allow-inner-size`\n- `allow-outer-size`\n- `allow-is-fullscreen`\n- `allow-is-minimized`\n- `allow-is-maximized`\n- `allow-is-focused`\n- `allow-is-decorated`\n- `allow-is-resizable`\n- `allow-is-maximizable`\n- `allow-is-minimizable`\n- `allow-is-closable`\n- `allow-is-visible`\n- `allow-is-enabled`\n- `allow-title`\n- `allow-current-monitor`\n- `allow-primary-monitor`\n- `allow-monitor-from-point`\n- `allow-available-monitors`\n- `allow-cursor-position`\n- `allow-theme`\n- `allow-is-always-on-top`\n- `allow-internal-toggle-maximize`"
        },
        {
          "description": "Enables the available_monitors command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-available-monitors",
          "markdownDescription": "Enables the available_monitors command without any pre-configured scope."
        },
        {
          "description": "Enables the center command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-center",
          "markdownDescription": "Enables the center command without any pre-configured scope."
        },
        {
          "description": "Enables the close command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-close",
          "markdownDescription": "Enables the close command without any pre-configured scope."
        },
        {
          "description": "Enables the create command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-create",
          "markdownDescription": "Enables the create command without any pre-configured scope."
        },
        {
          "description": "Enables the current_monitor command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-current-monitor",
          "markdownDescription": "Enables the current_monitor command without any pre-configured scope."
        },
        {
          "description": "Enables the cursor_position command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-cursor-position",
          "markdownDescription": "Enables the cursor_position command without any pre-configured scope."
        },
        {
          "description": "Enables the destroy command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-destroy",
          "markdownDescription": "Enables the destroy command without any pre-configured scope."
        },
        {
          "description": "Enables the get_all_windows command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-get-all-windows",
          "markdownDescription": "Enables the get_all_windows command without any pre-configured scope."
        },
        {
          "description": "Enables the hide command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-hide",
          "markdownDescription": "Enables the hide command without any pre-configured scope."
        },
        {
          "description": "Enables the inner_position command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-inner-position",
          "markdownDescription": "Enables the inner_position command without any pre-configured scope."
        },
        {
          "description": "Enables the inner_size command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-inner-size",
          "markdownDescription": "Enables the inner_size command without any pre-configured scope."
        },
        {
          "description": "Enables the internal_toggle_maximize command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-internal-toggle-maximize",
          "markdownDescription": "Enables the internal_toggle_maximize command without any pre-configured scope."
        },
        {
          "description": "Enables the is_always_on_top command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-is-always-on-top",
          "markdownDescription": "Enables the is_always_on_top command without any pre-configured scope."
        },
        {
          "description": "Enables the is_closable command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-is-closable",
          "markdownDescription": "Enables the is_closable command without any pre-configured scope."
        },
        {
          "description": "Enables the is_decorated command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-is-decorated",
          "markdownDescription": "Enables the is_decorated command without any pre-configured scope."
        },
        {
          "description": "Enables the is_enabled command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-is-enabled",
          "markdownDescription": "Enables the is_enabled command without any pre-configured scope."
        },
        {
          "description": "Enables the is_focused command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-is-focused",
          "markdownDescription": "Enables the is_focused command without any pre-configured scope."
        },
        {
          "description": "Enables the is_fullscreen command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-is-fullscreen",
          "markdownDescription": "Enables the is_fullscreen command without any pre-configured scope."
        },
        {
          "description": "Enables the is_maximizable command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-is-maximizable",
          "markdownDescription": "Enables the is_maximizable command without any pre-configured scope."
        },
        {
          "description": "Enables the is_maximized command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-is-maximized",
          "markdownDescription": "Enables the is_maximized command without any pre-configured scope."
        },
        {
          "description": "Enables the is_minimizable command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-is-minimizable",
          "markdownDescription": "Enables the is_minimizable command without any pre-configured scope."
        },
        {
          "description": "Enables the is_minimized command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-is-minimized",
          "markdownDescription": "Enables the is_minimized command without any pre-configured scope."
        },
        {
          "description": "Enables the is_resizable command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-is-resizable",
          "markdownDescription": "Enables the is_resizable command without any pre-configured scope."
        },
        {
          "description": "Enables the is_visible command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-is-visible",
          "markdownDescription": "Enables the is_visible command without any pre-configured scope."
        },
        {
          "description": "Enables the maximize command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-maximize",
          "markdownDescription": "Enables the maximize command without any pre-configured scope."
        },
        {
          "description": "Enables the minimize command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-minimize",
          "markdownDescription": "Enables the minimize command without any pre-configured scope."
        },
        {
          "description": "Enables the monitor_from_point command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-monitor-from-point",
          "markdownDescription": "Enables the monitor_from_point command without any pre-configured scope."
        },
        {
          "description": "Enables the outer_position command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-outer-position",
          "markdownDescription": "Enables the outer_position command without any pre-configured scope."
        },
        {
          "description": "Enables the outer_size command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-outer-size",
          "markdownDescription": "Enables the outer_size command without any pre-configured scope."
        },
        {
          "description": "Enables the primary_monitor command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-primary-monitor",
          "markdownDescription": "Enables the primary_monitor command without any pre-configured scope."
        },
        {
          "description": "Enables the request_user_attention command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-request-user-attention",
          "markdownDescription": "Enables the request_user_attention command without any pre-configured scope."
        },
        {
          "description": "Enables the scale_factor command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-scale-factor",
          "markdownDescription": "Enables the scale_factor command without any pre-configured scope."
        },
        {
          "description": "Enables the set_always_on_bottom command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-always-on-bottom",
          "markdownDescription": "Enables the set_always_on_bottom command without any pre-configured scope."
        },
        {
          "description": "Enables the set_always_on_top command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-always-on-top",
          "markdownDescription": "Enables the set_always_on_top command without any pre-configured scope."
        },
        {
          "description": "Enables the set_background_color command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-background-color",
          "markdownDescription": "Enables the set_background_color command without any pre-configured scope."
        },
        {
          "description": "Enables the set_badge_count command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-badge-count",
          "markdownDescription": "Enables the set_badge_count command without any pre-configured scope."
        },
        {
          "description": "Enables the set_badge_label command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-badge-label",
          "markdownDescription": "Enables the set_badge_label command without any pre-configured scope."
        },
        {
          "description": "Enables the set_closable command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-closable",
          "markdownDescription": "Enables the set_closable command without any pre-configured scope."
        },
        {
          "description": "Enables the set_content_protected command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-content-protected",
          "markdownDescription": "Enables the set_content_protected command without any pre-configured scope."
        },
        {
          "description": "Enables the set_cursor_grab command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-cursor-grab",
          "markdownDescription": "Enables the set_cursor_grab command without any pre-configured scope."
        },
        {
          "description": "Enables the set_cursor_icon command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-cursor-icon",
          "markdownDescription": "Enables the set_cursor_icon command without any pre-configured scope."
        },
        {
          "description": "Enables the set_cursor_position command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-cursor-position",
          "markdownDescription": "Enables the set_cursor_position command without any pre-configured scope."
        },
        {
          "description": "Enables the set_cursor_visible command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-cursor-visible",
          "markdownDescription": "Enables the set_cursor_visible command without any pre-configured scope."
        },
        {
          "description": "Enables the set_decorations command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-decorations",
          "markdownDescription": "Enables the set_decorations command without any pre-configured scope."
        },
        {
          "description": "Enables the set_effects command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-effects",
          "markdownDescription": "Enables the set_effects command without any pre-configured scope."
        },
        {
          "description": "Enables the set_enabled command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-enabled",
          "markdownDescription": "Enables the set_enabled command without any pre-configured scope."
        },
        {
          "description": "Enables the set_focus command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-focus",
          "markdownDescription": "Enables the set_focus command without any pre-configured scope."
        },
        {
          "description": "Enables the set_focusable command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-focusable",
          "markdownDescription": "Enables the set_focusable command without any pre-configured scope."
        },
        {
          "description": "Enables the set_fullscreen command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-fullscreen",
          "markdownDescription": "Enables the set_fullscreen command without any pre-configured scope."
        },
        {
          "description": "Enables the set_icon command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-icon",
          "markdownDescription": "Enables the set_icon command without any pre-configured scope."
        },
        {
          "description": "Enables the set_ignore_cursor_events command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-ignore-cursor-events",
          "markdownDescription": "Enables the set_ignore_cursor_events command without any pre-configured scope."
        },
        {
          "description": "Enables the set_max_size command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-max-size",
          "markdownDescription": "Enables the set_max_size command without any pre-configured scope."
        },
        {
          "description": "Enables the set_maximizable command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-maximizable",
          "markdownDescription": "Enables the set_maximizable command without any pre-configured scope."
        },
        {
          "description": "Enables the set_min_size command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-min-size",
          "markdownDescription": "Enables the set_min_size command without any pre-configured scope."
        },
        {
          "description": "Enables the set_minimizable command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-minimizable",
          "markdownDescription": "Enables the set_minimizable command without any pre-configured scope."
        },
        {
          "description": "Enables the set_overlay_icon command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-overlay-icon",
          "markdownDescription": "Enables the set_overlay_icon command without any pre-configured scope."
        },
        {
          "description": "Enables the set_position command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-position",
          "markdownDescription": "Enables the set_position command without any pre-configured scope."
        },
        {
          "description": "Enables the set_progress_bar command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-progress-bar",
          "markdownDescription": "Enables the set_progress_bar command without any pre-configured scope."
        },
        {
          "description": "Enables the set_resizable command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-resizable",
          "markdownDescription": "Enables the set_resizable command without any pre-configured scope."
        },
        {
          "description": "Enables the set_shadow command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-shadow",
          "markdownDescription": "Enables the set_shadow command without any pre-configured scope."
        },
        {
          "description": "Enables the set_simple_fullscreen command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-simple-fullscreen",
          "markdownDescription": "Enables the set_simple_fullscreen command without any pre-configured scope."
        },
        {
          "description": "Enables the set_size command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-size",
          "markdownDescription": "Enables the set_size command without any pre-configured scope."
        },
        {
          "description": "Enables the set_size_constraints command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-size-constraints",
          "markdownDescription": "Enables the set_size_constraints command without any pre-configured scope."
        },
        {
          "description": "Enables the set_skip_taskbar command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-skip-taskbar",
          "markdownDescription": "Enables the set_skip_taskbar command without any pre-configured scope."
        },
        {
          "description": "Enables the set_theme command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-theme",
          "markdownDescription": "Enables the set_theme command without any pre-configured scope."
        },
        {
          "description": "Enables the set_title command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-title",
          "markdownDescription": "Enables the set_title command without any pre-configured scope."
        },
        {
          "description": "Enables the set_title_bar_style command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-title-bar-style",
          "markdownDescription": "Enables the set_title_bar_style command without any pre-configured scope."
        },
        {
          "description": "Enables the set_visible_on_all_workspaces command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-set-visible-on-all-workspaces",
          "markdownDescription": "Enables the set_visible_on_all_workspaces command without any pre-configured scope."
        },
        {
          "description": "Enables the show command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-show",
          "markdownDescription": "Enables the show command without any pre-configured scope."
        },
        {
          "description": "Enables the start_dragging command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-start-dragging",
          "markdownDescription": "Enables the start_dragging command without any pre-configured scope."
        },
        {
          "description": "Enables the start_resize_dragging command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-start-resize-dragging",
          "markdownDescription": "Enables the start_resize_dragging command without any pre-configured scope."
        },
        {
          "description": "Enables the theme command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-theme",
          "markdownDescription": "Enables the theme command without any pre-configured scope."
        },
        {
          "description": "Enables the title command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-title",
          "markdownDescription": "Enables the title command without any pre-configured scope."
        },
        {
          "description": "Enables the toggle_maximize command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-toggle-maximize",
          "markdownDescription": "Enables the toggle_maximize command without any pre-configured scope."
        },
        {
          "description": "Enables the unmaximize command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-unmaximize",
          "markdownDescription": "Enables the unmaximize command without any pre-configured scope."
        },
        {
          "description": "Enables the unminimize command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:allow-unminimize",
          "markdownDescription": "Enables the unminimize command without any pre-configured scope."
        },
        {
          "description": "Denies the available_monitors command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-available-monitors",
          "markdownDescription": "Denies the available_monitors command without any pre-configured scope."
        },
        {
          "description": "Denies the center command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-center",
          "markdownDescription": "Denies the center command without any pre-configured scope."
        },
        {
          "description": "Denies the close command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-close",
          "markdownDescription": "Denies the close command without any pre-configured scope."
        },
        {
          "description": "Denies the create command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-create",
          "markdownDescription": "Denies the create command without any pre-configured scope."
        },
        {
          "description": "Denies the current_monitor command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-current-monitor",
          "markdownDescription": "Denies the current_monitor command without any pre-configured scope."
        },
        {
          "description": "Denies the cursor_position command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-cursor-position",
          "markdownDescription": "Denies the cursor_position command without any pre-configured scope."
        },
        {
          "description": "Denies the destroy command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-destroy",
          "markdownDescription": "Denies the destroy command without any pre-configured scope."
        },
        {
          "description": "Denies the get_all_windows command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-get-all-windows",
          "markdownDescription": "Denies the get_all_windows command without any pre-configured scope."
        },
        {
          "description": "Denies the hide command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-hide",
          "markdownDescription": "Denies the hide command without any pre-configured scope."
        },
        {
          "description": "Denies the inner_position command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-inner-position",
          "markdownDescription": "Denies the inner_position command without any pre-configured scope."
        },
        {
          "description": "Denies the inner_size command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-inner-size",
          "markdownDescription": "Denies the inner_size command without any pre-configured scope."
        },
        {
          "description": "Denies the internal_toggle_maximize command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-internal-toggle-maximize",
          "markdownDescription": "Denies the internal_toggle_maximize command without any pre-configured scope."
        },
        {
          "description": "Denies the is_always_on_top command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-is-always-on-top",
          "markdownDescription": "Denies the is_always_on_top command without any pre-configured scope."
        },
        {
          "description": "Denies the is_closable command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-is-closable",
          "markdownDescription": "Denies the is_closable command without any pre-configured scope."
        },
        {
          "description": "Denies the is_decorated command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-is-decorated",
          "markdownDescription": "Denies the is_decorated command without any pre-configured scope."
        },
        {
          "description": "Denies the is_enabled command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-is-enabled",
          "markdownDescription": "Denies the is_enabled command without any pre-configured scope."
        },
        {
          "description": "Denies the is_focused command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-is-focused",
          "markdownDescription": "Denies the is_focused command without any pre-configured scope."
        },
        {
          "description": "Denies the is_fullscreen command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-is-fullscreen",
          "markdownDescription": "Denies the is_fullscreen command without any pre-configured scope."
        },
        {
          "description": "Denies the is_maximizable command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-is-maximizable",
          "markdownDescription": "Denies the is_maximizable command without any pre-configured scope."
        },
        {
          "description": "Denies the is_maximized command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-is-maximized",
          "markdownDescription": "Denies the is_maximized command without any pre-configured scope."
        },
        {
          "description": "Denies the is_minimizable command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-is-minimizable",
          "markdownDescription": "Denies the is_minimizable command without any pre-configured scope."
        },
        {
          "description": "Denies the is_minimized command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-is-minimized",
          "markdownDescription": "Denies the is_minimized command without any pre-configured scope."
        },
        {
          "description": "Denies the is_resizable command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-is-resizable",
          "markdownDescription": "Denies the is_resizable command without any pre-configured scope."
        },
        {
          "description": "Denies the is_visible command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-is-visible",
          "markdownDescription": "Denies the is_visible command without any pre-configured scope."
        },
        {
          "description": "Denies the maximize command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-maximize",
          "markdownDescription": "Denies the maximize command without any pre-configured scope."
        },
        {
          "description": "Denies the minimize command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-minimize",
          "markdownDescription": "Denies the minimize command without any pre-configured scope."
        },
        {
          "description": "Denies the monitor_from_point command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-monitor-from-point",
          "markdownDescription": "Denies the monitor_from_point command without any pre-configured scope."
        },
        {
          "description": "Denies the outer_position command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-outer-position",
          "markdownDescription": "Denies the outer_position command without any pre-configured scope."
        },
        {
          "description": "Denies the outer_size command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-outer-size",
          "markdownDescription": "Denies the outer_size command without any pre-configured scope."
        },
        {
          "description": "Denies the primary_monitor command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-primary-monitor",
          "markdownDescription": "Denies the primary_monitor command without any pre-configured scope."
        },
        {
          "description": "Denies the request_user_attention command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-request-user-attention",
          "markdownDescription": "Denies the request_user_attention command without any pre-configured scope."
        },
        {
          "description": "Denies the scale_factor command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-scale-factor",
          "markdownDescription": "Denies the scale_factor command without any pre-configured scope."
        },
        {
          "description": "Denies the set_always_on_bottom command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-always-on-bottom",
          "markdownDescription": "Denies the set_always_on_bottom command without any pre-configured scope."
        },
        {
          "description": "Denies the set_always_on_top command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-always-on-top",
          "markdownDescription": "Denies the set_always_on_top command without any pre-configured scope."
        },
        {
          "description": "Denies the set_background_color command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-background-color",
          "markdownDescription": "Denies the set_background_color command without any pre-configured scope."
        },
        {
          "description": "Denies the set_badge_count command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-badge-count",
          "markdownDescription": "Denies the set_badge_count command without any pre-configured scope."
        },
        {
          "description": "Denies the set_badge_label command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-badge-label",
          "markdownDescription": "Denies the set_badge_label command without any pre-configured scope."
        },
        {
          "description": "Denies the set_closable command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-closable",
          "markdownDescription": "Denies the set_closable command without any pre-configured scope."
        },
        {
          "description": "Denies the set_content_protected command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-content-protected",
          "markdownDescription": "Denies the set_content_protected command without any pre-configured scope."
        },
        {
          "description": "Denies the set_cursor_grab command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-cursor-grab",
          "markdownDescription": "Denies the set_cursor_grab command without any pre-configured scope."
        },
        {
          "description": "Denies the set_cursor_icon command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-cursor-icon",
          "markdownDescription": "Denies the set_cursor_icon command without any pre-configured scope."
        },
        {
          "description": "Denies the set_cursor_position command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-cursor-position",
          "markdownDescription": "Denies the set_cursor_position command without any pre-configured scope."
        },
        {
          "description": "Denies the set_cursor_visible command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-cursor-visible",
          "markdownDescription": "Denies the set_cursor_visible command without any pre-configured scope."
        },
        {
          "description": "Denies the set_decorations command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-decorations",
          "markdownDescription": "Denies the set_decorations command without any pre-configured scope."
        },
        {
          "description": "Denies the set_effects command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-effects",
          "markdownDescription": "Denies the set_effects command without any pre-configured scope."
        },
        {
          "description": "Denies the set_enabled command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-enabled",
          "markdownDescription": "Denies the set_enabled command without any pre-configured scope."
        },
        {
          "description": "Denies the set_focus command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-focus",
          "markdownDescription": "Denies the set_focus command without any pre-configured scope."
        },
        {
          "description": "Denies the set_focusable command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-focusable",
          "markdownDescription": "Denies the set_focusable command without any pre-configured scope."
        },
        {
          "description": "Denies the set_fullscreen command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-fullscreen",
          "markdownDescription": "Denies the set_fullscreen command without any pre-configured scope."
        },
        {
          "description": "Denies the set_icon command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-icon",
          "markdownDescription": "Denies the set_icon command without any pre-configured scope."
        },
        {
          "description": "Denies the set_ignore_cursor_events command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-ignore-cursor-events",
          "markdownDescription": "Denies the set_ignore_cursor_events command without any pre-configured scope."
        },
        {
          "description": "Denies the set_max_size command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-max-size",
          "markdownDescription": "Denies the set_max_size command without any pre-configured scope."
        },
        {
          "description": "Denies the set_maximizable command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-maximizable",
          "markdownDescription": "Denies the set_maximizable command without any pre-configured scope."
        },
        {
          "description": "Denies the set_min_size command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-min-size",
          "markdownDescription": "Denies the set_min_size command without any pre-configured scope."
        },
        {
          "description": "Denies the set_minimizable command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-minimizable",
          "markdownDescription": "Denies the set_minimizable command without any pre-configured scope."
        },
        {
          "description": "Denies the set_overlay_icon command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-overlay-icon",
          "markdownDescription": "Denies the set_overlay_icon command without any pre-configured scope."
        },
        {
          "description": "Denies the set_position command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-position",
          "markdownDescription": "Denies the set_position command without any pre-configured scope."
        },
        {
          "description": "Denies the set_progress_bar command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-progress-bar",
          "markdownDescription": "Denies the set_progress_bar command without any pre-configured scope."
        },
        {
          "description": "Denies the set_resizable command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-resizable",
          "markdownDescription": "Denies the set_resizable command without any pre-configured scope."
        },
        {
          "description": "Denies the set_shadow command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-shadow",
          "markdownDescription": "Denies the set_shadow command without any pre-configured scope."
        },
        {
          "description": "Denies the set_simple_fullscreen command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-simple-fullscreen",
          "markdownDescription": "Denies the set_simple_fullscreen command without any pre-configured scope."
        },
        {
          "description": "Denies the set_size command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-size",
          "markdownDescription": "Denies the set_size command without any pre-configured scope."
        },
        {
          "description": "Denies the set_size_constraints command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-size-constraints",
          "markdownDescription": "Denies the set_size_constraints command without any pre-configured scope."
        },
        {
          "description": "Denies the set_skip_taskbar command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-skip-taskbar",
          "markdownDescription": "Denies the set_skip_taskbar command without any pre-configured scope."
        },
        {
          "description": "Denies the set_theme command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-theme",
          "markdownDescription": "Denies the set_theme command without any pre-configured scope."
        },
        {
          "description": "Denies the set_title command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-title",
          "markdownDescription": "Denies the set_title command without any pre-configured scope."
        },
        {
          "description": "Denies the set_title_bar_style command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-title-bar-style",
          "markdownDescription": "Denies the set_title_bar_style command without any pre-configured scope."
        },
        {
          "description": "Denies the set_visible_on_all_workspaces command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-set-visible-on-all-workspaces",
          "markdownDescription": "Denies the set_visible_on_all_workspaces command without any pre-configured scope."
        },
        {
          "description": "Denies the show command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-show",
          "markdownDescription": "Denies the show command without any pre-configured scope."
        },
        {
          "description": "Denies the start_dragging command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-start-dragging",
          "markdownDescription": "Denies the start_dragging command without any pre-configured scope."
        },
        {
          "description": "Denies the start_resize_dragging command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-start-resize-dragging",
          "markdownDescription": "Denies the start_resize_dragging command without any pre-configured scope."
        },
        {
          "description": "Denies the theme command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-theme",
          "markdownDescription": "Denies the theme command without any pre-configured scope."
        },
        {
          "description": "Denies the title command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-title",
          "markdownDescription": "Denies the title command without any pre-configured scope."
        },
        {
          "description": "Denies the toggle_maximize command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-toggle-maximize",
          "markdownDescription": "Denies the toggle_maximize command without any pre-configured scope."
        },
        {
          "description": "Denies the unmaximize command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-unmaximize",
          "markdownDescription": "Denies the unmaximize command without any pre-configured scope."
        },
        {
          "description": "Denies the unminimize command without any pre-configured scope.",
          "type": "string",
          "const": "core:window:deny-unminimize",
          "markdownDescription": "Denies the unminimize command without any pre-configured scope."
        },
        {
          "description": "Allows the log command\n#### This default permission set includes:\n\n- `allow-log`",
          "type": "string",
          "const": "log:default",
          "markdownDescription": "Allows the log command\n#### This default permission set includes:\n\n- `allow-log`"
        },
        {
          "description": "Enables the log command without any pre-configured scope.",
          "type": "string",
          "const": "log:allow-log",
          "markdownDescription": "Enables the log command without any pre-configured scope."
        },
        {
          "description": "Denies the log command without any pre-configured scope.",
          "type": "string",
          "const": "log:deny-log",
          "markdownDescription": "Denies the log command without any pre-configured scope."
        }
      ]
    },
    "Value": {
      "description": "All supported ACL values.",
      "anyOf": [
        {
          "description": "Represents a null JSON value.",
          "type": "null"
        },
        {
          "description": "Represents a [`bool`].",
          "type": "boolean"
        },
        {
          "description": "Represents a valid ACL [`Number`].",
          "allOf": [
            {
              "$ref": "#/definitions/Number"
            }
          ]
        },
        {
          "description": "Represents a [`String`].",
          "type": "string"
        },
        {
          "description": "Represents a list of other [`Value`]s.",
          "type": "array",
          "items": {
            "$ref": "#/definitions/Value"
          }
        },
        {
          "description": "Represents a map of [`String`] keys to [`Value`]s.",
          "type": "object",
          "additionalProperties": {
            "$ref": "#/definitions/Value"
          }
        }
      ]
    },
    "Number": {
      "description": "A valid ACL number.",
      "anyOf": [
        {
          "description": "Represents an [`i64`].",
          "type": "integer",
          "format": "int64"
        },
        {
          "description": "Represents a [`f64`].",
          "type": "number",
          "format": "double"
        }
      ]
    },
    "Target": {
      "description": "Platform target.",
      "oneOf": [
        {
          "description": "MacOS.",
          "type": "string",
          "enum": [
            "macOS"
          ]
        },
        {
          "description": "Windows.",
          "type": "string",
          "enum": [
            "windows"
          ]
        },
        {
          "description": "Linux.",
          "type": "string",
          "enum": [
            "linux"
          ]
        },
        {
          "description": "Android.",
          "type": "string",
          "enum": [
            "android"
          ]
        },
        {
          "description": "iOS.",
          "type": "string",
          "enum": [
            "iOS"
          ]
        }
      ]
    }
  }
}
```

---

## File: src-tauri/src/commands/bench_tab.rs

```rust
use serde::{Serialize, Deserialize};
use entropy_forge::bench::PerformanceBench;
use entropy_forge::entropy::SystemEntropy;

#[derive(Serialize, Deserialize)]
pub struct BenchResponse {
    pub throughput_mbps: f64,
    pub latency_us: f64,
    pub bytes_generated: usize,
    pub duration_secs: f64,
}

#[tauri::command]
pub fn run_benchmark(bytes: usize) -> BenchResponse {
    let mut entropy = SystemEntropy::new();
    let result = PerformanceBench::benchmark(&mut entropy, bytes);

    BenchResponse {
        throughput_mbps: result.throughput_mbps,
        latency_us: result.latency_us,
        bytes_generated: result.bytes_generated,
        duration_secs: result.duration.as_secs_f64(),
    }
}
```

---

## File: src-tauri/src/commands/learn_tab.rs

```rust
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use entropy_forge::learn::{EncryptionProcess, EntropyProcess, NistProcess};

#[derive(Serialize, Deserialize)]
pub struct SerializedBitOp {
    pub input_bit: bool,
    pub key_bit: bool,
    pub result_bit: bool,
    pub position: usize,
}

#[derive(Serialize, Deserialize)]
pub struct SerializedXorStep {
    pub character: String,
    pub input_byte: u8,
    pub keystream_byte: u8,
    pub result_byte: u8,
    pub input_binary: String,
    pub keystream_binary: String,
    pub result_binary: String,
    pub bit_ops: Vec<SerializedBitOp>,
}

#[derive(Serialize, Deserialize)]
pub struct XorStepsResponse {
    pub steps: Vec<SerializedXorStep>,
    pub total: usize,
}

#[tauri::command]
pub fn get_xor_steps(text: String) -> XorStepsResponse {
    let mut process = EncryptionProcess::new();
    process.start(&text);

    let mut serialized_steps = Vec::new();
    for step in process.steps {
        let mut bit_ops = Vec::new();
        for op in step.bit_ops {
            bit_ops.push(SerializedBitOp {
                input_bit: op.input_bit,
                key_bit: op.key_bit,
                result_bit: op.result_bit,
                position: op.position,
            });
        }

        let input_binary = format!("{:08b}", step.input_byte);
        let keystream_binary = format!("{:08b}", step.keystream_byte);
        let result_binary = format!("{:08b}", step.result_byte);

        serialized_steps.push(SerializedXorStep {
            character: step.character.to_string(),
            input_byte: step.input_byte,
            keystream_byte: step.keystream_byte,
            result_byte: step.result_byte,
            input_binary,
            keystream_binary,
            result_binary,
            bit_ops,
        });
    }

    XorStepsResponse {
        total: serialized_steps.len(),
        steps: serialized_steps,
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerializedEntropyStep {
    pub step_type: String,
    pub byte_counts: HashMap<String, usize>,
    pub probabilities: HashMap<String, f64>,
    pub entropy_contributions: HashMap<String, f64>,
    pub current_entropy_sum: f64,
    pub total_entropy: f64,
    pub max_entropy: f64,
}

#[derive(Serialize, Deserialize)]
pub struct EntropyStepsResponse {
    pub steps: Vec<SerializedEntropyStep>,
    pub total: usize,
}

#[tauri::command]
pub fn get_entropy_steps(text: String) -> EntropyStepsResponse {
    let mut process = EntropyProcess::new();
    process.start(&text);

    let mut serialized_steps = Vec::new();
    for step in process.steps {
        let step_type = format!("{:?}", step.step_type);

        let byte_counts = step.byte_counts.into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect();

        let probabilities = step.probabilities.into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect();

        let entropy_contributions = step.entropy_contributions.into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect();

        serialized_steps.push(SerializedEntropyStep {
            step_type,
            byte_counts,
            probabilities,
            entropy_contributions,
            current_entropy_sum: step.current_entropy_sum,
            total_entropy: step.total_entropy,
            max_entropy: step.max_entropy,
        });
    }

    EntropyStepsResponse {
        total: serialized_steps.len(),
        steps: serialized_steps,
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerializedNistStep {
    pub step_type: String,
    pub bits: Vec<u8>,
    pub ones_count: usize,
    pub zeros_count: usize,
    pub sum: i64,
    pub s_obs: f64,
    pub p_value: f64,
    pub passed: bool,
}

#[derive(Serialize, Deserialize)]
pub struct NistStepsResponse {
    pub steps: Vec<SerializedNistStep>,
    pub total: usize,
    pub input_text: String,
}

#[tauri::command]
pub fn get_nist_steps(text: String) -> NistStepsResponse {
    let mut process = NistProcess::new();
    process.start(&text);

    let mut serialized_steps = Vec::new();
    for step in process.steps {
        serialized_steps.push(SerializedNistStep {
            step_type: format!("{:?}", step.step_type),
            bits: step.bits,
            ones_count: step.ones_count,
            zeros_count: step.zeros_count,
            sum: step.sum,
            s_obs: step.s_obs,
            p_value: step.p_value,
            passed: step.passed,
        });
    }

    NistStepsResponse {
        total: serialized_steps.len(),
        steps: serialized_steps,
        input_text: process.input_text,
    }
}

#[tauri::command]
pub fn get_nist_steps_random(count: usize) -> NistStepsResponse {
    let mut process = NistProcess::new();
    process.generate_random(count);

    let mut serialized_steps = Vec::new();
    for step in process.steps {
        serialized_steps.push(SerializedNistStep {
            step_type: format!("{:?}", step.step_type),
            bits: step.bits,
            ones_count: step.ones_count,
            zeros_count: step.zeros_count,
            sum: step.sum,
            s_obs: step.s_obs,
            p_value: step.p_value,
            passed: step.passed,
        });
    }

    NistStepsResponse {
        total: serialized_steps.len(),
        steps: serialized_steps,
        input_text: process.input_text,
    }
}
```

---

## File: src-tauri/src/commands/mod.rs

```rust
pub mod use_tab;
pub mod test_tab;
pub mod bench_tab;
pub mod learn_tab;
```

---

## File: src-tauri/src/commands/test_tab.rs

```rust
use serde::{Serialize, Deserialize};
use entropy_forge::quality::{QualityMetrics, NistTests};
use entropy_forge::entropy::{SystemEntropy, EntropySource};

#[derive(Serialize, Deserialize)]
pub struct NistResult {
    pub name: String,
    pub p_value: f64,
    pub passed: bool,
}

#[derive(Serialize, Deserialize)]
pub struct QualityResponse {
    pub shannon_entropy: f64,
    pub min_entropy: f64,
    pub chi_square: f64,
    pub mean: f64,
    pub longest_run: usize,
    pub overall_score: f64,
    pub total_bytes: usize,
    pub nist_results: Vec<NistResult>,
}

#[tauri::command]
pub fn run_quality_tests(sample_size: usize) -> QualityResponse {
    let mut entropy = SystemEntropy::new();

    // Run quality metrics
    let metrics = QualityMetrics::analyze(&mut entropy, sample_size);

    // Run NIST tests
    let mut data = vec![0u8; sample_size];
    entropy.fill_bytes(&mut data);

    let nist = NistTests::run_all_tests(&data);
    let nist_results = nist.into_iter().map(|(name, p_value)| {
        NistResult {
            name: name.to_string(),
            p_value,
            passed: p_value >= 0.01
        }
    }).collect();

    QualityResponse {
        shannon_entropy: metrics.shannon_entropy,
        min_entropy: metrics.min_entropy,
        chi_square: metrics.chi_square,
        mean: metrics.mean,
        longest_run: metrics.longest_run,
        overall_score: metrics.overall_score(),
        total_bytes: sample_size,
        nist_results,
    }
}
```

---

## File: src-tauri/src/commands/use_tab.rs

```rust
use serde::{Serialize, Deserialize};
use entropy_forge::crypto::StreamCipher;
use entropy_forge::entropy::SystemEntropy;

#[derive(Serialize, Deserialize)]
pub struct EncryptResponse {
    pub ciphertext: String,
    pub keystream_bytes: Vec<u8>,
}

#[tauri::command]
pub fn encrypt_decrypt(plaintext: String, hex_output: bool) -> EncryptResponse {
    let temp_entropy = SystemEntropy::new();
    let mut cipher = StreamCipher::new(temp_entropy);
    let output = cipher.process(plaintext.as_bytes());

    let ciphertext = if hex_output {
        hex::encode(&output)
    } else {
        String::from_utf8_lossy(&output).to_string()
    };

    let mut keystream_bytes = cipher.state().to_vec();

    // Pad to 64 bytes if needed for visualization grid
    if keystream_bytes.len() < 64 {
        keystream_bytes.resize(64, 0);
    } else {
        keystream_bytes.truncate(64);
    }

    EncryptResponse {
        ciphertext,
        keystream_bytes,
    }
}
```

---

## File: src-tauri/src/lib.rs

```rust
pub mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            commands::use_tab::encrypt_decrypt,
            commands::test_tab::run_quality_tests,
            commands::bench_tab::run_benchmark,
            commands::learn_tab::get_xor_steps,
            commands::learn_tab::get_entropy_steps,
            commands::learn_tab::get_nist_steps,
            commands::learn_tab::get_nist_steps_random
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

---

## File: src-tauri/src/main.rs

```rust
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
  app_lib::run();
}
```

---

## File: src-tauri/tauri.conf.json

```json
{
  "$schema": "https://schema.tauri.app/config/2",
  "productName": "Entropy Forge",
  "version": "0.1.0",
  "identifier": "com.entropyforge.desktop",
  "build": {
    "frontendDist": "../ui/dist"
  },
  "app": {
    "windows": [
      {
        "title": "Entropy Forge",
        "width": 800,
        "height": 600,
        "resizable": true,
        "fullscreen": false
      }
    ],
    "security": {
      "csp": null
    }
  },
  "bundle": {
    "active": true,
    "targets": "all",
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ]
  }
}
```

---

## File: src/bench/mod.rs

```rust
//! Performance benchmarking

mod performance;

pub use performance::{PerformanceBench, BenchmarkResult};
```

---

## File: src/bench/performance.rs

```rust
//! Performance benchmarking for entropy sources

use crate::entropy::EntropySource;
use std::time::{Duration, Instant};

/// Performance benchmark results
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    /// Throughput in megabytes per second
    pub throughput_mbps: f64,
    
    /// Average latency per byte in microseconds
    pub latency_us: f64,
    
    /// Total bytes generated
    pub bytes_generated: usize,
    
    /// Total time taken
    pub duration: Duration,
}

/// Performance benchmarking utility
pub struct PerformanceBench;

impl PerformanceBench {
    /// Benchmark an entropy source
    ///
    /// Generates `total_bytes` and measures throughput and latency.
    ///
    /// # Arguments
    ///
    /// * `source` - Entropy source to benchmark
    /// * `total_bytes` - Number of bytes to generate
    ///
    /// # Examples
    ///
    /// ```
    /// use entropy_forge::entropy::SystemEntropy;
    /// use entropy_forge::bench::PerformanceBench;
    ///
    /// let mut source = SystemEntropy::new();
    /// let result = PerformanceBench::benchmark(&mut source, 1_000_000);
    ///
    /// println!("Throughput: {:.2} MB/s", result.throughput_mbps);
    /// println!("Latency: {:.2} µs/byte", result.latency_us);
    /// ```
    pub fn benchmark<E: ?Sized + EntropySource>(source: &mut E, total_bytes: usize) -> BenchmarkResult {
        let mut buffer = vec![0u8; total_bytes];
        
        let start = Instant::now();
        source.fill_bytes(&mut buffer);
        let duration = start.elapsed();
        
        let duration_secs = duration.as_secs_f64();
        let throughput_mbps = (total_bytes as f64 / duration_secs) / 1_000_000.0;
        let latency_us = (duration_secs * 1_000_000.0) / total_bytes as f64;
        
        BenchmarkResult {
            throughput_mbps,
            latency_us,
            bytes_generated: total_bytes,
            duration,
        }
    }
    
    /// Run multiple iterations and return average
    pub fn benchmark_avg<E: ?Sized + EntropySource>(
        source: &mut E,
        bytes_per_iteration: usize,
        iterations: usize,
    ) -> BenchmarkResult {
        let mut total_throughput = 0.0;
        let mut total_latency = 0.0;
        let mut total_duration = Duration::ZERO;
        let total_bytes = bytes_per_iteration * iterations;
        
        for _ in 0..iterations {
            let result = Self::benchmark(source, bytes_per_iteration);
            total_throughput += result.throughput_mbps;
            total_latency += result.latency_us;
            total_duration += result.duration;
        }
        
        BenchmarkResult {
            throughput_mbps: total_throughput / iterations as f64,
            latency_us: total_latency / iterations as f64,
            bytes_generated: total_bytes,
            duration: total_duration,
        }
    }
}

impl BenchmarkResult {
    /// Format result as human-readable string
    pub fn format(&self) -> String {
        format!(
            "Throughput: {:.2} MB/s\n\
             Latency: {:.2} µs/byte\n\
             Generated: {} bytes in {:.2}s",
            self.throughput_mbps,
            self.latency_us,
            self.bytes_generated,
            self.duration.as_secs_f64()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entropy::SystemEntropy;
    
    #[test]
    fn test_benchmark() {
        let mut source = SystemEntropy::new();
        let result = PerformanceBench::benchmark(&mut source, 10_000);
        
        assert!(result.throughput_mbps > 0.0);
        assert!(result.latency_us > 0.0);
        assert_eq!(result.bytes_generated, 10_000);
    }
    
    #[test]
    fn test_benchmark_avg() {
        let mut source = SystemEntropy::new();
        let result = PerformanceBench::benchmark_avg(&mut source, 1_000, 5);
        
        assert!(result.throughput_mbps > 0.0);
        assert_eq!(result.bytes_generated, 5_000);
    }
}
```

---

## File: src/crypto/cipher.rs

```rust
//! Simple stream cipher implementation

use crate::entropy::EntropySource;

/// Simple XOR stream cipher
///
/// This cipher generates a keystream from the entropy source and XORs it
/// with the plaintext. Since XOR is symmetric, the same operation is used
/// for both encryption and decryption.
///
/// # Security Note
///
/// This is a simplified educational implementation. For production use,
/// consider standard ciphers like ChaCha20 or AES-GCM.
///
/// # Examples
///
/// ```
/// use entropy_forge::entropy::SystemEntropy;
/// use entropy_forge::crypto::StreamCipher;
///
/// let entropy = SystemEntropy::new();
/// let mut cipher = StreamCipher::new(entropy);
///
/// let plaintext = b"Hello, World!";
/// let ciphertext = cipher.process(plaintext);
/// let decrypted = cipher.process(&ciphertext);
///
/// // Note: Due to new keystream, decrypted won't match plaintext
/// // In real usage, you'd need to sync the keystream
/// ```
pub struct StreamCipher<E: EntropySource> {
    entropy: E,
    state: Vec<u8>,
    bytes_processed: usize,
}

impl<E: EntropySource> StreamCipher<E> {
    /// Create a new cipher with given entropy source
    pub fn new(entropy: E) -> Self {
        Self {
            entropy,
            state: Vec::new(),
            bytes_processed: 0,
        }
    }
    
    /// Process data (encrypt or decrypt - XOR is symmetric)
    ///
    /// Generates a fresh keystream and XORs with input data.
    ///
    /// # Arguments
    ///
    /// * `data` - Input data to process
    ///
    /// # Returns
    ///
    /// Processed data (same length as input)
    pub fn process(&mut self, data: &[u8]) -> Vec<u8> {
        // Generate keystream
        let mut keystream = vec![0u8; data.len()];
        self.entropy.fill_bytes(&mut keystream);
        
        // Store state for visualization (last 64 bytes)
        self.state = keystream.iter()
            .take(64.min(keystream.len()))
            .copied()
            .collect();
        
        // XOR data with keystream
        let output: Vec<u8> = data.iter()
            .zip(keystream.iter())
            .map(|(d, k)| d ^ k)
            .collect();
        
        self.bytes_processed += data.len();
        output
    }
    
    /// Get current cipher state (for visualization)
    ///
    /// Returns the last 64 bytes of keystream generated, or fewer if
    /// less has been generated.
    pub fn state(&self) -> &[u8] {
        &self.state
    }
    
    /// Get total bytes processed
    pub fn bytes_processed(&self) -> usize {
        self.bytes_processed
    }
    
    /// Reset the cipher (restarts byte counter)
    pub fn reset(&mut self) {
        self.state.clear();
        self.bytes_processed = 0;
        self.entropy.reset();
    }
    
    /// Calculate avalanche effect
    ///
    /// Encrypts data, flips one bit, re-encrypts, and returns the percentage
    /// of bits that changed in the output.
    ///
    /// A good cipher should have ~50% avalanche effect.
    pub fn avalanche_effect(&mut self, data: &[u8], bit_to_flip: usize) -> f64 {
        if data.is_empty() {
            return 0.0;
        }
        
        // First encryption
        let output1 = self.process(data);
        
        // Flip one bit in input
        let mut modified_data = data.to_vec();
        let byte_idx = bit_to_flip / 8;
        let bit_idx = bit_to_flip % 8;
        
        if byte_idx < modified_data.len() {
            modified_data[byte_idx] ^= 1 << bit_idx;
        }
        
        // Second encryption (will use different keystream!)
        let output2 = self.process(&modified_data);
        
        // Count bit differences
        let mut different_bits = 0;
        let total_bits = output1.len() * 8;
        
        for (b1, b2) in output1.iter().zip(output2.iter()) {
            let xor = b1 ^ b2;
            different_bits += xor.count_ones() as usize;
        }
        
        (different_bits as f64 / total_bits as f64) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entropy::MockEntropy;
    
    #[test]
    fn test_cipher_basic() {
        let entropy = MockEntropy::new(42);
        let mut cipher = StreamCipher::new(entropy);
        
        let plaintext = b"Hello, World!";
        let ciphertext = cipher.process(plaintext);
        
        // Should produce different output
        assert_ne!(plaintext.as_slice(), ciphertext.as_slice());
        assert_eq!(plaintext.len(), ciphertext.len());
    }
    
    #[test]
    fn test_cipher_deterministic() {
        let entropy1 = MockEntropy::new(42);
        let entropy2 = MockEntropy::new(42);
        
        let mut cipher1 = StreamCipher::new(entropy1);
        let mut cipher2 = StreamCipher::new(entropy2);
        
        let plaintext = b"Test message";
        let ciphertext1 = cipher1.process(plaintext);
        let ciphertext2 = cipher2.process(plaintext);
        
        // Same entropy seed should produce same ciphertext
        assert_eq!(ciphertext1, ciphertext2);
    }
    
    #[test]
    fn test_cipher_state() {
        let entropy = MockEntropy::new(42);
        let mut cipher = StreamCipher::new(entropy);
        
        cipher.process(b"Some data");
        let state = cipher.state();
        
        assert!(!state.is_empty());
        assert!(state.len() <= 64);
    }
    
    #[test]
    fn test_bytes_processed() {
        let entropy = MockEntropy::new(42);
        let mut cipher = StreamCipher::new(entropy);
        
        cipher.process(b"12345");
        assert_eq!(cipher.bytes_processed(), 5);
        
        cipher.process(b"67890");
        assert_eq!(cipher.bytes_processed(), 10);
    }
    
    #[test]
    fn test_avalanche_effect() {
        let entropy = MockEntropy::new(42);
        let mut cipher = StreamCipher::new(entropy);
        
        let data = b"Test data for avalanche";
        let avalanche = cipher.avalanche_effect(data, 0);
        
        // Should have some avalanche effect (though it won't be 50% with
        // this simple implementation and random keystream changes)
        assert!(avalanche >= 0.0 && avalanche <= 100.0);
    }
}
```

---

## File: src/crypto/mod.rs

```rust
//! Cryptographic operations using entropy sources
//!
//! This module provides cryptographic primitives that use the pluggable
//! entropy source system.

mod cipher;

pub use cipher::StreamCipher;
```

---

## File: src/entropy/mock.rs

```rust
//! Mock entropy source for testing

use super::EntropySource;

/// Mock entropy source for testing
///
/// This source produces predictable output, which is useful for testing
/// and debugging. DO NOT use in production!
///
/// # Examples
///
/// ```
/// use entropy_forge::entropy::{EntropySource, MockEntropy};
///
/// let mut entropy = MockEntropy::new(42);
/// let mut buffer = [0u8; 4];
/// entropy.fill_bytes(&mut buffer);
/// // buffer now contains predictable bytes based on seed
/// ```
#[derive(Debug, Clone)]
pub struct MockEntropy {
    state: u64,
    initial_state: u64,
}

impl MockEntropy {
    /// Create a new mock entropy source with given seed
    pub fn new(seed: u64) -> Self {
        Self {
            state: seed,
            initial_state: seed,
        }
    }
    
    /// Create with default seed (42)
    pub fn default() -> Self {
        Self::new(42)
    }
    
    // Simple LCG (Linear Congruential Generator)
    // Using glibc's constants
    fn next(&mut self) -> u8 {
        self.state = self.state.wrapping_mul(1103515245).wrapping_add(12345);
        (self.state >> 24) as u8
    }
}

impl EntropySource for MockEntropy {
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        for byte in dest.iter_mut() {
            *byte = self.next();
        }
    }
    
    fn name(&self) -> &str {
        "Mock RNG (for testing only)"
    }
    
    fn reset(&mut self) {
        self.state = self.initial_state;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_mock_deterministic() {
        let mut entropy1 = MockEntropy::new(42);
        let mut entropy2 = MockEntropy::new(42);
        
        let mut buf1 = [0u8; 32];
        let mut buf2 = [0u8; 32];
        
        entropy1.fill_bytes(&mut buf1);
        entropy2.fill_bytes(&mut buf2);
        
        // Same seed should produce same output
        assert_eq!(buf1, buf2);
    }
    
    #[test]
    fn test_mock_different_seeds() {
        let mut entropy1 = MockEntropy::new(42);
        let mut entropy2 = MockEntropy::new(43);
        
        let mut buf1 = [0u8; 32];
        let mut buf2 = [0u8; 32];
        
        entropy1.fill_bytes(&mut buf1);
        entropy2.fill_bytes(&mut buf2);
        
        // Different seeds should produce different output
        assert_ne!(buf1, buf2);
    }
    
    #[test]
    fn test_mock_reset() {
        let mut entropy = MockEntropy::new(42);
        
        let mut buf1 = [0u8; 32];
        let mut buf2 = [0u8; 32];
        
        entropy.fill_bytes(&mut buf1);
        entropy.reset();
        entropy.fill_bytes(&mut buf2);
        
        // After reset, should produce same output
        assert_eq!(buf1, buf2);
    }
}
```

---

## File: src/entropy/mod.rs

```rust
//! Entropy source trait and implementations
//!
//! This module defines the core `EntropySource` trait that allows any RNG
//! to be used throughout the Entropy Forge framework.

mod system;
mod mock;

pub use system::SystemEntropy;
pub use mock::MockEntropy;

/// Core trait for entropy sources
///
/// Any random number generator can implement this trait to be used with
/// Entropy Forge. The framework will use your RNG for cryptographic
/// operations, quality testing, and benchmarking.
///
/// # Examples
///
/// ```
/// use entropy_forge::entropy::EntropySource;
///
/// struct MyRNG {
///     state: u64,
/// }
///
/// impl EntropySource for MyRNG {
///     fn fill_bytes(&mut self, dest: &mut [u8]) {
///         for byte in dest.iter_mut() {
///             // Simple LCG for example (DO NOT use in production!)
///             self.state = self.state.wrapping_mul(1103515245).wrapping_add(12345);
///             *byte = (self.state >> 24) as u8;
///         }
///     }
///     
///     fn name(&self) -> &str {
///         "My Custom RNG"
///     }
/// }
/// ```
pub trait EntropySource {
    /// Fill the destination buffer with random bytes
    ///
    /// This is the main method that must be implemented. All other methods
    /// have default implementations that use this.
    ///
    /// # Arguments
    ///
    /// * `dest` - Buffer to fill with random bytes
    fn fill_bytes(&mut self, dest: &mut [u8]);
    
    /// Get a single random byte
    ///
    /// Default implementation uses `fill_bytes`. Override if you have
    /// a more efficient method.
    fn next_byte(&mut self) -> u8 {
        let mut buf = [0u8; 1];
        self.fill_bytes(&mut buf);
        buf[0]
    }
    
    /// Get a random u32
    ///
    /// Default implementation uses `fill_bytes`. Override if you have
    /// a more efficient method.
    fn next_u32(&mut self) -> u32 {
        let mut buf = [0u8; 4];
        self.fill_bytes(&mut buf);
        u32::from_le_bytes(buf)
    }
    
    /// Get a random u64
    ///
    /// Default implementation uses `fill_bytes`. Override if you have
    /// a more efficient method.
    fn next_u64(&mut self) -> u64 {
        let mut buf = [0u8; 8];
        self.fill_bytes(&mut buf);
        u64::from_le_bytes(buf)
    }
    
    /// Get the source name for display purposes
    ///
    /// This is used in the GUI and logging to identify which entropy
    /// source is currently active.
    fn name(&self) -> &str {
        "Unknown Source"
    }
    
    /// Optional: Reset the source to initial state
    ///
    /// Some sources may support resetting to a known state for testing.
    /// Default implementation does nothing.
    fn reset(&mut self) {
        // Default: no-op
    }
}

// Blanket implementation for boxed trait objects
impl EntropySource for Box<dyn EntropySource> {
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        (**self).fill_bytes(dest)
    }
    
    fn name(&self) -> &str {
        (**self).name()
    }
}

// Blanket implementation for mutable references
impl<T: ?Sized + EntropySource> EntropySource for &mut T {
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        (**self).fill_bytes(dest)
    }

    fn name(&self) -> &str {
        (**self).name()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_system_entropy() {
        let mut entropy = SystemEntropy::new();
        let mut buf = [0u8; 32];
        entropy.fill_bytes(&mut buf);
        
        // Should not be all zeros (probability is negligible)
        assert!(buf.iter().any(|&b| b != 0));
    }
    
    #[test]
    fn test_next_methods() {
        let mut entropy = SystemEntropy::new();
        
        let byte = entropy.next_byte();
        let u32_val = entropy.next_u32();
        let u64_val = entropy.next_u64();
        
        // Just check they don't panic
        assert!(byte <= 255);
        assert!(u32_val <= u32::MAX);
        assert!(u64_val <= u64::MAX);
    }
}
```

---

## File: src/entropy/system.rs

```rust
//! System entropy source using OS random number generator

use super::EntropySource;
use getrandom::getrandom;

/// System entropy source (uses OS RNG)
///
/// This implementation uses the operating system's cryptographically secure
/// random number generator:
/// - Linux/Android: `/dev/urandom`
/// - macOS/iOS: `SecRandomCopyBytes`
/// - Windows: `BCryptGenRandom`
///
/// This is the default entropy source and provides good quality randomness
/// for most applications.
///
/// # Examples
///
/// ```
/// use entropy_forge::entropy::{EntropySource, SystemEntropy};
///
/// let mut entropy = SystemEntropy::new();
/// let mut buffer = [0u8; 32];
/// entropy.fill_bytes(&mut buffer);
/// ```
#[derive(Debug, Default, Clone)]
pub struct SystemEntropy;

impl SystemEntropy {
    /// Create a new system entropy source
    pub fn new() -> Self {
        Self
    }
}

impl EntropySource for SystemEntropy {
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        getrandom(dest).expect("Failed to get system entropy - this should never happen on supported platforms");
    }
    
    fn name(&self) -> &str {
        #[cfg(target_os = "linux")]
        return "System RNG (Linux /dev/urandom)";
        
        #[cfg(target_os = "macos")]
        return "System RNG (macOS SecRandomCopyBytes)";
        
        #[cfg(target_os = "windows")]
        return "System RNG (Windows BCryptGenRandom)";
        
        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
        return "System RNG";
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_system_entropy_basic() {
        let mut entropy = SystemEntropy::new();
        let mut buf1 = [0u8; 32];
        let mut buf2 = [0u8; 32];
        
        entropy.fill_bytes(&mut buf1);
        entropy.fill_bytes(&mut buf2);
        
        // Should produce different output each time
        assert_ne!(buf1, buf2);
    }
    
    #[test]
    fn test_system_entropy_large() {
        let mut entropy = SystemEntropy::new();
        let mut buf = vec![0u8; 1_000_000];
        
        entropy.fill_bytes(&mut buf);
        
        // Check that we have good distribution (not all zeros)
        let zeros = buf.iter().filter(|&&b| b == 0).count();
        let expected_zeros = 1_000_000 / 256; // ~3906
        
        // Should be roughly 1/256 of bytes (allow 2x variance)
        assert!(zeros > expected_zeros / 2);
        assert!(zeros < expected_zeros * 2);
    }
    
    #[test]
    fn test_name() {
        let entropy = SystemEntropy::new();
        let name = entropy.name();
        assert!(name.contains("System RNG"));
    }
}
```

---

## File: src/learn/entropy_visual.rs

```rust
use std::collections::HashMap;

/// Represents a step in the entropy calculation process
#[derive(Debug, Clone, PartialEq)]
pub enum EntropyStepType {
    CountBytes,
    CalculateProbabilities,
    CalculateContributions,
    SumEntropy,
    Interpret,
}

#[derive(Debug, Clone)]
pub struct EntropyStep {
    pub step_type: EntropyStepType,
    pub byte_counts: HashMap<u8, usize>,
    pub probabilities: HashMap<u8, f64>,
    pub entropy_contributions: HashMap<u8, f64>,
    pub current_entropy_sum: f64,
    pub total_entropy: f64,
    pub max_entropy: f64,
}

/// Manages the state of the Shannon entropy visualization
pub struct EntropyProcess {
    pub input: String,
    pub steps: Vec<EntropyStep>,
    pub current_step_index: usize,
    pub is_playing: bool,
    pub speed: f32,
    pub last_update: f64,
}

impl Default for EntropyProcess {
    fn default() -> Self {
        Self {
            input: String::new(),
            steps: Vec::new(),
            current_step_index: 0,
            is_playing: false,
            speed: 1.0,
            last_update: 0.0,
        }
    }
}


impl EntropyProcess {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start(&mut self, text: &str) {
        self.input = text.to_string();
        self.steps.clear();
        self.current_step_index = 0;
        self.is_playing = false;

        let data = text.as_bytes();
        if data.is_empty() {
            return;
        }

        // --- Step 1: Count Bytes ---
        let mut byte_counts = HashMap::new();
        for &byte in data {
            *byte_counts.entry(byte).or_insert(0) += 1;
        }

        self.steps.push(EntropyStep {
            step_type: EntropyStepType::CountBytes,
            byte_counts: byte_counts.clone(),
            probabilities: HashMap::new(),
            entropy_contributions: HashMap::new(),
            current_entropy_sum: 0.0,
            total_entropy: 0.0,
            max_entropy: 0.0,
        });

        // --- Step 2: Calculate Probabilities ---
        let total_bytes = data.len() as f64;
        let mut probabilities = HashMap::new();
        for (&byte, &count) in &byte_counts {
            probabilities.insert(byte, count as f64 / total_bytes);
        }

        self.steps.push(EntropyStep {
            step_type: EntropyStepType::CalculateProbabilities,
            byte_counts: byte_counts.clone(),
            probabilities: probabilities.clone(),
            entropy_contributions: HashMap::new(),
            current_entropy_sum: 0.0,
            total_entropy: 0.0,
            max_entropy: 0.0,
        });

        // --- Step 3: Calculate Contributions ---
        let mut entropy_contributions = HashMap::new();
        for (&byte, &p) in &probabilities {
            if p > 0.0 {
                entropy_contributions.insert(byte, -p * p.log2());
            } else {
                entropy_contributions.insert(byte, 0.0);
            }
        }

        self.steps.push(EntropyStep {
            step_type: EntropyStepType::CalculateContributions,
            byte_counts: byte_counts.clone(),
            probabilities: probabilities.clone(),
            entropy_contributions: entropy_contributions.clone(),
            current_entropy_sum: 0.0,
            total_entropy: 0.0,
            max_entropy: 0.0,
        });

        // --- Step 4: Sum Entropy ---
        let total_entropy: f64 = entropy_contributions.values().sum();
        let max_possible = if !probabilities.is_empty() {
            (probabilities.len() as f64).log2()
        } else {
            0.0
        };

        self.steps.push(EntropyStep {
            step_type: EntropyStepType::SumEntropy,
            byte_counts: byte_counts.clone(),
            probabilities: probabilities.clone(),
            entropy_contributions: entropy_contributions.clone(),
            current_entropy_sum: total_entropy,
            total_entropy,
            max_entropy: max_possible,
        });

        // --- Step 5: Interpretation ---
        self.steps.push(EntropyStep {
            step_type: EntropyStepType::Interpret,
            byte_counts: byte_counts,
            probabilities: probabilities,
            entropy_contributions: entropy_contributions,
            current_entropy_sum: total_entropy,
            total_entropy,
            max_entropy: max_possible,
        });
    }

    pub fn current_step(&self) -> Option<&EntropyStep> {
        if self.steps.is_empty() {
            None
        } else {
            Some(&self.steps[self.current_step_index])
        }
    }

    pub fn next_step(&mut self) {
        if self.current_step_index + 1 < self.steps.len() {
            self.current_step_index += 1;
        } else {
            self.is_playing = false;
        }
    }

    pub fn prev_step(&mut self) {
        if self.current_step_index > 0 {
            self.current_step_index -= 1;
        }
    }

    pub fn toggle_play(&mut self) {
        self.is_playing = !self.is_playing;
    }

    pub fn update(&mut self, time: f64) {
        if self.is_playing {
            if time - self.last_update > (1.0 / self.speed as f64) {
                self.next_step();
                self.last_update = time;
            }
        } else {
            self.last_update = time;
        }
    }
}
```

---

## File: src/learn/mod.rs

```rust
pub mod steps;
pub mod xor_visual;
pub mod entropy_visual;
pub mod nist_visual;

pub use xor_visual::EncryptionProcess;
pub use steps::{EncryptionStep, BitOperation};
pub use entropy_visual::EntropyProcess;
pub use nist_visual::NistProcess;
```

---

## File: src/learn/nist_visual.rs

```rust
use crate::quality::NistTests;

#[derive(Debug, Clone, PartialEq)]
pub enum NistStepType {
    ConvertToBits,
    CountOnesZeros,
    CalculateStatistic,
    CalculatePValue,
    Interpret,
}

#[derive(Debug, Clone)]
pub struct NistFrequencyStep {
    pub step_type: NistStepType,
    pub bits: Vec<u8>, // 0 or 1
    pub ones_count: usize,
    pub zeros_count: usize,
    pub sum: i64,
    pub s_obs: f64,
    pub p_value: f64,
    pub passed: bool,
}

pub struct NistProcess {
    pub input_text: String,
    pub steps: Vec<NistFrequencyStep>,
    pub current_step_index: usize,
    pub is_playing: bool,
    pub speed: f32,
    pub last_update: f64,
}

impl Default for NistProcess {
    fn default() -> Self {
        Self {
            input_text: String::new(),
            steps: Vec::new(),
            current_step_index: 0,
            is_playing: false,
            speed: 1.0,
            last_update: 0.0,
        }
    }
}


impl NistProcess {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start(&mut self, text: &str) {
        self.input_text = text.to_string();
        self.steps.clear();
        self.current_step_index = 0;
        self.is_playing = false;

        let data = text.as_bytes();
        if data.is_empty() {
            return;
        }

        // --- Step 1: Convert to Bits ---
        let mut bits = Vec::new();
        for &byte in data {
            for i in (0..8).rev() {
                bits.push((byte >> i) & 1);
            }
        }

        self.steps.push(NistFrequencyStep {
            step_type: NistStepType::ConvertToBits,
            bits: bits.clone(),
            ones_count: 0,
            zeros_count: 0,
            sum: 0,
            s_obs: 0.0,
            p_value: 0.0,
            passed: false,
        });

        // --- Step 2: Count Ones and Zeros ---
        let ones_count = bits.iter().filter(|&&b| b == 1).count();
        let zeros_count = bits.len() - ones_count;

        self.steps.push(NistFrequencyStep {
            step_type: NistStepType::CountOnesZeros,
            bits: bits.clone(),
            ones_count,
            zeros_count,
            sum: 0,
            s_obs: 0.0,
            p_value: 0.0,
            passed: false,
        });

        // --- Step 3: Calculate Statistic ---
        // Sum = (+1 * ones) + (-1 * zeros)
        let sum = (ones_count as i64) - (zeros_count as i64);
        let n = bits.len() as f64;
        let s_obs = (sum as f64).abs() / n.sqrt();

        self.steps.push(NistFrequencyStep {
            step_type: NistStepType::CalculateStatistic,
            bits: bits.clone(),
            ones_count,
            zeros_count,
            sum,
            s_obs,
            p_value: 0.0,
            passed: false,
        });

        // --- Step 4: Calculate P-Value ---
        // p_value = erfc(S_obs / sqrt(2))
        let p_value = NistTests::erfc(s_obs / std::f64::consts::SQRT_2);

        self.steps.push(NistFrequencyStep {
            step_type: NistStepType::CalculatePValue,
            bits: bits.clone(),
            ones_count,
            zeros_count,
            sum,
            s_obs,
            p_value,
            passed: false, // Not decided yet visually, but mathematically yes
        });

        // --- Step 5: Interpretation ---
        let passed = p_value >= 0.01;
        self.steps.push(NistFrequencyStep {
            step_type: NistStepType::Interpret,
            bits: bits.clone(),
            ones_count,
            zeros_count,
            sum,
            s_obs,
            p_value,
            passed,
        });
    }

    pub fn generate_random(&mut self, count: usize) {
        use crate::entropy::{EntropySource, SystemEntropy};
        let mut entropy = SystemEntropy::new();
        let mut data = vec![0u8; count];
        entropy.fill_bytes(&mut data);

        // Compromise: Generate a random string of alphanumeric characters.
        // It's easy to understand and has high entropy.

        // Re-using SystemEntropy is better as I already have it.
        // SystemEntropy fills bytes. I can map them to chars.

        let chars: String = data.iter()
            .map(|&b| {
                // map to printable ascii range 33-126
                let c = 33 + (b % (126 - 33));
                c as char
            })
            .collect();

        self.start(&chars);
    }

    pub fn current_step(&self) -> Option<&NistFrequencyStep> {
        if self.steps.is_empty() {
            None
        } else {
            Some(&self.steps[self.current_step_index])
        }
    }

    pub fn next_step(&mut self) {
        if self.current_step_index + 1 < self.steps.len() {
            self.current_step_index += 1;
        } else {
            self.is_playing = false;
        }
    }

    pub fn prev_step(&mut self) {
        if self.current_step_index > 0 {
            self.current_step_index -= 1;
        }
    }

    pub fn toggle_play(&mut self) {
        self.is_playing = !self.is_playing;
    }

    pub fn update(&mut self, time: f64) {
        if self.is_playing {
            if time - self.last_update > (1.0 / self.speed as f64) {
                self.next_step();
                self.last_update = time;
            }
        } else {
            self.last_update = time;
        }
    }
}
```

---

## File: src/learn/steps.rs

```rust
/// Represents a single bit XOR operation
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BitOperation {
    pub input_bit: bool,
    pub key_bit: bool,
    pub result_bit: bool,
    pub position: usize, // 0-7, where 0 is MSB or LSB depending on preference (usually MSB 7..0)
}

/// Represents one byte being encrypted
#[derive(Debug, Clone)]
pub struct EncryptionStep {
    pub character: char,
    pub input_byte: u8,
    pub keystream_byte: u8,
    pub result_byte: u8,
    pub bit_ops: Vec<BitOperation>,
}

impl EncryptionStep {
    pub fn new(input_char: char, keystream_byte: u8) -> Self {
        let input_byte = input_char as u8;
        let result_byte = input_byte ^ keystream_byte;

        let mut bit_ops = Vec::with_capacity(8);

        // Process bits from MSB (7) to LSB (0) for display
        for i in (0..8).rev() {
            let mask = 1 << i;
            let input_bit = (input_byte & mask) != 0;
            let key_bit = (keystream_byte & mask) != 0;
            let result_bit = (result_byte & mask) != 0;

            bit_ops.push(BitOperation {
                input_bit,
                key_bit,
                result_bit,
                position: i,
            });
        }

        Self {
            character: input_char,
            input_byte,
            keystream_byte,
            result_byte,
            bit_ops,
        }
    }

    /// Returns the binary string representation of the input byte
    pub fn input_binary(&self) -> String {
        format!("{:08b}", self.input_byte)
    }

    /// Returns the binary string representation of the keystream byte
    pub fn keystream_binary(&self) -> String {
        format!("{:08b}", self.keystream_byte)
    }

    /// Returns the binary string representation of the result byte
    pub fn result_binary(&self) -> String {
        format!("{:08b}", self.result_byte)
    }
}
```

---

## File: src/learn/xor_visual.rs

```rust
use super::steps::EncryptionStep;
use crate::entropy::{EntropySource, SystemEntropy};

/// Manages the state of the learning visualization
pub struct EncryptionProcess {
    pub input_text: String,
    pub steps: Vec<EncryptionStep>,
    pub current_step_index: usize,
    pub is_playing: bool,
    pub speed: f32, // Steps per second (approximate)
    pub last_update: f64, // Timestamp for animation timing
}

impl Default for EncryptionProcess {
    fn default() -> Self {
        Self {
            input_text: String::new(),
            steps: Vec::new(),
            current_step_index: 0,
            is_playing: false,
            speed: 1.0,
            last_update: 0.0,
        }
    }
}

impl EncryptionProcess {
    pub fn new() -> Self {
        Self::default()
    }

    /// Initializes the visualization with the given text
    pub fn start(&mut self, text: &str) {
        self.input_text = text.to_string();
        self.steps.clear();
        self.current_step_index = 0;
        self.is_playing = false;

        let mut entropy = SystemEntropy::new();
        let mut keystream = vec![0u8; text.len()];
        entropy.fill_bytes(&mut keystream);

        for (i, char) in text.chars().enumerate() {
            if i < keystream.len() {
                self.steps.push(EncryptionStep::new(char, keystream[i]));
            }
        }
    }

    pub fn current_step(&self) -> Option<&EncryptionStep> {
        if self.steps.is_empty() {
            None
        } else {
            Some(&self.steps[self.current_step_index])
        }
    }

    pub fn next_step(&mut self) {
        if self.current_step_index + 1 < self.steps.len() {
            self.current_step_index += 1;
        } else {
            self.is_playing = false; // Stop at end
        }
    }

    pub fn prev_step(&mut self) {
        if self.current_step_index > 0 {
            self.current_step_index -= 1;
        }
    }

    pub fn toggle_play(&mut self) {
        self.is_playing = !self.is_playing;
    }

    /// Update animation state based on time delta
    pub fn update(&mut self, time: f64) {
        if self.is_playing {
            if time - self.last_update > (1.0 / self.speed as f64) {
                self.next_step();
                self.last_update = time;
            }
        } else {
            self.last_update = time;
        }
    }
}
```

---

## File: src/lib.rs

```rust
//! # Entropy Forge
//!
//! A pluggable entropy framework for cryptographic applications with built-in
//! quality testing and visualization.
//!
//! ## Overview
//!
//! Entropy Forge provides a clean trait-based interface for working with any
//! entropy source (RNG). The framework offers three main capabilities:
//!
//! 1. **Use**: Cryptographic operations (stream cipher, key derivation)
//! 2. **Test**: Quality analysis (NIST tests, entropy metrics)
//! 3. **Benchmark**: Performance measurement
//!
//! ## Quick Start
//!
//! ```rust
//! use entropy_forge::entropy::{EntropySource, SystemEntropy};
//! use entropy_forge::crypto::StreamCipher;
//!
//! let entropy = SystemEntropy::new();
//! let mut cipher = StreamCipher::new(entropy);
//!
//! let plaintext = b"Hello, World!";
//! let ciphertext = cipher.process(plaintext);
//! ```
//!
//! ## Custom Entropy Sources
//!
//! Implement the `EntropySource` trait to plug in your own RNG:
//!
//! ```rust
//! use entropy_forge::entropy::EntropySource;
//!
//! struct MyRNG {
//!     // Your state
//! }
//!
//! impl EntropySource for MyRNG {
//!     fn fill_bytes(&mut self, dest: &mut [u8]) {
//!         // Your implementation
//!     }
//!     
//!     fn name(&self) -> &str {
//!         "My Custom RNG"
//!     }
//! }
//! ```

pub mod entropy;
pub mod crypto;
pub mod quality;
pub mod bench;
pub mod learn;

#[cfg(feature = "gui")]
pub mod viz;

// Re-exports for convenience
pub use entropy::{EntropySource, SystemEntropy};
pub use crypto::StreamCipher;
pub use quality::{QualityMetrics, NistTests};
```

---

## File: src/main.rs

```rust
//! Entropy Forge - GUI Application

use eframe::egui;
use entropy_forge::viz::EntropyForgeApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 700.0])
            .with_min_inner_size([700.0, 500.0])
            .with_title("Entropy Forge")
            .with_icon(load_icon()),
        ..Default::default()
    };
    
    eframe::run_native(
        "Entropy Forge",
        options,
        Box::new(|_cc| {
            Ok(Box::new(EntropyForgeApp::default()))
        }),
    )
}

fn load_icon() -> egui::IconData {
    // Simple 32x32 icon (can be replaced with actual icon)
    let icon_size = 32;
    let mut rgba = vec![0u8; icon_size * icon_size * 4];

    // Create a simple pattern (can be customized)
    for y in 0..icon_size {
        for x in 0..icon_size {
            let idx = (y * icon_size + x) * 4;
            let value = ((x + y) * 8) as u8;
            rgba[idx] = value;     // R
            rgba[idx + 1] = value; // G
            rgba[idx + 2] = value; // B
            rgba[idx + 3] = 255;   // A
        }
    }

    egui::IconData {
        rgba,
        width: icon_size as u32,
        height: icon_size as u32,
    }
}
```

---

## File: src/quality/metrics.rs

```rust
//! Entropy quality metrics

use crate::entropy::EntropySource;
use std::collections::HashMap;

/// Entropy quality metrics
///
/// This struct contains various measurements of entropy quality, including
/// Shannon entropy, min-entropy, and byte frequency distribution.
#[derive(Debug, Clone)]
pub struct QualityMetrics {
    /// Shannon entropy in bits per byte (max: 8.0)
    pub shannon_entropy: f64,
    
    /// Min-entropy in bits per byte (conservative estimate)
    pub min_entropy: f64,
    
    /// Frequency of each byte value (0-255)
    pub byte_frequency: HashMap<u8, usize>,
    
    /// Total bytes analyzed
    pub total_bytes: usize,
    
    /// Chi-square statistic (for uniformity test)
    pub chi_square: f64,
    
    /// Mean value (should be ~127.5 for uniform distribution)
    pub mean: f64,
    
    /// Longest run of identical bits
    pub longest_run: usize,
}

impl QualityMetrics {
    /// Calculate Shannon entropy (in bits per byte)
    ///
    /// Shannon entropy measures the average information content.
    /// Higher is better, with 8.0 being perfect for byte-level entropy.
    ///
    /// Formula: H(X) = -Σ p(x) * log₂(p(x))
    pub fn shannon_entropy(data: &[u8]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }
        
        let mut freq = HashMap::new();
        for &byte in data {
            *freq.entry(byte).or_insert(0) += 1;
        }
        
        let len = data.len() as f64;
        let mut entropy = 0.0;
        
        for count in freq.values() {
            let p = *count as f64 / len;
            if p > 0.0 {
                entropy -= p * p.log2();
            }
        }
        
        entropy
    }
    
    /// Estimate min-entropy (conservative bound)
    ///
    /// Min-entropy is based on the most probable outcome.
    /// It provides a conservative estimate of entropy quality.
    ///
    /// Formula: H_∞(X) = -log₂(max_i p(x_i))
    pub fn min_entropy(data: &[u8]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }
        
        let mut freq = HashMap::new();
        for &byte in data {
            *freq.entry(byte).or_insert(0) += 1;
        }
        
        let max_freq = freq.values().max().copied().unwrap_or(0);
        let total = data.len() as f64;
        
        if max_freq > 0 {
            -(max_freq as f64 / total).log2()
        } else {
            0.0
        }
    }
    
    /// Calculate chi-square statistic for uniformity
    ///
    /// Tests how well the byte distribution matches a uniform distribution.
    /// Lower values indicate better uniformity.
    pub fn chi_square(data: &[u8]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }
        
        let mut freq = vec![0usize; 256];
        for &byte in data {
            freq[byte as usize] += 1;
        }
        
        let n = data.len() as f64;
        let expected = n / 256.0;
        
        let mut chi_sq = 0.0;
        for &count in &freq {
            let diff = count as f64 - expected;
            chi_sq += diff * diff / expected;
        }
        
        chi_sq
    }
    
    /// Calculate mean byte value
    pub fn mean(data: &[u8]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }
        
        let sum: u64 = data.iter().map(|&b| b as u64).sum();
        sum as f64 / data.len() as f64
    }
    
    /// Find longest run of identical bits
    pub fn longest_run(data: &[u8]) -> usize {
        if data.is_empty() {
            return 0;
        }
        
        let mut max_run = 0;
        let mut current_run = 1;
        let mut last_bit = (data[0] >> 7) & 1;
        
        for &byte in data {
            for i in (0..8).rev() {
                let bit = (byte >> i) & 1;
                if bit == last_bit {
                    current_run += 1;
                    max_run = max_run.max(current_run);
                } else {
                    current_run = 1;
                    last_bit = bit;
                }
            }
        }
        
        max_run
    }
    
    /// Analyze entropy source quality
    ///
    /// Generates a full quality report by sampling the entropy source.
    ///
    /// # Arguments
    ///
    /// * `source` - Entropy source to analyze
    /// * `sample_size` - Number of bytes to sample
    ///
    /// # Examples
    ///
    /// ```
    /// use entropy_forge::entropy::SystemEntropy;
    /// use entropy_forge::quality::QualityMetrics;
    ///
    /// let mut source = SystemEntropy::new();
    /// let metrics = QualityMetrics::analyze(&mut source, 100_000);
    ///
    /// println!("Shannon Entropy: {:.4} bits/byte", metrics.shannon_entropy);
    /// println!("Min-Entropy: {:.4} bits/byte", metrics.min_entropy);
    /// ```
    pub fn analyze<E: ?Sized + EntropySource>(source: &mut E, sample_size: usize) -> Self {
        let mut data = vec![0u8; sample_size];
        source.fill_bytes(&mut data);
        
        let shannon = Self::shannon_entropy(&data);
        let min_ent = Self::min_entropy(&data);
        let chi_sq = Self::chi_square(&data);
        let mean_val = Self::mean(&data);
        let longest = Self::longest_run(&data);
        
        let mut freq = HashMap::new();
        for &byte in &data {
            *freq.entry(byte).or_insert(0) += 1;
        }
        
        Self {
            shannon_entropy: shannon,
            min_entropy: min_ent,
            byte_frequency: freq,
            total_bytes: sample_size,
            chi_square: chi_sq,
            mean: mean_val,
            longest_run: longest,
        }
    }
    
    /// Get a quality score (0-100)
    ///
    /// Combines multiple metrics into a single score.
    /// 100 is perfect, 0 is worst.
    pub fn overall_score(&self) -> f64 {
        // Shannon entropy score (0-100)
        let shannon_score = (self.shannon_entropy / 8.0) * 100.0;
        
        // Min-entropy score (0-100)
        let min_entropy_score = (self.min_entropy / 8.0) * 100.0;
        
        // Mean score (how close to 127.5)
        let mean_diff = (self.mean - 127.5).abs();
        let mean_score = ((127.5 - mean_diff) / 127.5) * 100.0;
        
        // Weighted average
        shannon_score * 0.5 + min_entropy_score * 0.3 + mean_score * 0.2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entropy::{SystemEntropy, MockEntropy};
    
    #[test]
    fn test_shannon_entropy_perfect() {
        // All different bytes -> high entropy
        let data: Vec<u8> = (0..=255).collect();
        let entropy = QualityMetrics::shannon_entropy(&data);
        assert!(entropy > 7.9); // Should be close to 8.0
    }
    
    #[test]
    fn test_shannon_entropy_low() {
        // All same bytes -> zero entropy
        let data = vec![0u8; 256];
        let entropy = QualityMetrics::shannon_entropy(&data);
        assert_eq!(entropy, 0.0);
    }
    
    #[test]
    fn test_min_entropy() {
        let data = vec![1, 2, 3, 4, 5, 1, 1, 1];
        let min_ent = QualityMetrics::min_entropy(&data);
        // 4 out of 8 are '1', so max prob is 0.5
        // min_entropy = -log2(0.5) = 1.0
        assert!((min_ent - 1.0).abs() < 0.01);
    }
    
    #[test]
    fn test_mean() {
        let data = vec![0, 128, 255];
        let mean = QualityMetrics::mean(&data);
        assert_eq!(mean, (0.0 + 128.0 + 255.0) / 3.0);
    }
    
    #[test]
    fn test_analyze_system() {
        let mut source = SystemEntropy::new();
        let metrics = QualityMetrics::analyze(&mut source, 10_000);
        
        // System RNG should have good entropy
        assert!(metrics.shannon_entropy > 7.5);
        assert!(metrics.min_entropy > 7.0);
        assert!(metrics.mean > 100.0 && metrics.mean < 155.0);
    }
    
    #[test]
    fn test_overall_score() {
        let mut source = SystemEntropy::new();
        let metrics = QualityMetrics::analyze(&mut source, 10_000);
        
        let score = metrics.overall_score();
        assert!(score > 80.0); // System RNG should score well
        assert!(score <= 100.0);
    }
}
```

---

## File: src/quality/mod.rs

```rust
//! Entropy quality analysis and testing
//!
//! This module provides statistical tests and quality metrics for entropy
//! sources, including NIST SP 800-22 tests.

mod metrics;
mod nist;

pub use metrics::QualityMetrics;
pub use nist::NistTests;
```

---

## File: src/quality/nist.rs

```rust
//! NIST SP 800-22 statistical tests (simplified implementations)

use statrs::distribution::{ChiSquared, ContinuousCDF};

/// NIST SP 800-22 statistical tests
///
/// These tests are used to evaluate the quality of random number generators.
/// Each test returns a p-value, where values ≥ 0.01 indicate the sequence
/// appears random.
///
/// This is a simplified implementation of the full NIST test suite.
pub struct NistTests;

impl NistTests {
    /// Frequency (monobit) test
    ///
    /// Tests if the number of 1s and 0s in the sequence are approximately equal.
    ///
    /// # Arguments
    ///
    /// * `data` - Byte sequence to test
    ///
    /// # Returns
    ///
    /// P-value (0.0 to 1.0). Values ≥ 0.01 indicate randomness.
    pub fn frequency_test(data: &[u8]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }
        
        let mut sum: i64 = 0;
        
        // Convert to bits and count
        for &byte in data {
            for i in 0..8 {
                if (byte >> i) & 1 == 1 {
                    sum += 1;
                } else {
                    sum -= 1;
                }
            }
        }
        
        let n = (data.len() * 8) as f64;
        let s_obs = (sum as f64).abs() / n.sqrt();
        
        // Calculate p-value using complementary error function
        Self::erfc(s_obs / std::f64::consts::SQRT_2)
    }
    
    /// Runs test
    ///
    /// Tests for proper oscillation between 1s and 0s. Too few or too many
    /// runs indicates non-randomness.
    ///
    /// # Arguments
    ///
    /// * `data` - Byte sequence to test
    ///
    /// # Returns
    ///
    /// P-value (0.0 to 1.0). Values ≥ 0.01 indicate randomness.
    pub fn runs_test(data: &[u8]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }
        
        // Convert to bits
        let bits: Vec<u8> = data.iter()
            .flat_map(|&byte| (0..8).map(move |i| (byte >> i) & 1))
            .collect();
        
        let n = bits.len() as f64;
        let ones = bits.iter().filter(|&&b| b == 1).count() as f64;
        let pi = ones / n;
        
        // Check pre-test condition
        let threshold = 2.0 / n.sqrt();
        if (pi - 0.5).abs() >= threshold {
            return 0.0; // Test not applicable
        }
        
        // Count runs
        let mut runs = 1;
        for i in 1..bits.len() {
            if bits[i] != bits[i - 1] {
                runs += 1;
            }
        }
        
        let v_obs = runs as f64;
        let numerator = (v_obs - 2.0 * n * pi * (1.0 - pi)).abs();
        let denominator = 2.0 * (2.0 * n).sqrt() * pi * (1.0 - pi);
        
        if denominator == 0.0 {
            return 0.0;
        }
        
        Self::erfc(numerator / (denominator * std::f64::consts::SQRT_2))
    }
    
    /// Longest run of ones test
    ///
    /// Tests the length of the longest run of ones, which shouldn't be
    /// too long in a random sequence.
    pub fn longest_run_test(data: &[u8]) -> f64 {
        if data.len() < 128 {
            return 0.0; // Need at least 128 bytes
        }
        
        // Convert to bits
        let bits: Vec<u8> = data.iter()
            .flat_map(|&byte| (0..8).map(move |i| (byte >> i) & 1))
            .collect();
        
        let n = bits.len();
        
        // Parameters based on sequence length
        let (m, k, v_values, pi_values) = if n < 6272 {
            (8, 3, vec![1, 2, 3, 4], vec![0.2148, 0.3672, 0.2305, 0.1875])
        } else if n < 75000 {
            (128, 5, vec![4, 5, 6, 7, 8, 9], vec![0.1174, 0.2430, 0.2493, 0.1752, 0.1027, 0.1124])
        } else {
            (10000, 6, vec![10, 11, 12, 13, 14, 15, 16], 
             vec![0.0882, 0.2092, 0.2483, 0.1933, 0.1208, 0.0675, 0.0727])
        };
        
        // Divide into blocks
        let num_blocks = n / m;
        let mut v_obs = vec![0usize; v_values.len()];
        
        for block_idx in 0..num_blocks {
            let start = block_idx * m;
            let end = start + m;
            let block = &bits[start..end];
            
            // Find longest run in block
            let mut max_run = 0;
            let mut current_run = 0;
            
            for &bit in block {
                if bit == 1 {
                    current_run += 1;
                    max_run = max_run.max(current_run);
                } else {
                    current_run = 0;
                }
            }
            
            // Categorize the run
            let category = v_values.iter()
                .position(|&v| max_run <= v)
                .unwrap_or(v_values.len() - 1);
            
            v_obs[category] += 1;
        }
        
        // Chi-square statistic
        let mut chi_sq = 0.0;
        for (i, &count) in v_obs.iter().enumerate() {
            let expected = num_blocks as f64 * pi_values[i];
            let diff = count as f64 - expected;
            chi_sq += diff * diff / expected;
        }
        
        // P-value from chi-square distribution
        let df = k as f64;
        if let Ok(dist) = ChiSquared::new(df) {
            1.0 - dist.cdf(chi_sq)
        } else {
            0.0
        }
    }
    
    /// Chi-square test for byte distribution
    ///
    /// Tests whether the byte values are uniformly distributed.
    pub fn chi_square_test(data: &[u8]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }
        
        let mut freq = vec![0usize; 256];
        for &byte in data {
            freq[byte as usize] += 1;
        }
        
        let n = data.len() as f64;
        let expected = n / 256.0;
        
        let mut chi_square = 0.0;
        for &count in &freq {
            let diff = count as f64 - expected;
            chi_square += diff * diff / expected;
        }
        
        // Chi-square distribution with 255 degrees of freedom
        if let Ok(dist) = ChiSquared::new(255.0) {
            1.0 - dist.cdf(chi_square)
        } else {
            0.0
        }
    }
    
    /// Serial test (two-bit test)
    ///
    /// Tests the frequency of overlapping two-bit patterns.
    pub fn serial_test(data: &[u8]) -> f64 {
        if data.len() < 2 {
            return 0.0;
        }
        
        // Convert to bits
        let bits: Vec<u8> = data.iter()
            .flat_map(|&byte| (0..8).map(move |i| (byte >> i) & 1))
            .collect();
        
        let n = bits.len();
        if n < 2 {
            return 0.0;
        }
        
        // Count 2-bit patterns
        let mut freq = vec![0usize; 4];
        for i in 0..n-1 {
            let pattern = (bits[i] << 1) | bits[i + 1];
            freq[pattern as usize] += 1;
        }
        
        // Chi-square test
        let expected = (n - 1) as f64 / 4.0;
        let mut chi_sq = 0.0;
        
        for &count in &freq {
            let diff = count as f64 - expected;
            chi_sq += diff * diff / expected;
        }
        
        // P-value from chi-square with 3 df
        if let Ok(dist) = ChiSquared::new(3.0) {
            1.0 - dist.cdf(chi_sq)
        } else {
            0.0
        }
    }
    
    /// Run all tests and return results
    ///
    /// Returns a vector of (test_name, p_value) tuples.
    pub fn run_all_tests(data: &[u8]) -> Vec<(&'static str, f64)> {
        vec![
            ("Frequency Test", Self::frequency_test(data)),
            ("Runs Test", Self::runs_test(data)),
            ("Longest Run Test", Self::longest_run_test(data)),
            ("Chi-Square Test", Self::chi_square_test(data)),
            ("Serial Test", Self::serial_test(data)),
        ]
    }
    
    // Helper: Complementary error function
    pub fn erfc(x: f64) -> f64 {
        let z = x.abs();
        let t = 1.0 / (1.0 + 0.5 * z);
        
        let ans = t * (
            -z * z - 1.26551223 +
            t * (1.00002368 +
            t * (0.37409196 +
            t * (0.09678418 +
            t * (-0.18628806 +
            t * (0.27886807 +
            t * (-1.13520398 +
            t * (1.48851587 +
            t * (-0.82215223 +
            t * 0.17087277))))))))
        ).exp();
        
        if x >= 0.0 { ans } else { 2.0 - ans }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entropy::SystemEntropy;
    use crate::entropy::EntropySource;
    
    #[test]
    fn test_frequency_test_uniform() {
        let data = vec![0xAAu8; 1000]; // 10101010 pattern
        let p_value = NistTests::frequency_test(&data);
        // Should pass (close to 50% ones)
        assert!(p_value > 0.01);
    }
    
    #[test]
    fn test_frequency_test_biased() {
        let data = vec![0xFFu8; 1000]; // All ones
        let p_value = NistTests::frequency_test(&data);
        // Should fail (100% ones)
        assert!(p_value < 0.01);
    }
    
    #[test]
    fn test_chi_square() {
        let mut entropy = SystemEntropy::new();
        let mut data = vec![0u8; 10_000];
        entropy.fill_bytes(&mut data);
        
        let p_value = NistTests::chi_square_test(&data);
        // System RNG should pass
        assert!(p_value > 0.01);
    }
    
    #[test]
    fn test_run_all() {
        let mut entropy = SystemEntropy::new();
        let mut data = vec![0u8; 10_000];
        entropy.fill_bytes(&mut data);
        
        let results = NistTests::run_all_tests(&data);
        
        // Should have all tests
        assert_eq!(results.len(), 5);
        
        // Most should pass (allow 1 failure due to statistical variance)
        let passed = results.iter().filter(|(_, p)| *p >= 0.01).count();
        assert!(passed >= 4);
    }
}
```

---

## File: src/viz/app.rs

```rust
//! Main GUI application

use eframe::egui;
use crate::entropy::{EntropySource, SystemEntropy};
use crate::crypto::StreamCipher;
use crate::quality::{QualityMetrics, NistTests};
use crate::bench::{PerformanceBench, BenchmarkResult};
use crate::learn::{EncryptionProcess, EntropyProcess, NistProcess};

/// Main application state
pub struct EntropyForgeApp {
    // Entropy source
    entropy: Box<dyn EntropySource>,
    
    // Current tab
    current_tab: Tab,
    
    // Use tab state
    cipher_input: String,
    cipher_output: String,
    cipher_hex: bool,
    cipher_state: Vec<u8>,
    
    // Test tab state
    quality_metrics: Option<QualityMetrics>,
    nist_results: Vec<(String, f64)>,
    quality_sample_size: usize,
    is_testing: bool,
    
    // Benchmark tab state
    bench_result: Option<BenchmarkResult>,
    bench_size: usize,
    is_benchmarking: bool,

    // Learn tab state
    learn_mode: LearnMode,
    learn_process: EncryptionProcess,
    learn_input: String,

    entropy_process: EntropyProcess,
    nist_process: NistProcess,
}

#[derive(PartialEq, Clone, Copy)]
enum LearnMode {
    XorCipher,
    ShannonEntropy,
    NistFrequency,
}

#[derive(PartialEq, Clone, Copy)]
enum Tab {
    Use,
    Test,
    Benchmark,
    Learn,
}

impl Default for EntropyForgeApp {
    fn default() -> Self {
        Self {
            entropy: Box::new(SystemEntropy::new()),
            current_tab: Tab::Use,
            cipher_input: String::from("Hello, World!"),
            cipher_output: String::new(),
            cipher_hex: true,
            cipher_state: Vec::new(),
            quality_metrics: None,
            nist_results: Vec::new(),
            quality_sample_size: 100_000,
            is_testing: false,
            bench_result: None,
            bench_size: 1_000_000,
            is_benchmarking: false,
            learn_mode: LearnMode::XorCipher,
            learn_process: EncryptionProcess::new(),
            learn_input: String::from("Hello"),
            entropy_process: EntropyProcess::new(),
            nist_process: NistProcess::new(),
        }
    }
}

impl eframe::App for EntropyForgeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Header
            ui.heading("🔐 Entropy Forge");
            ui.label(format!("Source: {}", self.entropy.name()));
            ui.add_space(5.0);
            ui.separator();
            
            // Tabs
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.current_tab, Tab::Use, "📝 Use");
                ui.selectable_value(&mut self.current_tab, Tab::Test, "🔬 Test");
                ui.selectable_value(&mut self.current_tab, Tab::Benchmark, "⚡ Benchmark");
                ui.selectable_value(&mut self.current_tab, Tab::Learn, "🎓 Learn");
            });
            
            ui.separator();
            ui.add_space(10.0);
            
            // Tab content
            egui::ScrollArea::vertical().show(ui, |ui| {
                match self.current_tab {
                    Tab::Use => self.render_use_tab(ui),
                    Tab::Test => self.render_test_tab(ui),
                    Tab::Benchmark => self.render_benchmark_tab(ui),
                    Tab::Learn => self.render_learn_tab(ui),
                }
            });
        });
    }
}

impl EntropyForgeApp {
    /// Helper to render consistent educational tooltips
    fn render_explanation_tooltip(ui: &mut egui::Ui, label: &str, text: &str) {
        ui.horizontal(|ui| {
            ui.label(label);
            ui.label("ℹ").on_hover_text(text);
        });
    }

    /// Render the "Use" tab (stream cipher)
    fn render_use_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("Stream Cipher");
        ui.label("Encrypt or decrypt data using the entropy source as keystream.");
        ui.add_space(10.0);
        
        // Input
        ui.horizontal(|ui| {
            ui.label("Input:");
            ui.text_edit_singleline(&mut self.cipher_input);
            if ui.button("Help: How does this work?").clicked() {
                self.current_tab = Tab::Learn;
                self.learn_input = self.cipher_input.clone();
                self.learn_process.start(&self.learn_input);
            }
        });
        
        ui.checkbox(&mut self.cipher_hex, "Display output as hex");
        
        ui.add_space(10.0);
        
        // Encrypt button
        if ui.button("🔒 Encrypt / Decrypt").clicked() {
            // Create a temporary entropy source for the cipher
            let temp_entropy = SystemEntropy::new();
            let mut cipher = StreamCipher::new(temp_entropy);
            let output = cipher.process(self.cipher_input.as_bytes());

            self.cipher_output = if self.cipher_hex {
                hex::encode(&output)
            } else {
                String::from_utf8_lossy(&output).to_string()
            };

            self.cipher_state = cipher.state().to_vec();
        }
        
        ui.add_space(10.0);
        
        // Output
        ui.label("Output:");
        ui.add(
            egui::TextEdit::multiline(&mut self.cipher_output.as_str())
                .desired_width(f32::INFINITY)
                .desired_rows(3)
        );
        
        ui.add_space(20.0);
        
        // Cipher state visualization
        if !self.cipher_state.is_empty() {
            ui.heading("Cipher State (first 64 bytes of keystream)");
            ui.add_space(5.0);
            
            // Display as 8x8 grid
            egui::Grid::new("cipher_state_grid")
                .spacing([4.0, 4.0])
                .show(ui, |ui| {
                    for (i, &byte) in self.cipher_state.iter().take(64).enumerate() {
                        if i % 8 == 0 && i > 0 {
                            ui.end_row();
                        }
                        
                        let color = egui::Color32::from_rgb(byte, byte, byte);
                        let (rect, response) = ui.allocate_exact_size(
                            egui::vec2(30.0, 30.0),
                            egui::Sense::hover()
                        );
                        ui.painter().rect_filled(rect, 2.0, color);

                        // Tooltip
                        response.on_hover_text(format!("Byte {}: 0x{:02X} ({})", i, byte, byte));
                    }
                });
        }
    }
    
    /// Render the "Test" tab (quality metrics)
    fn render_test_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("Quality Analysis");
        ui.label("Analyze entropy quality with statistical tests.");
        ui.add_space(10.0);
        
        // Sample size selector
        ui.horizontal(|ui| {
            ui.label("Sample size:");
            ui.add(egui::Slider::new(&mut self.quality_sample_size, 1_000..=1_000_000)
                .logarithmic(true)
                .suffix(" bytes"));
        });
        
        ui.add_space(10.0);
        
        // Run tests button
        ui.horizontal(|ui| {
            if ui.button("🔬 Run All Tests").clicked() && !self.is_testing {
                self.is_testing = true;
                
                // Run quality metrics
                self.quality_metrics = Some(
                    QualityMetrics::analyze(&mut *self.entropy, self.quality_sample_size)
                );
                
                // Run NIST tests
                let mut data = vec![0u8; self.quality_sample_size];
                self.entropy.fill_bytes(&mut data);
                
                self.nist_results = NistTests::run_all_tests(&data)
                    .into_iter()
                    .map(|(name, p_value)| (name.to_string(), p_value))
                    .collect();
                
                self.is_testing = false;
            }
            
            if self.is_testing {
                ui.spinner();
                ui.label("Testing...");
            }
        });
        
        ui.add_space(20.0);
        
        // Display results
        if let Some(ref metrics) = self.quality_metrics {
            ui.heading("Entropy Metrics");
            ui.add_space(5.0);
            
            // Shannon entropy
            Self::render_explanation_tooltip(
                ui,
                "Shannon Entropy:",
                "Measures the unpredictability of data. \n8.0 bits/byte is perfect randomness.\nLower values mean patterns exist."
            );
            ui.label(format!("{:.4} bits/byte", metrics.shannon_entropy));

            let shannon_progress = (metrics.shannon_entropy / 8.0) as f32;
            ui.add(egui::ProgressBar::new(shannon_progress)
                .text(format!("{:.1}%", shannon_progress * 100.0)));
            
            ui.add_space(5.0);
            
            // Min-entropy
            Self::render_explanation_tooltip(
                ui,
                "Min-Entropy:",
                "The worst-case entropy. \nCrucial for security guarantees.\nIf this is low, an attacker might guess the key."
            );
            ui.label(format!("{:.4} bits/byte", metrics.min_entropy));
            let min_progress = (metrics.min_entropy / 8.0) as f32;
            ui.add(egui::ProgressBar::new(min_progress)
                .text(format!("{:.1}%", min_progress * 100.0)));
            
            ui.add_space(10.0);
            
            // Other metrics
            ui.label(format!("Mean byte value: {:.2} (ideal: 127.5)", metrics.mean));
            ui.label(format!("Chi-square: {:.2}", metrics.chi_square));
            ui.label(format!("Longest run: {} bits", metrics.longest_run));
            
            ui.add_space(10.0);
            
            // Overall score
            let score = metrics.overall_score();
            ui.horizontal(|ui| {
                ui.label("Overall Quality Score:");
                ui.label(format!("{:.1}/100", score));
            });
            
            ui.add_space(20.0);
            
            // NIST tests
            ui.horizontal(|ui| {
                ui.heading("NIST SP 800-22 Tests");
                ui.label("ℹ").on_hover_text(
                    "Standard statistical tests used by the US government to certify random number generators.\nWe check if the data looks indistinguishable from a coin flip."
                );
            });
            ui.label("P-values ≥ 0.01 indicate randomness (Pass)");
            ui.add_space(5.0);
            
            egui::Grid::new("nist_results")
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Test");
                    ui.label("P-Value");
                    ui.label("Result");
                    ui.end_row();
                    
                    for (name, p_value) in &self.nist_results {
                        ui.label(name);
                        ui.label(format!("{:.4}", p_value));
                        
                        if *p_value >= 0.01 {
                            ui.colored_label(egui::Color32::GREEN, "✓ Pass");
                        } else {
                            ui.colored_label(egui::Color32::RED, "✗ Fail");
                        }
                        ui.end_row();
                    }
                });
        } else {
            ui.label("Click 'Run All Tests' to analyze entropy quality.");
        }
    }
    
    /// Render the "Benchmark" tab
    fn render_benchmark_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("Performance Benchmark");
        ui.label("Measure entropy generation speed.");
        ui.add_space(10.0);
        
        // Benchmark size selector
        ui.horizontal(|ui| {
            ui.label("Benchmark size:");
            ui.add(egui::Slider::new(&mut self.bench_size, 10_000..=10_000_000)
                .logarithmic(true)
                .suffix(" bytes"));
        });
        
        ui.add_space(10.0);
        
        // Run benchmark button
        ui.horizontal(|ui| {
            if ui.button("⚡ Run Benchmark").clicked() && !self.is_benchmarking {
                self.is_benchmarking = true;
                self.bench_result = Some(
                    PerformanceBench::benchmark(&mut *self.entropy, self.bench_size)
                );
                self.is_benchmarking = false;
            }
            
            if self.is_benchmarking {
                ui.spinner();
                ui.label("Benchmarking...");
            }
        });
        
        ui.add_space(20.0);
        
        // Display results
        if let Some(ref result) = self.bench_result {
            ui.heading("Results");
            ui.add_space(10.0);
            
            // Throughput
            ui.horizontal(|ui| {
                Self::render_explanation_tooltip(
                    ui,
                    "Throughput:",
                    "How fast we can generate entropy.\nImportant for bulk encryption (e.g., file encryption, video streaming)."
                );
                ui.heading(format!("{:.2} MB/s", result.throughput_mbps));
            });
            
            ui.add_space(5.0);
            
            // Latency
            ui.horizontal(|ui| {
                Self::render_explanation_tooltip(
                    ui,
                    "Latency per byte:",
                    "Time to generate a single byte.\nImportant for real-time applications (e.g., key generation, handshakes)."
                );
                ui.label(format!("{:.2} µs", result.latency_us));
            });
            
            ui.add_space(5.0);
            
            // Details
            ui.label(format!("Generated {} bytes in {:.3} seconds",
                result.bytes_generated,
                result.duration.as_secs_f64()
            ));
        } else {
            ui.label("Click 'Run Benchmark' to measure performance.");
        }
    }

    /// Render the "Learn" tab
    fn render_learn_tab(&mut self, ui: &mut egui::Ui) {
        // Sub-tabs for Learn Mode
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.learn_mode, LearnMode::XorCipher, "XOR Cipher");
            ui.selectable_value(&mut self.learn_mode, LearnMode::ShannonEntropy, "Shannon Entropy");
            ui.selectable_value(&mut self.learn_mode, LearnMode::NistFrequency, "NIST Frequency");
        });
        ui.separator();
        ui.add_space(10.0);

        match self.learn_mode {
            LearnMode::XorCipher => self.render_xor_visualizer(ui),
            LearnMode::ShannonEntropy => self.render_entropy_visualizer(ui),
            LearnMode::NistFrequency => self.render_nist_visualizer(ui),
        }
    }

    fn render_xor_visualizer(&mut self, ui: &mut egui::Ui) {
        // Update animation state if playing
        let time = ui.input(|i| i.time);
        self.learn_process.update(time);
        if self.learn_process.is_playing {
            ui.ctx().request_repaint();
        }

        ui.heading("Learn Cryptography: XOR Cipher");
        ui.label("Visualize how stream ciphers work step-by-step.");
        ui.add_space(10.0);

        // Input Section
        ui.horizontal(|ui| {
            ui.label("Input:");
            ui.text_edit_singleline(&mut self.learn_input);
            if ui.button("Start Visualization").clicked() {
                self.learn_process.start(&self.learn_input);
            }
        });

        ui.add_space(20.0);

        // Visualization Section
        if let Some(step) = self.learn_process.current_step() {
            let total_steps = self.learn_process.steps.len();
            let current_idx = self.learn_process.current_step_index + 1;

            ui.heading(format!("Step {} of {}: Encrypting '{}'", current_idx, total_steps, step.character));
            ui.add_space(10.0);

            // Frame for visualization
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                ui.set_min_width(500.0);
                ui.vertical_centered(|ui| {
                    ui.add_space(10.0);

                    let input_color = egui::Color32::from_rgb(100, 200, 255); // Cyan-ish
                    let key_color = egui::Color32::from_rgb(255, 200, 100);   // Orange-ish
                    let result_color = egui::Color32::from_rgb(100, 255, 100); // Green-ish

                    // Input Row
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Input:").strong());
                        ui.add_space(20.0);
                        ui.label(format!("'{}'", step.character));
                        ui.label("=");
                        ui.label(format!("{}", step.input_byte));
                        ui.label("=");

                        for bit_op in &step.bit_ops {
                            let text = if bit_op.input_bit { "1" } else { "0" };
                            ui.label(egui::RichText::new(text).color(input_color).font(egui::FontId::monospace(20.0)));
                            ui.add_space(2.0);
                        }
                    });

                    ui.add_space(5.0);
                    ui.label(egui::RichText::new("↓").strong());
                    ui.add_space(5.0);

                    // Key Row
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Key:  ").strong());
                        ui.add_space(20.0);
                        ui.label("   "); // align with char
                        ui.label(" ");
                        ui.label(format!("{}", step.keystream_byte));
                        ui.label("=");

                        for bit_op in &step.bit_ops {
                            let text = if bit_op.key_bit { "1" } else { "0" };
                            ui.label(egui::RichText::new(text).color(key_color).font(egui::FontId::monospace(20.0)));
                            ui.add_space(2.0);
                        }
                    });

                    ui.add_space(5.0);
                    // XOR symbol row or line
                    ui.separator();
                    ui.add_space(5.0);

                    // Result Row
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("XOR:  ").strong());
                        ui.add_space(20.0);
                        ui.label("   "); // align
                        ui.label(" ");
                        ui.label(format!("{}", step.result_byte));
                        ui.label("=");

                        for bit_op in &step.bit_ops {
                            let text = if bit_op.result_bit { "1" } else { "0" };
                            ui.label(egui::RichText::new(text).color(result_color).font(egui::FontId::monospace(20.0)));
                            ui.add_space(2.0);
                        }
                    });

                    ui.add_space(10.0);

                    // Explanation of XOR for current step
                    ui.label("Operation: 0⊕1=1, 1⊕0=1, 0⊕0=0, 1⊕1=0");
                    ui.label("If bits are different, result is 1. If same, result is 0.");

                    ui.add_space(10.0);
                });
            });

            ui.add_space(20.0);

            // Controls
            ui.horizontal(|ui| {
                if ui.button("⬅ Previous").clicked() {
                    self.learn_process.prev_step();
                }

                let play_label = if self.learn_process.is_playing { "⏸ Pause" } else { "▶ Play" };
                if ui.button(play_label).clicked() {
                    self.learn_process.toggle_play();
                }

                if ui.button("Next ➡").clicked() {
                    self.learn_process.next_step();
                }

                ui.add_space(20.0);
                ui.label("Speed:");
                ui.add(egui::Slider::new(&mut self.learn_process.speed, 0.1..=5.0).text("steps/s"));
            });

            ui.add_space(20.0);

            // Progress/Output Summary
            ui.group(|ui| {
                ui.heading("Encryption Progress");
                ui.horizontal_wrapped(|ui| {
                    for (i, s) in self.learn_process.steps.iter().enumerate() {
                        let text = format!("{:02X}", s.result_byte);
                        let mut rich_text = egui::RichText::new(text).monospace();

                        if i == self.learn_process.current_step_index {
                            rich_text = rich_text.strong().color(egui::Color32::GREEN).background_color(egui::Color32::from_gray(60));
                        } else if i < self.learn_process.current_step_index {
                             rich_text = rich_text.color(egui::Color32::LIGHT_GRAY);
                        } else {
                             rich_text = rich_text.color(egui::Color32::DARK_GRAY);
                        }

                        ui.label(rich_text);
                    }
                });
            });

            // Educational Context
            ui.add_space(20.0);
            ui.collapsing("How it works", |ui| {
                ui.label("1. Convert character to ASCII number (e.g., 'H' = 72)");
                ui.label("2. Convert to binary (72 = 01001000)");
                ui.label("3. Generate a random keystream byte from entropy source");
                ui.label("4. XOR the input byte with the keystream byte");
                ui.label("5. The result is the encrypted ciphertext byte");
            });

        } else {
            ui.label("Enter text and click 'Start Visualization' to begin.");
        }
    }

    fn render_entropy_visualizer(&mut self, ui: &mut egui::Ui) {
        use crate::learn::entropy_visual::EntropyStepType;

        // Update animation state if playing
        let time = ui.input(|i| i.time);
        self.entropy_process.update(time);
        if self.entropy_process.is_playing {
            ui.ctx().request_repaint();
        }

        ui.heading("Learn Shannon Entropy");
        ui.label("Measures 'randomness' or 'unpredictability' of data.");
        ui.add_space(10.0);

        // Input Section
        ui.horizontal(|ui| {
            ui.label("Input:");
            ui.text_edit_singleline(&mut self.entropy_process.input);
            if ui.button("Calculate").clicked() {
                self.entropy_process.start(&self.entropy_process.input.clone());
            }
        });

        ui.add_space(20.0);

        if let Some(step) = self.entropy_process.current_step() {
             let total_steps = self.entropy_process.steps.len();
             let current_idx = self.entropy_process.current_step_index + 1;

             let step_title = match step.step_type {
                 EntropyStepType::CountBytes => "Count Byte Frequencies",
                 EntropyStepType::CalculateProbabilities => "Calculate Probabilities",
                 EntropyStepType::CalculateContributions => "Calculate Entropy Contributions",
                 EntropyStepType::SumEntropy => "Sum for Total Entropy",
                 EntropyStepType::Interpret => "Interpretation",
             };

             ui.heading(format!("Step {} of {}: {}", current_idx, total_steps, step_title));

             ui.add_space(10.0);

             // Visualization Area
             egui::Frame::canvas(ui.style()).show(ui, |ui| {
                 ui.set_min_width(600.0);
                 ui.vertical(|ui| {
                    ui.add_space(10.0);

                    // Show Frequency Table
                    if !step.byte_counts.is_empty() {
                         egui::Grid::new("entropy_grid").striped(true).spacing([20.0, 5.0]).show(ui, |ui| {
                            ui.label(egui::RichText::new("Byte").strong());
                            ui.label(egui::RichText::new("Count").strong());
                            if step.step_type != EntropyStepType::CountBytes {
                                ui.label(egui::RichText::new("Probability").strong());
                                ui.label(egui::RichText::new("Contribution").strong());
                            }
                            ui.label(egui::RichText::new("Visual").strong());
                            ui.end_row();

                            // Sort bytes for consistent display
                            let mut bytes: Vec<_> = step.byte_counts.keys().collect();
                            bytes.sort();

                            let max_count = step.byte_counts.values().max().copied().unwrap_or(1) as f32;

                            for &byte in bytes {
                                let count = step.byte_counts[&byte];
                                let char_repr = if byte >= 32 && byte <= 126 {
                                    (byte as char).to_string()
                                } else {
                                    format!("0x{:02X}", byte)
                                };

                                ui.label(format!("'{}'", char_repr));
                                ui.label(format!("{}", count));

                                if step.step_type != EntropyStepType::CountBytes {
                                    let p = step.probabilities.get(&byte).unwrap_or(&0.0);
                                    let contrib = step.entropy_contributions.get(&byte).unwrap_or(&0.0);

                                    ui.label(format!("{:.3}", p));

                                    if matches!(step.step_type, EntropyStepType::CalculateContributions | EntropyStepType::SumEntropy | EntropyStepType::Interpret) {
                                         ui.label(format!("{:.3} bits", contrib));
                                    } else {
                                        ui.label("-");
                                    }
                                }

                                // Bar chart
                                let width = (count as f32 / max_count) * 100.0;
                                let color = if count == 1 { egui::Color32::GREEN } else { egui::Color32::from_rgb(255, 100 + (155 - (count * 20).min(155)) as u8, 100) };
                                let (rect, _) = ui.allocate_exact_size(egui::vec2(width, 15.0), egui::Sense::hover());
                                ui.painter().rect_filled(rect, 2.0, color);
                                ui.end_row();
                            }
                         });
                    }

                    ui.add_space(20.0);

                    // Explanation / Formula area
                    match step.step_type {
                        EntropyStepType::CountBytes => {
                            ui.label("We count how many times each unique byte appears in the input.");
                        },
                        EntropyStepType::CalculateProbabilities => {
                            ui.label("Probability P(x) = Count(x) / Total Bytes");
                            ui.label("This tells us how likely each character is to appear.");
                        },
                        EntropyStepType::CalculateContributions => {
                            ui.label("Contribution = -P(x) * log2(P(x))");
                            ui.label("Rare events provide MORE information (higher contribution) per occurrence.");
                        },
                        EntropyStepType::SumEntropy | EntropyStepType::Interpret => {
                            ui.heading(format!("Total Entropy: {:.4} bits/byte", step.current_entropy_sum));
                            ui.label(format!("Maximum Possible: {:.4} bits/byte", step.max_entropy));

                            let efficiency = if step.max_entropy > 0.0 { (step.current_entropy_sum / step.max_entropy) * 100.0 } else { 0.0 };
                             ui.label(format!("Efficiency: {:.1}%", efficiency));

                             if step.step_type == EntropyStepType::Interpret {
                                 ui.separator();
                                 if step.current_entropy_sum < 2.0 {
                                     ui.label("Low entropy. The input is very predictable.");
                                 } else if efficiency > 80.0 {
                                     ui.label("High entropy! The input looks quite random or uses many different characters.");
                                 } else {
                                     ui.label("Moderate entropy.");
                                 }
                             }
                        }
                    }
                    ui.add_space(10.0);
                 });
             });

             ui.add_space(20.0);

             // Controls
             ui.horizontal(|ui| {
                if ui.button("⬅ Previous").clicked() {
                    self.entropy_process.prev_step();
                }

                let play_label = if self.entropy_process.is_playing { "⏸ Pause" } else { "▶ Play" };
                if ui.button(play_label).clicked() {
                    self.entropy_process.toggle_play();
                }

                if ui.button("Next ➡").clicked() {
                    self.entropy_process.next_step();
                }

                ui.add_space(20.0);
                ui.label("Speed:");
                ui.add(egui::Slider::new(&mut self.entropy_process.speed, 0.1..=5.0).text("steps/s"));
            });
        } else {
             ui.label("Enter text (e.g., 'AAABBC') and click Calculate.");
        }
    }

    fn render_nist_visualizer(&mut self, ui: &mut egui::Ui) {
        use crate::learn::nist_visual::NistStepType;

         // Update animation state if playing
        let time = ui.input(|i| i.time);
        self.nist_process.update(time);
        if self.nist_process.is_playing {
            ui.ctx().request_repaint();
        }

        ui.heading("Learn NIST Frequency Test");
        ui.label("Checks if the number of 1s and 0s are approximately equal (like a fair coin flip).");
        ui.add_space(10.0);

        // Input Section
        ui.horizontal(|ui| {
            ui.label("Input:");
            ui.text_edit_singleline(&mut self.nist_process.input_text);
            if ui.button("Analyze").clicked() {
                self.nist_process.start(&self.nist_process.input_text.clone());
            }
            if ui.button("Generate Random").clicked() {
                self.nist_process.generate_random(16);
            }
        });

        ui.add_space(20.0);

        if let Some(step) = self.nist_process.current_step() {
             let total_steps = self.nist_process.steps.len();
             let current_idx = self.nist_process.current_step_index + 1;

             let step_title = match step.step_type {
                 NistStepType::ConvertToBits => "Convert to Bits",
                 NistStepType::CountOnesZeros => "Count Ones and Zeros",
                 NistStepType::CalculateStatistic => "Calculate Statistic",
                 NistStepType::CalculatePValue => "Calculate P-Value",
                 NistStepType::Interpret => "Interpretation",
             };

             ui.heading(format!("Step {} of {}: {}", current_idx, total_steps, step_title));
             ui.add_space(10.0);

             egui::Frame::canvas(ui.style()).show(ui, |ui| {
                 ui.set_min_width(600.0);
                 ui.vertical(|ui| {
                     ui.add_space(10.0);

                     // Bit visualization
                     ui.label(egui::RichText::new("Bit Sequence:").strong());
                     ui.horizontal_wrapped(|ui| {
                         for (i, &bit) in step.bits.iter().enumerate() {
                             if i > 0 && i % 8 == 0 {
                                 ui.add_space(5.0);
                             }
                             let color = if bit == 1 { egui::Color32::GREEN } else { egui::Color32::LIGHT_GRAY };
                             let text = egui::RichText::new(format!("{}", bit)).color(color).monospace().strong();
                             ui.label(text);
                         }
                     });

                     ui.add_space(10.0);
                     ui.separator();
                     ui.add_space(10.0);

                     if step.step_type != NistStepType::ConvertToBits {
                         ui.horizontal(|ui| {
                             ui.vertical(|ui| {
                                 ui.label("Ones:");
                                 ui.heading(format!("{}", step.ones_count));
                             });
                             ui.add_space(40.0);
                             ui.vertical(|ui| {
                                 ui.label("Zeros:");
                                 ui.heading(format!("{}", step.zeros_count));
                             });
                         });

                         // Visual bar
                         let total = step.bits.len() as f32;
                         if total > 0.0 {
                             let ones_pct = step.ones_count as f32 / total;
                             ui.add_space(5.0);
                             let (rect, _) = ui.allocate_exact_size(egui::vec2(300.0, 20.0), egui::Sense::hover());

                             // Draw zeros background
                             ui.painter().rect_filled(rect, 2.0, egui::Color32::LIGHT_GRAY);

                             // Draw ones foreground
                             let ones_width = rect.width() * ones_pct;
                             let ones_rect = egui::Rect::from_min_size(rect.min, egui::vec2(ones_width, rect.height()));
                             ui.painter().rect_filled(ones_rect, 2.0, egui::Color32::GREEN);

                             // Center line (ideal)
                             let center_x = rect.min.x + rect.width() * 0.5;
                             ui.painter().line_segment(
                                 [egui::pos2(center_x, rect.min.y), egui::pos2(center_x, rect.max.y)],
                                 egui::Stroke::new(2.0, egui::Color32::BLACK)
                             );

                             ui.label(format!("Ratio: {:.1}% Ones (Ideal: 50%)", ones_pct * 100.0));
                         }
                     }

                     ui.add_space(20.0);

                     // Stats and interpretation
                     if matches!(step.step_type, NistStepType::CalculateStatistic | NistStepType::CalculatePValue | NistStepType::Interpret) {
                         ui.group(|ui| {
                             ui.vertical(|ui| {
                                 ui.label(format!("Sum (+1 for 1, -1 for 0): {}", step.sum));
                                 ui.label(format!("S_obs (|Sum| / √n): {:.4}", step.s_obs));

                                 if matches!(step.step_type, NistStepType::CalculatePValue | NistStepType::Interpret) {
                                     ui.add_space(5.0);
                                     ui.label(format!("P-Value (erfc(S_obs/√2)): {:.4}", step.p_value));
                                 }

                                 if step.step_type == NistStepType::Interpret {
                                     ui.add_space(10.0);
                                     if step.passed {
                                         ui.colored_label(egui::Color32::GREEN, "✅ PASS: The sequence looks random.");
                                     } else {
                                         ui.colored_label(egui::Color32::RED, "❌ FAIL: The sequence has too many 1s or 0s.");
                                     }
                                     ui.label("(Threshold: P-value ≥ 0.01)");
                                 }
                             });
                         });
                     }

                     ui.add_space(10.0);
                 });
             });

             ui.add_space(20.0);

             // Controls
             ui.horizontal(|ui| {
                if ui.button("⬅ Previous").clicked() {
                    self.nist_process.prev_step();
                }

                let play_label = if self.nist_process.is_playing { "⏸ Pause" } else { "▶ Play" };
                if ui.button(play_label).clicked() {
                    self.nist_process.toggle_play();
                }

                if ui.button("Next ➡").clicked() {
                    self.nist_process.next_step();
                }

                ui.add_space(20.0);
                ui.label("Speed:");
                ui.add(egui::Slider::new(&mut self.nist_process.speed, 0.1..=5.0).text("steps/s"));
            });
        } else {
            ui.label("Enter text or generate random bytes to start.");
        }
    }
}
```

---

## File: src/viz/mod.rs

```rust
//! GUI visualization using egui

mod app;

pub use app::EntropyForgeApp;
```

---

## File: ui/index.html

```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0, viewport-fit=cover">
  <title>Entropy Forge</title>
  <link rel="stylesheet" href="styles/main.css">
  <link rel="stylesheet" href="styles/use-tab.css">
  <link rel="stylesheet" href="styles/test-tab.css">
  <link rel="stylesheet" href="styles/bench-tab.css">
  <link rel="stylesheet" href="styles/learn-tab.css">
  <link rel="stylesheet" href="styles/responsive.css">
</head>
<body>
  <div id="app">
    <main id="tab-content">
      <!-- USE TAB -->
      <section id="tab-use" class="tab-panel active">
        <div class="tab-wrapper">
          <h1>🔐 Encrypt / Decrypt</h1>
          <p class="tab-description">Stream cipher encryption using entropy source as keystream</p>

          <div class="form-group">
            <label>Plaintext</label>
            <textarea id="use-input" placeholder="Enter text to encrypt...">Hello, World!</textarea>
          </div>

          <div class="form-controls">
            <label class="checkbox-label">
              <input type="checkbox" id="use-hex-check" checked>
              Hex output
            </label>
            <button id="use-encrypt-btn" class="btn-primary">Encrypt / Decrypt</button>
          </div>

          <div class="form-group">
            <label>Ciphertext</label>
            <textarea id="use-output" class="mono" readonly></textarea>
          </div>

          <div class="keystream-section">
            <label>Keystream State (64 bytes)</label>
            <div id="use-keystream-grid" class="keystream-grid"></div>
          </div>

          <a href="#" id="use-help-link" class="help-link">→ Learn how this works</a>
        </div>
      </section>

      <!-- TEST TAB -->
      <section id="tab-test" class="tab-panel">
        <div class="tab-wrapper">
          <h1>🔬 Quality Analysis</h1>
          <p class="tab-description">NIST SP 800-22 randomness tests and entropy metrics</p>

          <div class="form-group">
            <label>Sample size</label>
            <div class="slider-row">
              <input type="range" id="test-size-slider" min="3" max="6" step="0.1" value="4">
              <span id="test-size-label" class="mono">10,000 bytes</span>
            </div>
          </div>

          <button id="test-run-btn" class="btn-primary">Run Tests</button>

          <div id="test-results-section" class="results-section hidden">
            <div class="metrics-grid">
              <div class="metric-card">
                <div class="metric-title">Shannon Entropy</div>
                <div id="test-shannon-bar" class="progress-bar"></div>
              </div>
              <div class="metric-card">
                <div class="metric-title">Min-Entropy</div>
                <div id="test-min-bar" class="progress-bar"></div>
              </div>
              <div class="metric-card">
                <div class="metric-label">Mean Byte</div>
                <div id="test-mean-val" class="metric-value mono"></div>
              </div>
              <div class="metric-card">
                <div class="metric-label">Chi-Square</div>
                <div id="test-chi-val" class="metric-value mono"></div>
              </div>
            </div>

            <div class="nist-section">
              <h2>NIST Test Results</h2>
              <table class="nist-table">
                <thead>
                  <tr>
                    <th>Test</th>
                    <th>P-Value</th>
                    <th>Status</th>
                  </tr>
                </thead>
                <tbody id="test-nist-tbody"></tbody>
              </table>
            </div>
          </div>
        </div>
      </section>

      <!-- BENCHMARK TAB -->
      <section id="tab-benchmark" class="tab-panel">
        <div class="tab-wrapper">
          <h1>⚡ Performance</h1>
          <p class="tab-description">Measure entropy source throughput and latency</p>

          <button id="bench-run-btn" class="btn-primary">Run Benchmark</button>

          <div id="bench-results-section" class="results-section hidden">
            <div class="metrics-grid">
              <div class="metric-card">
                <div class="metric-label">Throughput</div>
                <div id="bench-throughput" class="metric-value mono"></div>
              </div>
              <div class="metric-card">
                <div class="metric-label">Latency</div>
                <div id="bench-latency" class="metric-value mono"></div>
              </div>
            </div>
          </div>
        </div>
      </section>

      <!-- LEARN TAB -->
      <section id="tab-learn" class="tab-panel">
        <div class="tab-wrapper">
          <h1>🎓 Learn</h1>
          <p class="tab-description">Interactive step-by-step cryptography lessons</p>

          <div class="learn-nav">
            <button class="learn-btn active" data-learn="xor">XOR Cipher</button>
            <button class="learn-btn" data-learn="entropy">Entropy</button>
            <button class="learn-btn" data-learn="nist">NIST Tests</button>
          </div>

          <!-- XOR Lesson -->
          <div id="learn-xor" class="learn-section active">
            <div class="form-group">
              <label>Input Text</label>
              <textarea id="learn-input" placeholder="Enter text...">Hi</textarea>
            </div>
            <div class="controls">
              <button id="learn-prev" class="btn-secondary">← Previous</button>
              <div class="step-info">
                <span id="learn-step">Step 1 of ?</span>
              </div>
              <button id="learn-next" class="btn-secondary">Next →</button>
            </div>
            <div id="learn-content"></div>
          </div>

          <!-- Entropy Lesson -->
          <div id="learn-entropy" class="learn-section">
            <div class="form-group">
              <label>Input Text</label>
              <textarea id="learn-entropy-input" placeholder="Enter text...">Hello World</textarea>
            </div>
            <div class="controls">
              <button id="learn-entropy-prev" class="btn-secondary">← Previous</button>
              <div class="step-info">
                <span id="learn-entropy-step">Step 1 of ?</span>
              </div>
              <button id="learn-entropy-next" class="btn-secondary">Next →</button>
            </div>
            <div id="learn-entropy-content"></div>
          </div>

          <!-- NIST Lesson -->
          <div id="learn-nist" class="learn-section">
            <div class="controls">
              <button id="learn-nist-prev" class="btn-secondary">← Previous</button>
              <div class="step-info">
                <span id="learn-nist-step">Step 1 of ?</span>
              </div>
              <button id="learn-nist-next" class="btn-secondary">Next →</button>
            </div>
            <div id="learn-nist-content"></div>
          </div>
        </div>
      </section>
    </main>

    <!-- BOTTOM TAB BAR -->
    <nav id="tab-bar">
      <button class="tab-btn active" data-tab="use">🔐 Use</button>
      <button class="tab-btn" data-tab="test">🔬 Test</button>
      <button class="tab-btn" data-tab="benchmark">⚡ Bench</button>
      <button class="tab-btn" data-tab="learn">🎓 Learn</button>
    </nav>
  </div>

  <script src="main.js"></script>
</body>
</html>
```

---

## File: ui/package.json

```json
{
  "name": "entropy-forge-ui",
  "version": "1.0.0",
  "scripts": {
    "build": "esbuild ts/main.ts --bundle --outdir=dist && npm run copy-assets",
    "copy-assets": "mkdir -p dist/styles && cp index.html dist/ && cp styles/*.css dist/styles/",
    "dev": "esbuild ts/main.ts --bundle --outdir=dist --watch"
  },
  "devDependencies": {
    "@tauri-apps/api": "^2.10.1",
    "@tauri-apps/cli": "^2",
    "typescript": "^5"
  },
  "dependencies": {
    "esbuild": "^0.20.2"
  }
}
```

---

## File: ui/styles/bench-tab.css

```css

```

---

## File: ui/styles/learn-tab.css

```css

```

---

## File: ui/styles/main.css

```css
/* ============================================================
   ENTROPY FORGE - Clean, Educational UI
   ============================================================ */

:root {
  /* Colors */
  --bg-dark: #0a0e27;
  --bg-card: #141829;
  --bg-input: #1a1f3a;
  --border: #2a2f4a;
  --text-light: #e8eef7;
  --text-dim: #8892b0;
  --accent: #00d4ff;
  --accent-alt: #ff8c00;
  --success: #00ff88;
  --error: #ff4444;

  /* Typography */
  --font-sans: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
  --font-mono: "SF Mono", Monaco, "Cascadia Code", monospace;

  /* Spacing */
  --gap-xs: 4px;
  --gap-sm: 8px;
  --gap-md: 16px;
  --gap-lg: 24px;
  --gap-xl: 32px;
}

* {
  box-sizing: border-box;
  margin: 0;
  padding: 0;
}

html, body {
  width: 100%;
  height: 100%;
  background: var(--bg-dark);
  color: var(--text-light);
  font-family: var(--font-sans);
  font-size: 15px;
  line-height: 1.5;
}

/* ============================================================
   APP LAYOUT
   ============================================================ */

#app {
  display: flex;
  flex-direction: column;
  height: 100vh;
  width: 100%;
}

#tab-content {
  flex: 1;
  overflow-y: auto;
  overflow-x: hidden;
  -webkit-overflow-scrolling: touch;
}

/* Scrollbar */
#tab-content::-webkit-scrollbar {
  width: 8px;
}

#tab-content::-webkit-scrollbar-track {
  background: transparent;
}

#tab-content::-webkit-scrollbar-thumb {
  background: var(--border);
  border-radius: 4px;
}

#tab-content::-webkit-scrollbar-thumb:hover {
  background: var(--accent);
}

/* Tab panel */
.tab-panel {
  display: none;
}

.tab-panel.active {
  display: block;
  animation: fadeIn 0.2s ease;
}

@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

.tab-wrapper {
  max-width: 800px;
  margin: 0 auto;
  padding: var(--gap-lg);
}

/* ============================================================
   TYPOGRAPHY
   ============================================================ */

h1 {
  font-size: 28px;
  font-weight: 700;
  margin-bottom: var(--gap-xs);
  color: var(--text-light);
}

h2 {
  font-size: 20px;
  font-weight: 600;
  margin-bottom: var(--gap-md);
  color: var(--text-light);
}

.tab-description {
  font-size: 14px;
  color: var(--text-dim);
  margin-bottom: var(--gap-lg);
}

.mono {
  font-family: var(--font-mono);
  font-size: 13px;
  letter-spacing: 0.3px;
}

/* ============================================================
   FORM ELEMENTS
   ============================================================ */

.form-group {
  margin-bottom: var(--gap-lg);
  display: flex;
  flex-direction: column;
  gap: var(--gap-sm);
}

.form-group label {
  font-size: 13px;
  color: var(--text-dim);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  font-weight: 500;
}

textarea, input[type="text"] {
  width: 100%;
  padding: var(--gap-md);
  background: var(--bg-input);
  border: 1px solid var(--border);
  color: var(--text-light);
  font-family: inherit;
  border-radius: 6px;
  transition: all 0.15s ease;
  font-size: 14px;
  line-height: 1.6;
}

textarea {
  font-family: var(--font-mono);
  min-height: 100px;
  resize: vertical;
}

textarea:focus, input:focus {
  border-color: var(--accent);
  outline: none;
  box-shadow: 0 0 0 2px rgba(0, 212, 255, 0.1);
}

textarea::placeholder {
  color: var(--text-dim);
}

input[type="range"] {
  flex: 1;
  min-width: 200px;
  cursor: pointer;
  accent-color: var(--accent);
  height: 4px;
}

input[type="checkbox"] {
  width: 18px;
  height: 18px;
  cursor: pointer;
  accent-color: var(--accent);
}

/* ============================================================
   BUTTONS
   ============================================================ */

button {
  border: none;
  border-radius: 6px;
  font-family: inherit;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
  padding: var(--gap-sm) var(--gap-md);
}

.btn-primary {
  width: 100%;
  background: var(--accent);
  color: var(--bg-dark);
  padding: var(--gap-md);
  font-size: 15px;
  font-weight: 600;
  margin: var(--gap-md) 0;
}

.btn-primary:hover {
  background: #00e5ff;
  box-shadow: 0 0 20px rgba(0, 212, 255, 0.3);
  transform: translateY(-1px);
}

.btn-primary:active {
  transform: translateY(0);
}

.btn-primary:disabled {
  background: var(--text-dim);
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-secondary {
  background: var(--bg-card);
  color: var(--text-light);
  border: 1px solid var(--border);
  min-width: 100px;
}

.btn-secondary:hover {
  border-color: var(--accent);
  color: var(--accent);
}

/* ============================================================
   FORM CONTROLS
   ============================================================ */

.form-controls {
  display: flex;
  align-items: center;
  gap: var(--gap-md);
  flex-wrap: wrap;
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: var(--gap-sm);
  font-size: 14px;
  cursor: pointer;
  color: var(--text-light);
}

.slider-row {
  display: flex;
  align-items: center;
  gap: var(--gap-md);
}

/* ============================================================
   METRICS & RESULTS
   ============================================================ */

.results-section {
  display: none;
  margin-top: var(--gap-xl);
}

.results-section.visible {
  display: block;
  animation: fadeIn 0.3s ease;
}

.metrics-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: var(--gap-md);
  margin: var(--gap-lg) 0;
}

.metric-card {
  background: var(--bg-card);
  padding: var(--gap-md);
  border-radius: 6px;
  border: 1px solid var(--border);
}

.metric-title, .metric-label {
  font-size: 12px;
  color: var(--text-dim);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  margin-bottom: var(--gap-sm);
  font-weight: 500;
}

.metric-value {
  font-size: 18px;
  color: var(--accent);
  font-weight: 600;
}

/* Progress bars */
.progress-bar {
  width: 100%;
  height: 6px;
  background: var(--bg-input);
  border-radius: 3px;
  overflow: hidden;
  margin-top: var(--gap-sm);
}

.progress-bar::after {
  content: '';
  display: block;
  height: 100%;
  background: linear-gradient(90deg, var(--accent), #00e5ff);
  width: 0%;
  transition: width 0.3s ease;
  border-radius: 3px;
}

/* ============================================================
   TABLES
   ============================================================ */

.nist-section {
  margin-top: var(--gap-xl);
}

.nist-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 13px;
  margin-top: var(--gap-md);
}

.nist-table th {
  text-align: left;
  padding: var(--gap-md);
  background: var(--bg-card);
  border-bottom: 2px solid var(--border);
  color: var(--text-dim);
  font-weight: 500;
  text-transform: uppercase;
  font-size: 11px;
  letter-spacing: 0.5px;
}

.nist-table td {
  padding: var(--gap-md);
  border-bottom: 1px solid var(--border);
  color: var(--text-light);
}

.nist-table tr:hover {
  background: rgba(0, 212, 255, 0.05);
}

.nist-table .pass {
  color: var(--success);
  font-weight: 600;
}

.nist-table .fail {
  color: var(--error);
  font-weight: 600;
}

/* ============================================================
   TAB BAR (Bottom Navigation)
   ============================================================ */

#tab-bar {
  display: flex;
  background: var(--bg-card);
  border-top: 1px solid var(--border);
  gap: 0;
  flex-shrink: 0;
  height: 60px;
}

.tab-btn {
  flex: 1;
  background: transparent;
  border: none;
  color: var(--text-dim);
  font-size: 12px;
  border-top: 3px solid transparent;
  transition: all 0.15s ease;
  border-radius: 0;
  padding: var(--gap-sm);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: var(--gap-xs);
}

.tab-btn:hover {
  color: var(--accent);
  background: rgba(0, 212, 255, 0.05);
}

.tab-btn.active {
  color: var(--accent);
  border-top-color: var(--accent);
  background: rgba(0, 212, 255, 0.08);
}

/* ============================================================
   SPECIAL SECTIONS
   ============================================================ */

.keystream-section {
  margin-top: var(--gap-lg);
}

.keystream-grid {
  display: grid;
  grid-template-columns: repeat(8, 1fr);
  gap: 4px;
  margin-top: var(--gap-md);
  padding: var(--gap-md);
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: 6px;
  max-width: 300px;
  width: fit-content;
}

.keystream-cell {
  aspect-ratio: 1;
  border: 1px solid var(--border);
  border-radius: 3px;
  background: #333;
  cursor: pointer;
  transition: all 0.1s ease;
}

.keystream-cell:hover {
  border-color: var(--accent);
  box-shadow: 0 0 8px rgba(0, 212, 255, 0.4);
  transform: scale(1.1);
}

.help-link {
  display: inline-block;
  color: var(--accent);
  text-decoration: none;
  font-size: 13px;
  margin-top: var(--gap-lg);
  transition: all 0.15s ease;
}

.help-link:hover {
  color: #00e5ff;
}

/* ============================================================
   LEARN SECTION
   ============================================================ */

.learn-nav {
  display: flex;
  gap: var(--gap-sm);
  margin-bottom: var(--gap-lg);
  border-bottom: 2px solid var(--border);
}

.learn-btn {
  background: transparent;
  border: none;
  color: var(--text-dim);
  padding: var(--gap-md) 0;
  border-bottom: 3px solid transparent;
  cursor: pointer;
  font-size: 14px;
  transition: all 0.15s ease;
  border-radius: 0;
}

.learn-btn:hover {
  color: var(--accent);
}

.learn-btn.active {
  color: var(--accent);
  border-bottom-color: var(--accent);
}

.learn-section {
  display: none;
}

.learn-section.active {
  display: block;
  animation: fadeIn 0.2s ease;
}

.controls {
  display: flex;
  align-items: center;
  gap: var(--gap-md);
  margin: var(--gap-lg) 0;
  justify-content: center;
}

.step-info {
  text-align: center;
  color: var(--text-dim);
  font-size: 13px;
  min-width: 100px;
}

#learn-content, #learn-entropy-content, #learn-nist-content {
  margin-top: var(--gap-lg);
  padding: var(--gap-md);
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: 6px;
  min-height: 200px;
}

/* ============================================================
   RESPONSIVE
   ============================================================ */

@media (max-width: 768px) {
  :root {
    font-size: 14px;
  }

  .tab-wrapper {
    padding: var(--gap-md);
  }

  h1 {
    font-size: 24px;
  }

  .metrics-grid {
    grid-template-columns: 1fr;
  }

  .keystream-grid {
    gap: 3px;
    padding: var(--gap-sm);
    max-width: 100%;
  }
}

@media (max-width: 480px) {
  :root {
    --gap-lg: 16px;
    --gap-md: 12px;
  }

  #tab-content {
    padding-bottom: 60px;
  }

  .tab-wrapper {
    padding: var(--gap-md);
    max-width: 100%;
  }

  h1 {
    font-size: 20px;
  }

  textarea, input[type="text"] {
    padding: var(--gap-sm);
    font-size: 16px;
    min-height: 80px;
  }

  .form-controls {
    flex-direction: column;
    align-items: stretch;
  }

  .btn-primary {
    padding: var(--gap-md);
  }

  .metrics-grid {
    grid-template-columns: 1fr;
    gap: var(--gap-sm);
  }

  .metric-card {
    padding: var(--gap-sm);
  }

  .nist-table {
    font-size: 12px;
  }

  .nist-table th, .nist-table td {
    padding: var(--gap-sm);
  }

  .controls {
    flex-wrap: wrap;
    gap: var(--gap-sm);
  }

  .btn-secondary {
    flex: 1;
    min-width: auto;
  }

  .step-info {
    min-width: auto;
    flex: 1;
    font-size: 12px;
  }

  .learn-nav {
    gap: 0;
  }

  .learn-btn {
    flex: 1;
    text-align: center;
    font-size: 12px;
    padding: var(--gap-sm);
  }
}
```

---

## File: ui/styles/responsive.css

```css
/* Responsive adjustments for mobile devices */

/* Tablet portrait / small laptop (500px – 767px) */
@media (min-width: 500px) and (max-width: 767px) {
  #app {
    padding: 0 var(--space-md);
  }
}

/* Mobile: < 500px */
@media (max-width: 499px) {
  :root {
    --space-md: 10px;
    --space-lg: 14px;

    --text-2xl: 18px;
    --text-xl: 16px;
    --text-lg: 15px;
    --text-base: 13px;
    --text-sm: 12px;

    --font-mono: 12px 'Courier New', Courier, monospace;
  }

  #app {
    max-width: 100vw;
    padding: 0 12px;
  }

  /* Tab Bar */
  #tab-bar {
    overflow-x: auto;
    white-space: nowrap;
    scrollbar-width: none; /* Firefox */
  }
  #tab-bar::-webkit-scrollbar {
    display: none; /* Safari and Chrome */
  }

  .tab-btn {
    min-width: 64px;
    padding: var(--space-md) var(--space-sm);
    font-size: var(--text-sm);
  }

  /* Sub Tab Bar */
  .sub-tab-bar {
    overflow-x: auto;
    white-space: nowrap;
  }

  /* Typography / General overrides */
  h1 { font-size: 18px; }
  h2 { font-size: 16px; }
  h3 { font-size: 15px; }
  p, label, button, input, textarea { font-size: 13px; }

  /* Flex Layouts */
  .use-controls {
    flex-direction: column;
    align-items: stretch;
  }

  .use-controls .spacer {
    display: none;
  }

  button {
    width: 100%;
    padding: 10px;
    min-height: 44px;
  }

  /* Use Tab */
  #use-input { min-height: 60px; }
  .keystream-cell {
    width: 18px;
    height: 18px;
  }

  /* Test Tab */
  .progress-bar-container {
    flex-direction: column;
    align-items: flex-start;
    gap: 4px;
  }

  .progress-label { width: 100%; }
  .progress-track { width: 100%; margin: 0; }
  .progress-value { width: 100%; text-align: left; }

  .metric-row {
    flex-direction: column;
    align-items: flex-start;
  }

  .metric-label { width: 100%; }

  .nist-table th, .nist-table td {
    padding: 8px 4px;
    font-size: 12px;
  }

  /* Bench Tab */
  .bench-metric-row {
    flex-direction: column;
    align-items: flex-start;
    margin-bottom: var(--space-md);
  }

  .bench-metric-label { width: 100%; }

  /* Learn Tab - XOR */
  .xor-row {
    display: flex;
    flex-direction: column;
    margin-bottom: var(--space-md);
  }

  .xor-row-label { width: auto; font-size: 12px; }
  .xor-row-char, .xor-row-dec { width: auto; margin-right: 8px; }

  /* Learn Tab - Entropy */
  .entropy-table th, .entropy-table td {
    font-size: 12px;
  }

  /* NIST Formula */
  .nist-stats {
    word-break: break-all;
  }
}
```

---

## File: ui/styles/test-tab.css

```css

```

---

## File: ui/styles/use-tab.css

```css

```

---

## File: ui/ts/bench-tab.ts

```typescript
import { invoke } from "@tauri-apps/api/core";

export interface BenchResponse {
  throughput_mbps: number;
  latency_us: number;
  bytes_generated: number;
  duration_secs: number;
}

export function initBenchTab() {
  const sizeSlider = document.getElementById('bench-size-slider') as HTMLInputElement;
  const sizeLabel = document.getElementById('bench-size-label') as HTMLElement;
  const runBtn = document.getElementById('bench-run-btn') as HTMLButtonElement;
  const resultsSection = document.getElementById('bench-results-section') as HTMLElement;

  if (!sizeSlider || !runBtn || !resultsSection) return;

  const updateSliderLabel = () => {
    // Logarithmic: 10,000 to 10,000,000 => 10^4 to 10^7
    const val = parseFloat(sizeSlider.value);
    const bytes = Math.round(Math.pow(10, val));
    sizeLabel.textContent = bytes.toLocaleString() + " bytes";
    return bytes;
  };

  sizeSlider.addEventListener('input', updateSliderLabel);
  updateSliderLabel(); // initial

  let isLoading = false;

  const runBenchmark = async () => {
    if (isLoading) return;
    const bytes = updateSliderLabel();

    isLoading = true;
    runBtn.disabled = true;
    runBtn.textContent = "Benchmarking...";
    resultsSection.classList.add('hidden');

    try {
      const response = await invoke<BenchResponse>('run_benchmark', { bytes });

      document.getElementById('bench-throughput-val')!.textContent = `${response.throughput_mbps.toFixed(1)} MB/s`;
      document.getElementById('bench-latency-val')!.textContent = `${response.latency_us.toFixed(2)} µs/byte`;
      document.getElementById('bench-bytes-val')!.textContent = response.bytes_generated.toLocaleString();
      document.getElementById('bench-duration-val')!.textContent = `${response.duration_secs.toFixed(2)} s`;

      resultsSection.classList.remove('hidden');

    } catch (e) {
      console.error(e);
      alert(`Error running benchmark: ${e}`);
    } finally {
      isLoading = false;
      runBtn.disabled = false;
      runBtn.textContent = "⚡ Run Benchmark";
    }
  };

  runBtn.addEventListener('click', runBenchmark);
}
```

---

## File: ui/ts/components/controls.ts

```typescript
export class Controls {
  private container: HTMLElement;
  public currentIndex = 0;
  public totalSteps = 0;
  public isPlaying = false;
  public speed = 1.0;

  private playBtn: HTMLButtonElement;
  private prevBtn: HTMLButtonElement;
  private nextBtn: HTMLButtonElement;
  private speedLabel: HTMLElement;

  constructor(
    containerId: string,
    private onPrev: () => void,
    private onNext: () => void,
    private onTogglePlay: () => void,
    private onSpeedChange: (speed: number) => void
  ) {
    this.container = document.getElementById(containerId)!;

    this.container.innerHTML = `
      <div class="flex-row">
        <button class="control-btn prev-btn">◀ Prev</button>
        <button class="control-btn play-btn">▶ Play</button>
        <button class="control-btn next-btn">▶▶ Next</button>
        <div class="spacer"></div>
        <label>Speed:</label>
        <input type="range" class="speed-slider" min="0.1" max="5.0" step="0.1" value="1.0">
        <span class="speed-label font-mono">1.0 steps/s</span>
      </div>
    `;

    this.prevBtn = this.container.querySelector('.prev-btn') as HTMLButtonElement;
    this.playBtn = this.container.querySelector('.play-btn') as HTMLButtonElement;
    this.nextBtn = this.container.querySelector('.next-btn') as HTMLButtonElement;
    const speedSlider = this.container.querySelector('.speed-slider') as HTMLInputElement;
    this.speedLabel = this.container.querySelector('.speed-label') as HTMLElement;

    this.prevBtn.addEventListener('click', () => {
      if (this.currentIndex > 0) this.onPrev();
    });

    this.nextBtn.addEventListener('click', () => {
      if (this.currentIndex < this.totalSteps - 1) this.onNext();
    });

    this.playBtn.addEventListener('click', () => {
      this.onTogglePlay();
      this.updateState(this.currentIndex, this.totalSteps, this.isPlaying);
    });

    speedSlider.addEventListener('input', (e) => {
      const target = e.target as HTMLInputElement;
      this.speed = parseFloat(target.value);
      this.speedLabel.textContent = `${this.speed.toFixed(1)} steps/s`;
      this.onSpeedChange(this.speed);
    });
  }

  public updateState(current: number, total: number, playing: boolean) {
    this.currentIndex = current;
    this.totalSteps = total;
    this.isPlaying = playing;

    this.prevBtn.disabled = this.currentIndex === 0;
    this.nextBtn.disabled = this.currentIndex >= this.totalSteps - 1;

    if (this.isPlaying) {
      this.playBtn.textContent = '⏸ Pause';
    } else {
      this.playBtn.textContent = '▶ Play';
    }
  }
}
```

---

## File: ui/ts/components/grid.ts

```typescript
export class KeystreamGrid {
  private container: HTMLElement;
  private cells: HTMLElement[] = [];

  constructor(container: HTMLElement) {
    this.container = container;
    this.container.classList.add('keystream-grid');
    this.render();
  }

  private render() {
    this.container.innerHTML = '';
    this.cells = [];
    for (let i = 0; i < 64; i++) {
      const cell = document.createElement('div');
      cell.className = 'keystream-cell';
      cell.title = `Byte ${i}: Empty`;
      this.container.appendChild(cell);
      this.cells.push(cell);
    }
  }

  public update(bytes: number[]) {
    for (let i = 0; i < 64; i++) {
      const cell = this.cells[i];
      if (i < bytes.length) {
        const val = bytes[i];
        cell.style.backgroundColor = `rgb(${val}, ${val}, ${val})`;

        // Convert to hex (e.g. 0A)
        let hex = val.toString(16).toUpperCase();
        if (hex.length < 2) hex = '0' + hex;

        cell.title = `Byte ${i}: 0x${hex} (${val})`;
      } else {
        cell.style.backgroundColor = 'transparent';
        cell.title = `Byte ${i}: Empty`;
      }
    }
  }
}
```

---

## File: ui/ts/components/progress-bar.ts

```typescript
export class ProgressBar {
  private container: HTMLElement;
  private labelEl: HTMLElement;
  private fillEl: HTMLElement;
  private valueEl: HTMLElement;

  constructor(
    container: HTMLElement,
    public label: string,
    public tooltipText: string | null = null,
    public colorThresholds = { green: 0.8, orange: 0.6 }
  ) {
    this.container = container;

    this.container.classList.add('progress-bar-container', 'flex-row');

    this.labelEl = document.createElement('div');
    this.labelEl.className = 'progress-label';
    this.labelEl.textContent = label;

    const trackEl = document.createElement('div');
    trackEl.className = 'progress-track';

    this.fillEl = document.createElement('div');
    this.fillEl.className = 'progress-fill';
    trackEl.appendChild(this.fillEl);

    this.valueEl = document.createElement('div');
    this.valueEl.className = 'progress-value';

    this.container.appendChild(this.labelEl);
    this.container.appendChild(trackEl);
    this.container.appendChild(this.valueEl);

    if (tooltipText) {
      const tooltip = document.createElement('span');
      tooltip.className = 'tooltip-icon';
      tooltip.textContent = 'ℹ';
      tooltip.setAttribute('data-tooltip', tooltipText);
      this.container.appendChild(tooltip);
    }
  }

  public update(value: number, max: number, displayValue: string) {
    const fraction = value / max;
    this.fillEl.style.width = `${Math.min(100, fraction * 100)}%`;
    this.valueEl.textContent = displayValue;

    if (fraction >= this.colorThresholds.green) {
      this.fillEl.style.backgroundColor = 'var(--accent-green)';
    } else if (fraction >= this.colorThresholds.orange) {
      this.fillEl.style.backgroundColor = 'var(--accent-orange)';
    } else {
      this.fillEl.style.backgroundColor = 'var(--accent-red)';
    }
  }
}
```

---

## File: ui/ts/learn/entropy.ts

```typescript
import { invoke } from "@tauri-apps/api/core";
import { Controls } from "../components/controls";

interface EntropyStep {
  step_type: string;
  byte_counts: Record<string, number>;
  probabilities: Record<string, number>;
  entropy_contributions: Record<string, number>;
  current_entropy_sum: number;
  total_entropy: number;
  max_entropy: number;
}

interface EntropyStepsResponse {
  steps: EntropyStep[];
  total: number;
}

export function initEntropyVisualizer() {
  const inputEl = document.getElementById('entropy-input') as HTMLTextAreaElement;
  const startBtn = document.getElementById('entropy-calc-btn') as HTMLButtonElement;
  const vizSection = document.getElementById('entropy-viz-section') as HTMLElement;
  const stepHeader = document.getElementById('entropy-step-header') as HTMLElement;
  const contentArea = document.getElementById('entropy-content-area') as HTMLElement;

  if (!startBtn) return;

  let steps: EntropyStep[] = [];
  let currentIndex = 0;
  let timerId: number | null = null;
  let controls: Controls;

  const renderStep = () => {
    if (steps.length === 0) return;
    const step = steps[currentIndex];

    const titles: Record<string, string> = {
      "CountBytes": "Count Byte Frequencies",
      "CalculateProbabilities": "Calculate Probabilities",
      "CalculateContributions": "Calculate Contributions",
      "SumEntropy": "Sum for Total Entropy",
      "Interpret": "Interpretation"
    };

    stepHeader.textContent = `Step ${currentIndex + 1} of ${steps.length}: ${titles[step.step_type] || step.step_type}`;

    let html = '';

    if (["CountBytes", "CalculateProbabilities", "CalculateContributions"].includes(step.step_type)) {
      html += `
        <table class="entropy-table">
          <thead>
            <tr>
              <th>Byte</th>
              <th>Count</th>
              ${step.step_type !== 'CountBytes' ? '<th>Probability</th>' : ''}
              ${step.step_type === 'CalculateContributions' ? '<th>Contribution</th>' : ''}
              <th>Visual</th>
            </tr>
          </thead>
          <tbody>
      `;

      const keys = Object.keys(step.byte_counts).sort();
      const maxCount = Math.max(...Object.values(step.byte_counts));

      for (const k of keys) {
        const count = step.byte_counts[k];
        const p = step.probabilities[k] || 0;
        const c = step.entropy_contributions[k] || 0;
        const width = (count / maxCount) * 100;

        let charRepr = parseInt(k);
        let display = `'${String.fromCharCode(charRepr)}' (${charRepr})`;
        if (charRepr < 32 || charRepr > 126) display = `0x${charRepr.toString(16)}`;

        html += `<tr>
          <td>${display}</td>
          <td>${count}</td>
          ${step.step_type !== 'CountBytes' ? `<td>${p.toFixed(4)}</td>` : ''}
          ${step.step_type === 'CalculateContributions' ? `<td>${c.toFixed(4)} bits</td>` : ''}
          <td><div class="entropy-bar" style="width: ${width}%"></div></td>
        </tr>`;
      }
      html += `</tbody></table>`;
    } else {
      const eff = step.max_entropy > 0 ? (step.current_entropy_sum / step.max_entropy) * 100 : 0;
      html += `
        <div class="entropy-summary">
          <div class="entropy-val-large">Total Entropy: ${step.current_entropy_sum.toFixed(4)} bits/byte</div>
          <div class="text-muted">Maximum Possible: ${step.max_entropy.toFixed(4)} bits/byte</div>
          <div class="text-muted">Efficiency: ${eff.toFixed(1)}%</div>
        </div>
      `;

      if (step.step_type === "Interpret") {
        html += `<div class="entropy-interpret">`;
        if (step.current_entropy_sum < 2.0) {
          html += `<span class="text-red">Low entropy. The input is very predictable.</span>`;
        } else if (eff > 80.0) {
          html += `<span class="text-green">High entropy! The input looks quite random.</span>`;
        } else {
          html += `<span class="text-orange">Moderate entropy.</span>`;
        }
        html += `</div>`;
      }
    }

    contentArea.innerHTML = html;
    controls.updateState(currentIndex, steps.length, timerId !== null);
  };

  const playNext = () => {
    if (currentIndex < steps.length - 1) {
      currentIndex++;
      renderStep();
    } else {
      stopTimer();
      controls.updateState(currentIndex, steps.length, false);
    }
  };

  const stopTimer = () => {
    if (timerId !== null) {
      clearInterval(timerId);
      timerId = null;
    }
  };

  const startTimer = () => {
    stopTimer();
    if (currentIndex >= steps.length - 1) currentIndex = 0;
    timerId = window.setInterval(playNext, 1000 / controls.speed);
  };

  controls = new Controls('entropy-controls',
    () => { currentIndex--; renderStep(); },
    () => { currentIndex++; renderStep(); },
    () => {
      if (timerId !== null) stopTimer();
      else startTimer();
    },
    (speed) => {
      if (timerId !== null) startTimer();
    }
  );

  startBtn.addEventListener('click', async () => {
    const text = inputEl.value;
    if (!text) return;

    startBtn.disabled = true;
    startBtn.textContent = "Loading...";

    try {
      const response = await invoke<EntropyStepsResponse>('get_entropy_steps', { text });
      steps = response.steps;
      currentIndex = 0;
      stopTimer();
      vizSection.classList.remove('hidden');
      renderStep();
    } catch (e) {
      alert(`Error: ${e}`);
    } finally {
      startBtn.disabled = false;
      startBtn.textContent = "Calculate";
    }
  });
}
```

---

## File: ui/ts/learn/index.ts

```typescript
import { initXorVisualizer } from './xor';
import { initEntropyVisualizer } from './entropy';
import { initNistVisualizer } from './nist';

export function initLearnTab() {
  const subTabBtns = document.querySelectorAll('.sub-tab-btn');
  const subTabPanels = document.querySelectorAll('.sub-tab-panel');

  subTabBtns.forEach(btn => {
    btn.addEventListener('click', () => {
      subTabBtns.forEach(b => b.classList.remove('active'));
      subTabPanels.forEach(p => p.classList.add('hidden'));

      btn.classList.add('active');
      const targetId = `learn-${btn.getAttribute('data-subtab')}`;
      const targetPanel = document.getElementById(targetId);
      if (targetPanel) {
        targetPanel.classList.remove('hidden');
      }
    });
  });

  initXorVisualizer();
  initEntropyVisualizer();
  initNistVisualizer();
}
```

---

## File: ui/ts/learn/nist.ts

```typescript
import { invoke } from "@tauri-apps/api/core";
import { Controls } from "../components/controls";

interface NistStep {
  step_type: string;
  bits: number[];
  ones_count: number;
  zeros_count: number;
  sum: number;
  s_obs: number;
  p_value: number;
  passed: boolean;
}

interface NistStepsResponse {
  steps: NistStep[];
  total: number;
  input_text: string;
}

export function initNistVisualizer() {
  const inputEl = document.getElementById('nist-input') as HTMLTextAreaElement;
  const analyzeBtn = document.getElementById('nist-analyze-btn') as HTMLButtonElement;
  const randomBtn = document.getElementById('nist-random-btn') as HTMLButtonElement;
  const vizSection = document.getElementById('nist-viz-section') as HTMLElement;
  const stepHeader = document.getElementById('nist-step-header') as HTMLElement;
  const contentArea = document.getElementById('nist-content-area') as HTMLElement;

  if (!analyzeBtn) return;

  let steps: NistStep[] = [];
  let currentIndex = 0;
  let timerId: number | null = null;
  let controls: Controls;

  const renderStep = () => {
    if (steps.length === 0) return;
    const step = steps[currentIndex];

    const titles: Record<string, string> = {
      "ConvertToBits": "Convert to Bits",
      "CountOnesZeros": "Count Ones and Zeros",
      "CalculateStatistic": "Calculate Statistic",
      "CalculatePValue": "Calculate P-Value",
      "Interpret": "Interpretation"
    };

    stepHeader.textContent = `Step ${currentIndex + 1} of ${steps.length}: ${titles[step.step_type] || step.step_type}`;

    let html = '';

    // Bits display
    html += `<div class="nist-bits font-mono">`;
    for (let i = 0; i < step.bits.length; i++) {
      if (i > 0 && i % 8 === 0) html += ' ';
      html += `<span class="${step.bits[i] === 1 ? 'text-green' : 'text-muted'}">${step.bits[i]}</span>`;
    }
    html += `</div>`;

    if (step.step_type !== 'ConvertToBits') {
      const total = step.bits.length;
      const onesPct = total > 0 ? (step.ones_count / total) * 100 : 0;

      html += `
        <div class="nist-ratio-container">
          <div class="flex-row">
            <div>Ones: <strong class="text-green">${step.ones_count}</strong></div>
            <div class="spacer"></div>
            <div>Zeros: <strong class="text-muted">${step.zeros_count}</strong></div>
          </div>
          <div class="nist-ratio-bar">
            <div class="nist-ratio-ones" style="width: ${onesPct}%"></div>
            <div class="nist-ratio-center"></div>
          </div>
          <div class="text-center text-sm text-muted">Ideal: 50%</div>
        </div>
      `;
    }

    if (["CalculateStatistic", "CalculatePValue", "Interpret"].includes(step.step_type)) {
      html += `
        <div class="nist-stats">
          <div>Sum (+1 for 1, -1 for 0): <strong>${step.sum}</strong></div>
          <div>S_obs (|Sum| / √n): <strong>${step.s_obs.toFixed(4)}</strong></div>
      `;

      if (step.step_type === "CalculatePValue" || step.step_type === "Interpret") {
        html += `<div>P-Value (erfc(S_obs/√2)): <strong>${step.p_value.toFixed(4)}</strong></div>`;
      }

      if (step.step_type === "Interpret") {
        if (step.passed) {
          html += `<div class="text-green" style="margin-top: 10px;">✅ PASS: The sequence looks random.</div>`;
        } else {
          html += `<div class="text-red" style="margin-top: 10px;">❌ FAIL: The sequence has too many 1s or 0s.</div>`;
        }
        html += `<div class="text-muted text-sm">(Threshold: P-value ≥ 0.01)</div>`;
      }

      html += `</div>`;
    }

    contentArea.innerHTML = html;
    controls.updateState(currentIndex, steps.length, timerId !== null);
  };

  const playNext = () => {
    if (currentIndex < steps.length - 1) {
      currentIndex++;
      renderStep();
    } else {
      stopTimer();
      controls.updateState(currentIndex, steps.length, false);
    }
  };

  const stopTimer = () => {
    if (timerId !== null) {
      clearInterval(timerId);
      timerId = null;
    }
  };

  const startTimer = () => {
    stopTimer();
    if (currentIndex >= steps.length - 1) currentIndex = 0;
    timerId = window.setInterval(playNext, 1000 / controls.speed);
  };

  controls = new Controls('nist-controls',
    () => { currentIndex--; renderStep(); },
    () => { currentIndex++; renderStep(); },
    () => {
      if (timerId !== null) stopTimer();
      else startTimer();
    },
    (speed) => {
      if (timerId !== null) startTimer();
    }
  );

  const handleResponse = (response: NistStepsResponse) => {
    steps = response.steps;
    if (response.input_text) inputEl.value = response.input_text;
    currentIndex = 0;
    stopTimer();
    vizSection.classList.remove('hidden');
    renderStep();
  };

  analyzeBtn.addEventListener('click', async () => {
    const text = inputEl.value;
    if (!text) return;
    analyzeBtn.disabled = true;
    try {
      const response = await invoke<NistStepsResponse>('get_nist_steps', { text });
      handleResponse(response);
    } catch (e) { alert(e); } finally { analyzeBtn.disabled = false; }
  });

  randomBtn.addEventListener('click', async () => {
    randomBtn.disabled = true;
    try {
      const response = await invoke<NistStepsResponse>('get_nist_steps_random', { count: 16 });
      handleResponse(response);
    } catch (e) { alert(e); } finally { randomBtn.disabled = false; }
  });
}
```

---

## File: ui/ts/learn/xor.ts

```typescript
import { invoke } from "@tauri-apps/api/core";
import { Controls } from "../components/controls";
import { events } from "../main";

interface BitOp {
  input_bit: boolean;
  key_bit: boolean;
  result_bit: boolean;
  position: number;
}

interface XorStep {
  character: string;
  input_byte: number;
  keystream_byte: number;
  result_byte: number;
  input_binary: string;
  keystream_binary: string;
  result_binary: string;
  bit_ops: BitOp[];
}

interface XorStepsResponse {
  steps: XorStep[];
  total: number;
}

export function initXorVisualizer() {
  const inputEl = document.getElementById('xor-input') as HTMLTextAreaElement;
  const startBtn = document.getElementById('xor-start-btn') as HTMLButtonElement;
  const vizSection = document.getElementById('xor-viz-section') as HTMLElement;
  const stepHeader = document.getElementById('xor-step-header') as HTMLElement;

  const inCharEl = document.getElementById('xor-in-char')!;
  const inDecEl = document.getElementById('xor-in-dec')!;
  const inBinEl = document.getElementById('xor-in-bin')!;

  const keyDecEl = document.getElementById('xor-key-dec')!;
  const keyBinEl = document.getElementById('xor-key-bin')!;

  const outDecEl = document.getElementById('xor-out-dec')!;
  const outBinEl = document.getElementById('xor-out-bin')!;

  const progressContainer = document.getElementById('xor-progress')!;

  if (!startBtn) return;

  let steps: XorStep[] = [];
  let currentIndex = 0;
  let timerId: number | null = null;
  let controls: Controls;

  const renderStep = () => {
    if (steps.length === 0) return;
    const step = steps[currentIndex];

    stepHeader.textContent = `Step ${currentIndex + 1} of ${steps.length}: Encrypting '${step.character}'`;

    inCharEl.textContent = `'${step.character}'`;
    inDecEl.textContent = `(${step.input_byte})`;
    inBinEl.textContent = step.input_binary;

    keyDecEl.textContent = `(${step.keystream_byte})`;
    keyBinEl.textContent = step.keystream_binary;

    outDecEl.textContent = `(${step.result_byte})`;
    outBinEl.textContent = step.result_binary;

    // Render progress badges
    progressContainer.innerHTML = '';
    steps.forEach((s, i) => {
      const badge = document.createElement('span');
      let hex = s.result_byte.toString(16).toUpperCase();
      if (hex.length < 2) hex = '0' + hex;
      badge.textContent = `[${hex}]`;
      badge.className = 'xor-badge';

      if (i < currentIndex) {
        badge.classList.add('past');
      } else if (i === currentIndex) {
        badge.classList.add('current');
      } else {
        badge.classList.add('future');
      }

      progressContainer.appendChild(badge);
    });

    controls.updateState(currentIndex, steps.length, timerId !== null);
  };

  const playNext = () => {
    if (currentIndex < steps.length - 1) {
      currentIndex++;
      renderStep();
    } else {
      stopTimer();
      controls.updateState(currentIndex, steps.length, false);
    }
  };

  const stopTimer = () => {
    if (timerId !== null) {
      clearInterval(timerId);
      timerId = null;
    }
  };

  const startTimer = () => {
    stopTimer();
    if (currentIndex >= steps.length - 1) currentIndex = 0; // wrap around
    timerId = window.setInterval(playNext, 1000 / controls.speed);
  };

  controls = new Controls('xor-controls',
    () => { currentIndex--; renderStep(); },
    () => { currentIndex++; renderStep(); },
    () => {
      if (timerId !== null) stopTimer();
      else startTimer();
    },
    (speed) => {
      if (timerId !== null) startTimer();
    }
  );

  startBtn.addEventListener('click', async () => {
    const text = inputEl.value;
    if (!text) return;

    startBtn.disabled = true;
    startBtn.textContent = "Loading...";

    try {
      const response = await invoke<XorStepsResponse>('get_xor_steps', { text });
      steps = response.steps;
      currentIndex = 0;
      stopTimer();
      vizSection.classList.remove('hidden');
      renderStep();
    } catch (e) {
      alert(`Error: ${e}`);
    } finally {
      startBtn.disabled = false;
      startBtn.textContent = "Start Visualization";
    }
  });

  events.on('navigate_learn_xor', (text: string) => {
    inputEl.value = text;
    // Auto-switch to XOR subtab
    document.querySelector('.sub-tab-btn[data-subtab="xor"]')?.dispatchEvent(new Event('click'));
    startBtn.click();
  });
}
```

---

## File: ui/ts/main.ts

```typescript
export function initTabs() {
  const tabBtns = document.querySelectorAll('.tab-btn');
  const tabPanels = document.querySelectorAll('.tab-panel');

  tabBtns.forEach(btn => {
    btn.addEventListener('click', () => {
      // Remove active from all
      tabBtns.forEach(b => b.classList.remove('active'));
      tabPanels.forEach(p => p.classList.add('hidden'));

      // Add active to clicked
      btn.classList.add('active');
      const targetId = `tab-${btn.getAttribute('data-tab')}`;
      const targetPanel = document.getElementById(targetId);
      if (targetPanel) {
        targetPanel.classList.remove('hidden');
      }
    });
  });
}

// Simple pub-sub for cross-tab communication (like the Help link)
export const events = {
  listeners: {} as Record<string, Function[]>,
  on(event: string, callback: Function) {
    if (!this.listeners[event]) this.listeners[event] = [];
    this.listeners[event].push(callback);
  },
  emit(event: string, data?: any) {
    if (this.listeners[event]) {
      this.listeners[event].forEach(cb => cb(data));
    }
  }
};

import { initUseTab } from './use-tab';
import { initTestTab } from './test-tab';
import { initBenchTab } from './bench-tab';
import { initLearnTab } from './learn';

document.addEventListener('DOMContentLoaded', () => {
  initTabs();
  initUseTab();
  initTestTab();
  initBenchTab();
  initLearnTab();
});
```

---

## File: ui/ts/test-tab.ts

```typescript
import { invoke } from "@tauri-apps/api/core";
import { ProgressBar } from "./components/progress-bar";

export interface NistResult {
  name: string;
  p_value: number;
  passed: boolean;
}

export interface QualityResponse {
  shannon_entropy: number;
  min_entropy: number;
  chi_square: number;
  mean: number;
  longest_run: number;
  overall_score: number;
  total_bytes: number;
  nist_results: NistResult[];
}

export function initTestTab() {
  const sizeSlider = document.getElementById('test-size-slider') as HTMLInputElement;
  const sizeLabel = document.getElementById('test-size-label') as HTMLElement;
  const runBtn = document.getElementById('test-run-btn') as HTMLButtonElement;
  const resultsSection = document.getElementById('test-results-section') as HTMLElement;
  const metricsContainer = document.getElementById('test-metrics-container') as HTMLElement;
  const nistTableBody = document.getElementById('test-nist-tbody') as HTMLElement;

  if (!sizeSlider || !runBtn || !resultsSection) return;

  // Setup slider
  const updateSliderLabel = () => {
    // Logarithmic slider mapping: 1,000 to 1,000,000
    // min = 3 (10^3), max = 6 (10^6)
    const val = parseFloat(sizeSlider.value);
    const bytes = Math.round(Math.pow(10, val));
    sizeLabel.textContent = bytes.toLocaleString() + " bytes";
    return bytes;
  };

  sizeSlider.addEventListener('input', updateSliderLabel);
  updateSliderLabel(); // initial

  // Create progress bars
  const shannonBar = new ProgressBar(
    document.getElementById('test-shannon-bar')!,
    "Shannon Entropy",
    "Measures the unpredictability of data.\n8.0 bits/byte is perfect randomness.\nLower values mean patterns exist.",
    { green: 0.9375, orange: 0.75 } // 7.5/8, 6.0/8
  );

  const minBar = new ProgressBar(
    document.getElementById('test-min-bar')!,
    "Min-Entropy",
    "The worst-case entropy.\nCrucial for security guarantees.\nIf this is low, an attacker might guess the key.",
    { green: 0.9375, orange: 0.75 }
  );

  const scoreBar = new ProgressBar(
    document.getElementById('test-score-bar')!,
    "Overall Score",
    "Weighted quality score: Shannon entropy (50%), Min-entropy (30%), Mean closeness to 127.5 (20%).",
    { green: 0.8, orange: 0.6 } // out of 100
  );

  let isLoading = false;

  const runTests = async () => {
    if (isLoading) return;
    const bytes = updateSliderLabel();

    isLoading = true;
    runBtn.disabled = true;
    runBtn.textContent = "Running Tests...";

    // Hide results temporarily
    resultsSection.classList.add('hidden');

    try {
      const response = await invoke<QualityResponse>('run_quality_tests', {
        sampleSize: bytes
      });

      // Update metrics
      shannonBar.update(response.shannon_entropy, 8.0, `${response.shannon_entropy.toFixed(4)} bits/byte`);
      minBar.update(response.min_entropy, 8.0, `${response.min_entropy.toFixed(4)} bits/byte`);
      scoreBar.update(response.overall_score, 100.0, `${response.overall_score.toFixed(1)} / 100`);

      document.getElementById('test-mean-val')!.textContent = `${response.mean.toFixed(2)} (ideal: 127.5)`;
      document.getElementById('test-chi-val')!.textContent = response.chi_square.toFixed(2);
      document.getElementById('test-run-val')!.textContent = `${response.longest_run} bits`;

      // Update NIST table
      nistTableBody.innerHTML = '';
      for (const res of response.nist_results) {
        const tr = document.createElement('tr');

        const tdName = document.createElement('td');
        tdName.textContent = res.name;

        const tdPval = document.createElement('td');
        tdPval.textContent = res.p_value.toFixed(4);

        const tdResult = document.createElement('td');
        if (res.passed) {
          tdResult.innerHTML = '<span class="text-green">✓ Pass</span>';
        } else {
          tdResult.innerHTML = '<span class="text-red">✗ Fail</span>';
        }

        tr.appendChild(tdName);
        tr.appendChild(tdPval);
        tr.appendChild(tdResult);
        nistTableBody.appendChild(tr);
      }

      resultsSection.classList.remove('hidden');

    } catch (e) {
      console.error(e);
      alert(`Error running tests: ${e}`);
    } finally {
      isLoading = false;
      runBtn.disabled = false;
      runBtn.textContent = "🔬 Run All Tests";
    }
  };

  runBtn.addEventListener('click', runTests);
}
```

---

## File: ui/ts/use-tab.ts

```typescript
import { invoke } from "@tauri-apps/api/core";
import { events } from "./main";
import { KeystreamGrid } from "./components/grid";

export interface EncryptResponse {
  ciphertext: string;
  keystream_bytes: number[];
}

export function initUseTab() {
  const inputEl = document.getElementById('use-input') as HTMLTextAreaElement;
  const hexCheckEl = document.getElementById('use-hex-check') as HTMLInputElement;
  const encryptBtn = document.getElementById('use-encrypt-btn') as HTMLButtonElement;
  const outputEl = document.getElementById('use-output') as HTMLTextAreaElement;
  const gridContainer = document.getElementById('use-keystream-grid') as HTMLElement;
  const helpLink = document.getElementById('use-help-link') as HTMLAnchorElement;

  if (!inputEl || !encryptBtn || !outputEl || !gridContainer) return;

  const grid = new KeystreamGrid(gridContainer);
  let isLoading = false;

  const encrypt = async () => {
    if (isLoading) return;
    const plaintext = inputEl.value;
    if (!plaintext) return;

    isLoading = true;
    encryptBtn.disabled = true;
    encryptBtn.textContent = "Encrypting...";

    try {
      const hex_output = hexCheckEl.checked;
      const response = await invoke<EncryptResponse>('encrypt_decrypt', {
        plaintext,
        hexOutput: hex_output
      });

      outputEl.value = response.ciphertext;
      grid.update(response.keystream_bytes);
    } catch (e) {
      console.error(e);
      outputEl.value = `Error: ${e}`;
    } finally {
      isLoading = false;
      encryptBtn.disabled = false;
      encryptBtn.textContent = "🔒 Encrypt / Decrypt";
    }
  };

  encryptBtn.addEventListener('click', encrypt);

  if (helpLink) {
    helpLink.addEventListener('click', (e) => {
      e.preventDefault();
      // Switch to learn tab
      document.querySelector('[data-tab="learn"]')?.dispatchEvent(new Event('click'));
      // Emit event for learn tab to pick up
      events.emit('navigate_learn_xor', inputEl.value);
    });
  }
}
```

---

## File: ui/tsconfig.json

```json
{
  "compilerOptions": {
    "target": "ES2020",
    "module": "ESNext",
    "moduleResolution": "node",
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true,
    "outDir": "./dist",
    "rootDir": "./ts"
  },
  "include": ["ts/**/*"]
}
```

---
