use std::mem;
use objc;
use {objc_bool_to_rust, objc_to_rust, Object, AnyObject, Id, ShareId,
     IsNSObject, NSObject, SubNSObject, NSApplication, NSNotification,
     IntoObjC, ObjCInto, SubAnyObject, RawObjCObject};

#[repr(usize)]
pub enum NSApplicationActivationPolicy {
    Regular = 0,
    Accessory = 1,
    Prohibited = 2
}

impl IntoObjC<usize> for NSApplicationActivationPolicy {
    fn into_objc(self) -> usize {
        self as usize
    }
}

impl ObjCInto<NSApplicationActivationPolicy> for usize {
    unsafe fn objc_into(self) -> NSApplicationActivationPolicy {
        // TODO: Do this safely!
        mem::transmute(self)
    }
}

#[repr(usize)]
pub enum NSApplicationTerminateReply {
    TerminateCancel = 0,
    TerminateNow = 1,
    TerminateLater = 2
}

impl IntoObjC<usize> for NSApplicationTerminateReply {
    fn into_objc(self) -> usize {
        self as usize
    }
}

impl ObjCInto<NSApplicationTerminateReply> for usize {
    unsafe fn objc_into(self) -> NSApplicationTerminateReply {
        // TODO: Do this safely!
        mem::transmute(self)
    }
}



pub struct NSApplicationDelegate {
    super_: NSObject
}

unsafe impl objc::Message for NSApplicationDelegate { }

unsafe impl RawObjCObject for NSApplicationDelegate { }

impl NSApplicationDelegate {
    pub unsafe fn from_object_unchecked(mut self_: Id<NSObject>) -> Id<Self> {
        let self_: *mut NSObject = &mut *self_;
        let self_ = self_ as *mut NSApplicationDelegate;
        Id::from_retained_ptr(self_)
    }
}

impl SubAnyObject for NSApplicationDelegate {
    type AnySuper = NSObject;

    fn any_super_ref(&self) -> &Self::AnySuper {
        &self.super_
    }

    fn any_super_mut(&mut self) -> &mut Self::AnySuper {
        &mut self.super_
    }
}

impl SubNSObject for NSApplicationDelegate {
    type ClassSuper = NSObject;

    fn class_super_ref(&self) -> &Self::ClassSuper {
        &self.super_
    }

    fn class_super_mut(&mut self) -> &mut Self::ClassSuper {
        &mut self.super_
    }
}

pub trait IsNSApplicationDelegate {
    fn application_will_finish_launching(&self, _notification: ShareId<NSNotification>)
    {

    }

    fn application_did_finish_launching(&self, _notification: ShareId<NSNotification>) {

    }

    fn application_should_terminate(&self, _sender: ShareId<NSApplication>)
        -> NSApplicationTerminateReply
    {
        NSApplicationTerminateReply::TerminateNow
    }

    fn application_should_terminate_after_last_window_closed(&self, _sender: ShareId<NSApplication>)
        -> bool
    {
        false
    }

    fn application_will_terminate(&self, _notification: ShareId<NSNotification>) {

    }
}

impl IsNSApplicationDelegate for NSApplicationDelegate {
    fn application_will_finish_launching(&self, notification: ShareId<NSNotification>) {
        unsafe {
            if objc_bool_to_rust(msg_send![self, respondsToSelector:sel!(applicationWillFinishLaunching:)]) {
                let notification_ptr: *const NSNotification = &*notification;
                let notification_ptr = notification_ptr as *const AnyObject;
                msg_send![self, applicationWillFinishLaunching:notification_ptr];
            }
        }
    }

    fn application_did_finish_launching(&self, notification: ShareId<NSNotification>) {
        unsafe {
            if objc_bool_to_rust(msg_send![self, respondsToSelector:sel!(applicationDidFinishLaunching:)]) {
                let notification_ptr: *const NSNotification = &*notification;
                let notification_ptr = notification_ptr as *const AnyObject;
                msg_send![self, applicationDidFinishLaunching:notification_ptr];
            }
        }
    }

    fn application_should_terminate(&self, sender: ShareId<NSApplication>)
        -> NSApplicationTerminateReply
    {
        unsafe {
            if objc_bool_to_rust(msg_send![self, respondsToSelector:sel!(applicationShouldTerminate:)]) {
                let sender_ptr: *const NSApplication = &*sender;
                let sender_ptr = sender_ptr as *const AnyObject;
                let application_should_terminate: usize = msg_send![self, applicationShouldTerminate:sender_ptr];
                objc_to_rust(application_should_terminate)
            }
            else {
                // TODO: DRY default impl for optional methods
                NSApplicationTerminateReply::TerminateNow
            }
        }
    }

    fn application_should_terminate_after_last_window_closed(&self, sender: ShareId<NSApplication>)
        -> bool
    {
        unsafe {
            if objc_bool_to_rust(msg_send![self, respondsToSelector:sel!(applicationShouldTerminate:)]) {
                let sender_ptr: *const NSApplication = &*sender;
                let sender_ptr = sender_ptr as *const AnyObject;
                objc_bool_to_rust(msg_send![self, applicationShouldTerminate:sender_ptr])
            }
            else {
                // TODO: DRY default impl for optional methods
                false
            }
        }
    }

    fn application_will_terminate(&self, notification: ShareId<NSNotification>) {
        unsafe {
            if objc_bool_to_rust(msg_send![self, respondsToSelector:sel!(applicationWillTerminate:)]) {
                let notification_ptr: *const NSNotification = &*notification;
                let notification_ptr = notification_ptr as *const AnyObject;
                msg_send![self, applicationWillTerminate:notification_ptr];
            }
        }
    }
}

impl<T> IsNSApplicationDelegate for T
    where T: IsNSObject + SubNSApplicationDelegate
{
    default fn application_will_finish_launching(&self, notification: ShareId<NSNotification>)
    {
        self.super_ns_application_delegate_ref().application_will_finish_launching(notification);
    }

    default fn application_did_finish_launching(&self, notification: ShareId<NSNotification>) {
        self.super_ns_application_delegate_ref().application_did_finish_launching(notification)
    }

    default fn application_should_terminate(&self, sender: ShareId<NSApplication>)
        -> NSApplicationTerminateReply
    {
        self.super_ns_application_delegate_ref().application_should_terminate(sender)
    }

    default fn application_should_terminate_after_last_window_closed(&self, sender: ShareId<NSApplication>)
        -> bool
    {
        self.super_ns_application_delegate_ref().application_should_terminate_after_last_window_closed(sender)
    }

    default fn application_will_terminate(&self, notification: ShareId<NSNotification>) {
        self.super_ns_application_delegate_ref().application_will_terminate(notification);
    }
}

pub trait SubNSApplicationDelegate {
    type SuperNSApplicationDelegate: IsNSApplicationDelegate;

    fn super_ns_application_delegate_ref(&self) -> &Self::SuperNSApplicationDelegate;
    fn super_ns_application_delegate_mut(&mut self) -> &mut Self::SuperNSApplicationDelegate;
}

impl<T> SubNSApplicationDelegate for T
    where T: Object, T::Super: IsNSApplicationDelegate
{
    type SuperNSApplicationDelegate = T::Super;

    fn super_ns_application_delegate_ref(&self) -> &Self::SuperNSApplicationDelegate {
        self.super_ref()
    }

    fn super_ns_application_delegate_mut(&mut self) -> &mut Self::SuperNSApplicationDelegate {
        self.super_mut()
    }
}
