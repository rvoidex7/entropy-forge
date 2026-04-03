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
