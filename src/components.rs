use bitflags::bitflags;
use cocoa::base::id;
use cocoa::foundation::{NSRect, NSUInteger};
use objc::{class, msg_send, sel, sel_impl};

#[link(name = "AppKit", kind = "framework")]
bitflags! {
  pub struct NSEventModifierFlags: NSUInteger {
      const NSAlphaShiftKeyMask                     = 1 << 16;
      const NSShiftKeyMask                          = 1 << 17;
      const NSControlKeyMask                        = 1 << 18;
      const NSAlternateKeyMask                      = 1 << 19;
      const NSCommandKeyMask                        = 1 << 20;
      const NSNumericPadKeyMask                     = 1 << 21;
      const NSHelpKeyMask                           = 1 << 22;
      const NSFunctionKeyMask                       = 1 << 23;
      const NSDeviceIndependentModifierFlagsMask    = 0xffff0000;
  }
}

pub trait NSNumber {
    unsafe fn as_u32(self) -> u32;
}

impl NSNumber for id {
    unsafe fn as_u32(self) -> u32 {
        msg_send![self, unsignedIntegerValue]
    }
}

pub trait NSSlider: Sized {
    unsafe fn alloc(_: Self) -> id {
        msg_send![class!(NSSlider), alloc]
    }

    unsafe fn init_with_frame_(self, frameRect: NSRect) -> id;
}

impl NSSlider for id {
    unsafe fn init_with_frame_(self, frameRect: NSRect) -> id {
        msg_send![self, initWithFrame: frameRect]
    }
}
