use objc;
use objc::runtime as rt;
use {Id, ShareId, Object, AnyObject,
     RawObjCObject, NSObject, NSResponder, IsNSResponder};

// NOTE: CGFloat can either be an f32 or an f64
pub type CGFloat = f64;

#[repr(C)]
pub struct NSPoint {
    pub x: CGFloat,
    pub y: CGFloat
}

unsafe impl objc::Encode for NSPoint {
    fn encode() -> objc::Encoding {
        let x = CGFloat::encode();
        let y = CGFloat::encode();
        let encoding = format!("{{_NSPoint={}{}}}", x.as_str(), y.as_str());
        unsafe { objc::Encoding::from_str(&encoding) }
    }
}

#[repr(C)]
pub struct NSSize {
    pub width: CGFloat,
    pub height: CGFloat
}

unsafe impl objc::Encode for NSSize {
    fn encode() -> objc::Encoding {
        let width = CGFloat::encode();
        let height = CGFloat::encode();
        let encoding = format!("{{_NSSize={}{}}}", width.as_str(), height.as_str());
        unsafe { objc::Encoding::from_str(&encoding) }
    }
}

#[repr(C)]
pub struct NSRect {
    pub origin: NSPoint,
    pub size: NSSize
}

unsafe impl objc::Encode for NSRect {
    fn encode() -> objc::Encoding {
        let origin = NSPoint::encode();
        let size = NSSize::encode();
        let encoding = format!("{{_NSRect={}{}}}", origin.as_str(), size.as_str());
        unsafe { objc::Encoding::from_str(&encoding) }
    }
}

pub fn ns_make_rect(x: f64, y: f64, width: f64, height: f64) -> NSRect {
    NSRect {
        origin: NSPoint { x: x, y: y },
        size: NSSize { width: width, height: height }
    }
}

#[repr(usize)]
pub enum NSBackingStoreType {
    Retained = 0,
    Nonretained = 1,
    Buffered = 2
}

#[repr(C)]
pub struct NSWindow {
    super_: NSResponder
}

unsafe impl objc::Message for NSWindow { }

unsafe impl RawObjCObject for NSWindow { }

impl Object for NSWindow {
    type Super = NSResponder;

    fn super_ref(&self) -> &Self::Super {
        &self.super_
    }

    fn super_mut(&mut self) -> &mut Self::Super {
        &mut self.super_
    }
}

impl NSWindow {
    pub fn new(content_rect: NSRect, style_mask: usize, backing: NSBackingStoreType, defer: bool) -> Id<Self> {
        unsafe {
            let defer = match defer {
                true => rt::YES,
                false => rt::NO
            };
            let ns_window = rt::Class::get("NSWindow").unwrap();
            let self_: *mut AnyObject = msg_send![ns_window, alloc];
            let self_: *mut AnyObject = msg_send![self_, initWithContentRect:content_rect styleMask:style_mask backing:backing as usize defer:defer];
            let self_ = self_ as *mut NSWindow;
            Id::from_retained_ptr(self_)
        }
    }
}

pub trait IsNSWindow: IsNSResponder {
    unsafe fn delegate(&self) -> Option<ShareId<NSObject>>;
    unsafe fn set_delegate(&self, delegate: Option<ShareId<NSObject>>);
    fn title(&self) -> String;
    fn set_title(&self, title: &str);
}

objc_trait! {
    pub unsafe objc trait IsNSWindow: IsNSResponder {
        type Base = NSWindow;
        trait Sub = SubNSWindow;

        unsafe fn delegate(&self) -> Option<ShareId<NSObject>>
            => [self, delegate] -> *mut AnyObject;
        unsafe fn set_delegate(&self, delegate: Option<ShareId<NSObject>>)
            => [self, setDelegate:(delegate: *mut AnyObject)];
        fn title(&self) -> String
            => [self, title] -> *mut AnyObject;
        fn set_title(&self, title: &str)
            => [self, setTitle:(title: *mut AnyObject)];
    }
}

#[macro_export]
macro_rules! NSWindow {
    ($args:tt) => {
        __objc_inheritance_for! {
            $crate::NSWindow => $crate::SubNSWindow: NSResponder!;
            $args
        }
    };
}
