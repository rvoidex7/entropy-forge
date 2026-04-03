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
