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
                      ObjCInto, objc_to_rust, objc_id_to_rust, objc_bool_to_rust};
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

// pub struct Nil(*mut rt::Object);
//
// pub static mut NIL: Nil = Nil(0_usize as *mut rt::Object);
//
// lazy_static! {
//     static ref NIL: Nil = Nil(ptr::null_mut());
// }


pub struct Nil(*mut AnyObject);

// TODO: Remove (if https://github.com/rust-lang/rfcs/pull/1414 gets accepted)
pub static mut NIL: Nil = Nil(0_usize as *mut AnyObject);

unsafe impl objc::Message for Nil { }
