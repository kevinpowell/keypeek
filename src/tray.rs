use image::load_from_memory;
use tray_icon::{menu::Menu, menu::MenuItem, Icon, TrayIcon, TrayIconBuilder};

fn create_icon() -> Icon {
    const ICON_BYTES: &[u8] = include_bytes!("../resources/icon.ico");

    let icon = load_from_memory(ICON_BYTES)
        .expect("Failed to load icon.")
        .into_rgba8();

    let (width, height) = icon.dimensions();
    Icon::from_rgba(icon.into_raw(), width, height).expect("Failed to create icon.")
}

pub struct TraySetup {
    pub tray_icon: TrayIcon,
    pub settings_id: tray_icon::menu::MenuId,
    pub quit_id: tray_icon::menu::MenuId,
}

pub fn create_tray_icon() -> TraySetup {
    #[cfg(target_os = "linux")]
    gtk::init().expect("Failed to initialize GTK. Is a display available?");

    let settings = MenuItem::new("Settings", true, None);
    let quit = MenuItem::new("Quit", true, None);
    let menu = Menu::new();
    menu.append(&settings).expect("Failed to append menu item.");
    menu.append(&quit).expect("Failed to append menu item.");

    let icon = create_icon();

    let tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_icon(icon)
        .with_tooltip("KeyPeek")
        .build()
        .unwrap();

    TraySetup {
        tray_icon,
        settings_id: settings.id().clone(),
        quit_id: quit.id().clone(),
    }
}
