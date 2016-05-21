use std::mem;
use std::sync::{Once, ONCE_INIT};
use objc;
use objc::runtime as rt;
use objc::declare as decl;
use {Duck, Object, AnyObject, RawObjCObject, Id,
     NSObject, IsNSObject, SRBWrapper};
use super::{get_boxed_ref, new_wrapper_with_boxed};

#[repr(C)]
pub struct SRBObject {
    super_: NSObject
}

unsafe impl objc::Message for SRBObject { }

unsafe impl RawObjCObject for SRBObject { }

impl Object for SRBObject {
    type Super = NSObject;

    fn super_ref(&self) -> &Self::Super {
        &self.super_
    }

    fn super_mut(&mut self) -> &mut Self::Super {
        &mut self.super_
    }
}

impl SRBWrapper for SRBObject {
    type Boxed = Box<IsNSObject>;

    fn class_initializer() -> &'static Once {
        static CLASS_INIT: Once = ONCE_INIT;
        &CLASS_INIT
    }

    fn class_name() -> &'static str {
        "SRBObject"
    }

    fn superclass() -> &'static rt::Class {
        rt::Class::get("NSObject").unwrap()
    }

    fn create_class(class_decl: &mut decl::ClassDecl) {
        extern "C" fn hash(self_: &AnyObject, _sel: rt::Sel) -> usize {
            unsafe {
                let self_: &SRBObject = mem::transmute(self_);
                let boxed = get_boxed_ref(self_);
                boxed.hash()
            }
        }

        let hash = hash as extern "C" fn(&AnyObject, rt::Sel) -> usize;

        unsafe { class_decl.add_method(sel!(hash), hash); }
    }
}

impl SRBObject {
    pub fn new(object: Box<IsNSObject>) -> Id<NSObject> {
        unsafe {
            let self_: *mut SRBObject = new_wrapper_with_boxed(Box::new(object));
            let self_ = self_ as *mut NSObject;
            Id::from_retained_ptr(self_)
        }
    }
}



impl<T> Duck<Id<NSObject>> for T
    where T: IsNSObject + 'static
{
    default fn duck(self) -> Id<NSObject> {
        SRBObject::new(Box::new(self))
    }
}
