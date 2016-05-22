use objc;
use objc::runtime as rt;
use {AnyObject, AsAnyObject, RawObjCObject, Object};

#[repr(C)]
pub struct NSObject {
    super_: AnyObject
}

unsafe impl objc::Message for NSObject { }

unsafe impl RawObjCObject for NSObject { }

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

objc! {
    pub unsafe class trait IsNSObject {
        type Base = NSObject;
        trait Sub = SubNSObject;

        fn instance_class(&self) -> &rt::Class
            => [self, class] -> *const rt::Class;
        fn instance_superclass(&self) -> Option<&rt::Class>
            => [self, superclass] -> *const rt::Class;
        fn is_equal(&self, other: Option<&AnyObject>) -> bool
            => [self, isEqual:(other: *const AnyObject)] -> rt::BOOL;
        fn hash(&self) -> usize
            => [self, hash] -> usize;
        fn is_kind_of_class(&self, class: &rt::Class) -> bool
            => [self, isKindOfClass:(class: *const rt::Class)] -> rt::BOOL;
        fn is_member_of_class(&self, class: &rt::Class) -> bool
            => [self, isMemberOfClass:(class: *const rt::Class)] -> rt::BOOL;
        fn responds_to_selector(&self, sel: rt::Sel) -> bool
            => [self, respondsToSelector:(sel: rt::Sel)] -> rt::BOOL;
        fn description(&self) -> String
            => [self, description] -> *mut AnyObject;
        fn debug_description(&self) -> String
            => [self, debugDescription] -> *mut AnyObject;
    }
}
