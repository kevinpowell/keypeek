use crate::keycode_labels::get_basic_layout_key;
use crate::layout_key::{KeycodeKind, Label, LayoutKey};
use zmk_studio_api::{Behavior, Keycode};

const PAGE_KEYBOARD: u32 = 0x0007;
const PAGE_KEYBOARD_SHIFTED: u32 = 0x0207;
const PAGE_CONSUMER: u32 = 0x000C;
const PAGE_SYSTEM: u32 = 0x0001;

pub fn behavior_to_layout_key(behavior: &Behavior) -> Option<LayoutKey> {
    match behavior {
        Behavior::Transparent => None,

        Behavior::None => Some(LayoutKey {
            tap: Label::new(""),
            ..Default::default()
        }),
        Behavior::KeyPress(keycode) => Some(keycode_to_layout_key(keycode)),
        Behavior::KeyToggle(keycode) => {
            let mut key = keycode_to_layout_key(keycode);
            key.hold = Some(Label::new("Toggle"));
            Some(key)
        }
        Behavior::MomentaryLayer { layer_id } => Some(layer_layout_key("MO", *layer_id)),
        Behavior::ToggleLayer { layer_id } => Some(layer_layout_key("TG", *layer_id)),
        Behavior::ToLayer { layer_id } => Some(layer_layout_key("TO", *layer_id)),
        Behavior::StickyLayer { layer_id } => Some(layer_layout_key("SL", *layer_id)),
        Behavior::LayerTap { layer_id, tap } => {
            let tap_key = keycode_to_layout_key(tap);
            Some(LayoutKey {
                tap: tap_key.tap,
                hold: Some(Label::with_short(
                    format!("L{}", layer_id),
                    format!("L{}", layer_id),
                )),
                symbol: tap_key.symbol,
                kind: KeycodeKind::Special,
                layer_ref: Some(*layer_id as u8),
            })
        }
        Behavior::ModTap { hold, tap } => {
            let hold_key = keycode_to_layout_key(hold);
            let tap_key = keycode_to_layout_key(tap);
            Some(LayoutKey {
                tap: tap_key.tap,
                hold: Some(hold_key.tap),
                symbol: tap_key.symbol,
                kind: KeycodeKind::Modifier,
                layer_ref: None,
            })
        }
        Behavior::StickyKey(keycode) => {
            let key = keycode_to_layout_key(keycode);
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
        Behavior::Raw(binding) => {
            let label = if binding.param2 != 0 {
                format!(
                    "0x{:X} {} {}",
                    binding.behavior_id, binding.param1, binding.param2
                )
            } else if binding.param1 != 0 {
                format!("0x{:X} {}", binding.behavior_id, binding.param1)
            } else {
                format!("0x{:X}", binding.behavior_id)
            };
            Some(LayoutKey {
                tap: Label::new(label),
                ..Default::default()
            })
        }
    }
}

fn layer_layout_key(abbreviation: &str, layer_id: u32) -> LayoutKey {
    LayoutKey {
        tap: Label::with_short(
            format!("{} {}", abbreviation, layer_id),
            format!("{}{}", abbreviation, layer_id),
        ),
        kind: KeycodeKind::Special,
        layer_ref: Some(layer_id as u8),
        ..Default::default()
    }
}

fn keycode_to_layout_key(keycode: &Keycode) -> LayoutKey {
    let raw = keycode.to_hid_usage();
    let page = (raw >> 16) & 0xFFFF;
    let usage_id = raw & 0xFFFF;

    if page == PAGE_KEYBOARD && usage_id <= 0xFF {
        // We reuse basic qmk keycodes here
        if let Some(key) = get_basic_layout_key(usage_id as u16) {
            return key;
        }
    }
    if page == PAGE_KEYBOARD_SHIFTED {
        if let Some(key) = shifted_key_label(usage_id) {
            return key;
        }
    }
    if page == PAGE_CONSUMER {
        if let Some(key) = consumer_key_label(usage_id) {
            return key;
        }
    }
    if page == PAGE_SYSTEM {
        if let Some(key) = system_key_label(usage_id) {
            return key;
        }
    }

    let name = keycode.to_name();
    LayoutKey {
        tap: Label::new(name),
        ..Default::default()
    }
}

fn shifted_key_label(usage_id: u32) -> Option<LayoutKey> {
    let (full, short, symbol): (&str, &str, Option<&str>) = match usage_id {
        0x1E => ("!", "!", None),
        0x1F => ("@", "@", None),
        0x20 => ("#", "#", None),
        0x21 => ("$", "$", None),
        0x22 => ("%", "%", None),
        0x23 => ("^", "^", None),
        0x24 => ("&", "&", None),
        0x25 => ("*", "*", None),
        0x2D => ("_", "_", None),
        0x2E => ("+", "+", None),
        0x31 => ("|", "|", None),
        0x32 => ("~", "~", None),
        0x33 => (":", ":", None),
        0x35 => ("~", "~", None),
        0x36 => ("<", "<", None),
        0x38 => ("?", "?", None),
        0x53 => ("Clear", "Clr", None),
        0x64 => ("|", "|", None),
        _ => return None,
    };

    Some(LayoutKey {
        tap: Label::with_short(full, short),
        symbol: symbol.map(String::from),
        ..Default::default()
    })
}

fn consumer_key_label(usage_id: u32) -> Option<LayoutKey> {
    // Labels and symbols match QMK basic.rs where possible
    match usage_id {
        // Power / Sleep
        0x0030 => Some(LayoutKey {
            tap: Label::new("Power"),
            ..Default::default()
        }),
        0x0032 => Some(LayoutKey {
            tap: Label::new("Sleep"),
            ..Default::default()
        }),
        // Menu
        0x0040 => Some(LayoutKey {
            tap: Label::new("Menu"),
            ..Default::default()
        }),
        // Transport controls
        0x00B0 => Some(LayoutKey {
            tap: Label::new("Play"),
            ..Default::default()
        }),
        0x00B1 => Some(LayoutKey {
            tap: Label::new("Pause"),
            ..Default::default()
        }),
        0x00B2 => Some(LayoutKey {
            tap: Label::with_short("Record", "Rec"),
            ..Default::default()
        }),
        0x00B3 => Some(LayoutKey {
            symbol: Some(egui_phosphor::regular::FAST_FORWARD.to_string()),
            ..Default::default()
        }),
        0x00B4 => Some(LayoutKey {
            symbol: Some(egui_phosphor::regular::REWIND.to_string()),
            ..Default::default()
        }),
        0x00B5 => Some(LayoutKey {
            symbol: Some(egui_phosphor::regular::SKIP_FORWARD.to_string()),
            ..Default::default()
        }),
        0x00B6 => Some(LayoutKey {
            symbol: Some(egui_phosphor::regular::SKIP_BACK.to_string()),
            ..Default::default()
        }),
        0x00B7 => Some(LayoutKey {
            symbol: Some(egui_phosphor::regular::STOP.to_string()),
            ..Default::default()
        }),
        0x00B8 => Some(LayoutKey {
            tap: Label::with_short("Eject", "Ejct"),
            ..Default::default()
        }),
        0x00CD => Some(LayoutKey {
            symbol: Some(egui_phosphor::regular::PLAY_PAUSE.to_string()),
            ..Default::default()
        }),
        // Volume
        0x00E2 => Some(LayoutKey {
            symbol: Some(egui_phosphor::regular::SPEAKER_X.to_string()),
            ..Default::default()
        }),
        0x00E9 => Some(LayoutKey {
            symbol: Some(egui_phosphor::regular::SPEAKER_HIGH.to_string()),
            ..Default::default()
        }),
        0x00EA => Some(LayoutKey {
            symbol: Some(egui_phosphor::regular::SPEAKER_LOW.to_string()),
            ..Default::default()
        }),
        // Application launch
        0x0183 => Some(LayoutKey {
            tap: Label::with_short("Select", "Sel"),
            ..Default::default()
        }),
        0x018A => Some(LayoutKey {
            tap: Label::new("Mail"),
            ..Default::default()
        }),
        0x0192 => Some(LayoutKey {
            tap: Label::new("Calc"),
            ..Default::default()
        }),
        0x0196 => Some(LayoutKey {
            tap: Label::new("WWW"),
            ..Default::default()
        }),
        0x0201 => Some(LayoutKey {
            tap: Label::new("New"),
            ..Default::default()
        }),
        0x0202 => Some(LayoutKey {
            tap: Label::new("Open"),
            ..Default::default()
        }),
        0x0203 => Some(LayoutKey {
            tap: Label::new("Close"),
            ..Default::default()
        }),
        0x0204 => Some(LayoutKey {
            tap: Label::new("Exit"),
            ..Default::default()
        }),
        0x0207 => Some(LayoutKey {
            tap: Label::new("Save"),
            ..Default::default()
        }),
        0x0208 => Some(LayoutKey {
            tap: Label::new("Print"),
            ..Default::default()
        }),
        0x021A => Some(LayoutKey {
            tap: Label::new("Undo"),
            ..Default::default()
        }),
        0x021B => Some(LayoutKey {
            tap: Label::new("Copy"),
            ..Default::default()
        }),
        0x021C => Some(LayoutKey {
            tap: Label::new("Cut"),
            ..Default::default()
        }),
        0x021D => Some(LayoutKey {
            tap: Label::new("Paste"),
            ..Default::default()
        }),
        0x021F => Some(LayoutKey {
            tap: Label::new("Find"),
            ..Default::default()
        }),
        0x0221 => Some(LayoutKey {
            tap: Label::new("Search"),
            ..Default::default()
        }),
        0x0222 => Some(LayoutKey {
            tap: Label::with_short("Go To", "GoTo"),
            ..Default::default()
        }),
        0x0223 => Some(LayoutKey {
            tap: Label::new("Home"),
            ..Default::default()
        }),
        0x0224 => Some(LayoutKey {
            tap: Label::new("Back"),
            ..Default::default()
        }),
        0x0225 => Some(LayoutKey {
            tap: Label::new("Forward"),
            ..Default::default()
        }),
        0x0226 => Some(LayoutKey {
            tap: Label::new("Stop"),
            ..Default::default()
        }),
        0x0227 => Some(LayoutKey {
            tap: Label::new("Refresh"),
            ..Default::default()
        }),
        0x022A => Some(LayoutKey {
            tap: Label::new("Favorites"),
            ..Default::default()
        }),
        0x022D => Some(LayoutKey {
            tap: Label::with_short("Zoom In", "Z+"),
            ..Default::default()
        }),
        0x022E => Some(LayoutKey {
            tap: Label::with_short("Zoom Out", "Z-"),
            ..Default::default()
        }),
        0x029D => Some(LayoutKey {
            symbol: Some(egui_phosphor::regular::SUN.to_string()),
            ..Default::default()
        }),
        _ => None,
    }
}

fn system_key_label(usage_id: u32) -> Option<LayoutKey> {
    let label: &str = match usage_id {
        0x81 => "Power",
        0x82 => "Sleep",
        0x83 => "Wake",
        _ => return None,
    };

    Some(LayoutKey {
        tap: Label::new(label),
        ..Default::default()
    })
}
