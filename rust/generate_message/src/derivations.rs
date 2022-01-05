use hex;

use db_handling::identities::prepare_derivations_export;
use definitions::error::ErrorActive;
use qrcode_rtx::make_pretty_qr;

use crate::helpers::get_address_book_entry;
use crate::parser::{Derivations, Goal};

pub fn process_derivations (x: Derivations) -> Result<(), ErrorActive> {
    let address_book_entry = get_address_book_entry(&x.title)?;
    let content = prepare_derivations_export(&address_book_entry.encryption, &address_book_entry.genesis_hash, &x.derivations)?;
    let prelude = hex::decode("53ffde").expect("known static value");
    let complete_message = [prelude, content.to_transfer()].concat();
    let output_name = format!("derivations-{}", x.title);
    match x.goal {
        Goal::Qr => {
            if let Err(e) = make_pretty_qr(&complete_message, &output_name) {return Err(ErrorActive::Qr(e.to_string()))}
        },
        Goal::Text => {
            if let Err(e) = std::fs::write(&format!("{}.txt", output_name), &hex::encode(&complete_message)) {return Err(ErrorActive::Output(e))}
        },
        Goal::Both => {
            if let Err(e) = std::fs::write(&format!("{}.txt", output_name), &hex::encode(&complete_message)) {return Err(ErrorActive::Output(e))}
            if let Err(e) = make_pretty_qr(&complete_message, &output_name) {return Err(ErrorActive::Qr(e.to_string()))}
        },
    }
    Ok(())
}
