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
