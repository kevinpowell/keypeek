use crate::device_discovery::DiscoveredDevice;
use crate::settings::Settings;
use crate::tray::TraySetup;

use eframe::egui;

mod connection_flow;
mod settings_sync;
mod state;
mod ui_overlay;
mod ui_settings;
use state::{
    AppConnectionState, ConnectDraftState, ConnectionDraft, SessionState, SettingsState, UiState,
};

const SETTINGS_FILE: &str = "settings.ini";

pub struct OverlayApp {
    _tray_icon: tray_icon::TrayIcon,
    settings_id: tray_icon::menu::MenuId,
    quit_id: tray_icon::menu::MenuId,
    ui: UiState,
    settings: SettingsState,
    session: SessionState,
    connect: ConnectDraftState,
}

impl OverlayApp {
    pub fn new(
        tray_setup: TraySetup,
        base_settings: Settings,
        available_devices: Vec<DiscoveredDevice>,
    ) -> Self {
        Self {
            _tray_icon: tray_setup.tray_icon,
            settings_id: tray_setup.settings_id,
            quit_id: tray_setup.quit_id,
            ui: UiState {
                settings_visible: true,
                last_settings_visible: None,
                settings_error: None,
                settings_warning: None,
                #[cfg(target_os = "macos")]
                macos_maximized: false,
                file_dialog: egui_file_dialog::FileDialog::new(),
            },
            settings: SettingsState {
                active: base_settings.clone(),
                draft: base_settings,
            },
            session: SessionState {
                connection: AppConnectionState::Disconnected,
                ever_connected: false,
                connected_definition: None,
                layout_names: Vec::new(),
                active_layout_name: String::new(),
                draft_layout_name: String::new(),
            },
            connect: ConnectDraftState {
                available_devices,
                selected_device_index: None,
                draft: ConnectionDraft::Via {
                    json_path: String::new(),
                },
                pending_connect: None,
            },
        }
    }
}

impl eframe::App for OverlayApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        if self.ui.settings_visible {
            egui::Rgba::from_black_alpha(0.65).to_array()
        } else {
            egui::Rgba::TRANSPARENT.to_array()
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        #[cfg(target_os = "linux")]
        {
            while gtk::events_pending() {
                gtk::main_iteration_do(false);
            }
        }

        let ctx = ui.ctx();

        // Handle Tray Events
        if let Ok(tray_icon::TrayIconEvent::Click { .. }) =
            tray_icon::TrayIconEvent::receiver().try_recv()
        {
            self.ui.settings_visible = !self.ui.settings_visible;
        }

        if let Ok(event) = tray_icon::menu::MenuEvent::receiver().try_recv() {
            if event.id == self.settings_id {
                self.ui.settings_visible = !self.ui.settings_visible;
            } else if event.id == self.quit_id {
                std::process::exit(0);
            }
        }

        // On macOS, with_maximized(true) doesn't work for undecorated transparent
        // windows. Explicitly size the window to fill the monitor on the first frame.
        #[cfg(target_os = "macos")]
        if !self.ui.macos_maximized {
            if let Some(monitor_size) = ctx.input(|i| i.viewport().monitor_size) {
                ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(egui::pos2(0.0, 0.0)));
                ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(monitor_size));
                self.ui.macos_maximized = true;
            }
        }

        self.poll_connect_result();
        self.apply_live_visual_settings();
        self.apply_live_layout_settings();
        self.ui.file_dialog.update(ctx);

        if let Some(path) = self.ui.file_dialog.take_picked() {
            if let ConnectionDraft::Via { json_path } = &mut self.connect.draft {
                *json_path = path.to_string_lossy().to_string();
            }
            self.connect_from_ui(ctx);
        }

        if self.ui.last_settings_visible != Some(self.ui.settings_visible) {
            ctx.send_viewport_cmd(egui::ViewportCommand::MousePassthrough(
                !self.ui.settings_visible,
            ));
            self.ui.last_settings_visible = Some(self.ui.settings_visible);
        }

        if let AppConnectionState::Connected { keyboard } = &self.session.connection {
            if self.overlay_visible() {
                self.draw_overlay_window(ctx, keyboard);
            }
        }

        if self.ui.settings_visible {
            self.draw_settings_window(ctx);
        }

        if let Some(error_message) = self.ui.settings_error.clone() {
            egui::Window::new("Error")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.label(error_message);
                    ui.add_space(10.0);
                    if ui.button("OK").clicked() {
                        self.ui.settings_error = None;
                    }
                });
        }

        if let Some(warning_message) = self.ui.settings_warning.clone() {
            egui::Window::new("Notice")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.label(warning_message);
                    ui.add_space(10.0);
                    if ui.button("OK").clicked() {
                        self.ui.settings_warning = None;
                    }
                });
        }

        ctx.request_repaint_after(std::time::Duration::from_millis(500));
    }
}
