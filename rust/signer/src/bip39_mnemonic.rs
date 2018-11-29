use bip39::{Language, Mnemonic, MnemonicType};
use string::StringPtr;

#[no_mangle]
pub unsafe extern "C" fn bip39_mnemonic_random(words_count: usize) -> *mut Mnemonic {
	let mnemonic_type = MnemonicType::for_word_count(words_count).unwrap();
	Box::into_raw(
		Box::new(
			Mnemonic::new(
				mnemonic_type,
				Language::English,
				""
			).expect("Unable to generate random phrase")
		)
	)
}

#[no_mangle]
pub unsafe extern "C" fn bip39_mnemonic_from_string(string: *mut StringPtr) -> *mut Mnemonic {
	Box::into_raw(
		Box::new(
			Mnemonic::from_string(
				(*string).as_str(),
				Language::English,
				""
			).expect("Unable to restore mnemonic from phrase string")
		)
	)
}

#[no_mangle]
pub unsafe extern fn bip39_mnemonic_destroy(mnemonic: *mut Mnemonic) {
  let _ = Box::from_raw(mnemonic);
}

#[no_mangle]
pub unsafe extern fn bip39_mnemonic_string(mnemonic: *mut Mnemonic) -> *mut String {
  let string = format!("{:?}", (*mnemonic).get_string());
  Box::into_raw(Box::new(string))
}
