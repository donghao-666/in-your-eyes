use crate::components::NSSlider;
use crate::gamma_control::{
    change_gamma, CGDisplayChangeSummaryFlags, CGDisplayRegisterReconfigurationCallback,
};
use crate::monitor::get_monitor_list;
use crate::utils::{app_relaunch, load_image, nsstring_decode};
use cocoa::quartzcore::CALayer;
use cocoa::{
    appkit::{
        NSApp, NSApplication, NSApplicationActivateIgnoringOtherApps, NSButton, NSMenu, NSMenuItem,
        NSRunningApplication, NSStatusBar, NSStatusItem, NSTextField, NSView,
    },
    base::{id, nil, NO, YES},
    foundation::{NSAutoreleasePool, NSPoint, NSRect, NSSize, NSString},
};
use core_graphics::color::CGColor;
use core_graphics::display::CGDirectDisplayID;
use libc::c_void;
use objc::declare::ClassDecl;
use objc::{
    class, msg_send,
    runtime::{Class, Object, Sel},
    sel, sel_impl,
};
use std::sync::Once;

pub fn run_app() {
    unsafe {
        let _pool = NSAutoreleasePool::new(nil);
        let app = NSApp();
        app.activateIgnoringOtherApps_(YES);

        let status_bar = NSStatusBar::systemStatusBar(nil).statusItemWithLength_(-1.0);
        // app_icon
        let status_bar_button = status_bar.button();
        let status_bar_image = load_image(include_bytes!("../assets/sun.max.fill.png"));
        let _: () = msg_send![status_bar_image, setTemplate: YES];
        let _: () = msg_send![status_bar_image, setSize: NSSize::new(15.0, 15.0)];
        status_bar_button.setImage_(status_bar_image);

        let status_bar_menu = NSMenu::new(nil).autorelease();
        let monitor_list = get_monitor_list();
        for monitor in monitor_list {
            // menu_view
            let status_bar_menu_parent_view = NSView::initWithFrame_(
                NSView::alloc(nil),
                NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(230.0, 90.0)),
            );
            let view_layer = CALayer::new();
            let _: () = msg_send![view_layer.id(), setBackgroundColor: CGColor::rgb(225.0, 225.0, 225.0, 0.5)];
            let _: () = msg_send![view_layer.id(), setCornerRadius: 7.0];
            let status_bar_menu_child_view = NSView::initWithFrame_(
                NSView::alloc(nil),
                NSRect::new(NSPoint::new(15.0, 10.0), NSSize::new(200.0, 70.0)),
            );
            status_bar_menu_child_view.setLayer(view_layer.id());
            let text_view = NSTextField::initWithFrame_(
                NSTextField::alloc(nil),
                NSRect::new(NSPoint::new(10.0, 30.0), NSSize::new(180.0, 30.0)),
            );
            let font: *mut Object = msg_send![class!(NSFont), boldSystemFontOfSize: 12.0];
            let _: () = msg_send![text_view, setEditable: NO];
            let _: () = msg_send![text_view, setDrawsBackground: NO];
            let _: () = msg_send![text_view, setBezeled: NO];
            let _: () = msg_send![text_view, setAlphaValue: 0.6];
            let _: () = msg_send![text_view, setFont: font];
            text_view.setStringValue_(NSString::alloc(nil).init_str(&monitor.name));
            status_bar_menu_child_view.addSubview_(text_view);
            // slider
            static mut RESPONDER_CLASS: *const Class = 0 as *const Class;
            static INIT: Once = Once::new();
            INIT.call_once(|| {
                let superclass = Class::get("NSObject").expect("slider - NSObject to exist");
                let mut decl = ClassDecl::new("SliderResponder", superclass)
                    .expect("slider - responder to declare");

                decl.add_ivar::<u64>("_name");

                decl.add_method(
                    sel!(onMouseMove:),
                    on_slider_move as extern "C" fn(this: &Object, _: Sel, _: id),
                );

                RESPONDER_CLASS = decl.register();
            });
            let responder: id = msg_send![RESPONDER_CLASS, new];
            let slider_rect = NSRect::new(NSPoint::new(10.0, 5.0), NSSize::new(180.0, 25.0));
            let slider = NSSlider::init_with_frame_(NSSlider::alloc(nil), slider_rect);
            let objc_text = NSString::alloc(nil).init_str(&monitor.id.to_string());
            (*responder).set_ivar("_name", objc_text as u64);
            let _: () = msg_send![slider, setTarget: responder];
            let _: () = msg_send![slider, setAction: sel!(onMouseMove:)];
            let _: () = msg_send![slider, setMinValue: 2500.0];
            let _: () = msg_send![slider, setMaxValue: 6500.0];
            status_bar_menu_child_view.addSubview_(slider);

            status_bar_menu_parent_view.addSubview_(status_bar_menu_child_view);
            let status_bar_menu_item = NSMenuItem::alloc(nil).autorelease();
            let _: () = msg_send![status_bar_menu_item, setView: status_bar_menu_parent_view];

            status_bar_menu.addItem_(status_bar_menu_item);
        }

        let status_bar_menu_item = NSMenuItem::alloc(nil).autorelease();
        let status_bar_menu_view = NSView::initWithFrame_(
            NSView::alloc(nil),
            NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(230.0, 30.0)),
        );
        let quite_image = load_image(include_bytes!("../assets/close.png"));
        let _: () = msg_send![quite_image, setSize: NSSize::new(25.0, 25.0)];
        let quite_button = NSButton::initWithFrame_(
            NSButton::alloc(nil),
            NSRect::new(NSPoint::new(190.0, 0.0), NSSize::new(25.0, 25.0)),
        );
        quite_button.setImage_(quite_image);
        quite_button.setAction_(sel!(terminate:));
        let _: () = msg_send![quite_button, setBordered: NO];
        status_bar_menu_view.addSubview_(quite_button);
        let _: () = msg_send![status_bar_menu_item, setView: status_bar_menu_view];
        status_bar_menu.addItem_(status_bar_menu_item);

        status_bar.setMenu_(status_bar_menu);

        let _result =
            CGDisplayRegisterReconfigurationCallback(display_callback, app as *const c_void);

        let current_app = NSRunningApplication::currentApplication(nil);
        current_app.activateWithOptions_(NSApplicationActivateIgnoringOtherApps);

        app.run();
    }
}

extern "C" fn on_slider_move(this: &Object, _cmd: Sel, target: id) {
    let name = unsafe {
        let ptr: u64 = *this.get_ivar("_name");
        nsstring_decode(ptr as id)
    };

    let value: f64 = unsafe { msg_send![target, doubleValue] };
    let device_temp = (6500 - (value as u16 / 100) * 100) + 2500;
    let device_id = name.parse().unwrap();
    change_gamma(device_id, device_temp);
}

unsafe extern "C" fn display_callback(
    _display: CGDirectDisplayID,
    _flags: CGDisplayChangeSummaryFlags,
    user_info: *const c_void,
) {
    app_relaunch();
    let app = user_info as id;
    let _: () = msg_send![app, terminate: nil];
}
