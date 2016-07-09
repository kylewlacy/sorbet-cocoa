use objc;
use objc::runtime as rt;
use {Id, ShareId, Object, AnyObject, RawObjCObject, rust_to_objc_id,
     OptionSel, NSObject, IsNSObject, NSMenuItem};

#[repr(C)]
pub struct NSMenu {
    super_: NSObject
}

unsafe impl objc::Message for NSMenu { }

unsafe impl RawObjCObject for NSMenu { }

impl Object for NSMenu {
    type Super = NSObject;

    fn super_ref(&self) -> &Self::Super {
        &self.super_
    }

    fn super_mut(&mut self) -> &mut Self::Super {
        &mut self.super_
    }
}

impl NSMenu {
    pub fn new(title: &str) -> Id<Self> {
        unsafe {
            let title = rust_to_objc_id(title);

            let ns_menu = rt::Class::get("NSMenu").unwrap();
            let self_: *mut AnyObject = msg_send![ns_menu, alloc];
            let self_: *mut AnyObject = msg_send![self_, initWithTitle:title];
            let self_ = self_ as *mut NSMenu;
            Id::from_retained_ptr(self_)
        }
    }
}

pub trait IsNSMenu: IsNSObject {
    fn add_item(&self, item: ShareId<NSMenuItem>);

    fn add_item_with_title_action_key_equivalent(&self,
                                                 title: &str,
                                                 action: Option<rt::Sel>,
                                                 key_equivalent: &str)
        -> Option<ShareId<NSMenuItem>>;

    fn set_submenu_for_item(&self,
                            submenu: Option<ShareId<NSMenu>>,
                            item: ShareId<NSMenuItem>);
}

objc_trait! {
    pub unsafe objc trait IsNSMenu: IsNSObject {
        type Base = NSMenu;
        trait Sub = SubNSMenu;

        fn add_item(&self, item: ShareId<NSMenuItem>)
            => [self, addItem:(item: *mut AnyObject)];

        fn add_item_with_title_action_key_equivalent(&self,
                                                     title: &str,
                                                     action: Option<rt::Sel>,
                                                     key_equivalent: &str)
            -> Option<ShareId<NSMenuItem>>
            => [self, addItemWithTitle:(title: *mut AnyObject)
                                action:(action: OptionSel)
                         keyEquivalent:(key: *mut AnyObject)]
                -> *mut AnyObject;

        fn set_submenu_for_item(&self,
                                submenu: Option<ShareId<NSMenu>>,
                                item: ShareId<NSMenuItem>)
            => [self, setSubmenu:(submenu: *mut AnyObject)
                         forItem:(item: *mut AnyObject)];
    }
}

#[macro_export]
macro_rules! NSMenu {
    ($args:tt) => {
        __objc_inheritance_for! {
            $crate::NSMenu => $crate::SubNSMenu: NSObject!;
            $args
        }
    };
}
