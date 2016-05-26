#[macro_export]
macro_rules! objc {
    (
        pub unsafe objc trait $name:ident $(: $parent:ident),* { $($body:tt)+ }
    ) => {
        __objc_trait! {
            @name: $name;
            @parent: [$($parent),*];
            @vis: [pub];
            body: { $($body)* };
        }
    };
    (
        unsafe objc trait $name:ident $(: $parent:ident),* { $($body:tt)+ }
    ) => {
        __objc_trait! {
            @name: $name;
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
macro_rules! __objc_default_impl {
    ($method_name:ident, [], [$($args:expr),*]) => {
        ()
    };
    ($method_name:ident, [$default:ty], [$($args:expr),*]) => {
        {
            let default_impl: $default = ::std::default::Default::default();
            default_impl.$method_name($($args),*)
        }
    }
}

#[macro_export]
macro_rules! __objc_expand_method {
    {
        @pass: $pass:ident! { @_ $($pass_args:tt)* };
        @body: {
            $(#[$meta:ident])*
            fn $fn_name:ident(&self) $(-> $fn_ret:ty),*
                => [self, $msg_sel:ident] $(-> $msg_ret:ty),*;
        };
    } => {
        $pass! {
            @fn {
                @name: $fn_name;
                @args: [];
                @ret: [$($fn_ret),*];
                @meta: [$(#[$meta])*];
                @msg_args: [];
                @msg_ret: [$($msg_ret),*];
                @msg_sel: sel!($msg_sel);
                @qualifiers: [];
            }
            $($pass_args)*
        }
    };

    {
        @pass: $pass:ident! { @_ $($pass_args:tt)* };
        @body: {
            $(#[$meta:ident])*
            unsafe fn $fn_name:ident(&self) $(-> $fn_ret:ty),*
                => [self, $msg_sel:ident] $(-> $msg_ret:ty),*;
        };
    } => {
        $pass! {
            @fn {
                @name: $fn_name;
                @args: [];
                @ret: [$($fn_ret),*];
                @meta: [$(#[$meta])*];
                @msg_args: [];
                @msg_ret: [$($msg_ret),*];
                @msg_sel: sel!($msg_sel);
                @qualifiers: [unsafe ];
            }
            $($pass_args)*
        }
    };

    {
        @pass: $pass:ident! { @_ $($pass_args:tt)* };
        @body: {
            $(#[$meta:ident])*
            fn $fn_name:ident(&self $(, $fn_arg:ident: $fn_arg_ty:ty)+) $(-> $fn_ret:ty),*
                => [self, $($msg_sel:ident: ($msg_arg:ident: $msg_arg_ty:ty))+] $(-> $msg_ret:ty),*;
        };
    } => {
        $pass! {
            @fn {
                @name: $fn_name;
                @args: [$($fn_arg: $fn_arg_ty),+];
                @ret: [$($fn_ret),*];
                @meta: [$(#[$meta])*];
                @msg_args: [$($msg_arg: $msg_arg_ty),+];
                @msg_ret: [$($msg_ret),*];
                @msg_sel: sel!($($msg_sel:)+);
                @qualifiers: [];
            }
            $($pass_args)*
        }
    };

    {
        @pass: $pass:ident! { @_ $($pass_args:tt)* };
        @body: {
            $(#[$meta:ident])*
            unsafe fn $fn_name:ident(&self $(, $fn_arg:ident: $fn_arg_ty:ty)+) $(-> $fn_ret:ty),*
                => [self, $($msg_sel:ident: ($msg_arg:ident: $msg_arg_ty:ty))+] $(-> $msg_ret:ty),*;
        };
    } => {
        $pass! {
            @fn {
                @name: $fn_name;
                @args: [$($fn_arg: $fn_arg_ty),+];
                @ret: [$($fn_ret),*];
                @meta: [$(#[$meta])*];
                @msg_args: [$($msg_arg: $msg_arg_ty),+];
                @msg_ret: [$($msg_ret),*];
                @msg_sel: sel!($($msg_sel:)+);
                @qualifiers: [unsafe];
            }
            $($pass_args)*
        }
    };
}

#[macro_export]
macro_rules! __objc_trait_add_fn {
    {
        @fn {
            @name: $fn_name:ident;
            @args: [$($fn_args:tt)*];
            @ret: [$($fn_ret:tt)*];
            @meta: [];
            @msg_args: [$($fn_msg_args:tt)*];
            @msg_ret: [$($fn_msg_ret:tt)*];
            @msg_sel: $fn_msg_sel:expr;
            @qualifiers: [$($fn_qualifiers:tt)*];
        };
        @objc_trait: {
            @base: $base:ty;
            @name: $name:ident;
            @default: [$($default:ty),*];
            @fns: {
                @methods: [$($methods:tt)*];
                @optional: [$($optional:tt)*];
            };
            @parent: [$($parent:ident),*];
            @sub: $sub:ident;
            @vis: [$($vis:ident),*];
            body: { $($body:tt)* };
        };
    } => {
        __objc_trait! {
            @base: $base;
            @name: $name;
            @default: [$($default),*];
            @fns: {
                @methods: [
                    $($methods)*
                    @fn {
                        @name: $fn_name;
                        @args: [$($fn_args)*];
                        @ret: [$($fn_ret)*];
                        @meta: [];
                        @msg_args: [$($fn_msg_args)*];
                        @msg_ret: [$($fn_msg_ret)*];
                        @msg_sel: $fn_msg_sel;
                        @qualifiers: [$($fn_qualifiers)*];
                    };
                ];
                @optional: [$($optional)*];
            };
            @parent: [$($parent),*];
            @sub: $sub;
            @vis: [$($vis),*];
            body: { $($body)* };
        }
    };

    {
        @fn {
            @name: $fn_name:ident;
            @args: [$($fn_args:tt)*];
            @ret: [$($fn_ret:tt)*];
            @meta: [#[optional]];
            @msg_args: [$($fn_msg_args:tt)*];
            @msg_ret: [$($fn_msg_ret:tt)*];
            @msg_sel: $fn_msg_sel:expr;
            @qualifiers: [$($fn_qualifiers:tt)*];
        };
        @objc_trait: {
            @base: $base:ty;
            @name: $name:ident;
            @default: [$($default:ty),*];
            @fns: {
                @methods: [$($methods:tt)*];
                @optional: [$($optional:tt)*];
            };
            @parent: [$($parent:ident),*];
            @sub: $sub:ident;
            @vis: [$($vis:ident),*];
            body: { $($body:tt)* };
        };
    } => {
        __objc_trait! {
            @base: $base;
            @name: $name;
            @default: [$($default),*];
            @fns: {
                @methods: [$($methods)*];
                @optional: [
                    $($optional)*
                    @fn {
                        @name: $fn_name;
                        @args: [$($fn_args)*];
                        @ret: [$($fn_ret)*];
                        @optional_default: [$($default),*];
                        @meta: [#[optional]];
                        @msg_args: [$($fn_msg_args)*];
                        @msg_ret: [$($fn_msg_ret)*];
                        @msg_sel: $fn_msg_sel;
                        @qualifiers: [$($fn_qualifiers)*];
                    };
                ];
            };
            @parent: [$($parent),*];
            @sub: $sub;
            @vis: [$($vis),*];
            body: { $($body)* };
        }
    };
}

#[macro_export]
macro_rules! __objc_trait {
    {
        @name: $name:ident;
        @parent: [$($parent:ident),*];
        @vis: [$($vis:ident),*];
        body: {
            type Base = $base:ty;
            trait Sub = $sub:ident;

            $($body_rest:tt)*
        };
    } => {
        __objc_trait! {
            @base: $base;
            @name: $name;
            @default: [];
            @fns: {
                @methods: [];
                @optional: [];
            };
            @parent: [$($parent),*];
            @sub: $sub;
            @vis: [$($vis),*];
            body: {
                $($body_rest)*
            };
        }
    };



    {
        @base: $base:ty;
        @name: $name:ident;
        @default: [];
        @fns: { $($fns:tt)* };
        @parent: [$($parent:ident),*];
        @sub: $sub:ident;
        @vis: [$($vis:ident),*];
        body: {
            type DefaultImpl = $default:ty;
            $($body_rest:tt)*
        };
    } => {
        __objc_trait! {
            @base: $base;
            @name: $name;
            @default: [$default];
            @fns: { $($fns)* };
            @parent: [$($parent),*];
            @sub: $sub;
            @vis: [$($vis),*];
            body: {
                $($body_rest)*
            };
        }
    };

    {
        @base: $base:ty;
        @name: $name:ident;
        @default: [$($default:ty),*];
        @fns: { $($fns:tt)* };
        @parent: [$($parent:ident),*];
        @sub: $sub:ident;
        @vis: [$($vis:ident),*];
        body: {
            $(#[$meta:ident])*
            fn $fn_name:ident($($args:tt)*) $(-> $fn_ret:ty),*
                => [$($msg:tt)*] $(-> $msg_ret:ty),*;
            $($body_rest:tt)*
        };
    } => {
        __objc_expand_method! {
            @pass: __objc_trait_add_fn! {
                @_;
                @objc_trait: {
                    @base: $base;
                    @name: $name;
                    @default: [$($default),*];
                    @fns: { $($fns)* };
                    @parent: [$($parent),*];
                    @sub: $sub;
                    @vis: [$($vis),*];
                    body: { $($body_rest)* };
                };
            };
            @body: {
                $(#[$meta])*
                fn $fn_name($($args)*) $(-> $fn_ret),*
                    => [$($msg)*] $(-> $msg_ret),*;
            };
        }
    };

    {
        @base: $base:ty;
        @name: $name:ident;
        @default: [$($default:ty),*];
        @fns: { $($fns:tt)* };
        @parent: [$($parent:ident),*];
        @sub: $sub:ident;
        @vis: [$($vis:ident),*];
        body: {
            $(#[$meta:ident])*
            unsafe fn $fn_name:ident($($args:tt)*) $(-> $fn_ret:ty),*
                => [$($msg:tt)*] $(-> $msg_ret:ty),*;
            $($body_rest:tt)*
        };
    } => {
        __objc_expand_method! {
            @pass: __objc_trait_add_fn! {
                @_;
                @objc_trait: {
                    @base: $base;
                    @name: $name;
                    @default: [$($default),*];
                    @fns: { $($fns)* };
                    @parent: [$($parent),*];
                    @sub: $sub;
                    @vis: [$($vis),*];
                    body: { $($body_rest)* };
                };
            };
            @body: {
                $(#[$meta])*
                unsafe fn $fn_name($($args)*) $(-> $fn_ret),*
                    => [$($msg)*] $(-> $msg_ret),*;
            };
        }
    };



    {
        @base: $base:ty;
        @name: $name:ident;
        @default: [$($default:ty),*];
        @fns: { $($fns:tt)* };
        @parent: [$($parent:ident),*];
        @sub: $sub:ident;
        @vis: [$($vis:ident),*];
        body: { };
    } => {
        __objc_trait! {
            @base: $base;
            @name: $name;
            @default: [$($default),*];
            @fns: { $($fns)* };
            @parent: [$($parent),*];
            @sub: $sub;
            @vis: [$($vis),*];
        }
    };

    {
        @base: $base:ty;
        @name: $name:ident;
        @default: [$($default:ty),*];
        @fns: {
            @methods: [
                $(
                    @fn {
                        @name: $method_name:ident;
                        @args: [$($method_arg:ident: $method_arg_ty:ty),*];
                        @ret: [$($method_ret:ty),*];
                        @meta: [];
                        @msg_args: [$($method_msg_arg:ident: $method_msg_arg_ty:ty),*];
                        @msg_ret: [$($method_msg_ret:ty),*];
                        @msg_sel: $method_msg_sel:expr;
                        @qualifiers: [$($method_qualifiers:ident),*];
                    };
                )*
            ];
            @optional: [
                $(
                    @fn {
                        @name: $optional_name:ident;
                        @args: [$($optional_arg:ident: $optional_arg_ty:ty),*];
                        @ret: [$($optional_ret:ty),*];
                        @optional_default: [$($optional_default:ty),*];
                        @meta: [#[optional]];
                        @msg_args: [$($optional_msg_arg:ident: $optional_msg_arg_ty:ty),*];
                        @msg_ret: [$($optional_msg_ret:ty),*];
                        @msg_sel: $optional_msg_sel:expr;
                        @qualifiers: [$($optional_qualifiers:ident),*];
                    };
                )*
            ];
        };
        @parent: [$($parent:ident),*];
        @sub: $sub:ident;
        @vis: [$($vis:ident),*];
    } => {
        #[allow(unused_unsafe)]
        impl $name for $base {
            $(
                $($method_qualifiers)* fn $method_name(&self, $($method_arg: $method_arg_ty),*) $(-> $method_ret),* {
                    unsafe {
                        let msg_args = __objc_msg_args!($($method_arg_ty, $method_msg_arg_ty, $method_arg),*);
                        let result: __objc_ty!($($method_msg_ret),*) = ::objc::Message::send_message(self, $method_msg_sel, msg_args).unwrap();
                        $crate::objc_to_rust::<__objc_ty!($($method_msg_ret),*), __objc_ty!($($method_ret),*)>(result)
                    }
                }
            )*

            $(
                $($optional_qualifiers)* fn $optional_name(&self, $($optional_arg: $optional_arg_ty),*) $(-> $optional_ret),* {
                    unsafe {
                        let sel = $optional_msg_sel;
                        if $crate::objc_bool_to_rust(msg_send![self, respondsToSelector:sel]) {
                            let msg_args = __objc_msg_args!($($optional_arg_ty, $optional_msg_arg_ty, $optional_arg),*);
                            let result: __objc_ty!($($optional_msg_ret),*) = ::objc::Message::send_message(self, $optional_msg_sel, msg_args).unwrap();
                            $crate::objc_to_rust::<__objc_ty!($($optional_msg_ret),*), __objc_ty!($($optional_ret),*)>(result)
                        }
                        else {
                            __objc_default_impl!($optional_name, [$($optional_default),*], [$($optional_arg),*])
                        }
                    }
                }
            )*
        }

        impl<T> $name for T
            where T: $sub + $($parent),*
        {
            $(
                default $($method_qualifiers)* fn $method_name(&self, $($method_arg: $method_arg_ty),*) $(-> $method_ret),* {
                    <T as $sub>::class_super_ref(self).$method_name($($method_arg),*)
                }
            )*

            $(
                default $($optional_qualifiers)* fn $optional_name(&self, $($optional_arg: $optional_arg_ty),*) $(-> $optional_ret),* {
                    <T as $sub>::class_super_ref(self).$optional_name($($optional_arg),*)
                }
            )*
        }

        $($vis),* trait $sub {
            type ClassSuper: $name;

            fn class_super_ref(&self) -> &Self::ClassSuper;
            fn class_super_mut(&mut self) -> &mut Self::ClassSuper;
        }

        impl<T> $sub for T
            where T: $crate::Object, <T as $crate::Object>::Super: $name
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

        fn bar(&self, _a: bool) { }

        fn baz(&self, a: bool, b: &AnyObject) -> bool;

        unsafe fn qux(&self, _a: bool, _b: &AnyObject, _c: &AnyObject)
            -> *mut AnyObject
        {
                unimplemented!()
        }
    }

    #[derive(Default)]
    struct DefaultImplMyObject;

    impl DefaultImplMyObject {
        fn bar(&self, _a: bool) { }

        fn qux(&self, _a: bool, _b: &AnyObject, _c: &AnyObject) -> *mut AnyObject {
            unimplemented!();
        }
    }

    objc! {
        unsafe objc trait IsMyObject: IsNSObject {
            type Base = MyObject;
            trait Sub = SubMyObject;
            type DefaultImpl = DefaultImplMyObject;

            fn foo(&self) => [self, foo];

            #[optional]
            fn bar(&self, a: bool) => [self, barWithA:(a: rt::BOOL)];

            fn baz(&self, a: bool, b: &AnyObject) -> bool
                => [self, bazWithA:(a: rt::BOOL) B:(b: *const AnyObject)]
                    -> rt::BOOL;

            #[optional]
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
