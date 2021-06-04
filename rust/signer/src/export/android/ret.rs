use crate::export::Return;
use jni::objects::JThrowable;
use jni::sys::{jboolean, jlong, jstring, JNI_FALSE};
use jni::JNIEnv;

impl<'a> Return<'a> for () {
	type Ext = jboolean;
	type Env = &'a JNIEnv<'a>;
	fn convert(_: Self::Env, _val: Self) -> Self::Ext {
		JNI_FALSE
	}
}

impl<'a> Return<'a> for i64 {
	type Ext = jlong;
	type Env = &'a JNIEnv<'a>;
	fn convert(_: Self::Env, val: Self) -> Self::Ext {
		val as Self::Ext
	}
}

impl<'a> Return<'a> for bool {
	type Ext = jboolean;
	type Env = &'a JNIEnv<'a>;
	fn convert(_: Self::Env, val: Self) -> Self::Ext {
		val as Self::Ext
	}
}

impl<'a> Return<'a> for String {
	type Ext = jstring;
	type Env = &'a JNIEnv<'a>;
	fn convert(env: Self::Env, val: Self) -> Self::Ext {
		env.new_string(val)
			.expect("Could not create java string")
			.into_inner()
	}
}

impl<'a, Inner: Return<'a, Env = &'a JNIEnv<'a>> + Default> Return<'a> for Option<Inner> {
	type Ext = Inner::Ext;
	type Env = Inner::Env;

	fn convert(env: Self::Env, val: Self) -> Self::Ext {
		match val {
			Some(inner) => Return::convert(env, inner),
			None => {
				// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
				// !!!!																 !!!!
				// !!!! RETURN VALUE HAS TO BE CREATED BEFORE THROWING THE EXCEPTION !!!!
				// !!!!																 !!!!
				// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
				let ret = Return::convert_without_exception(env, Inner::default());
				let class = env
					.find_class("java/lang/Exception")
					.expect("Must have the Exception class; qed");
				let exception: JThrowable<'a> = env
					.new_object(class, "()V", &[])
					.expect("Must be able to instantiate the Exception; qed")
					.into();
				// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
				// !!!!														   !!!!
				// !!!! WE CAN NO LONGER INTERACT WITH JNIENV AFTER THROWING   !!!!
				// !!!!														   !!!!
				// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
				env.throw(exception)
					.expect("Must be able to throw the Exception; qed");
				ret
			}
		}
	}

	fn convert_without_exception(env: Self::Env, val: Self) -> Self::Ext {
		match val {
			Some(inner) => Return::convert_without_exception(env, inner),
			None => Return::convert_without_exception(env, Inner::default()),
		}
	}
}

impl<'a, Inner, D> Return<'a> for Result<Inner, D>
where
	Inner: Return<'a, Env = &'a JNIEnv<'a>> + Default,
	D: core::fmt::Debug,
{
	type Ext = Inner::Ext;
	type Env = Inner::Env;

	fn convert(env: Self::Env, val: Self) -> Self::Ext {
		match val {
			Ok(inner) => Return::convert(env, inner),
			Err(e) => {
				// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
				// !!!!																 !!!!
				// !!!! RETURN VALUE HAS TO BE CREATED BEFORE THROWING THE EXCEPTION !!!!
				// !!!!																 !!!!
				// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
				let ret = Return::convert_without_exception(env, Inner::default());
				// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
				// !!!!														   !!!!
				// !!!! WE CAN NO LONGER INTERACT WITH JNIENV AFTER THROWING   !!!!
				// !!!!														   !!!!
				// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
				env.throw(format!("{:?}", e))
					.expect("Must be able to throw the Exception; qed");
				ret
			}
		}
	}

	fn convert_without_exception(env: Self::Env, val: Self) -> Self::Ext {
		match val {
			Ok(inner) => Return::convert(env, inner),
			Err(_) => Return::convert_without_exception(env, Inner::default()),
		}
	}
}
