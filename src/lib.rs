#![feature(specialization)]

#[macro_use] extern crate objc;
extern crate objc_id;
extern crate objc_exception;

use objc::runtime as rt;

pub use objc_id::{Id, WeakId, ShareId, Ownership, Owned, Shared};

mod conversions;
mod classes;
mod protocols;
mod wrappers;

pub use conversions::{IntoObjC, rust_to_objc, rust_to_objc_id, rust_to_objc_bool,
                      ObjCInto, objc_to_rust, objc_id_to_rust, objc_bool_to_rust,
                      AsAnyObject, SubAnyObject, FromAnyObject};
pub use classes::*;
pub use protocols::*;
pub use wrappers::*;

// Forces rustc to link to Obj-C frameworks
#[link(name = "Foundation", kind = "framework")]
extern { }

#[link(name = "AppKit", kind = "framework")]
extern { }



pub type AnyObject = rt::Object;

pub trait Object {
    type Super;

    fn super_ref(&self) -> &Self::Super;
    fn super_mut(&mut self) -> &mut Self::Super;
}

pub unsafe trait RawObjCObject: objc::Message { }

impl<T> FromAnyObject for T
    where T: RawObjCObject + Sized
{
    unsafe fn from_any(any: *mut AnyObject) -> ShareId<Self> {
        let self_ = any as *mut Self;
        ShareId::from_ptr(self_)
    }
}
