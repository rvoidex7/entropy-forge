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
