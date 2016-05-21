#[macro_export]
macro_rules! objc {
    (
        pub unsafe class trait $class:ident $(: $parent:ident),* { $($body:tt)+ }
    ) => {
        __objc_class_trait! {
            @class: $class;
            @parent: [$($parent),*];
            @vis: [pub];
            body: { $($body)* };
        }
    };
    (
        unsafe class trait $class:ident $(: $parent:ident),* { $($body:tt)* }
    ) => {
        __objc_class_trait! {
            @class: $class;
            @parent: [$($parent),*];
            @vis: [];
            body: { $($body)* };
        }
    };
}

#[macro_export]
macro_rules! __objc_ty {
    () => { () };
    ($t:ty) => { $t };
}

#[macro_export]
macro_rules! __objc_msg_args {
    () => {
        ()
    };
    ($fn_arg_ty:ty, $msg_arg_ty:ty, $fn_arg:expr) => {
        ($crate::rust_to_objc::<$fn_arg_ty, $msg_arg_ty>($fn_arg),)
    };
    ($($fn_arg_ty:ty, $msg_arg_ty:ty, $fn_arg:expr),+) => {
        ($($crate::rust_to_objc::<$fn_arg_ty, $msg_arg_ty>($fn_arg)),*)
    };
}

#[macro_export]
macro_rules! __objc_class_trait {
    {
        @class: $class:ident;
        @parent: [$($parent:ident),*];
        @vis: [$($vis:ident),*];
        body: {
            type Base = $base:ty;
            trait Sub = $sub:ident;

            $($body_rest:tt)*
        };
    } => {
        __objc_class_trait! {
            @base: $base;
            @class: $class;
            @fns: [];
            @parent: [$($parent),*];
            @sub: $sub;
            @vis: [$($vis),*];
            body: {
                $($body_rest)*
            };
        }
    };

    (
        @base: $base:ty;
        @class: $class:ident;
        @fns: [$($fns:tt)*];
        @parent: [$($parent:ident),*];
        @sub: $sub:ident;
        @vis: [$($vis:ident),*];
        body: {
            fn $fn_name:ident(&self) $(-> $fn_ret:ty),*
                => [self, $msg_sel:ident] $(-> $msg_ret:ty),*;
            $($body_rest:tt)*
        };
    ) => {
        __objc_class_trait! {
            @base: $base;
            @class: $class;
            @fns: [
                $($fns)*
                @fn {
                    @name: $fn_name;
                    @args: [];
                    @ret: [$($fn_ret),*];
                    @msg_args: [];
                    @msg_ret: [$($msg_ret),*];
                    @msg_sel: sel!($msg_sel);
                    @qualifiers: [];
                };
            ];
            @parent: [$($parent),*];
            @sub: $sub;
            @vis: [$($vis),*];
            body: {
                $($body_rest)*
            };
        }
    };
    (
        @base: $base:ty;
        @class: $class:ident;
        @fns: [$($fns:tt)*];
        @parent: [$($parent:ident),*];
        @sub: $sub:ident;
        @vis: [$($vis:ident),*];
        body: {
            unsafe fn $fn_name:ident(&self) $(-> $fn_ret:ty),*
                => [self, $msg_sel:ident] $(-> $msg_ret:ty),*;
            $($body_rest:tt)*
        };
    ) => {
        __objc_class_trait! {
            @base: $base;
            @class: $class;
            @fns: [
                $($fns)*
                @fn {
                    @name: $fn_name;
                    @args: [];
                    @ret: [$($fn_ret),*];
                    @msg_args: [];
                    @msg_ret: [$($msg_ret),*];
                    @msg_sel: sel!($msg_sel);
                    @qualifiers: [unsafe];
                };
            ];
            @parent: [$($parent),*];
            @sub: $sub;
            @vis: [$($vis),*];
            body: {
                $($body_rest)*
            };
        }
    };
    (
        @base: $base:ty;
        @class: $class:ident;
        @fns: [$($fns:tt)*];
        @parent: [$($parent:ident),*];
        @sub: $sub:ident;
        @vis: [$($vis:ident),*];
        body: {
            fn $fn_name:ident(&self $(, $fn_arg:ident: $fn_arg_ty:ty)+) $(-> $fn_ret:ty),*
                => [self, $($msg_sel:ident: ($msg_arg:ident: $msg_arg_ty:ty))+] $(-> $msg_ret:ty),*;
            $($body_rest:tt)*
        };
    ) => {
        __objc_class_trait! {
            @base: $base;
            @class: $class;
            @fns: [
                $($fns)*
                @fn {
                    @name: $fn_name;
                    @args: [$($fn_arg: $fn_arg_ty),+];
                    @ret: [$($fn_ret),*];
                    @msg_args: [$($msg_arg: $msg_arg_ty),+];
                    @msg_ret: [$($msg_ret),*];
                    @msg_sel: sel!($($msg_sel:)+);
                    @qualifiers: [];
                };
            ];
            @parent: [$($parent),*];
            @sub: $sub;
            @vis: [$($vis),*];
            body: {
                $($body_rest)*
            };
        }
    };
    (
        @base: $base:ty;
        @class: $class:ident;
        @fns: [$($fns:tt)*];
        @parent: [$($parent:ident),*];
        @sub: $sub:ident;
        @vis: [$($vis:ident),*];
        body: {
            unsafe fn $fn_name:ident(&self $(, $fn_arg:ident: $fn_arg_ty:ty)+) $(-> $fn_ret:ty),*
                => [self, $($msg_sel:ident: ($msg_arg:ident: $msg_arg_ty:ty))+] $(-> $msg_ret:ty),*;
            $($body_rest:tt)*
        };
    ) => {
        __objc_class_trait! {
            @base: $base;
            @class: $class;
            @fns: [
                $($fns)*
                @fn {
                    @name: $fn_name;
                    @args: [$($fn_arg: $fn_arg_ty),+];
                    @ret: [$($fn_ret),*];
                    @msg_args: [$($msg_arg: $msg_arg_ty),+];
                    @msg_ret: [$($msg_ret),*];
                    @msg_sel: sel!($($msg_sel:)+);
                    @qualifiers: [unsafe];
                };
            ];
            @parent: [$($parent),*];
            @sub: $sub;
            @vis: [$($vis),*];
            body: {
                $($body_rest)*
            };
        }
    };


    (
        @base: $base:ty;
        @class: $class:ident;
        @fns: [$($fns:tt)*];
        @parent: [$($parent:ident),*];
        @sub: $sub:ident;
        @vis: [$($vis:ident),*];
        body: { };
    ) => {
        __objc_class_trait! {
            @base: $base;
            @class: $class;
            @fns: [$($fns)*];
            @parent: [$($parent),*];
            @sub: $sub;
            @vis: [$($vis),*];
        }
    };

    (
        @base: $base:ty;
        @class: $class:ident;
        @fns: [
            $(
                @fn {
                    @name: $fn_name:ident;
                    @args: [$($fn_arg:ident: $fn_arg_ty:ty),*];
                    @ret: [$($fn_ret:ty),*];
                    @msg_args: [$($msg_arg:ident: $msg_arg_ty:ty),*];
                    @msg_ret: [$($msg_ret:ty),*];
                    @msg_sel: $msg_sel:expr;
                    @qualifiers: [$($qualifiers:ident),*];
                };
            )*
        ];
        @parent: [$($parent:ident),*];
        @sub: $sub:ident;
        @vis: [$($vis:ident),*];
    ) => {
        #[allow(unused_unsafe)]
        impl $class for $base {
            $(
                $($qualifiers)* fn $fn_name(&self, $($fn_arg: $fn_arg_ty),*) $(-> $fn_ret),* {
                    unsafe {
                        let msg_args = __objc_msg_args!($($fn_arg_ty, $msg_arg_ty, $fn_arg),*);
                        let result: __objc_ty!($($msg_ret),*) = ::objc::Message::send_message(self, $msg_sel, msg_args).unwrap();
                        $crate::objc_to_rust::<__objc_ty!($($msg_ret),*), __objc_ty!($($fn_ret),*)>(result)
                    }
                }
            )*
        }

        impl<T> $class for T
            where T: $sub + $($parent),*
        {
            $(
                default $($qualifiers)* fn $fn_name(&self, $($fn_arg: $fn_arg_ty),*) $(-> $fn_ret),* {
                    <T as $sub>::class_super_ref(self).$fn_name($($fn_arg),*)
                }
            )*
        }

        $($vis),* trait $sub {
            type ClassSuper: $class;

            fn class_super_ref(&self) -> &Self::ClassSuper;
            fn class_super_mut(&mut self) -> &mut Self::ClassSuper;
        }

        impl<T> $sub for T
            where T: $crate::Object, <T as $crate::Object>::Super: $class
        {
            type ClassSuper = <T as $crate::Object>::Super;

            fn class_super_ref(&self) -> &Self::ClassSuper {
                self.super_ref()
            }

            fn class_super_mut(&mut self) -> &mut Self::ClassSuper {
                self.super_mut()
            }
        }
    }
}

// TODO: Write a proper test suite! This just tests the macro expansion
//       itself (and not inputs/outputs)
#[cfg(test)]
mod tests {
    use objc;
    use objc::runtime as rt;
    use {Object, AnyObject, NSObject, IsNSObject};

    struct MyObject {
        super_: NSObject
    }

    unsafe impl objc::Message for MyObject { }

    impl Object for MyObject {
        type Super = NSObject;

        fn super_ref(&self) -> &Self::Super { &self.super_ }

        fn super_mut(&mut self) -> &mut Self::Super { &mut self.super_ }
    }

    trait IsMyObject: IsNSObject {
        fn foo(&self);

        fn bar(&self, a: bool);

        fn baz(&self, a: bool, b: &AnyObject) -> bool;

        unsafe fn qux(&self, a: bool, b: &AnyObject, c: &AnyObject) -> *mut AnyObject;
    }

    objc! {
        unsafe class trait IsMyObject: IsNSObject {
            type Base = MyObject;
            trait Sub = SubMyObject;

            fn foo(&self) => [self, foo];
            fn bar(&self, a: bool) => [self, barWithA:(a: rt::BOOL)];
            fn baz(&self, a: bool, b: &AnyObject) -> bool
                => [self, bazWithA:(a: rt::BOOL) B:(b: *const AnyObject)]
                    -> rt::BOOL;
            unsafe fn qux(&self, a: bool, b: &AnyObject, c: &AnyObject)
                -> *mut AnyObject
                => [self, quxWithA:(a: rt::BOOL)
                                 B:(b: *const AnyObject)
                                 C:(c: *const AnyObject)]
                    -> *mut AnyObject;
        }
    }

    #[test]
    fn test_objc_macro_makes_object_safe_traits() {
        let _my_object: MyObject;
        let _is_my_object: Box<IsMyObject>;
        let _sub_my_object: Box<SubMyObject<ClassSuper=MyObject>>;
    }
}
