use std::mem;
use std::sync::{Once, ONCE_INIT};
use objc;
use objc::runtime as rt;
use objc::declare as decl;
use {Object, AnyObject, Id, NSObject,
     NSApplicationDelegate, IsNSApplicationDelegate,
     NSApplication, NSNotification, SRBWrapper};
use super::{get_boxed_ref, new_wrapper_with_boxed};

#[repr(C)]
pub struct SRBApplicationDelegate {
    super_: NSObject
}

unsafe impl objc::Message for SRBApplicationDelegate { }

impl Object for SRBApplicationDelegate {
    type Super = NSObject;

    fn super_ref(&self) -> &Self::Super {
        &self.super_
    }

    fn super_mut(&mut self) -> &mut Self::Super {
        &mut self.super_
    }
}

impl SRBWrapper for SRBApplicationDelegate {
    type Boxed = Box<IsNSApplicationDelegate>;

    fn class_initializer() -> &'static Once {
        static CLASS_INIT: Once = ONCE_INIT;
        &CLASS_INIT
    }

    fn superclass() -> &'static rt::Class {
        rt::Class::get("NSObject").unwrap()
    }

    fn class_name() -> &'static str {
        "SRBApplicationDelegate"
    }

    fn create_class(class_decl: &mut decl::ClassDecl) {
        extern "C" fn application_will_finish_launching(self_: &AnyObject, _sel: rt::Sel, notification: *mut AnyObject) {
            unsafe {
                let self_: &SRBApplicationDelegate = mem::transmute(self_);
                let boxed = get_boxed_ref(self_);

                let notification = notification as *mut NSNotification;
                let notification = Id::from_ptr(notification);
                boxed.application_will_finish_launching(notification);
            }
        }

        extern "C" fn application_did_finish_launching(self_: &AnyObject, _sel: rt::Sel, notification: *mut AnyObject) {
            unsafe {
                let self_: &SRBApplicationDelegate = mem::transmute(self_);
                let boxed = get_boxed_ref(self_);

                let notification = notification as *mut NSNotification;
                let notification = Id::from_ptr(notification);
                boxed.application_did_finish_launching(notification);
            }
        }

        extern "C" fn application_should_terminate(self_: &AnyObject, _sel: rt::Sel, application: *mut AnyObject) -> usize {
            unsafe {
                let self_: &SRBApplicationDelegate = mem::transmute(self_);
                let boxed = get_boxed_ref(self_);

                let application = application as *mut NSApplication;
                let application = Id::from_ptr(application);
                boxed.application_should_terminate(application) as usize
            }
        }

        extern "C" fn application_should_terminate_after_last_window_closed(self_: &AnyObject, _sel: rt::Sel, application: *mut AnyObject) -> rt::BOOL {
            unsafe {
                let self_: &SRBApplicationDelegate = mem::transmute(self_);
                let boxed = get_boxed_ref(self_);

                let application = application as *mut NSApplication;
                let application = Id::from_ptr(application);
                match boxed.application_should_terminate_after_last_window_closed(application) {
                    true => rt::YES,
                    false => rt::NO
                }
            }
        }

        extern "C" fn application_will_terminate(self_: &AnyObject, _sel: rt::Sel, notification: *mut AnyObject) {
            unsafe {
                let self_: &SRBApplicationDelegate = mem::transmute(self_);
                let boxed = get_boxed_ref(self_);

                let notification = notification as *mut NSNotification;
                let notification = Id::from_ptr(notification);
                boxed.application_will_terminate(notification);
            }
        }

        let application_will_finish_launching = application_will_finish_launching as extern fn(&AnyObject, rt::Sel, *mut AnyObject);
        let application_did_finish_launching = application_did_finish_launching as extern fn(&AnyObject, rt::Sel, *mut AnyObject);
        let applicaiton_should_terminate = application_should_terminate as extern fn(&AnyObject, rt::Sel, *mut AnyObject) -> usize;
        let application_should_terminate_after_last_window_closed = application_should_terminate_after_last_window_closed as extern fn(&AnyObject, rt::Sel, *mut AnyObject) -> rt::BOOL;
        let application_will_terminate = application_will_terminate as extern fn(&AnyObject, rt::Sel, *mut AnyObject);

        unsafe {
            class_decl.add_method(sel!(applicationWillFinishLaunching:), application_will_finish_launching);
            class_decl.add_method(sel!(applicationDidFinishLaunching:), application_did_finish_launching);
            class_decl.add_method(sel!(applicationShouldTerminate:), applicaiton_should_terminate);
            class_decl.add_method(sel!(applicationShouldTerminateAfterLastWindowClosed:), application_should_terminate_after_last_window_closed);
            class_decl.add_method(sel!(applicationWillTerminate:), application_will_terminate);
        }
        // let NSApplicationDelegate = rt::Protocol::get("NSApplicationDelegate").unwrap();
        // class_decl.add_protocol(NSApplicationDelegate);
    }
}

impl SRBApplicationDelegate {
    pub fn new(delegate: Box<IsNSApplicationDelegate>) -> Id<NSApplicationDelegate> {
        unsafe {
            let self_: *mut SRBApplicationDelegate = new_wrapper_with_boxed(Box::new(delegate));
            let self_ = self_ as *mut NSApplicationDelegate;
            Id::from_retained_ptr(self_)
        }
    }
}
