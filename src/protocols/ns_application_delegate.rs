use std::mem;
use objc;
use objc::runtime as rt;
use {Object, AnyObject, ShareId, IsNSObject, NSObject, NSApplication,
     NSNotification, IntoObjC, ObjCInto, RawObjCObject};

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

pub trait IsNSApplicationDelegate: IsNSObject {
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

objc! {
    pub unsafe objc trait IsNSApplicationDelegate: IsNSObject {
        type Base = NSApplicationDelegate;
        trait Sub = SubNSApplicationDelegate;
        type DefaultImpl = NSApplicationDelegateDefaultImpl;

        #[optional]
        fn application_will_finish_launching(&self, notification: ShareId<NSNotification>)
            => [self, applicationWillFinishLaunching:(notification: *mut AnyObject)];

        #[optional]
        fn application_did_finish_launching(&self, notification: ShareId<NSNotification>)
            => [self, applicationDidFinishLaunching:(notification: *mut AnyObject)];

        #[optional]
        fn application_should_terminate(&self, sender: ShareId<NSApplication>)
            -> NSApplicationTerminateReply
            => [self, applicationShouldTerminate:(sender: *mut AnyObject)]
                -> usize;

        #[optional]
        fn application_should_terminate_after_last_window_closed(&self, sender: ShareId<NSApplication>)
            -> bool
            => [self, applicationShouldTerminateAfterLastWindowClosed:(sender: *mut AnyObject)]
                -> rt::BOOL;

        #[optional]
        fn application_will_terminate(&self, notification: ShareId<NSNotification>)
            => [self, applicationWillTerminate:(notification: *mut AnyObject)];
    }
}
