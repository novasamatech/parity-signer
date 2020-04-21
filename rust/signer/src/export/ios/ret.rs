use super::super::Return;
use ffi_support::{rust_string_to_c, ErrorCode, ExternError};
use libc::c_char;

impl Return<'static> for () {
	type Ext = *mut std::ffi::c_void;
	type Env = &'static mut ExternError;
	fn convert(_: Self::Env, _val: Self) -> Self::Ext {
		std::ptr::null_mut()
	}
}

impl Return<'static> for i64 {
	type Ext = i64;
	type Env = &'static mut ExternError;
	fn convert(_: Self::Env, val: Self) -> Self::Ext {
		val
	}
}

impl Return<'static> for bool {
	type Ext = u8;
	type Env = &'static mut ExternError;
	fn convert(_: Self::Env, val: Self) -> Self::Ext {
		val as u8
	}
}

impl Return<'static> for String {
	type Ext = *mut c_char;
	type Env = &'static mut ExternError;
	fn convert(_: Self::Env, val: Self) -> Self::Ext {
		rust_string_to_c(val)
	}
}

impl<Inner: Return<'static, Env = &'static mut ExternError> + Default> Return<'static>
	for anyhow::Result<Inner>
{
	type Ext = Inner::Ext;
	type Env = Inner::Env;
	fn convert(env: Self::Env, val: Self) -> Self::Ext {
		let val = match val {
			Ok(inner) => inner,
			Err(e) => {
				*env = ExternError::new_error(ErrorCode::new(1), format!("{:?}", e));
				Inner::default()
			}
		};
		let ret = Return::convert(env, val);
		ret
	}
}
