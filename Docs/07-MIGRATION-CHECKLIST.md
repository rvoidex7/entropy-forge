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
