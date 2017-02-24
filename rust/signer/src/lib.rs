extern crate libc;
extern crate ethkey;

mod string;

use ethkey::{KeyPair, Generator, Brain, Message, sign};
use string::StringPtr;

// string ffi

#[no_mangle]
pub unsafe extern fn rust_string_ptr(s: *mut String) -> *mut StringPtr {
  Box::into_raw(Box::new(StringPtr::from(&**s)))
}

#[no_mangle]
pub unsafe extern fn rust_string_destroy(s: *mut String) {
  let _ = Box::from_raw(s);
}

#[no_mangle]
pub unsafe extern fn rust_string_ptr_destroy(s: *mut StringPtr) {
  let _ = Box::from_raw(s);
}

// ethkey ffi

#[no_mangle]
pub unsafe extern fn ethkey_keypair_destroy(keypair: *mut KeyPair) {
  let _ = Box::from_raw(keypair);
}

#[no_mangle]
pub unsafe extern fn ethkey_keypair_brainwallet(seed: *mut StringPtr) -> *mut KeyPair {
  let generator = Brain::new((*seed).as_str().to_owned());
  Box::into_raw(Box::new(generator.generate().unwrap()))
}

#[no_mangle]
pub unsafe extern fn ethkey_keypair_secret(keypair: *mut KeyPair) -> *mut String {
  let secret = format!("{:?}", (*keypair).secret());
  Box::into_raw(Box::new(secret))
}

#[no_mangle]
pub unsafe extern fn ethkey_keypair_address(keypair: *mut KeyPair) -> *mut String {
  let address = format!("{:?}", (*keypair).address());
  Box::into_raw(Box::new(address))
}

#[no_mangle]
pub unsafe extern fn ethkey_keypair_sign(keypair: *mut KeyPair, message: *mut StringPtr) -> *mut String {
  let secret = (*keypair).secret();
  let message: Message = (*message).as_str().parse().unwrap();
  let signature = format!("{:?}", sign(secret, &message).unwrap());
  Box::into_raw(Box::new(signature))
}

#[no_mangle]
pub unsafe extern fn rlp_item(rlp: *mut StringPtr, position: u32) -> *mut String {
  unimplemented!();
}
