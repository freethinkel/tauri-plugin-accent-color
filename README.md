# Tauri Plugin Accent Color

A Tauri plugin to get the system accent color for macOS

<video playsinline muted autoplay loop>
  <source src="./examples/screenshots/example.mp4"/>
</video>

## Install

```toml
# Cargo.toml
[dependencies]
tauri-plugin-accent-color = { path = "../../../" }
```

```json
// package.json
"dependencies": {
  "tauri-plugin-accent-color": "link:../../"
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
git clone [the repo]
cd tauri-plugin-accent-color/example/tauri-app
npm install
npm run tauri dev
```
