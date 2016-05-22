#![feature(specialization, type_macros)]

#[macro_use] extern crate objc;
extern crate objc_id;
extern crate objc_exception;

use std::mem;
use objc::runtime as rt;

pub use objc_id::{Id, WeakId, ShareId, Ownership, Owned, Shared};

#[macro_use] mod macros;
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



// NOTE: The type of `OptionSel` is used for Objective-C `SEL`s that
//       can be null. The safety of its implementation *depends* on
//       the `objc::runtime::Sel` type containing a single pointer!
#[derive(PartialEq)]
pub struct OptionSel {
    sel: rt::Sel
}

impl OptionSel {
    fn from_sel(sel: rt::Sel) -> OptionSel {
        OptionSel {
            sel: sel
        }
    }

    fn none() -> OptionSel {
        OptionSel {
            sel: unsafe { mem::zeroed() }
        }
    }
}

unsafe impl objc::Encode for OptionSel {
    fn encode() -> objc::Encoding {
        rt::Sel::encode()
    }
}



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
