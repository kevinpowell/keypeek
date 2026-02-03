//! Display-oriented key representation.
//!
//! `LayoutKey` is the unified abstraction for representing a key's display labels,
//! independent of the source firmware (QMK, ZMK, etc.). It merges the functionality
//! of the old `KeycodeLabel` struct with additional fields for hold-tap behaviors.
//!
//! # Transparency
//! Transparent keys are represented as `None` when stored in collections like
//! `Vec<Vec<Vec<Option<LayoutKey>>>>`. This makes layer fall-through logic simple:
//! just check `key.is_some()`.

use crate::keycode_labels::{get_keycode_label, KeycodeKind, KeycodeLabel};
use qmk_via_api::keycodes::Keycode;

/// A key's display representation, containing all label variants and metadata.
///
/// This struct is firmware-agnostic: both QMK keycodes and ZMK bindings
/// are converted into this unified format for rendering.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutKey {
    /// Primary key action label (e.g., "A", "Enter", "L1")
    pub tap: String,

    /// Hold action label for hold-tap keys (e.g., "Shift" for MT(LSFT, KC_A))
    pub hold: Option<String>,

    /// Short version of the tap label (e.g., "Ent" for "Enter")
    pub short: Option<String>,

    /// Symbol/icon for the key (using Phosphor icon font)
    pub symbol: Option<String>,

    /// Visual classification for coloring
    pub kind: KeycodeKind,

    /// Layer this key activates (for MO, LT, TO, etc.) - used for coloring
    pub layer_ref: Option<u8>,
}

impl Default for LayoutKey {
    fn default() -> Self {
        LayoutKey {
            tap: String::new(),
            hold: None,
            short: None,
            symbol: None,
            kind: KeycodeKind::Basic,
            layer_ref: None,
        }
    }
}

impl LayoutKey {
    /// Create a simple key with just a tap label.
    pub fn simple(tap: impl Into<String>) -> Self {
        LayoutKey {
            tap: tap.into(),
            ..Default::default()
        }
    }

    /// Create a hold-tap key with both tap and hold labels.
    pub fn hold_tap(tap: impl Into<String>, hold: impl Into<String>) -> Self {
        LayoutKey {
            tap: tap.into(),
            hold: Some(hold.into()),
            kind: KeycodeKind::Modifier,
            ..Default::default()
        }
    }

    /// Create a layer-switching key.
    pub fn layer(tap: impl Into<String>, layer: u8) -> Self {
        LayoutKey {
            tap: tap.into(),
            kind: KeycodeKind::Modifier,
            layer_ref: Some(layer),
            ..Default::default()
        }
    }

    /// Create a LayoutKey from a QMK keycode.
    ///
    /// Returns `None` for `KC_TRANSPARENT` (0x0001), which should be represented
    /// as `None` in the key matrix to enable proper layer fall-through.
    pub fn from_qmk_keycode(keycode: u16) -> Option<Self> {
        // KC_TRANSPARENT is special: return None to represent transparency
        if keycode == Keycode::KC_TRANSPARENT as u16 {
            return None;
        }

        let label = get_keycode_label(keycode);
        Some(Self::from(label))
    }

    /// Get the best available label for display, considering available space.
    ///
    /// Priority: symbol > short > tap (truncated if needed)
    pub fn display_label(&self, max_chars: usize) -> &str {
        if let Some(symbol) = &self.symbol {
            return symbol;
        }
        if let Some(short) = &self.short {
            if short.len() <= max_chars {
                return short;
            }
        }
        &self.tap
    }
}

impl From<KeycodeLabel> for LayoutKey {
    fn from(label: KeycodeLabel) -> Self {
        LayoutKey {
            // Use long label as tap, fall back to empty string
            tap: label.long.unwrap_or_default(),
            hold: None, // KeycodeLabel doesn't have hold; advanced.rs handles this differently
            short: label.short,
            symbol: label.symbol,
            kind: label.kind,
            layer_ref: label.layer_ref,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_qmk_keycode_basic() {
        let key = LayoutKey::from_qmk_keycode(Keycode::KC_A as u16);
        assert!(key.is_some());
        let key = key.unwrap();
        assert_eq!(key.tap, "A");
        assert_eq!(key.kind, KeycodeKind::Basic);
    }

    #[test]
    fn test_from_qmk_keycode_transparent() {
        let key = LayoutKey::from_qmk_keycode(Keycode::KC_TRANSPARENT as u16);
        assert!(key.is_none());
    }

    #[test]
    fn test_from_qmk_keycode_no() {
        let key = LayoutKey::from_qmk_keycode(Keycode::KC_NO as u16);
        assert!(key.is_some());
        // KC_NO should produce an empty tap label
        let key = key.unwrap();
        assert_eq!(key.tap, "");
    }

    #[test]
    fn test_simple_constructor() {
        let key = LayoutKey::simple("A");
        assert_eq!(key.tap, "A");
        assert!(key.hold.is_none());
    }

    #[test]
    fn test_hold_tap_constructor() {
        let key = LayoutKey::hold_tap("A", "Shift");
        assert_eq!(key.tap, "A");
        assert_eq!(key.hold, Some("Shift".to_string()));
        assert_eq!(key.kind, KeycodeKind::Modifier);
    }

    #[test]
    fn test_layer_constructor() {
        let key = LayoutKey::layer("L1", 1);
        assert_eq!(key.tap, "L1");
        assert_eq!(key.layer_ref, Some(1));
        assert_eq!(key.kind, KeycodeKind::Modifier);
    }
}
