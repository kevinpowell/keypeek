use crate::layout_key::{KeycodeKind, Label, LayoutKey};
use std::collections::HashMap;
use zmk_studio_api::Behavior;

use super::hid_usage::hid_usage_to_layout_key;

pub fn behavior_to_layout_key(
    behavior: &Behavior,
    layer_names: &HashMap<u32, String>,
) -> Option<LayoutKey> {
    match behavior {
        Behavior::Transparent => None,

        Behavior::None => Some(LayoutKey {
            tap: Label::new(""),
            ..Default::default()
        }),
        Behavior::KeyPress(keycode) => Some(hid_usage_to_layout_key(*keycode)),
        Behavior::KeyToggle(keycode) => {
            let mut key = hid_usage_to_layout_key(*keycode);
            key.hold = Some(Label::new("Toggle"));
            Some(key)
        }
        Behavior::MomentaryLayer { layer_id } => {
            Some(layer_layout_key("MO", *layer_id, layer_names))
        }
        Behavior::ToggleLayer { layer_id } => Some(layer_layout_key("TG", *layer_id, layer_names)),
        Behavior::ToLayer { layer_id } => Some(layer_layout_key("TO", *layer_id, layer_names)),
        Behavior::StickyLayer { layer_id } => Some(layer_layout_key("SL", *layer_id, layer_names)),
        Behavior::LayerTap { layer_id, tap } => {
            let tap_key = hid_usage_to_layout_key(*tap);
            let name = layer_names
                .get(layer_id)
                .cloned()
                .unwrap_or_else(|| format!("L{}", layer_id));
            let hold_label = Label::with_short(name.clone(), name);
            Some(LayoutKey {
                tap: combine_labels(tap_key.tap, hold_label.clone()),
                hold: Some(hold_label),
                symbol: tap_key.symbol,
                kind: KeycodeKind::Special,
                layer_ref: Some(*layer_id as u8),
            })
        }
        Behavior::ModTap { hold, tap } => {
            let hold_key = hid_usage_to_layout_key(*hold);
            let tap_key = hid_usage_to_layout_key(*tap);
            Some(LayoutKey {
                tap: combine_labels(tap_key.tap, hold_key.tap.clone()),
                hold: Some(hold_key.tap),
                symbol: tap_key.symbol,
                kind: KeycodeKind::Modifier,
                layer_ref: None,
            })
        }
        Behavior::StickyKey(keycode) => {
            let key = hid_usage_to_layout_key(*keycode);
            Some(LayoutKey {
                tap: Label::with_short(
                    format!("OS {}", key.tap.full),
                    format!("OS{}", key.tap.short.as_deref().unwrap_or(&key.tap.full)),
                ),
                kind: KeycodeKind::Modifier,
                ..Default::default()
            })
        }
        Behavior::CapsWord => Some(LayoutKey {
            tap: Label::with_short("Caps Word", "CW"),
            kind: KeycodeKind::Special,
            ..Default::default()
        }),
        Behavior::KeyRepeat => Some(LayoutKey {
            tap: Label::with_short("Key Repeat", "Rep"),
            kind: KeycodeKind::Special,
            ..Default::default()
        }),
        Behavior::Reset => Some(LayoutKey {
            tap: Label::new("Reset"),
            kind: KeycodeKind::Special,
            ..Default::default()
        }),
        Behavior::Bootloader => Some(LayoutKey {
            tap: Label::with_short("Bootloader", "Boot"),
            kind: KeycodeKind::Special,
            ..Default::default()
        }),
        Behavior::SoftOff => Some(LayoutKey {
            tap: Label::with_short("Soft Off", "Off"),
            kind: KeycodeKind::Special,
            ..Default::default()
        }),
        Behavior::StudioUnlock => Some(LayoutKey {
            tap: Label::with_short("Studio Unlock", "Unlock"),
            kind: KeycodeKind::Special,
            ..Default::default()
        }),
        Behavior::GraveEscape => Some(LayoutKey {
            tap: Label::with_short("Grave Esc", "G/E"),
            kind: KeycodeKind::Special,
            ..Default::default()
        }),
        Behavior::Bluetooth { command, .. } => {
            let label = match *command {
                0 => "BT Clr",
                1 => "BT Nxt",
                2 => "BT Prv",
                n => {
                    return Some(LayoutKey {
                        tap: Label::new(format!("BT {}", n)),
                        kind: KeycodeKind::Special,
                        ..Default::default()
                    })
                }
            };
            Some(LayoutKey {
                tap: Label::new(label),
                kind: KeycodeKind::Special,
                ..Default::default()
            })
        }
        Behavior::OutputSelection { value } => Some(LayoutKey {
            tap: Label::with_short(format!("Out {}", value), format!("Out{}", value)),
            kind: KeycodeKind::Special,
            ..Default::default()
        }),
        Behavior::ExternalPower { value } => Some(LayoutKey {
            tap: Label::with_short(format!("ExtPwr {}", value), format!("EP{}", value)),
            kind: KeycodeKind::Special,
            ..Default::default()
        }),
        Behavior::Backlight { command, .. } => Some(LayoutKey {
            tap: Label::with_short(format!("BL {}", command), format!("BL{}", command)),
            kind: KeycodeKind::Special,
            ..Default::default()
        }),
        Behavior::Underglow { command, .. } => Some(LayoutKey {
            tap: Label::with_short(format!("RGB {}", command), format!("RGB{}", command)),
            kind: KeycodeKind::Special,
            ..Default::default()
        }),
        Behavior::MouseKeyPress { value } => Some(LayoutKey {
            tap: Label::with_short(format!("Mouse {}", value), format!("M{}", value)),
            kind: KeycodeKind::Special,
            ..Default::default()
        }),
        Behavior::MouseMove { value } => Some(LayoutKey {
            tap: Label::with_short(format!("Move {}", value), format!("Mv{}", value)),
            kind: KeycodeKind::Special,
            ..Default::default()
        }),
        Behavior::MouseScroll { value } => Some(LayoutKey {
            tap: Label::with_short(format!("Scroll {}", value), format!("Scr{}", value)),
            kind: KeycodeKind::Special,
            ..Default::default()
        }),
        Behavior::Unknown {
            behavior_id,
            param1,
            param2,
        } => {
            // Heuristic for custom ModTap and LayerTap behaviors (e.g., u_mt, u_lt)
            // If param1 is a small integer (layer ID) and param2 is a HID usage, it's a LayerTap.
            if *param1 < 32 && *param2 >= 0x70000 {
                let tap_key =
                    hid_usage_to_layout_key(zmk_studio_api::HidUsage::from_encoded(*param2));
                let layer_id = *param1;
                let name = layer_names
                    .get(&layer_id)
                    .cloned()
                    .unwrap_or_else(|| format!("L{}", layer_id));
                let hold_label = Label::with_short(name.clone(), name);
                Some(LayoutKey {
                    tap: combine_labels(tap_key.tap, hold_label.clone()),
                    hold: Some(hold_label),
                    symbol: tap_key.symbol,
                    kind: KeycodeKind::Special,
                    layer_ref: Some(layer_id as u8),
                })
            }
            // If both param1 and param2 are HID usages, it's a ModTap.
            else if *param1 >= 0x70000 && *param2 >= 0x70000 {
                let tap_key =
                    hid_usage_to_layout_key(zmk_studio_api::HidUsage::from_encoded(*param2));
                let hold_key =
                    hid_usage_to_layout_key(zmk_studio_api::HidUsage::from_encoded(*param1));
                Some(LayoutKey {
                    tap: combine_labels(tap_key.tap, hold_key.tap.clone()),
                    hold: Some(hold_key.tap),
                    symbol: tap_key.symbol,
                    kind: KeycodeKind::Modifier,
                    layer_ref: None,
                })
            } else {
                let label = if *param2 != 0 {
                    format!("0x{:X} {} {}", behavior_id, param1, param2)
                } else if *param1 != 0 {
                    format!("0x{:X} {}", behavior_id, param1)
                } else {
                    format!("0x{:X}", behavior_id)
                };
                Some(LayoutKey {
                    tap: Label::new(label),
                    ..Default::default()
                })
            }
        }
    }
}

fn layer_layout_key(
    abbreviation: &str,
    layer_id: u32,
    layer_names: &HashMap<u32, String>,
) -> LayoutKey {
    let name = layer_names
        .get(&layer_id)
        .cloned()
        .unwrap_or_else(|| format!("L{}", layer_id));
    LayoutKey {
        tap: Label::with_short(
            format!("{} {}", abbreviation, name),
            format!("{}{}", abbreviation, name),
        ),
        kind: KeycodeKind::Special,
        layer_ref: Some(layer_id as u8),
        ..Default::default()
    }
}

fn combine_labels(tap: Label, hold: Label) -> Label {
    let full = if tap.full.is_empty() {
        format!("({})", hold.full)
    } else {
        format!("{}({})", tap.full, hold.full)
    };

    let short = match (tap.short, hold.short) {
        (Some(ts), Some(hs)) => Some(format!("{}({})", ts, hs)),
        (Some(ts), None) => Some(format!("{}({})", ts, hold.full)),
        (None, Some(hs)) => {
            if tap.full.is_empty() {
                Some(format!("({})", hs))
            } else {
                Some(format!("{}({})", tap.full, hs))
            }
        }
        (None, None) => None,
    };

    Label { full, short }
}
