#![no_main]
use libfuzzer_sys::fuzz_target;
use qr_reader_phone::get_length;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = get_length(s, false);
    }
});
