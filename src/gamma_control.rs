use crate::gamma_color::{colorramp_fill_float, Color};
use core_graphics::display::CGDirectDisplayID;
use core_graphics_types::base::CGError;
use libc;
use libc::c_void;

pub type CGGammaValue = libc::c_float;

pub type CGDisplayChangeSummaryFlags = libc::c_uint;

pub type CGDisplayReconfigurationCallBack = unsafe extern "C" fn(
    display: CGDirectDisplayID,
    flags: CGDisplayChangeSummaryFlags,
    user_info: *const c_void,
);

pub fn change_gamma(id: u32, temp: u16) {
    let mut r: Vec<f32> = vec![0.0; 256];
    let mut g: Vec<f32> = vec![0.0; 256];
    let mut b: Vec<f32> = vec![0.0; 256];
    let mut color = Color::default();
    color.temp = temp;

    colorramp_fill_float(&mut r, &mut g, &mut b, 256, color);

    let _result = unsafe {
        CGSetDisplayTransferByTable(id, 256, r.as_mut_ptr(), g.as_mut_ptr(), b.as_mut_ptr())
    };
}

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
    pub fn CGGetOnlineDisplayList(
        max_displays: u32,
        display_id: *mut CGDirectDisplayID,
        display_count: *mut u32,
    ) -> CGError;

    pub fn CGSetDisplayTransferByTable(
        display: CGDirectDisplayID,
        tab_size: u32,
        r: *mut CGGammaValue,
        g: *mut CGGammaValue,
        b: *mut CGGammaValue,
    ) -> CGError;

    pub fn CGGetDisplayTransferByTable(
        display: CGDirectDisplayID,
        capacity: u32,
        r: *mut CGGammaValue,
        g: *mut CGGammaValue,
        b: *mut CGGammaValue,
        sampleCount: *mut u32,
    ) -> CGError;

    pub fn CGDisplayGammaTableCapacity(display: CGDirectDisplayID) -> usize;

    pub fn CGDisplayRegisterReconfigurationCallback(
        callback: CGDisplayReconfigurationCallBack,
        user_info: *const c_void,
    ) -> CGError;

    pub fn CGDisplayRestoreColorSyncSettings();
}
