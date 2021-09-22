use super::result::*;

use jni::objects::JObject;
use jni::strings::JNIString;
use jni::sys::jsize;
use jni::JNIEnv;
use jni_glue::{ByteArray, Local, ObjectArray};

use android::content::{Context, Intent};
use android::security::keystore::{
	KeyGenParameterSpec, KeyGenParameterSpec_Builder, KeyProperties,
};
use android::util::Base64;
use java::lang::{CharSequence, Throwable};
use java::security::spec::AlgorithmParameterSpec;
use java::security::{Key, KeyStore};
use javax::crypto::spec::IvParameterSpec;
use javax::crypto::{Cipher, KeyGenerator, SecretKey};
use jni_android_sys::*;

pub type JavaString = java::lang::String;

pub fn java_string<'a, S>(env: &'a JNIEnv, s: &S) -> Local<'a, JavaString>
where
	S: Into<JNIString> + std::fmt::Debug + std::convert::AsRef<str>,
{
	unsafe {
		Local::from_env_object(
			env.get_native_interface(),
			env.new_string(s)
				.expect(&format!("Creating java string for '{:?}'", s))
				.into_inner(),
		)
	}
}

pub fn java_string_array<'a>(
	env: &'a JNIEnv,
	size: jsize,
) -> Result<Local<'a, ObjectArray<JavaString, Throwable>>> {
	unsafe {
		let class = env
			.find_class("java/lang/String")
			.map_err(|e| e.description().to_string())?;
		let object = env
			.new_object_array(size, class, JObject::null())
			.map_err(|e| e.description().to_string())?;
		let exception = env
			.exception_occurred()
			.map_err(|e| e.description().to_string())?;
		assert!(exception.is_null()); // Only sane exception here is an OOM exception
		Ok(Local::from_env_object(env.get_native_interface(), object))
	}
}

pub fn java_key_generator<'a>(
	env: &'a JNIEnv,
	algorithm: &'a JavaString,
	provider: &'a JavaString,
) -> Result<Local<'a, KeyGenerator>> {
	resopt!(KeyGenerator::getInstance_String_String(
		unsafe { jni_glue::Env::from_ptr(env.get_native_interface()) },
		Some(algorithm),
		Some(provider)
	))
}

pub fn java_algorithm_parameter_spec<'a>(
	env: &'a JNIEnv,
	alias: &'a JavaString,
	block_mode: &'a ObjectArray<JavaString, Throwable>,
	padding: &'a ObjectArray<JavaString, Throwable>,
	key_size: i32,
	with_biometry: bool,
) -> Result<Local<'a, AlgorithmParameterSpec>> {
	let x: Local<'a, KeyGenParameterSpec_Builder> =
		stringify_throwable!(KeyGenParameterSpec_Builder::new(
			unsafe { jni_glue::Env::from_ptr(env.get_native_interface()) },
			alias,
			KeyProperties::PURPOSE_ENCRYPT | KeyProperties::PURPOSE_DECRYPT,
		))?;
	r#try!(resopt!(x.setKeySize(key_size)));
	r#try!(resopt!(x.setBlockModes(Some(&*block_mode))));
	r#try!(resopt!(x.setEncryptionPaddings(Some(&*padding))));
	r#try!(resopt!(x.setRandomizedEncryptionRequired(true))); // indistinguishability under chosen-plaintext attack (IND-CPA)
	if with_biometry {
		r#try!(resopt!(x.setUserAuthenticationRequired(true))); // requires biometric auth before every key use; at least one fingerprint must be enrolled
	} else {
		r#try!(resopt!(x.setUserAuthenticationRequired(false)));
	}
	//    r#try!(resopt!(x.setInvalidatedByBiometricEnrollment(false))); // defaults to true
	let built = r#try!(resopt!(x.build()));
	Ok(unsafe {
		std::mem::transmute::<Local<'_, KeyGenParameterSpec>, Local<'_, AlgorithmParameterSpec>>(
			built,
		)
	})
}

pub fn java_algorithm_parameter_spec_from_bytes<'a>(
	env: &'a JNIEnv,
	iv: &'a ByteArray,
) -> Result<Local<'a, AlgorithmParameterSpec>> {
	let spec = stringify_throwable!(IvParameterSpec::new_byte_array(
		unsafe { jni_glue::Env::from_ptr(env.get_native_interface()) },
		Some(iv)
	))?;
	Ok(unsafe {
		std::mem::transmute::<Local<'_, IvParameterSpec>, Local<'_, AlgorithmParameterSpec>>(spec)
	})
}

pub fn java_generate_key<'a>(keygen: &'a KeyGenerator) -> Result<Local<'a, Key>> {
	let key = r#try!(resopt!(keygen.generateKey()));
	Ok(unsafe { std::mem::transmute::<Local<'_, SecretKey>, Local<'_, Key>>(key) })
}

pub fn java_cipher<'a>(
	env: &'a JNIEnv,
	transform: &'a JavaString,
	mode: i32,
	secret_key: Local<'a, Key>,
	spec: Option<&'a AlgorithmParameterSpec>,
) -> Result<Local<'a, Cipher>> {
	let cipher = r#try!(resopt!(Cipher::getInstance_String(
		unsafe { jni_glue::Env::from_ptr(env.get_native_interface()) },
		transform
	)));
	let _ = stringify_throwable!(cipher.init_int_Key_AlgorithmParameterSpec(
		mode,
		Some(&*secret_key),
		spec
	))?;
	Ok(cipher)
}

pub fn java_base64_encode<'a>(
	env: &'a JNIEnv,
	bytes: &'a ByteArray,
) -> Result<Local<'a, JavaString>> {
	resopt!(Base64::encodeToString_byte_array_int(
		unsafe { jni_glue::Env::from_ptr(env.get_native_interface()) },
		Some(bytes),
		Base64::DEFAULT
	))
}

pub fn java_base64_decode<'a>(env: &'a JNIEnv, s: &'a JavaString) -> Result<Local<'a, ByteArray>> {
	resopt!(Base64::decode_String_int(
		unsafe { jni_glue::Env::from_ptr(env.get_native_interface()) },
		Some(s),
		Base64::DEFAULT
	))
}

pub fn java_context<'a>(env: &'a JNIEnv, activity: &'a JObject) -> Local<'a, Context> {
	unsafe { Local::from_env_object(env.get_native_interface(), activity.into_inner()) }
}

pub fn java_keystore<'a>(env: &'a JNIEnv, provider: &'a JavaString) -> Result<Local<'a, KeyStore>> {
	resopt!(KeyStore::getInstance_String(
		unsafe { jni_glue::Env::from_ptr(env.get_native_interface()) },
		provider
	))
}
