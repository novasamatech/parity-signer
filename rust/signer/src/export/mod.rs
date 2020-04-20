#[cfg(feature = "jni")]
pub mod android;
#[cfg(not(feature = "jni"))]
pub mod ios;

pub use anyhow;
pub use ffi_support;

/// Trait for converting Rust types into FFI return values
pub trait Return<'a>: Sized {
	type Ext;
	type Env;
	fn convert(env: Self::Env, val: Self) -> Self::Ext;
	fn convert_without_exception(env: Self::Env, val: Self) -> Self::Ext {
		Return::convert(env, val)
	}
}

/// Trait for converting FFI arguments into Rust types
pub trait Argument<'a> {
	type Ext;
	type Env;
	fn convert(env: &Self::Env, val: Self::Ext) -> Self;
}

#[macro_export]
macro_rules! export {
    ($( @$jname:ident fn $name:ident($( $a:ident : $t:ty ),*) -> $ret:ty $code:block )*) => {
        $(
            pub fn $name(
                $( $a: $t ),*
            ) -> $ret $code
        )*

        #[cfg(feature = "jni")]
        pub mod android_export {
            use $crate::export::{Return, Argument};

            use jni::JNIEnv;
            use jni::objects::JClass;

            $(
                #[no_mangle]
                pub extern fn $jname<'jni>(
                    env: JNIEnv<'jni>,
                    _: JClass,
                    $( $a: <$t as Argument<'jni>>::Ext ),*
                ) -> <$ret as Return<'jni>>::Ext {
                    let ret = super::$name($( Argument::convert(&env, $a) ),*);
                    Return::convert(&env, ret)
                }
            )*
        }

        #[cfg(not(feature = "jni"))]
        pub mod ios_export {
            use $crate::export::{Return, Argument};

            use ffi_support::ExternError;

            $(
                #[no_mangle]
                pub extern "C" fn $name(
                    err: &'static mut ExternError,
                    $( $a: <$t as Argument<'static>>::Ext ),*
                ) -> <$ret as Return<'static>>::Ext {
                    let res = super::$name($( Argument::convert(&err, $a) ),*);
                    let ret = Return::convert(err, res);
                    ret
                }
            )*
        }
    }
}

#[cfg(test)]
mod tests {}
