extern crate bip39;
extern crate substrate_core;

use substrate_core::Pair;

#[no_mangle]
pub unsafe extern fn keypair_brainwallet(seed: *mut StringPtr) -> *mut KeyPair {
  let mut generator = Brain::new((*seed).as_str().to_owned());
  Box::into_raw(Box::new(generator.generate().unwrap()))
}
