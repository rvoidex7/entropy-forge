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
