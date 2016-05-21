use std::mem;
use std::ptr;
use std::sync::{Once, ONCE_INIT};
use objc;
use objc::runtime as rt;
use objc::declare as decl;
use {Duck, Object, AnyObject, RawObjCObject, Id, ShareId, NSObject, NSWindow,
     IsNSWindowController, NSWindowController, SRBWrapper};
use super::{get_boxed_ref, new_wrapper_with_boxed};

#[repr(C)]
pub struct SRBWindowController {
    super_: NSWindowController
}

unsafe impl objc::Message for SRBWindowController { }

unsafe impl RawObjCObject for SRBWindowController { }

impl Object for SRBWindowController {
    type Super = NSWindowController;

    fn super_ref(&self) -> &Self::Super {
        &self.super_
    }

    fn super_mut(&mut self) -> &mut Self::Super {
        &mut self.super_
    }
}

impl SRBWrapper for SRBWindowController {
    type Boxed = Box<IsNSWindowController>;

    fn class_initializer() -> &'static Once {
        static CLASS_INIT: Once = ONCE_INIT;
        &CLASS_INIT
    }

    fn superclass() -> &'static rt::Class {
        rt::Class::get("NSWindowController").unwrap()
    }

    fn class_name() -> &'static str {
        "SRBWindowController"
    }

    fn create_class(class_decl: &mut decl::ClassDecl) {
        extern "C" fn load_window(self_: &AnyObject, _sel: rt::Sel) {
            unsafe {
                let self_: &SRBWindowController = mem::transmute(self_);
                let boxed = get_boxed_ref(self_);
                boxed.load_window();
            }
        }

        extern "C" fn show_window(self_: &AnyObject, _sel: rt::Sel, sender: *mut AnyObject) {
            unsafe {
                let self_: &SRBWindowController = mem::transmute(self_);
                let boxed = get_boxed_ref(self_);
                let sender = sender as *mut NSObject;
                let sender = if sender.is_null() {
                    None
                }
                else {
                    Some(ShareId::from_ptr(sender))
                };
                boxed.show_window(sender);
            }
        }

        extern "C" fn is_window_loaded(self_: &AnyObject, _sel: rt::Sel) -> rt::BOOL {
            unsafe {
                let self_: &SRBWindowController = mem::transmute(self_);
                let boxed = get_boxed_ref(self_);
                match boxed.is_window_loaded() {
                    true => rt::YES,
                    false => rt::NO
                }
            }
        }

        extern "C" fn window(self_: &AnyObject, _sel: rt::Sel) -> *const AnyObject {
            unsafe {
                let self_: &SRBWindowController = mem::transmute(self_);
                let boxed = get_boxed_ref(self_);
                match boxed.window() {
                    Some(window) => {
                        let window: *const NSWindow = &*window;
                        window as *const AnyObject
                    }
                    None => {
                        ptr::null_mut()
                    }
                }
            }
        }

        extern "C" fn set_window(self_: &AnyObject, _sel: rt::Sel, window: *mut AnyObject) {
            unsafe {
                let self_: &SRBWindowController = mem::transmute(self_);
                let boxed = get_boxed_ref(self_);

                let window = window as *mut NSWindow;
                let window = if window.is_null() {
                    None
                }
                else {
                    Some(ShareId::from_ptr(window))
                };
                boxed.set_window(window);
            }
        }

        extern "C" fn window_did_load(self_: &AnyObject, _sel: rt::Sel) {
            unsafe {
                let self_: &SRBWindowController = mem::transmute(self_);
                let boxed = get_boxed_ref(self_);
                boxed.window_did_load();
            }
        }

        extern "C" fn window_will_load(self_: &AnyObject, _sel: rt::Sel) {
            unsafe {
                let self_: &SRBWindowController = mem::transmute(self_);
                let boxed = get_boxed_ref(self_);
                boxed.window_will_load();
            }
        }

        let load_window = load_window as extern fn(&AnyObject, rt::Sel);
        let show_window = show_window as extern fn(&AnyObject, rt::Sel, *mut AnyObject);
        let is_window_loaded = is_window_loaded as extern fn(&AnyObject, rt::Sel) -> rt::BOOL;
        let window = window as extern fn(&AnyObject, rt::Sel) -> *const AnyObject;
        let set_window = set_window as extern fn(&AnyObject, rt::Sel, *mut AnyObject);
        let window_did_load = window_did_load as extern fn(&AnyObject, rt::Sel);
        let window_will_load = window_will_load as extern fn(&AnyObject, rt::Sel);

        unsafe {
            class_decl.add_method(sel!(loadWindow), load_window);
            class_decl.add_method(sel!(showWindow:), show_window);
            class_decl.add_method(sel!(isWindowLoaded), is_window_loaded);
            class_decl.add_method(sel!(window), window);
            class_decl.add_method(sel!(setWindow:), set_window);
            class_decl.add_method(sel!(windowDidLoad), window_did_load);
            class_decl.add_method(sel!(windowWillLoad), window_will_load);
        }
    }
}

impl IsNSWindowController for SRBWindowController {
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

impl SRBWindowController {
    pub fn new(controller: Box<IsNSWindowController>) -> Id<NSWindowController> {
        unsafe {
            let self_: *mut SRBWindowController = new_wrapper_with_boxed(Box::new(controller));
            let self_ = self_ as *mut NSWindowController;
            Id::from_retained_ptr(self_)
        }
    }
}



impl<T> Duck<Id<NSWindowController>> for T
    where T: IsNSWindowController + 'static
{
    fn duck(self) -> Id<NSWindowController> {
        SRBWindowController::new(Box::new(self))
    }
}
