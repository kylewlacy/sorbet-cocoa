use std::ptr;
use std::mem;
use std::os::raw::{c_void, c_char};
use std::ffi::CStr;
use objc::runtime as rt;
use {AnyObject, ShareId, Object};

pub unsafe fn objc_to_rust<T, U>(x: T) -> U
    where T: ObjCInto<U>
{
    x.objc_into()
}

pub unsafe fn objc_id_to_rust<U>(id: *mut AnyObject) -> U
    where *mut AnyObject: ObjCInto<U>
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

impl<'a, T> ObjCInto<&'a T> for *const T {
    unsafe fn objc_into(self) -> &'a T {
        &*self
    }
}

impl<'a, T> ObjCInto<Option<&'a T>> for *const T {
    unsafe fn objc_into(self) -> Option<&'a T> {
        self.as_ref()
    }
}

impl<'a, T> ObjCInto<&'a T> for *mut T {
    unsafe fn objc_into(self) -> &'a T {
        &*self
    }
}

impl<'a, T> ObjCInto<Option<&'a T>> for *mut T {
    unsafe fn objc_into(self) -> Option<&'a T> {
        self.as_ref()
    }
}

impl<'a, T> ObjCInto<&'a mut T> for *mut T {
    unsafe fn objc_into(self) -> &'a mut T {
        &mut *self
    }
}

impl<'a, T> ObjCInto<Option<&'a mut T>> for *mut T {
    unsafe fn objc_into(self) -> Option<&'a mut T> {
        self.as_mut()
    }
}

impl<T> ObjCInto<ShareId<T>> for *mut AnyObject
    where T: FromAnyObject
{
    unsafe fn objc_into(self) -> ShareId<T> {
        T::from_any(self)
    }
}

impl<T> ObjCInto<Option<ShareId<T>>> for *mut AnyObject
    where T: FromAnyObject
{
    unsafe fn objc_into(self) -> Option<ShareId<T>> {
        if self.is_null() {
            None
        }
        else {
            Some(T::from_any(self))
        }
    }
}

impl ObjCInto<String> for *mut AnyObject {
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

impl<'a, T> IntoObjC<*const T> for Option<&'a T> {
    fn into_objc(self) -> *const T {
        match self {
            Some(x) => x,
            None => ptr::null()
        }
    }
}

impl<'a, T> IntoObjC<*const T> for Option<&'a mut T> {
    fn into_objc(self) -> *const T {
        match self {
            Some(x) => x,
            None => ptr::null()
        }
    }
}

impl<'a, T> IntoObjC<*mut T> for Option<&'a mut T> {
    fn into_objc(self) -> *mut T {
        match self {
            Some(x) => x,
            None => ptr::null_mut()
        }
    }
}

impl<'a, T> IntoObjC<*mut AnyObject> for &'a T
    where T: AsAnyObject
{
    fn into_objc(self) -> *mut AnyObject {
        self.any_ref() as *const _ as *mut _
    }
}

impl<'a, T> IntoObjC<*mut AnyObject> for Option<&'a T>
    where T: AsAnyObject
{
    fn into_objc(self) -> *mut AnyObject {
        match self {
            Some(x) => x.any_ref() as *const _ as *mut _,
            None => ptr::null_mut()
        }
    }
}

impl<'a, T> IntoObjC<*mut AnyObject> for &'a mut T
    where T: AsAnyObject
{
    fn into_objc(self) -> *mut AnyObject {
        self.any_mut()
    }
}

impl<'a, T> IntoObjC<*mut AnyObject> for Option<&'a mut T>
    where T: AsAnyObject
{
    fn into_objc(self) -> *mut AnyObject {
        match self {
            Some(x) => x.any_mut(),
            None => ptr::null_mut()
        }
    }
}

impl<T> IntoObjC<*mut AnyObject> for ShareId<T>
    where T: AsAnyObject
{
    fn into_objc(self) -> *mut AnyObject {
        // TODO: Do this cleanly
        let ptr = self.any_ref() as *const _ as *mut _;
        mem::forget(self);
        ptr
    }
}

impl<T> IntoObjC<*mut AnyObject> for Option<ShareId<T>>
    where T: AsAnyObject
{
    fn into_objc(self) -> *mut AnyObject {
        match self {
            Some(x) => {
                let ptr = x.any_ref() as *const _ as *mut _;
                mem::forget(x);
                ptr
            },
            None => {
                ptr::null_mut()
            }
        }
    }
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



pub trait AsAnyObject {
    fn any_ref(&self) -> &AnyObject;
    fn any_mut(&mut self) -> &mut AnyObject;
}

impl<T> AsAnyObject for T
    where T: Object, T::Super: AsAnyObject
{
    fn any_ref(&self) -> &AnyObject {
        self.super_ref().any_ref()
    }

    fn any_mut(&mut self) -> &mut AnyObject {
        self.super_mut().any_mut()
    }
}

pub trait SubAnyObject {
    type AnySuper: AsAnyObject;

    fn any_super_ref(&self) -> &Self::AnySuper;
    fn any_super_mut(&mut self) -> &mut Self::AnySuper;
}

impl<T> SubAnyObject for T
    where T: Object, T::Super: AsAnyObject
{
    type AnySuper = T::Super;

    fn any_super_ref(&self) -> &Self::AnySuper {
        self.super_ref()
    }

    fn any_super_mut(&mut self) -> &mut Self::AnySuper {
        self.super_mut()
    }
}

impl<T> AsAnyObject for T
    where T: SubAnyObject
{
    default fn any_ref(&self) -> &AnyObject {
        self.any_super_ref().any_ref()
    }

    default fn any_mut(&mut self) -> &mut AnyObject {
        self.any_super_mut().any_mut()
    }
}

pub trait FromAnyObject: Sized {
    unsafe fn from_any(any: *mut AnyObject) -> ShareId<Self>;
}
