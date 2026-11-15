//! GUI visualization using egui.

mod app;

pub use app::{EntropyForgeApp, Tab};

/// Vertical gap between related form fields.
///
/// Shared by every screen so spacing stays consistent instead of being
/// re-invented as magic numbers in each render function.
pub(crate) const FIELD_GAP: f32 = 8.0;

/// Vertical gap between logical sections of a screen.
pub(crate) const SECTION_GAP: f32 = 20.0;
