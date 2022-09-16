pub fn get_brightness(display_id: u32) -> Option<f32> {
    let mut brightness: f32 = -1.0;
    unsafe { DisplayServicesGetBrightness(display_id, &mut brightness) };
    match brightness {
        value if 0.0 <= value && value <= 1.0 => Some(value),
        _ => None,
    }
}

pub fn set_brightness(display_id: u32, value: f32) -> bool {
    unsafe { DisplayServicesSetBrightness(display_id, value) };
    get_brightness(display_id)
        .map(|brightness| brightness == value)
        .unwrap_or(false)
}

#[link(name = "DisplayServices", kind = "framework")]
extern "C" {
    fn DisplayServicesGetBrightness(dispaly_id: u32, brightness: *mut f32);
    fn DisplayServicesSetBrightness(display_id: u32, brightness: f32);
}
