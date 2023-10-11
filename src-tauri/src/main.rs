// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::{
    AppHandle, CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};
fn main() {
    let show = CustomMenuItem::new("show".to_string(), "显示主界面");
    let hide = CustomMenuItem::new("hide".to_string(), "隐藏");
    let quit = CustomMenuItem::new("quit".to_string(), "退出");

    let tray_menu = SystemTrayMenu::new()
        .add_item(show)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| menu_handle(app, event))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn menu_handle(_app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {
            println!("鼠标左键点击");
        }
        SystemTrayEvent::RightClick {
            position: _,
            size: _,
            ..
        } => {
            println!("鼠标右键点击");
        }
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "show" => {
                println!("{}", "点击显示主界面");
            }
            "quit" => {
                println!("{}", "点击退出");
                std::process::exit(0);
            }
            "hide" => {
                // let window = app.get_window("main").unwrap();
                // window.hide().unwrap();
                println!("{}", "点击隐藏")
            }
            _ => {}
        },
        _ => {}
    }
}
