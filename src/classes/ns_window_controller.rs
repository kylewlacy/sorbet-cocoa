use std::ptr;
use objc;
use objc::runtime as rt;
use {Id, ShareId, Object, AnyObject, RawObjCObject,
     NSObject, NSResponder, IsNSResponder, NSWindow};

#[repr(C)]
pub struct NSWindowController {
    super_: NSResponder
}

impl Object for NSWindowController {
    type Super = NSResponder;

    fn super_ref(&self) -> &Self::Super {
        &self.super_
    }

    fn super_mut(&mut self) -> &mut Self::Super {
        &mut self.super_
    }
}

unsafe impl objc::Message for NSWindowController { }

unsafe impl RawObjCObject for NSWindowController { }

impl NSWindowController {
    pub fn new(window: Option<ShareId<NSWindow>>) -> Id<Self> {
        let window_ptr: *const NSWindow = match window {
            Some(window) => &*window,
            None => ptr::null()
        };
        let window_ptr = window_ptr as *const AnyObject;

        let ns_window_controller = rt::Class::get("NSWindowController").unwrap();
        let id: *mut NSWindowController = unsafe {
            msg_send![ns_window_controller, alloc]
        };
        let id: *mut NSWindowController = unsafe {
            msg_send![id, initWithWindow:window_ptr]
        };

        unsafe { Id::from_retained_ptr(id) }
    }

    pub fn new_with_nib_name(_name: &str) -> Id<Self> {
        unimplemented!();
    }

    pub fn new_with_nib_name_owner(_name: &str, _owner: ShareId<NSObject>) -> Id<Self> {
        unimplemented!();
    }

    pub fn new_with_nib_path_owner(_path: &str, _owner: ShareId<NSObject>) -> Id<Self> {
        unimplemented!();
    }
}

pub trait IsNSWindowController: IsNSResponder {
    // fn new(window: Option<ShareId<NSWindow>>) -> Id<Self>
    //     where Self: Sized;
    // fn new_with_nib_name(name: &str) -> Id<Self>
    //     where Self: Sized;
    // fn new_with_nib_name_owner(name: &str, owner: ShareId<NSObject>) -> Id<Self>
    //     where Self: Sized;
    // fn new_with_nib_path_owner(path: &str, owner: ShareId<NSObject>) -> Id<Self>
    //     where Self: Sized;

    fn load_window(&self);
    fn show_window(&self, sender: Option<ShareId<NSObject>>);
    fn is_window_loaded(&self) -> bool;
    fn window(&self) -> Option<ShareId<NSWindow>>;
    fn set_window(&self, window: Option<ShareId<NSWindow>>);
    fn window_did_load(&self);
    fn window_will_load(&self);

    // unsafe fn document(&self) -> Option<WeakId<NSObject>>;
    // fn set_document_edited(&self, flag: bool);
    //
    // fn close(&self);
    // fn should_close_document(&self) -> bool;
    // fn set_should_close_document(&self, should_close_document: bool);
    //
    // unsafe fn owner(&self) -> WeakId<NSObject>;
    // fn storyboard(&self) -> Option<ShareId<NSStoryboard>>;
    // fn window_nib_name(&self) -> Option<String>;
    // fn window_nib_path(&self) -> Option<String>;
    //
    // fn should_cascade_windows(&self) -> bool;
    // fn set_should_cascade_windows(&self, should_cascade_windows: bool);
    // fn window_frame_autosave_name(&self) -> Option<String>;
    // fn set_window_frame_autosave_name(&self, window_frame_autosave_name: Option<String>);
    // fn synchronize_window_title_with_document_name(&self);
    // fn window_title_for_document_display_name(display_name: String) -> String;
    // fn content_view_controller(&self) -> Option<ShareId<NSViewController>>;
    // fn set_content_view_controller(&self, content_view_controller: Option<ShareId<NSViewController>>);
    // fn dismiss_controller(sender: Option<ShareId<NSObject>>);
}

objc! {
    pub unsafe objc trait IsNSWindowController: IsNSResponder {
        type Base = NSWindowController;
        trait Sub = SubNSWindowController;

        fn load_window(&self) => [self, loadWindow];
        fn show_window(&self, sender: Option<ShareId<NSObject>>)
            => [self, showWindow:(sender: *mut AnyObject)];
        fn is_window_loaded(&self) -> bool
            => [self, isWindowLoaded] -> rt::BOOL;
        fn window(&self) -> Option<ShareId<NSWindow>>
            => [self, window] -> *mut AnyObject;
        fn set_window(&self, window: Option<ShareId<NSWindow>>)
            => [self, setWindow:(window: *mut AnyObject)];
        fn window_did_load(&self) => [self, windowDidLoad];
        fn window_will_load(&self) => [self, windowWillLoad];
    }
}
