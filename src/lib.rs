#![feature(specialization)]

#[macro_use] extern crate objc;
extern crate objc_id;
extern crate objc_exception;

use std::os::raw::{c_void, c_char};
use std::ffi::CStr;
use objc::runtime as rt;

pub use objc_id::{Id, WeakId, ShareId, Ownership, Owned, Shared};

pub type AnyObject = rt::Object;

mod classes;
mod protocols;
mod wrappers;

pub use classes::*;
pub use protocols::*;
pub use wrappers::*;

// Forces rustc to link to Obj-C frameworks
#[link(name = "Foundation", kind = "framework")]
extern { }

#[link(name = "AppKit", kind = "framework")]
extern { }


fn into_bool(value: rt::BOOL) -> bool {
    value == rt::YES
}

fn into_string(value: &AnyObject) -> String {
    let c_str: *const c_void = unsafe { msg_send![value, UTF8String] };
    let c_str = unsafe { CStr::from_ptr(c_str as *const c_char) };
    c_str.to_str().unwrap().into()
}

fn from_string(value: &str) -> *mut AnyObject {
    let c_str = value.as_ptr();
    let c_str = c_str as *const c_void;
    let ns_string = rt::Class::get("NSString").unwrap();
    unsafe { msg_send![ns_string, stringWithUTF8String:c_str] }
}



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
