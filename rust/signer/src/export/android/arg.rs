use crate::export::Argument;
use jni::objects::JString;
use jni::sys::{jboolean, jint, jlong};
use jni::JNIEnv;

impl<'a> Argument<'a> for u32 {
	type Ext = jint;
	type Env = JNIEnv<'a>;
	fn convert(_: &Self::Env, val: Self::Ext) -> Self {
		val as u32
	}
}

impl<'a> Argument<'a> for i64 {
	type Ext = jlong;
	type Env = JNIEnv<'a>;
	fn convert(_: &Self::Env, val: Self::Ext) -> Self {
		val as i64
	}
}

impl<'a> Argument<'a> for u8 {
	type Ext = jint;
	type Env = JNIEnv<'a>;
	fn convert(_: &Self::Env, val: Self::Ext) -> Self {
		val as u8
	}
}

impl<'a> Argument<'a> for bool {
	type Ext = jboolean;
	type Env = JNIEnv<'a>;
	fn convert(_: &Self::Env, val: Self::Ext) -> Self {
		val != 0
	}
}

impl<'jni, 'a> Argument<'jni> for &'a str {
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

impl<'a> Argument<'a> for String {
	type Ext = JString<'a>;
	type Env = JNIEnv<'a>;
	fn convert(env: &Self::Env, val: Self::Ext) -> Self {
		env.get_string(val).expect("Invalid java string").into()
	}
}
