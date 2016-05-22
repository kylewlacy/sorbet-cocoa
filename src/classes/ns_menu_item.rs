use objc;
use objc::runtime as rt;
use {Id, AnyObject, rust_to_objc, rust_to_objc_id,
     Object, OptionSel, RawObjCObject, NSObject, IsNSObject};

#[repr(C)]
pub struct NSMenuItem {
    super_: NSObject
}

unsafe impl objc::Message for NSMenuItem { }

unsafe impl RawObjCObject for NSMenuItem { }

impl Object for NSMenuItem {
    type Super = NSObject;

    fn super_ref(&self) -> &Self::Super {
        &self.super_
    }

    fn super_mut(&mut self) -> &mut Self::Super {
        &mut self.super_
    }
}

impl NSMenuItem {
    pub fn new(title: &str, action: Option<rt::Sel>, key_equivalent: &str) -> Id<Self> {
        unsafe {
            let title = rust_to_objc_id(title);
            let action: OptionSel = rust_to_objc(action);
            let key_equivalent = rust_to_objc_id(key_equivalent);

            let ns_menu_item = rt::Class::get("NSMenuItem").unwrap();
            let self_: *mut AnyObject = msg_send![ns_menu_item, alloc];
            let self_: *mut AnyObject = msg_send![self_, initWithTitle:title action:action keyEquivalent:key_equivalent];
            let self_ = self_ as *mut NSMenuItem;
            Id::from_retained_ptr(self_)
        }
    }
}

pub trait IsNSMenuItem: IsNSObject {

}

objc! {
    pub unsafe objc trait IsNSMenuItem: IsNSObject {
        type Base = NSMenuItem;
        trait Sub = SubNSMenuItem;
    }
}
