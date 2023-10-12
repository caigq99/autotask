// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use autotask::sys_menu_handle::system_menu::{self, CustomMenu, MenuItem};

fn main() {
    let show = MenuItem {
        id: String::from("show"),
        title: String::from("显示主界面"),
    };
    let hide = MenuItem {
        id: String::from("hide"),
        title: String::from("隐藏"),
    };
    let quit = MenuItem {
        id: String::from("quit"),
        title: String::from("退出"),
    };
    let menu_item_collect: Vec<MenuItem> = vec![show, hide, quit];
    tauri::Builder::default()
        .system_tray(CustomMenu::new(menu_item_collect))
        .on_system_tray_event(|app, event| system_menu::system_tray_event_handle(app, event))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
