const COMMANDS: &[&str] = &["get_accent_color"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
