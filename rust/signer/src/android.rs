#![allow(non_snake_case)]

use wordlist;
use super::*;
use jni::JNIEnv;
use jni::objects::{JClass, JString, JThrowable};
use jni::sys::{jint, jstring};

fn new_exception<'a>(env: &'a JNIEnv<'a>) -> JThrowable<'a> {
  let exception = env.find_class("java/lang/Exception").unwrap();
  // javap -s java.lang.Exception
  env.new_object(exception, "()V", &[]).unwrap().into()
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_parity_signer_EthkeyBridge_ethkeyBrainwalletAddress(
  env: JNIEnv,
  _: JClass,
  seed: JString,
) -> jstring {
  let seed: String = env.get_string(seed).expect("Invalid seed").into();
  let keypair = Brain::new(seed).generate().unwrap();
  let java_address = env
    .new_string(format!("{:?}", keypair.address()))
    .expect("Could not create java string");
  java_address.into_inner()
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_parity_signer_EthkeyBridge_ethkeyBrainwalletSecret(
  env: JNIEnv,
  _: JClass,
  seed: JString,
) -> jstring {
  let seed: String = env.get_string(seed).expect("Invalid seed").into();
  let keypair = Brain::new(seed).generate().unwrap();
  let java_secret = env
    .new_string(format!("{:?}", keypair.secret()))
    .expect("Could not create java string");
  java_secret.into_inner()
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_parity_signer_EthkeyBridge_ethkeyBrainwalletSign(
  env: JNIEnv,
  _: JClass,
  seed: JString,
  message: JString,
) -> jstring {
  let seed: String = env.get_string(seed).expect("Invalid seed").into();
  let message: String = env.get_string(message).expect("Invalid message").into();
  let keypair = Brain::new(seed).generate().unwrap();
  let message: Message = message.parse().unwrap();
  let signature = sign(keypair.secret(), &message).unwrap();
  let java_signature = env
    .new_string(format!("{}", signature))
    .expect("Could not create java string");
  java_signature.into_inner()
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_parity_signer_EthkeyBridge_ethkeyRlpItem(
  env: JNIEnv,
  _: JClass,
  data: JString,
  position: jint,
) -> jstring {
  let data: String = env.get_string(data).expect("Invalid seed").into();
  match safe_rlp_item(&data, position as u32) {
    Ok(result) => env
      .new_string(result)
      .expect("Could not create java string")
      .into_inner(),
    Err(_) => {
      let res = env.new_string("").expect("").into_inner();
      env.throw(new_exception(&env)).unwrap();
      res
    }
  }
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_parity_signer_EthkeyBridge_ethkeyKeccak(
  env: JNIEnv,
  _: JClass,
  data: JString,
) -> jstring {
  let data: String = env.get_string(data).expect("Invalid seed").into();
  let hex = data.from_hex().unwrap();
  let mut res: [u8; 32] = [0; 32];
  let mut keccak = Keccak::new_keccak256();
  keccak.update(&hex);
  keccak.finalize(&mut res);
  env
    .new_string(res.to_hex())
    .expect("Could not create java string")
    .into_inner()
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_parity_signer_EthkeyBridge_ethkeyEthSign(
  env: JNIEnv,
  _: JClass,
  data: JString,
) -> jstring {
  let data: String = env.get_string(data).expect("Invalid seed").into();
  let hex = data.from_hex().unwrap();
  let message = format!("\x19Ethereum Signed Message:\n{}", hex.len()).into_bytes();
  let mut res: [u8; 32] = [0; 32];
  let mut keccak = Keccak::new_keccak256();
  keccak.update(&message);
  keccak.update(&hex);
  keccak.finalize(&mut res);
  env
    .new_string(res.to_hex())
    .expect("Could not create java string")
    .into_inner()
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_parity_signer_EthkeyBridge_ethkeyBlockiesIcon(
  env: JNIEnv,
  _: JClass,
  seed: JString,
) -> jstring {
  let seed: String = env.get_string(seed).expect("Invalid seed").into();
  let icon = blockies_icon_in_base64(seed.into());
  env
    .new_string(icon)
    .expect("Could not create java string")
    .into_inner()
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_parity_signer_EthkeyBridge_ethkeyRandomPhrase(
  env: JNIEnv,
  _: JClass,
  words: jint,
) -> jstring {
  let words = wordlist::random_phrase(words as usize);
  env
    .new_string(words)
    .expect("Could not create java string")
    .into_inner()
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_parity_signer_EthkeyBridge_ethkeyEncryptData(
  env: JNIEnv,
  _: JClass,
  data: JString,
  password: JString,
) -> jstring {
  let data: String = env.get_string(data).expect("Invalid data").into();
  let password: String = env.get_string(password).expect("Invalid password").into();
  let password = Password::from(password);
  let crypto = Crypto::with_plain(data.as_bytes(), &password, 10240).unwrap();
  env
    .new_string(String::from(crypto))
    .expect("Could not create java string")
    .into_inner()
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_parity_signer_EthkeyBridge_ethkeyDecryptData(
  env: JNIEnv,
  _: JClass,
  data: JString,
  password: JString,
) -> jstring {
  let data: String = env.get_string(data).expect("Invalid data").into();
  let password: String = env.get_string(password).expect("Invalid password").into();
  let password = Password::from(password);
  let crypto: Crypto = match data.parse() {
    Ok(crypto) => crypto,
    Err(_) => {
      let result = env
        .new_string("")
        .expect("first result to be created")
        .into_inner();
      env.throw(new_exception(&env)).expect("first throw failed");
      return result;
    }
  };

  match crypto.decrypt(&password) {
    Ok(decrypted) => env
      .new_string(String::from_utf8_unchecked(decrypted))
      .expect("Could not create java string")
      .into_inner(),
    Err(_) => {
      let result = env
        .new_string("")
        .expect("second result to be created")
        .into_inner();
      env.throw(new_exception(&env)).expect("second throw failed");
      result
    }
  }
}

#[cfg(test)]
mod tests {
  extern crate jni;
  use std::os::raw::c_void;
  use std::ptr;
  use self::jni::sys::{JNI_CreateJavaVM, JNI_VERSION_1_8, JavaVM, JavaVMInitArgs, JavaVMOption,
                       JNI_EDETACHED, JNI_EEXIST, JNI_EINVAL, JNI_ENOMEM, JNI_ERR, JNI_EVERSION,
                       JNI_FALSE, JNI_OK};
  use ethstore::Crypto;

  use super::{Java_io_parity_signer_EthkeyBridge_ethkeyDecryptData, Password};

  #[link(name = "jvm")]
  extern "C" {}

  struct TestVM {
    _jvm: *mut JavaVM,
    sys_env: *mut jni::sys::JNIEnv,
  }

  impl TestVM {
    fn env<'a>(&'a self) -> jni::JNIEnv<'a> {
      jni::JNIEnv::from(self.sys_env)
    }
  }

  unsafe fn test_vm() -> TestVM {
    let mut jvm_options = Vec::<JavaVMOption>::new();
    // Create the JVM arguments.
    let mut jvm_arguments = JavaVMInitArgs::default();
    jvm_arguments.version = JNI_VERSION_1_8;
    jvm_arguments.options = jvm_options.as_mut_ptr();
    jvm_arguments.nOptions = jvm_options.len() as i32;
    jvm_arguments.ignoreUnrecognized = JNI_FALSE;

    // Initialize space for a pointer to the JNI environment.
    let mut jvm: *mut JavaVM = ptr::null_mut();
    let mut jni_environment: *mut jni::sys::JNIEnv = ptr::null_mut();

    // Try to instantiate the JVM.
    let result = JNI_CreateJavaVM(
      &mut jvm,
      (&mut jni_environment as *mut *mut jni::sys::JNIEnv) as *mut *mut c_void,
      (&mut jvm_arguments as *mut JavaVMInitArgs) as *mut c_void,
    );

    // There was an error while trying to instantiate the JVM.
    if result != JNI_OK {
      // Translate the error code to a message.
      let error_message = match result {
        JNI_EDETACHED => "thread detached from JVM",
        JNI_EEXIST => "JVM exists already",
        JNI_EINVAL => "invalid arguments",
        JNI_ENOMEM => "not enough memory",
        JNI_ERR => "unknown error",
        JNI_EVERSION => "JNI version error",
        _ => "unknown JNI error value",
      };

      panic!("`JNI_CreateJavaVM()` signaled an error: {}", error_message);
    }

    TestVM {
      _jvm: jvm,
      sys_env: jni_environment,
    }
  }

  #[test]
  fn test_decrypt() {
    unsafe {
      let jvm = test_vm();

      let data = b"test_data";
      let password = Password::from("password");
      let crypto = Crypto::with_plain(data, &password, 10240).unwrap();
      let crypto_string: String = crypto.into();
      let env = jvm.env();
      let jni_crypto_str = env.new_string(crypto_string).unwrap();
      let jni_password_str = env.new_string(password).unwrap();
      let any_class = env.find_class("java/lang/Object").unwrap();

      let result = Java_io_parity_signer_EthkeyBridge_ethkeyDecryptData(
        jvm.env(),
        any_class,
        jni_crypto_str,
        jni_password_str,
      );

      let result: String = env
        .get_string(result.into())
        .expect("invalid result")
        .into();
      assert_eq!(result, "test_data".to_owned());
      assert_eq!(env.exception_check().unwrap(), false);

      let _ = Java_io_parity_signer_EthkeyBridge_ethkeyDecryptData(
        jvm.env(),
        any_class,
        jni_crypto_str,
        env.new_string("wrong password").unwrap(),
      );
      assert_eq!(env.exception_check().unwrap(), true);
    }
  }
}
