use std::ptr;
use objc;
use objc::runtime as rt;
use {objc_id_to_rust, rust_to_objc_id, Id, ShareId, WeakId, Object, AnyObject,
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
    unsafe fn delegate(&self) -> Option<WeakId<NSObject>>;
    unsafe fn set_delegate(&self, delegate: Option<WeakId<NSObject>>);
    fn title(&self) -> String;
    fn set_title(&self, title: &str);
}

impl IsNSWindow for NSWindow {
    unsafe fn delegate(&self) -> Option<WeakId<NSObject>> {
        let delegate: *mut AnyObject = msg_send![self, delegate];
        let delegate = delegate as *mut NSObject;
        if delegate.is_null() {
            None
        }
        else {
            let delegate = ShareId::from_retained_ptr(delegate);
            Some(WeakId::new(&delegate))
        }
    }

    unsafe fn set_delegate(&self, delegate: Option<WeakId<NSObject>>) {
        let delegate = delegate.and_then(|weak| weak.load());
        let delegate_ptr: *const NSObject = match delegate {
            Some(delegate) => &*delegate,
            None => ptr::null()
        };
        let delegate_ptr = delegate_ptr as *mut AnyObject;
        msg_send![self, setDelegate:delegate_ptr];
    }

    fn title(&self) -> String {
        unsafe { objc_id_to_rust(msg_send![self, title]) }
    }

    fn set_title(&self, title: &str) {
        unsafe { msg_send![self, setTitle:rust_to_objc_id(title)]; }
    }
}

impl<T> IsNSWindow for T
    where T: SubNSWindow + IsNSResponder
{
    unsafe fn delegate(&self) -> Option<WeakId<NSObject>> {
        self.super_ns_window_ref().delegate()
    }

    unsafe fn set_delegate(&self, delegate: Option<WeakId<NSObject>>) {
        self.super_ns_window_ref().set_delegate(delegate);
    }

    fn title(&self) -> String {
        self.super_ns_window_ref().title()
    }

    fn set_title(&self, title: &str) {
        self.super_ns_window_ref().set_title(title);
    }
}

pub trait SubNSWindow {
    type SuperNSWindow: IsNSWindow;

    fn super_ns_window_ref(&self) -> &Self::SuperNSWindow;
    fn super_ns_window_mut(&mut self) -> &mut Self::SuperNSWindow;
}

impl<T> SubNSWindow for T
    where T: Object, T::Super: IsNSWindow
{
    type SuperNSWindow = T::Super;

    fn super_ns_window_ref(&self) -> &Self::SuperNSWindow {
        self.super_ref()
    }

    fn super_ns_window_mut(&mut self) -> &mut Self::SuperNSWindow {
        self.super_mut()
    }
}
