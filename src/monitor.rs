use crate::brightness_services::get_brightness;
use crate::components::NSNumber;
use crate::utils::{make_nsstring, nsstring_to_str, AutoreleasePool};
use cocoa::appkit::NSScreen;
use cocoa::base::{id, nil};
use cocoa::foundation::{NSArray, NSDictionary, NSRect};
use core_graphics::display::CGDirectDisplayID;
use objc::runtime::{Object, BOOL, YES};
use objc::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Monitor {
    pub id: CGDirectDisplayID,
    pub name: String,
    pub temp: u32,
    pub brightness: f32,
}

pub fn get_monitor_list() -> Vec<Monitor> {
    let screens = unsafe { NSScreen::screens(nil) };
    let mut monitor_list = Vec::new();
    for index in 0..unsafe { NSArray::count(screens) } {
        let screen = unsafe { screens.objectAtIndex(index) };
        let screen_info = screen_info(screen);
        monitor_list.push(screen_info);
    }
    monitor_list
}

fn screen_info(screen: *mut Object) -> Monitor {
    let frame = screen_backing_frame(screen);
    let has_name: BOOL = unsafe { msg_send!(screen, respondsToSelector: sel!(localizedName)) };
    let name = if has_name == YES {
        unsafe { nsstring_to_str(msg_send!(screen, localizedName)) }.to_string()
    } else {
        format!(
            "{}x{}@{},{}",
            frame.size.width, frame.size.height, frame.origin.x, frame.origin.y
        )
    };
    // let has_max_fps: BOOL =
    //     unsafe { msg_send!(screen, respondsToSelector: sel!(maximumFramesPerSecond)) };
    // let max_fps = if has_max_fps == YES {
    //     let max_fps: NSInteger = unsafe { msg_send!(screen, maximumFramesPerSecond) };
    //     Some(max_fps as usize)
    // } else {
    //     None
    // };
    let device_id = device_id(screen);
    let brightness = get_brightness(device_id);
    Monitor {
        id: device_id,
        name,
        temp: 6500,
        brightness: brightness.unwrap_or(-1.0),
    }
}

fn screen_backing_frame(screen: *mut Object) -> NSRect {
    unsafe {
        let frame = NSScreen::frame(screen);
        NSScreen::convertRectToBacking_(screen, frame)
    }
}

#[allow(non_upper_case_globals)]
static NSScreenNumber: &'static str = "NSScreenNumber";

fn device_id(screen: id) -> CGDirectDisplayID {
    let _pool = AutoreleasePool::new();
    let key = make_nsstring(NSScreenNumber);

    unsafe {
        let description = NSScreen::deviceDescription(screen);
        let number_object = description.objectForKey_(key);
        number_object.as_u32() as CGDirectDisplayID
    }
}
