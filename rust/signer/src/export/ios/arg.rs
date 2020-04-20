use super::super::Argument;
use ffi_support::{ExternError, FfiStr};
use libc::c_char;

impl Argument<'static> for i64 {
	type Ext = i64;
	type Env = &'static mut ExternError;
	fn convert(_: &Self::Env, val: Self::Ext) -> Self {
		val
	}
}

impl Argument<'static> for u32 {
	type Ext = u32;
	type Env = &'static mut ExternError;
	fn convert(_: &Self::Env, val: Self::Ext) -> Self {
		val
	}
}

impl Argument<'static> for u8 {
	type Ext = u32;
	type Env = &'static mut ExternError;
	fn convert(_: &Self::Env, val: Self::Ext) -> Self {
		val as u8
	}
}

impl Argument<'static> for bool {
	type Ext = u8;
	type Env = &'static mut ExternError;
	fn convert(_: &Self::Env, val: Self::Ext) -> Self {
		val != 0
	}
}

impl<'a> Argument<'static> for &'a str {
	type Ext = *const c_char;
	type Env = &'static mut ExternError;
	fn convert(_: &Self::Env, val: Self::Ext) -> Self {
		unsafe { FfiStr::from_raw(val) }.as_str()
	}
}

impl Argument<'static> for String {
	type Ext = *const c_char;
	type Env = &'static mut ExternError;
	fn convert(_: &Self::Env, val: Self::Ext) -> Self {
		unsafe { FfiStr::from_raw(val) }.into_string()
	}
}
