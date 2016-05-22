use objc;
use objc::runtime as rt;
use {AnyObject, Object, ShareId, RawObjCObject, NSResponder, IsNSResponder,
     NSMenu, NSApplicationDelegate, NSApplicationActivationPolicy};

#[repr(C)]
pub struct NSApplication {
    super_: NSResponder
}

unsafe impl objc::Message for NSApplication { }

unsafe impl RawObjCObject for NSApplication { }

impl Object for NSApplication {
    type Super = NSResponder;

    fn super_ref(&self) -> &Self::Super {
        &self.super_
    }

    fn super_mut(&mut self) -> &mut Self::Super {
        &mut self.super_
    }
}

pub trait IsNSApplication: IsNSResponder {
    fn set_delegate(&self, delegate: ShareId<NSApplicationDelegate>);
    fn main_menu(&self) -> Option<ShareId<NSMenu>>;
    fn set_main_menu(&self, main_menu: Option<ShareId<NSMenu>>);
    fn activate_ignoring_other_apps(&self, flag: bool);
    fn set_activation_policy(&self, activation_policy: NSApplicationActivationPolicy) -> bool;
    fn run(&self);
}

impl NSApplication {
    pub fn shared_application() -> ShareId<Self> {
        unsafe {
            let ns_application = rt::Class::get("NSApplication").unwrap();
            ShareId::from_retained_ptr(msg_send![ns_application, sharedApplication])
        }
    }
}

objc! {
    pub unsafe class trait IsNSApplication: IsNSResponder {
        type Base = NSApplication;
        trait Sub = SubNSApplication;

        fn set_delegate(&self, delegate: ShareId<NSApplicationDelegate>)
            => [self, setDelegate:(delegate: *mut AnyObject)];
        fn main_menu(&self) -> Option<ShareId<NSMenu>>
            => [self, mainMenu] -> *mut AnyObject;
        fn set_main_menu(&self, main_menu: Option<ShareId<NSMenu>>)
            => [self, setMainMenu:(menu: *mut AnyObject)];
        fn activate_ignoring_other_apps(&self, flag: bool)
            => [self, activateIgnoringOtherApps:(flag: rt::BOOL)];
        fn set_activation_policy(&self, activation_policy: NSApplicationActivationPolicy) -> bool
            => [self, setActivationPolicy:(policy: usize)] -> rt::BOOL;
        fn run(&self)
            => [self, run];
    }
}
