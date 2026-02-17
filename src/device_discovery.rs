use crate::protocols::zmk_studio;
use qmk_via_api::scan::{scan_keyboards, KeyboardDeviceInfo};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeviceKind {
    Studio,
    Vial,
    Qmk,
}

impl DeviceKind {
    pub fn label(self) -> &'static str {
        match self {
            DeviceKind::Studio => "Studio",
            DeviceKind::Vial => "Vial",
            DeviceKind::Qmk => "QMK",
        }
    }
}

#[derive(Clone, Debug)]
pub struct DiscoveredDevice {
    pub base_name: String,
    pub vid: u16,
    pub pid: u16,
    pub serial_port: Option<String>,
    pub kind: DeviceKind,
}

impl DiscoveredDevice {
    pub fn display_name(&self) -> String {
        format!(
            "{} ({}, {:04X}:{:04X})",
            self.base_name,
            self.kind.label(),
            self.vid,
            self.pid
        )
    }
}

pub fn discover_devices() -> Vec<DiscoveredDevice> {
    let mut devices = Vec::new();

    for dev in scan_keyboards() {
        let base_name = dev
            .product
            .clone()
            .unwrap_or_else(|| format!("{:04X}:{:04X}", dev.vendor_id, dev.product_id));
        let kind = if is_vial_device(&dev) {
            DeviceKind::Vial
        } else {
            DeviceKind::Qmk
        };

        devices.push(DiscoveredDevice {
            base_name,
            vid: dev.vendor_id,
            pid: dev.product_id,
            serial_port: None,
            kind,
        });
    }

    for sp in zmk_studio::scan_serial_ports() {
        let already_listed = devices.iter().any(|d| d.vid == sp.vid && d.pid == sp.pid);
        let display_name = sp
            .product
            .unwrap_or_else(|| format!("{:04X}:{:04X}", sp.vid, sp.pid));

        if already_listed {
            if let Some(entry) = devices
                .iter_mut()
                .find(|d| d.vid == sp.vid && d.pid == sp.pid)
            {
                entry.serial_port = Some(sp.port_name);
                entry.kind = DeviceKind::Studio;
            }
        } else {
            devices.push(DiscoveredDevice {
                base_name: format!("{} [{}]", display_name, sp.port_name),
                vid: sp.vid,
                pid: sp.pid,
                serial_port: Some(sp.port_name),
                kind: DeviceKind::Studio,
            });
        }
    }

    devices
}

fn is_vial_device(dev: &KeyboardDeviceInfo) -> bool {
    dev.serial_number
        .as_deref()
        .is_some_and(|s| s.to_ascii_lowercase().starts_with("vial:"))
}

#[cfg(test)]
mod tests {
    use super::{DeviceKind, DiscoveredDevice};

    #[test]
    fn display_name_uses_kind_label() {
        let board = DiscoveredDevice {
            base_name: "Board".to_string(),
            vid: 0x1234,
            pid: 0xABCD,
            serial_port: None,
            kind: DeviceKind::Studio,
        };
        assert_eq!(board.display_name(), "Board (Studio, 1234:ABCD)");
    }

    #[test]
    fn kind_labels_match_expected_ui_text() {
        assert_eq!(DeviceKind::Studio.label(), "Studio");
        assert_eq!(DeviceKind::Vial.label(), "Vial");
        assert_eq!(DeviceKind::Qmk.label(), "QMK");
    }

    #[test]
    fn display_name_for_other_kinds() {
        let vial_board = DiscoveredDevice {
            base_name: "Board".to_string(),
            vid: 0,
            pid: 0,
            serial_port: None,
            kind: DeviceKind::Vial,
        };
        let qmk_board = DiscoveredDevice {
            base_name: "Board".to_string(),
            vid: 0x0A0B,
            pid: 0x0C0D,
            serial_port: None,
            kind: DeviceKind::Qmk,
        };
        assert_eq!(vial_board.display_name(), "Board (Vial, 0000:0000)");
        assert_eq!(qmk_board.display_name(), "Board (QMK, 0A0B:0C0D)");
    }
}
