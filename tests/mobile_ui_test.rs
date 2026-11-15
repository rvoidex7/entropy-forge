//! Headless narrow-screen (360x800 logical pixels) rendering test.
//!
//! This catches desktop-only fixed widths and colliding `egui::Id`s before an
//! Android build is attempted.

#![cfg(feature = "gui")]

use egui_kittest::kittest::{NodeT, Queryable};
use egui_kittest::{Harness, Node};
use entropy_forge::viz::{EntropyForgeApp, Tab};

const VIEWPORT_WIDTH: f32 = 360.0;

/// Recursively assert that every widget stays inside the viewport width.
///
/// A widget whose bounding box crosses the right edge would be clipped on a
/// phone-sized screen, which is exactly the regression this test guards
/// against.
fn assert_within_viewport_width(node: &Node<'_>, path: &str, pixels_per_point: f32) {
    for (index, child) in node.children().enumerate() {
        let accesskit_node = child.accesskit_node();
        let label = accesskit_node.label().unwrap_or_default();
        let role = format!("{:?}", accesskit_node.role());

        // Some synthetic/ignored nodes have no bounds; only laid-out widgets
        // can overflow.
        if let Some(bounds) = accesskit_node.bounding_box() {
            let min_x = bounds.x0 as f32 / pixels_per_point;
            let max_x = bounds.x1 as f32 / pixels_per_point;
            assert!(
                min_x >= -0.5 && max_x <= VIEWPORT_WIDTH + 0.5,
                "{path}/{index} {role} {label:?} exceeds the {VIEWPORT_WIDTH}px viewport: \
                 x = {min_x}..{max_x}"
            );
        }

        assert_within_viewport_width(&child, &format!("{path}/{index}"), pixels_per_point);
    }
}

#[test]
fn mobile_360px_viewport_renders_all_tabs_without_overflow_or_id_clash() {
    for tab in [Tab::Use, Tab::Test, Tab::Benchmark, Tab::Learn] {
        let app = EntropyForgeApp::for_tab(tab);
        let mut harness = Harness::builder()
            .with_size(egui::vec2(VIEWPORT_WIDTH, 800.0))
            // Icon glyphs are drawn as tofu headlessly; the desktop build loads
            // system emoji fonts instead.
            .allow_missing_glyphs()
            .build_eframe(|_cc| app);

        // Run several frames so immediate-mode layout settles and duplicate
        // IDs would be reported.
        harness.run_steps(3);
        assert_eq!(harness.state().current_tab(), tab);

        let root = harness.root();
        let pixels_per_point = harness.ctx.pixels_per_point();
        assert_within_viewport_width(&root, &format!("{tab:?}"), pixels_per_point);
    }
}

#[test]
fn use_tab_encrypt_then_decrypt_round_trip_through_widgets() {
    let mut harness = Harness::builder()
        .with_size(egui::vec2(VIEWPORT_WIDTH, 800.0))
        .allow_missing_glyphs()
        .build_eframe(|_cc| EntropyForgeApp::for_tab(Tab::Use));

    harness.run_steps(2);

    harness.get_by_label("🔒 Encrypt").click();
    harness.run_steps(2);

    let ciphertext = harness
        .get_by_label("Ciphertext (hex)")
        .value()
        .expect("ciphertext field should expose its value");
    assert_eq!(ciphertext.len(), "Hello, World!".len() * 2);
    assert!(ciphertext.chars().all(|c| c.is_ascii_hexdigit()));

    harness.get_by_label("↩ Use output as input").click();
    harness.run_steps(2);

    harness.get_by_label("🔓 Decrypt").click();
    harness.run_steps(2);

    let plaintext = harness
        .get_by_label("Plaintext")
        .value()
        .expect("plaintext field should expose its value");
    assert_eq!(plaintext, "Hello, World!");
}
