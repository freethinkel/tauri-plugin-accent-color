use tauri_plugin_accent_color;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_accent_color::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
