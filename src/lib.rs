use cocoa::base::id;
use notification::NSNotificationCenter;
use objc::{class, msg_send, sel, sel_impl};
use tauri::{
    plugin::{Builder, TauriPlugin},
    AppHandle, Manager, Runtime,
};

use std::sync::Mutex;

mod color;
mod commands;
mod error;
mod notification;

use color::NSColorConverter;
pub use error::{Error, Result};

fn listen_change_accent_color<R: Runtime>(app: AppHandle<R>) {
    let notification = NSNotificationCenter::new(String::from("NSDistributedNotificationCenter"));
    let app = Mutex::new(app);

    let _ = notification.listen(
        String::from("AppleColorPreferencesChangedNotification"),
        move |_| {
            let app = app.lock().unwrap();
            let color: id = unsafe { msg_send![class!(NSColor), controlAccentColor] };
            let _ = app
                .app_handle()
                .emit("on_change_accent_color", color.to_rgba_string());
        },
    );
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("accent-color")
        .invoke_handler(tauri::generate_handler![commands::get_accent_color])
        .setup(|app, _| {
            #[cfg(target_os = "macos")]
            listen_change_accent_color(app.clone());

            Ok(())
        })
        .build()
}
