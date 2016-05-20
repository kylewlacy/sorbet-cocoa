use std::ptr;
use objc;
use objc::runtime as rt;
use {AnyObject, AsAnyObject, Object, objc_id_to_rust, objc_bool_to_rust};

#[repr(C)]
pub struct NSObject {
    super_: AnyObject
}

unsafe impl objc::Message for NSObject { }

impl Object for NSObject {
    type Super = AnyObject;

    fn super_ref(&self) -> &Self::Super {
        &self.super_
    }

    fn super_mut(&mut self) -> &mut Self::Super {
        &mut self.super_
    }
}

impl AsAnyObject for NSObject {
    fn any_ref(&self) -> &AnyObject {
        &self.super_
    }

    fn any_mut(&mut self) -> &mut AnyObject {
        &mut self.super_
    }
}

pub trait IsNSObject {
    fn instance_class(&self) -> &rt::Class;
    fn instance_superclass(&self) -> Option<&rt::Class>;
    fn is_equal(&self, other: Option<&AnyObject>) -> bool;
    fn hash(&self) -> usize;
    fn is_kind_of_class(&self, class: &rt::Class) -> bool;
    fn is_member_of_class(&self, class: &rt::Class) -> bool;
    fn responds_to_selector(&self, sel: rt::Sel) -> bool;
    fn description(&self) -> String;
    fn debug_description(&self) -> String;
}

impl IsNSObject for NSObject {
    fn instance_class(&self) -> &rt::Class {
        unsafe { msg_send![self, class] }
    }

    fn instance_superclass(&self) -> Option<&rt::Class> {
        unsafe { msg_send![self, superclass] }
    }

    fn is_equal(&self, other: Option<&AnyObject>) -> bool {
        let other_ptr = match other {
            Some(ptr) => ptr,
            None => ptr::null()
        };
        unsafe { objc_bool_to_rust(msg_send![self, isEqual:other_ptr]) }
    }

    fn hash(&self) -> usize {
        unsafe { msg_send![self, hash] }
    }

    fn is_kind_of_class(&self, class: &rt::Class) -> bool {
        unsafe { objc_bool_to_rust(msg_send![self, isKindOfClass:class]) }
    }

    fn is_member_of_class(&self, class: &rt::Class) -> bool {
        unsafe { objc_bool_to_rust(msg_send![self, isMemberOfClass:class]) }
    }

    fn responds_to_selector(&self, sel: rt::Sel) -> bool {
        unsafe { objc_bool_to_rust(msg_send![self, respondsToSelector:sel]) }
    }

    fn description(&self) -> String {
        unsafe { objc_id_to_rust(msg_send![self, description]) }
    }

    fn debug_description(&self) -> String {
        unsafe { objc_id_to_rust(msg_send![self, debugDescription]) }
    }
}

impl<T> IsNSObject for T
    where T: SubNSObject + Object
{
    default fn instance_class(&self) -> &rt::Class {
        self.super_ns_object_ref().instance_class()
    }

    default fn instance_superclass(&self) -> Option<&rt::Class> {
        self.super_ns_object_ref().instance_superclass()
    }

    default fn is_equal(&self, other: Option<&AnyObject>) -> bool {
        self.super_ns_object_ref().is_equal(other)
    }

    default fn hash(&self) -> usize {
        self.super_ns_object_ref().hash()
    }

    default fn is_kind_of_class(&self, class: &rt::Class) -> bool {
        self.super_ns_object_ref().is_kind_of_class(class)
    }

    default fn is_member_of_class(&self, class: &rt::Class) -> bool {
        self.super_ns_object_ref().is_member_of_class(class)
    }

    default fn responds_to_selector(&self, sel: rt::Sel) -> bool {
        self.super_ns_object_ref().responds_to_selector(sel)
    }

    default fn description(&self) -> String {
        self.super_ns_object_ref().description()
    }

    default fn debug_description(&self) -> String {
        self.super_ns_object_ref().debug_description()
    }
}

pub trait SubNSObject {
    type SuperNSObject: IsNSObject;

    fn super_ns_object_ref(&self) -> &Self::SuperNSObject;
    fn super_ns_object_mut(&mut self) -> &mut Self::SuperNSObject;
}

impl<T> SubNSObject for T
    where T: Object, T::Super: IsNSObject
{
    type SuperNSObject = T::Super;

    fn super_ns_object_ref(&self) -> &Self::SuperNSObject {
        self.super_ref()
    }

    fn super_ns_object_mut(&mut self) -> &mut Self::SuperNSObject {
        self.super_mut()
    }
}
