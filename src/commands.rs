use cocoa::base::id;
use objc::{class, msg_send, sel, sel_impl};
use tauri::{command, AppHandle, Runtime, Window};

use crate::{color::NSColorConverter, Result};

#[command]
pub(crate) async fn get_accent_color<R: Runtime>(
    _app: AppHandle<R>,
    _window: Window<R>,
) -> Result<String> {
    #[cfg(target_os = "macos")]
    {
        let color: id = unsafe { msg_send![class!(NSColor), controlAccentColor] };
        Ok(color.to_rgba_string())
    }

    #[cfg(not(target_os = "macos"))]
    Err()
}
