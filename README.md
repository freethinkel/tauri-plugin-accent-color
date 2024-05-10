# Tauri Plugin Accent Color

A Tauri plugin to get the system accent color for macOS

![Example](https://raw.githubusercontent.com/freethinkel/tauri-plugin-accent-color/main/examples/screenshots/example.gif)

## Install

```toml
# Cargo.toml
[dependencies]
tauri-plugin-accent-color = { git = "https://github.com/freethinkel/tauri-plugin-accent-color" }
```

```json
// package.json
"dependencies": {
  "tauri-plugin-accent-color": "git+https://github.com/freethinkel/tauri-plugin-accent-color"
}
```

## Using

```rust
// src-tauri/main.rs

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_accent_color::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

```

```ts
import { accentColor } from "tauri-plugin-accent-color";

console.log(accentColor.get());

accentColor.subscribe((color) => {
  document.documentElement.style.setProperty("--color-accent", color);
});
```

## Example

```sh
git clone https://github.com/freethinkel/tauri-plugin-accent-color
cd tauri-plugin-accent-color/example/tauri-app
npm install
npm run tauri dev
```
