use std::mem;
use objc;
use {objc_bool_to_rust, objc_to_rust, Object, AnyObject, ShareId,
     IsNSObject, NSObject, NSApplication, NSNotification,
     IntoObjC, ObjCInto, RawObjCObject};

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



#[derive(Default)]
struct NSApplicationDelegateDefaultImpl;

impl NSApplicationDelegateDefaultImpl {
    fn application_will_finish_launching(&self, _notification: ShareId<NSNotification>) {

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

#[repr(C)]
pub struct NSApplicationDelegate {
    super_: NSObject
}

unsafe impl objc::Message for NSApplicationDelegate { }

unsafe impl RawObjCObject for NSApplicationDelegate { }

impl Object for NSApplicationDelegate {
    type Super = NSObject;

    fn super_ref(&self) -> &Self::Super {
        &self.super_
    }

    fn super_mut(&mut self) -> &mut Self::Super {
        &mut self.super_
    }
}

pub trait IsNSApplicationDelegate {
    fn application_will_finish_launching(&self, notification: ShareId<NSNotification>) {
        NSApplicationDelegateDefaultImpl.application_will_finish_launching(notification)
    }

    fn application_did_finish_launching(&self, notification: ShareId<NSNotification>) {
        NSApplicationDelegateDefaultImpl.application_did_finish_launching(notification)
    }

    fn application_should_terminate(&self, sender: ShareId<NSApplication>)
        -> NSApplicationTerminateReply
    {
        NSApplicationDelegateDefaultImpl.application_should_terminate(sender)
    }

    fn application_should_terminate_after_last_window_closed(&self, sender: ShareId<NSApplication>)
        -> bool
    {
        NSApplicationDelegateDefaultImpl.application_should_terminate_after_last_window_closed(sender)
    }

    fn application_will_terminate(&self, notification: ShareId<NSNotification>) {
        NSApplicationDelegateDefaultImpl.application_will_terminate(notification)
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
                NSApplicationDelegateDefaultImpl.application_should_terminate(sender)
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
                NSApplicationDelegateDefaultImpl.application_should_terminate_after_last_window_closed(sender)
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
