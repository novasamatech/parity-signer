// Copyright 2015-2019 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

use libc::size_t;

#[cfg(feature = "jni")]
use jni::{JNIEnv, objects::JString, sys::{jstring, jint}};
#[cfg(not(feature = "jni"))]
use std::cell::Cell;

// Helper struct that we'll use to give strings to C.
#[repr(C)]
pub struct StringPtr {
    pub ptr: *const u8,
    pub len: size_t,
}

impl<'a> From<&'a str> for StringPtr {
    fn from(s: &'a str) -> Self {
        StringPtr {
            ptr: s.as_ptr(),
            len: s.len() as size_t,
        }
    }
}

impl StringPtr {
	pub fn as_str(&self) -> &str {
		use std::{slice, str};

		unsafe {
			let slice = slice::from_raw_parts(self.ptr, self.len);
			str::from_utf8_unchecked(slice)
		}
	}
}

impl std::ops::Deref for StringPtr {
    type Target = str;

    fn deref(&self) -> &str {
    	self.as_str()
    }
}

/// Trait for converting FFI arguments into Rust types
pub trait Argument<'a> {
    type Ext;
    type Env;

    fn convert(env: &Self::Env, val: Self::Ext) -> Self;
}

/// Trait for converting Rust types into FFI return values
pub trait Return<'a> {
    type Ext;
    type Env;

    fn convert(env: &Self::Env, val: Self) -> Self::Ext;
}

#[cfg(not(feature = "jni"))]
impl Argument<'static> for u32 {
    type Ext = u32;
    type Env = Cell<u32>;

    fn convert(_: &Self::Env, val: Self::Ext) -> Self {
        val
    }
}

#[cfg(not(feature = "jni"))]
impl Argument<'static> for u8 {
    type Ext = u32;
    type Env = Cell<u32>;

    fn convert(_: &Self::Env, val: Self::Ext) -> Self {
        val as u8
    }
}

#[cfg(not(feature = "jni"))]
impl<'a> Argument<'static> for &'a str {
    type Ext = *const StringPtr;
    type Env = Cell<u32>;

    fn convert(_: &Self::Env, val: Self::Ext) -> Self {
        unsafe { &*val }.as_str()
    }
}

#[cfg(not(feature = "jni"))]
impl Argument<'static> for String {
    type Ext = *const StringPtr;
    type Env = Cell<u32>;

    fn convert(_: &Self::Env, val: Self::Ext) -> Self {
        unsafe { &*val }.as_str().to_owned()
    }
}

#[cfg(not(feature = "jni"))]
impl Return<'static> for String {
    type Ext = *mut String;
    type Env = Cell<u32>;

    fn convert(_: &Self::Env, val: Self) -> Self::Ext {
        let string = val.to_owned();

        Box::into_raw(Box::new(string))
    }
}

#[cfg(not(feature = "jni"))]
impl<Inner: Return<'static, Env = Cell<u32>> + Default> Return<'static> for Option<Inner> {
    type Ext = Inner::Ext;
    type Env = Inner::Env;

    fn convert(env: &Self::Env, val: Self) -> Self::Ext {
        let val = match val {
            Some(inner) => inner,
            None => {
                env.set(1);

                Inner::default()
            }
        };

        Return::convert(env, val)
    }
}

#[cfg(feature = "jni")]
impl<'jni> Argument<'jni> for u32 {
    type Ext = jint;
    type Env = JNIEnv<'jni>;

    fn convert(_: &Self::Env, val: Self::Ext) -> Self {
        val as u32
    }
}

#[cfg(feature = "jni")]
impl<'jni> Argument<'jni> for u8 {
    type Ext = jint;
    type Env = JNIEnv<'jni>;

    fn convert(_: &Self::Env, val: Self::Ext) -> Self {
        val as u8
    }
}

#[cfg(feature = "jni")]
impl<'a, 'jni> Argument<'jni> for &'a str {
    type Ext = JString<'jni>;
    type Env = JNIEnv<'jni>;

    fn convert(env: &Self::Env, val: Self::Ext) -> Self {
        use std::ffi::CStr;
        use std::str;

        unsafe {
            let ptr = env.get_string_utf_chars(val).expect("Invalid java string");
            let slice = CStr::from_ptr(ptr).to_bytes();

            str::from_utf8_unchecked(slice)
        }
    }
}

#[cfg(feature = "jni")]
impl<'jni> Argument<'jni> for String {
    type Ext = JString<'jni>;
    type Env = JNIEnv<'jni>;

    fn convert(env: &Self::Env, val: Self::Ext) -> Self {
        env.get_string(val).expect("Invalid java string").into()
    }
}

#[cfg(feature = "jni")]
impl<'jni> Return<'jni> for String {
    type Ext = jstring;
    type Env = JNIEnv<'jni>;

    fn convert(env: &Self::Env, val: Self) -> Self::Ext {
        env.new_string(val).expect("Could not create java string").into_inner()
    }
}

#[cfg(feature = "jni")]
impl<'jni, Inner: Return<'jni, Env = JNIEnv<'jni>> + Default> Return<'jni> for Option<Inner> {
    type Ext = Inner::Ext;
    type Env = Inner::Env;

    fn convert(env: &Self::Env, val: Self) -> Self::Ext {
        use jni::objects::JThrowable;

        match val {
            Some(inner) => Return::convert(env, inner),
            None => {
                // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
                // !!!!                                                              !!!!
                // !!!! RETURN VALUE HAS TO BE CREATED BEFORE THROWING THE EXCEPTION !!!!
                // !!!!                                                              !!!!
                // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
                let ret = Return::convert(env, Inner::default());

                let class = env.find_class("java/lang/Exception").expect("Must have the Exception class; qed");
                let exception: JThrowable<'jni> = env.new_object(class, "()V", &[]).expect("Must be able to instantiate the Exception; qed").into();

                // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
                // !!!!                                                        !!!!
                // !!!! WE CAN NO LONGER INTERACT WITH JNIENV AFTER THIS POINT !!!!
                // !!!!                                                        !!!!
                // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
                env.throw(exception).expect("Must be able to throw the Exception; qed");

                ret
            }
        }
    }
}

#[macro_export]
macro_rules! export {
    ($( @$jname:ident fn $name:ident($($par:ident : $t:ty),*) -> $ret:ty $code:block )*) => {
        $(
            pub fn $name($( $par: $t ),*) -> $ret $code
        )*

        #[cfg(feature = "jni")]
        #[allow(non_snake_case)]
        pub mod droid {
            use crate::util::Argument;
            use crate::util::Return;

            use jni::JNIEnv;
            use jni::objects::JClass;

            $(
                #[no_mangle]
                pub extern fn $jname<'jni>(env: JNIEnv<'jni>, _: JClass, $( $par: <$t as Argument<'jni>>::Ext ),*) -> <$ret as Return<'jni>>::Ext {
                    let ret = super::$name($(Argument::convert(&env, $par)),*);

                    Return::convert(&env, ret)
                }
            )*
        }

        #[cfg(not(feature = "jni"))]
        pub mod ios {
            use crate::util::Argument;
            use crate::util::Return;

            use std::cell::Cell;
            use libc::c_uint;

            $(
                #[no_mangle]
                pub extern fn $name(err: *mut c_uint, $( $par: <$t as Argument<'static>>::Ext ),*) -> <$ret as Return<'static>>::Ext {
                    let error = Cell::new(0);
                    let ret = super::$name($(Argument::convert(&error, $par)),*);
                    let ret = Return::convert(&error, ret);

                    unsafe { *err |= error.get() as c_uint };

                    ret
                }
            )*
        }
    }
}
