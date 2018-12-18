use ethstore::Crypto;
use ethkey::Password;
use string::StringPtr;
// data encryption ffi

#[no_mangle]
pub unsafe extern fn encrypt_data(data: *mut StringPtr, password: *mut StringPtr) -> *mut String {
  let data = (*data).as_str();
  let password = Password::from((*password).as_str());
  let crypto = Crypto::with_plain(data.as_bytes(), &password, 10240).unwrap();
  Box::into_raw(Box::new(crypto.into()))
}

#[no_mangle]
pub unsafe extern fn decrypt_data(encrypted_data: *mut StringPtr, password: *mut StringPtr, error: *mut u32) -> *mut String {
  let data = (*encrypted_data).as_str();
  let password = Password::from((*password).as_str());
  let crypto: Crypto = match data.parse() {
    Ok(crypto) => crypto,
    Err(_) => {
      *error = 1;
      return Box::into_raw(Box::new(String::new()))
    }
  };
  match crypto.decrypt(&password) {
    Ok(decrypted) => {
      Box::into_raw(Box::new(String::from_utf8_unchecked(decrypted)))
    },
    Err(_) => {
      *error = 2;
      Box::into_raw(Box::new(String::new()))
    },
  }
}
