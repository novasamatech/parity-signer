extern crate libc;
extern crate ethkey;

mod string;

use ethkey::{KeyPair, Generator, Brain};
use string::StringPtr;

#[no_mangle]
pub unsafe extern fn ethkey_string_destroy(s: *mut StringPtr) {
  let _ = Box::from_raw(s);
}

#[no_mangle]
pub extern fn ethkey_keypair_brainwallet(seed: *mut StringPtr) -> *mut KeyPair {
  let seed = unsafe { Box::from_raw(seed) };
  let generator = Brain::new(seed.as_str().to_owned());
  let keypair = generator.generate().unwrap();
  let boxed_keypair = Box::new(keypair);
  let _ = Box::into_raw(seed);
  Box::into_raw(boxed_keypair)
}

#[no_mangle]
pub unsafe extern fn ethkey_keypair_destroy(keypair: *mut KeyPair) {
  let _ = Box::from_raw(keypair);
}

#[no_mangle]
pub extern fn ethkey_keypair_secret(keypair: *mut KeyPair) -> *mut StringPtr {
  let keypair = unsafe { Box::from_raw(keypair) };
  let secret = keypair.secret().to_string();
  let secret_ptr = StringPtr::from(&*secret);
  let _ = Box::into_raw(keypair);
  let _ = Box::into_raw(Box::new(secret));
  Box::into_raw(Box::new(secret_ptr))
}

#[no_mangle]
pub extern fn ethkey_keypair_address(keypair: *mut KeyPair) -> *mut StringPtr {
  let keypair = unsafe { Box::from_raw(keypair) };
  let address = keypair.address().to_string();
  let address_ptr = StringPtr::from(&*address);
  let _ = Box::into_raw(keypair);
  let _ = Box::into_raw(Box::new(address));
  Box::into_raw(Box::new(address_ptr))
}
