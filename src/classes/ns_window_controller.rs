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

impl IsNSWindowController for NSWindowController {
    fn load_window(&self) {
        unsafe {
            msg_send![self, loadWindow];
        }
    }

    fn show_window(&self, sender: Option<ShareId<NSObject>>) {
        let sender_ptr: *const NSObject = match sender {
            Some(sender) => &*sender,
            None => ptr::null()
        };
        let sender_ptr = sender_ptr as *const AnyObject;
        unsafe {
            msg_send![self, showWindow:sender_ptr];
        }
    }

    fn is_window_loaded(&self) -> bool {
        unimplemented!();
    }

    fn window(&self) -> Option<ShareId<NSWindow>> {
        unsafe {
            let window_ptr: *mut AnyObject = msg_send![self, window];
            let window_ptr = window_ptr as *mut NSWindow;
            if window_ptr.is_null() {
                None
            }
            else {
                Some(ShareId::from_retained_ptr(window_ptr))
            }
        }
    }

    fn set_window(&self, window: Option<ShareId<NSWindow>>) {
        let window_ptr = match window {
            Some(window) => &*window,
            None => ptr::null()
        };
        let window_ptr = window_ptr as *const NSObject;
        unsafe { msg_send![self, setWindow:window_ptr]; }
    }

    fn window_did_load(&self) {
        unsafe { msg_send![self, windowDidLoad]; }
    }

    fn window_will_load(&self) {
        unsafe { msg_send![self, windowWillLoad]; }
    }
}

impl<T> IsNSWindowController for T
    where T: SubNSWindowController + IsNSResponder
{
    default fn load_window(&self) {
        self.super_ns_window_controller_ref().load_window()
    }

    default fn show_window(&self, sender: Option<ShareId<NSObject>>) {
        self.super_ns_window_controller_ref().show_window(sender);
    }

    default fn is_window_loaded(&self) -> bool {
        self.super_ns_window_controller_ref().is_window_loaded()
    }

    default fn window(&self) -> Option<ShareId<NSWindow>> {
        self.super_ns_window_controller_ref().window()
    }

    default fn set_window(&self, window: Option<ShareId<NSWindow>>) {
        self.super_ns_window_controller_ref().set_window(window);
    }

    default fn window_did_load(&self) {
        self.super_ns_window_controller_ref().window_did_load();
    }

    default fn window_will_load(&self) {
        self.super_ns_window_controller_ref().window_will_load();
    }
}

pub trait SubNSWindowController {
    type SuperNSWindowController: IsNSWindowController;

    fn super_ns_window_controller_ref(&self) -> &Self::SuperNSWindowController;
    fn super_ns_window_controller_mut(&mut self) -> &mut Self::SuperNSWindowController;
}

impl<T> SubNSWindowController for T
    where T: Object, T::Super: IsNSWindowController
{
    type SuperNSWindowController = T::Super;

    fn super_ns_window_controller_ref(&self) -> &Self::SuperNSWindowController {
        self.super_ref()
    }

    fn super_ns_window_controller_mut(&mut self) -> &mut Self::SuperNSWindowController {
        self.super_mut()
    }
}
