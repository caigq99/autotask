use tauri::{AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};

pub struct MenuItem {
    pub id: String,
    pub title: String,
}
pub struct CustomMenu();

impl CustomMenu {
    pub fn new(menu_item_collect: Vec<MenuItem>) -> SystemTray {
        let menu_collect = Self::create_menu_collect(menu_item_collect);
        let tray_menu = Self::registry_sys_tray(menu_collect);
        Self::create_sys_tray(tray_menu)
    }

    // 创建 托盘菜单集合
    fn create_menu_collect(menu_item_collect: Vec<MenuItem>) -> Vec<CustomMenuItem> {
        let mut menu_collect: Vec<CustomMenuItem> = vec![];
        for menu_item in menu_item_collect {
            menu_collect.push(CustomMenuItem::new(menu_item.id, menu_item.title));
        }
        menu_collect
    }

    // 注册 托盘菜单项
    fn registry_sys_tray(menu_collect: Vec<CustomMenuItem>) -> SystemTrayMenu {
        let mut tray_menu = SystemTrayMenu::new();
        for menu in menu_collect {
            tray_menu = tray_menu.clone().add_item(menu);
        }
        tray_menu
    }

    // 生成系统托盘
    fn create_sys_tray(tray_menu: SystemTrayMenu) -> SystemTray {
        SystemTray::new().with_menu(tray_menu)
    }
}

pub fn system_tray_event_handle(app: &AppHandle, event: SystemTrayEvent) {
    let window = app.get_window("main").unwrap();
    match event {
        SystemTrayEvent::LeftClick { .. } => match window.is_visible() {
            Ok(true) => {
                window.hide().expect("hide error");
            }
            Ok(false) => {
                window
                    .set_always_on_top(true)
                    .expect("Failed to set always on top ");
                if let Ok(state) = window.is_minimized() {
                    match state {
                        true => {
                            window.unminimize().expect("Failed to unminimize");
                        }
                        _ => {}
                    }
                }
                window.show().expect("Failed to show window");
                window
                    .set_always_on_top(false)
                    .expect("Failed to unset always on top");
            }
            Err(_) => (),
        },
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "show" => {
                window
                    .set_always_on_top(true)
                    .expect("Failed to set always on top ");
                if let Ok(state) = window.is_minimized() {
                    match state {
                        true => {
                            window.unminimize().expect("Failed to unminimize");
                        }
                        _ => {}
                    }
                }
                window.show().expect("Failed to show window");
                window
                    .set_always_on_top(false)
                    .expect("Failed to unset always on top");
            }
            "hide" => window.hide().expect("hide error"),
            "quit" => {
                window.close().expect("Failed to close");
            }
            _ => {}
        },

        _ => {}
    }
}
