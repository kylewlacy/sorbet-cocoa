use objc;
use objc::runtime as rt;
use {Object, ShareId, RawObjCObject, NSResponder, IsNSResponder,
     NSApplicationDelegate, NSApplicationActivationPolicy, objc_bool_to_rust};

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

impl IsNSApplication for NSApplication {
    fn set_delegate(&self, delegate: ShareId<NSApplicationDelegate>) {
        unsafe { msg_send![self, setDelegate:delegate]; }
    }

    fn activate_ignoring_other_apps(&self, flag: bool) {
        let flag = match flag {
            true => rt::YES,
            false => rt::NO
        };
        unsafe { msg_send![self, activateIgnoringOtherApps:flag]; }
    }

    fn set_activation_policy(&self, activation_policy: NSApplicationActivationPolicy) -> bool {
        let activation_policy = activation_policy as usize;
        unsafe {
            objc_bool_to_rust(msg_send![self, setActivationPolicy:activation_policy])
        }
    }

    fn run(&self) {
        unsafe { msg_send![self, run]; }
    }
}

impl<T> IsNSApplication for T
    where T: SubNSApplication + IsNSResponder
{
    default fn set_delegate(&self, delegate: ShareId<NSApplicationDelegate>) {
        self.super_ns_application_ref().set_delegate(delegate);
    }

    default fn activate_ignoring_other_apps(&self, flag: bool) {
        self.super_ns_application_ref().activate_ignoring_other_apps(flag);
    }

    default fn set_activation_policy(&self, activation_policy: NSApplicationActivationPolicy) -> bool {
        self.super_ns_application_ref().set_activation_policy(activation_policy)
    }

    default fn run(&self) {
        self.super_ns_application_ref().run();
    }
}

pub trait SubNSApplication {
    type SuperNSApplication: IsNSApplication;

    fn super_ns_application_ref(&self) -> &Self::SuperNSApplication;
    fn super_ns_application_mut(&mut self) -> &mut Self::SuperNSApplication;
}

impl<T> SubNSApplication for T
    where T: Object, T::Super: IsNSApplication
{
    type SuperNSApplication = T::Super;

    fn super_ns_application_ref(&self) -> &Self::SuperNSApplication {
        self.super_ref()
    }

    fn super_ns_application_mut(&mut self) -> &mut Self::SuperNSApplication {
        self.super_mut()
    }
}
