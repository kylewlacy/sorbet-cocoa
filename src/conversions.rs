use std::os::raw::{c_void, c_char};
use std::ffi::CStr;
use objc::runtime as rt;
use AnyObject;

pub unsafe fn objc_to_rust<T, U>(x: T) -> U
    where T: ObjCInto<U>
{
    x.objc_into()
}

pub unsafe fn objc_id_to_rust<U>(id: &AnyObject) -> U
    where for<'a> &'a AnyObject: ObjCInto<U>
{
    objc_to_rust(id)
}

pub unsafe fn objc_bool_to_rust(x: rt::BOOL) -> bool {
    objc_to_rust(x)
}

pub fn rust_to_objc<T, U>(x: T) -> U
    where T: IntoObjC<U>
{
    x.into_objc()
}

pub fn rust_to_objc_id<T>(x: T) -> *mut AnyObject
    where T: IntoObjC<*mut AnyObject>
{
    rust_to_objc(x)
}

pub fn rust_to_objc_bool(x: bool) -> rt::BOOL {
    rust_to_objc(x)
}



pub trait ObjCInto<Out> {
    unsafe fn objc_into(self) -> Out;
}

impl<T> ObjCInto<T> for T {
    unsafe fn objc_into(self) -> T { self }
}

impl<'a> ObjCInto<String> for &'a AnyObject {
    unsafe fn objc_into(self) -> String {
        let c_str: *const c_void = msg_send![self, UTF8String];
        let c_str = CStr::from_ptr(c_str as *const c_char);
        c_str.to_str().unwrap().into()
    }
}

impl ObjCInto<bool> for rt::BOOL {
    unsafe fn objc_into(self) -> bool {
        self == rt::YES
    }
}

impl<T, U> ObjCInto<U> for T
    where T: Into<U>
{
    default unsafe fn objc_into(self) -> U { self.into() }
}



pub trait IntoObjC<Out> {
    fn into_objc(self) -> Out;
}

impl<T> IntoObjC<T> for T {
    fn into_objc(self) -> T { self }
}

impl IntoObjC<rt::BOOL> for bool {
    fn into_objc(self) -> rt::BOOL {
        match self {
            true => rt::YES,
            false => rt::NO
        }
    }
}

impl<'a> IntoObjC<*mut AnyObject> for &'a str {
    fn into_objc(self) -> *mut AnyObject {
        let c_str = self.as_ptr();
        let c_str = c_str as *const c_void;

        let ns_string = rt::Class::get("NSString").unwrap();
        unsafe { msg_send![ns_string, stringWithUTF8String:c_str] }
    }
}

impl<'a, T> IntoObjC<*const T> for &'a T {
    fn into_objc(self) -> *const T { self }
}

impl<T, U> IntoObjC<U> for T
    where T: Into<U>
{
    default fn into_objc(self) -> U { self.into() }
}
