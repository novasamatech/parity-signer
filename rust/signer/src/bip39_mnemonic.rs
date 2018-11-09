use bip39::{Language, Mnemonic, MnemonicType};

pub unsafe extern "C" fn bip39_random_phrase(words: usize) -> *mut String {
	let mnemonic_type = MnemonicType::for_word_count(words).unwrap();
	Box::into_raw(Box::new(
		match Mnemonic::new(mnemonic_type, Language::English, "") {
			Ok(phrase) => phrase.get_string(),
			Err(_) => String::new(),
		},
	))
}

