use cocoa::appkit::NSImage;
use cocoa::base::{id, nil};
use cocoa::foundation::{NSAutoreleasePool, NSData, NSString};
use objc::runtime::Object;
use objc::*;
use objc::{msg_send, sel, sel_impl};

pub unsafe fn nsstring_to_str<'a>(mut ns: *mut Object) -> &'a str {
    let is_astring: bool = msg_send![ns, isKindOfClass: class!(NSAttributedString)];
    if is_astring {
        ns = msg_send![ns, string];
    }
    let data = NSString::UTF8String(ns as id) as *const u8;
    let len = NSString::len(ns as id);
    let bytes = std::slice::from_raw_parts(data, len);
    std::str::from_utf8_unchecked(bytes)
}

#[allow(dead_code)]
pub fn nsstring_decode(str: *mut Object) -> String {
    use std::ffi::CStr;
    unsafe {
        let cstr: *const std::os::raw::c_char = msg_send![str, UTF8String];
        let rstr = CStr::from_ptr(cstr).to_string_lossy().into_owned();
        rstr
    }
}

pub fn app_relaunch() {
    unsafe {
        let bundle: id = msg_send![class!(NSBundle), mainBundle];
        let path: id = msg_send![bundle, executablePath];

        let proc_info: id = msg_send![class!(NSProcessInfo), processInfo];
        let proc_id: i32 = msg_send![proc_info, processIdentifier];
        let proc_id_str: id = NSString::alloc(nil)
            .init_str(&format!("{}", proc_id))
            .autorelease();

        let args: id = msg_send![class!(NSMutableArray), new];
        let _: id = msg_send![args, addObject: path];
        let _: id = msg_send![args, addObject: proc_id_str];
        let _: id = msg_send![class!(NSTask), launchedTaskWithLaunchPath:path arguments:args];
    }
}

pub struct AutoreleasePool {
    inner: id,
}

impl AutoreleasePool {
    pub fn new() -> Self {
        let inner: id = unsafe { NSAutoreleasePool::new(nil) };
        AutoreleasePool { inner }
    }
}

impl Drop for AutoreleasePool {
    fn drop(&mut self) {
        unsafe { NSAutoreleasePool::drain(self.inner) }
    }
}

pub fn make_nsstring(s: &str) -> id {
    unsafe { NSString::alloc(nil).init_str(s).autorelease() }
}

pub fn load_image<T>(array: &[T]) -> id {
    return unsafe {
        let data = NSData::dataWithBytes_length_(
            nil,
            array.as_ptr() as *const std::os::raw::c_void,
            array.len() as u64,
        );
        NSImage::initWithData_(NSImage::alloc(nil), data)
    };
}
