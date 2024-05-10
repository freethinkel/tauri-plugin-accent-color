use cocoa::{appkit::CGFloat, base::id};
use objc::{class, msg_send, sel, sel_impl};

pub trait NSColorConverter: Sized {
    fn to_rgba_string(self: Self) -> String;
}

impl NSColorConverter for id {
    fn to_rgba_string(self: Self) -> String {
        let color = self;
        let cg_color: id = unsafe { msg_send![color, CGColor] };
        let color_id: cocoa::base::id =
            unsafe { msg_send![class!(CIColor), colorWithCGColor: cg_color] };
        let r: CGFloat = unsafe { msg_send![color_id, red] };
        let g: CGFloat = unsafe { msg_send![color_id, green] };
        let b: CGFloat = unsafe { msg_send![color_id, blue] };
        let a: CGFloat = unsafe { msg_send![color_id, alpha] };

        let formatted = format!("rgba({}, {}, {}, {})", r * 255.0, g * 255.0, b * 255.0, a);

        return formatted;
    }
}
