use std::mem;
use std::ptr;
use std::marker::PhantomData;
use std::os::raw::c_void;
use std::sync::Once;
use objc;
use objc::runtime as rt;
use objc::declare as decl;
use {AnyObject, Id, ShareId, IsNSObject};

mod srb_object;
mod srb_application_delegate;
mod srb_window_controller;

pub use self::srb_object::*;
pub use self::srb_application_delegate::*;
pub use self::srb_window_controller::*;

pub unsafe fn take_boxed<W>(self_: &mut W) -> Box<W::Boxed>
    where W: SRBWrapper + objc::Message
{
    let self_: &mut AnyObject = mem::transmute(self_);
    let boxed_ptr_ref = self_.get_mut_ivar::<*mut c_void>("_boxed");
    let boxed_ptr = mem::replace(boxed_ptr_ref, ptr::null_mut());
    let boxed_ptr = boxed_ptr as *mut W::Boxed;
    Box::from_raw(boxed_ptr)
}

pub unsafe fn get_boxed_ref<W>(self_: &W) -> &W::Boxed
    where W: SRBWrapper + objc::Message
{
    let self_: &AnyObject = mem::transmute(self_);
    let boxed_ptr = *self_.get_ivar::<*mut c_void>("_boxed") as *const W::Boxed;
    &*boxed_ptr
}

pub unsafe fn get_boxed_mut<W>(self_: &mut W) -> &mut W::Boxed
    where W: SRBWrapper + objc::Message
{
    let self_: &AnyObject = mem::transmute(self_);
    let boxed_ptr = *self_.get_ivar::<*mut c_void>("_boxed") as *mut W::Boxed;
    &mut *boxed_ptr
}

pub unsafe fn new_wrapper_with_boxed<W>(boxed: Box<W::Boxed>) -> *mut W
    where W: SRBWrapper + objc::Message
{
    let class = W::class();
    let self_: *mut AnyObject = msg_send![class, alloc];
    let self_: *mut AnyObject = msg_send![self_, init];

    if let Some(self_) = self_.as_mut() {
        let boxed_ptr = Box::into_raw(boxed);
        let boxed_ptr = boxed_ptr as *mut c_void;
        self_.set_ivar::<*mut c_void>("_boxed", boxed_ptr);
    }

    self_ as *mut W
}

struct Deallocator<W: SRBWrapper> {
    _phantom: PhantomData<*mut W>
}

impl<W: SRBWrapper> Deallocator<W> {
    extern "C" fn dealloc(self_: &mut AnyObject, _sel: rt::Sel) {
        unsafe {
            let self_: &mut W = mem::transmute(self_);
            let boxed = take_boxed(self_);
            drop(boxed);

            let superclass: &rt::Class = msg_send![self_, superclass];
            msg_send![super(self_, superclass), dealloc];
        }
    }

    unsafe fn dealloc_ptr() -> extern "C" fn(&mut AnyObject, rt::Sel) {
        Self::dealloc as _
    }
}

pub trait Duck<T> {
    fn duck(self) -> T;
}

impl<T> Duck<Id<T>> for Id<T> {
    fn duck(self) -> Id<T> {
        self
    }
}

impl<T, U> Duck<ShareId<U>> for T
    where T: Duck<Id<U>>, U: objc::Message
{
    fn duck(self) -> ShareId<U> {
        let id: Id<U> = self.duck();
        id.share()
    }
}

pub trait SRBWrapper: IsNSObject + objc::Message + Sized {
    type Boxed;

    fn class_initializer() -> &'static Once;

    fn superclass() -> &'static rt::Class;

    fn class_name() -> &'static str;

    fn create_class(class_decl: &mut decl::ClassDecl);

    fn class() -> &'static rt::Class {
        let name = Self::class_name();

        Self::class_initializer().call_once(|| {
            let dealloc = unsafe { Deallocator::<Self>::dealloc_ptr() };

            let superclass = Self::superclass();
            let mut decl = decl::ClassDecl::new(name, superclass).unwrap();
            decl.add_ivar::<*mut c_void>("_boxed");
            unsafe { decl.add_method(sel!(dealloc), dealloc) };

            Self::create_class(&mut decl);

            decl.register();
        });

        rt::Class::get(name).unwrap()
    }
}
