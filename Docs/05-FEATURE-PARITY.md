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
