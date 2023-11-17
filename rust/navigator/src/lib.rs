//! This is experimental cross-platform navigation for Vault.
//! Ideally it should replace almost everything and become the only interface

#![deny(unused_crate_dependencies)]
#![deny(rustdoc::broken_intra_doc_links)]

use db_handling::identities::{export_key_set_addrs, SignaturesBulk, SignaturesBulkV1};
//do we support mutex?
use lazy_static::lazy_static;
use sp_runtime::MultiSignature;
use std::{collections::HashMap, sync::Mutex};
use transaction_signing::{
    create_signature, sign_content, SignatureAndChecksum, SignatureType, SufficientContent,
};

use definitions::navigation::{
    ActionResult, Address, ExportedSet, MAddressCard, MKeysInfoExport, MKeysNew, MSignatureReady,
    MSignedTransaction, MSufficientCryptoReady, MTransaction, QrData, TransactionAction,
    TransactionSignAction, TransactionType,
};
use parity_scale_codec::Encode;
use qrcode_rtx::make_data_packs;

mod error;

mod actions;
pub use actions::Action;
use db_handling::helpers::get_address_details;
use definitions::helpers::{make_identicon_from_multisigner, print_multisigner_as_base58_or_eth};
use definitions::keyring::AddressKey;

pub mod alerts;
pub mod modals;
mod navstate;
mod states;
use navstate::State;
use transaction_parsing::parse_transaction::parse_dd_transaction;

pub mod screens;
#[cfg(test)]
mod tests;

pub use crate::error::{Error, Result};
use crate::states::{SignResult, TransactionState};

//TODO: multithread here some day!
lazy_static! {
    /// Navigation state of the app
    ///
    /// Navigation state is unsafe either way, since it has to persist
    /// No matter if here or beyond FFI
    pub static ref STATE: Mutex<Option<State>> = Mutex::new(
        None
    );
}

/// User actions handler.
///
/// This method is called on every user [`Action`] in the UI, performs changes in backend
/// and returns new UI information as [`ActionResult`].
pub fn do_action(
    action: Action,
    details_str: &str,
    secret_seed_phrase: &str,
) -> Result<ActionResult> {
    let mut navstate = STATE.lock().map_err(|_| Error::MutexPoisoned)?;
    navstate.as_mut().ok_or(Error::DbNotInitialized)?.perform(
        action,
        details_str,
        secret_seed_phrase,
    )
}

/// Should be called in the beginning to recall things stored only by phone
pub fn init_navigation(db: sled::Db, seed_names: Vec<String>) -> Result<()> {
    let mut navstate = STATE.lock().map_err(|_| Error::MutexPoisoned)?;
    *navstate = Some(State::init_navigation(db, seed_names));
    Ok(())
}

/// Generate transaction from qr codes assembled data
pub fn get_transaction(database: &sled::Db, payload: &str) -> Result<TransactionAction> {
	//return mtransaction data??? to know which keys used to sign and transaction action
    Ok(TransactionState::new(database, details_str)?.action()?)
}

//todo validate password for key function create

/// all passwords should be validated separately before and here we just signing
/// transaction, that can be bulk, and expecting that error should never ocure
pub fn sign_transaction(database: &sled::Db,
												//list of seeds if needed,
												transaction_state: TransactionAction,
												password: Option<&str>,//todo pass passwords for keys
												) -> Result<()> {
	match transaction_state.action() {
		transaction_parsing::TransactionAction::Sign { .. } => {
			let mut new = transaction_state.clone();
			new.update_seeds(secret_seed_phrase);
			match password {
				None => {}
				Some(pass) => {new.password_entered(pass);}
			}

			match new.handle_sign(database) {
				Ok(result) => {
					match result {
						SignResult::RequestPassword { .. } => {
							if t.ok() {
								// here juse return wrong password error
								new_navstate.screen = Screen::Transaction(new);
								new_navstate.modal = Modal::EnterPassword;
							} else {
								new_navstate = Navstate::clean_screen(Screen::Log);
							}
						}
						SignResult::Ready { signatures } => {
							// result
							new_navstate.modal = Modal::SignatureReady(signatures);
						}
					};
				}
				Err(e) => {
					// new_navstate.alert = Alert::Error;
					// let _ = write!(&mut errorline, "{e}");
				// 	todo pass our error displayed with str
				}
			}
		}
		transaction_parsing::TransactionAction::Stub {
			s: _,
			u: checksum,
			stub: stub_nav,
		} => match transaction_signing::handle_stub(&self.db, *checksum) {
			//todo check when we actually executing this actions in state machine. Not when transaction signing happening looks like
			Ok(()) => match stub_nav {
				transaction_parsing::StubNav::AddSpecs {
					n: network_specs_key,
				} => {
					// check in what we do here, we may returm type of transaction or just blank processed
					// check if we call sign for those transactions at all from mobile end
					new_navstate = Navstate::clean_screen(Screen::NetworkDetails(
						network_specs_key.clone(),
					));
				}
				transaction_parsing::StubNav::LoadMeta {
					l: network_specs_key,
				} => {
					new_navstate = Navstate::clean_screen(Screen::NetworkDetails(
						network_specs_key.clone(),
					));
				}
				transaction_parsing::StubNav::LoadTypes => {
					new_navstate = Navstate::clean_screen(Screen::ManageNetworks);
				}
			},
			Err(e) => {
				// new_navstate.alert = Alert::Error;
				// let _ = write!(&mut errorline, "{e}");
				// 	todo pass our error displayed with str
			}
		},
		transaction_parsing::TransactionAction::Read { .. } => {
		// 	do nothing
		}
		transaction_parsing::TransactionAction::Derivations { content: _ } => {
			new_navstate = Navstate::clean_screen(Screen::SeedSelector);
		}
	},

	Ok(()) //todo dmitry implement
}

/// Export key info with derivations.
pub fn export_key_info(
    database: &sled::Db,
    seed_name: &str,
    exported_set: ExportedSet,
) -> Result<MKeysInfoExport> {
    let export_all_addrs = export_key_set_addrs(database, seed_name, exported_set)?;

    let data = [&[0x53, 0xff, 0xde], export_all_addrs.encode().as_slice()].concat();
    let frames = make_data_packs(&data, 128).map_err(|e| Error::DataPacking(e.to_string()))?;

    Ok(MKeysInfoExport { frames })
}

/// Export signatures bulk.
pub fn export_signatures_bulk(
    signatures: &[(MultiSignature, SignatureType)],
) -> Result<MSignatureReady> {
    let signatures = if signatures.len() > 1 {
        let v1: SignaturesBulkV1 = signatures
            .iter()
            .map(|s| s.0.clone())
            .collect::<Vec<_>>()
            .as_slice()
            .into();
        let v1: SignaturesBulk = v1.into();
        let data = v1.encode();

        make_data_packs(&data, 128).map_err(|e| Error::DataPacking(e.to_string()))?
    } else {
        let encoded = match signatures[0].1 {
            SignatureType::Transaction => hex::encode(signatures[0].0.encode()),
            SignatureType::Message => match &signatures[0].0 {
                MultiSignature::Ed25519(a) => hex::encode(a),
                MultiSignature::Sr25519(a) => hex::encode(a),
                MultiSignature::Ecdsa(a) => hex::encode(a),
            },
        };
        vec![QrData::Regular {
            data: encoded.as_bytes().into(),
        }]
    };

    Ok(MSignatureReady { signatures })
}

/// Sign dynamic derivation transaction and return data for mobile
pub fn sign_dd_transaction(
    database: &sled::Db,
    payload_set: &[String],
    seeds: HashMap<String, String>,
) -> Result<MSignedTransaction> {
    let mut transactions = vec![];
    let mut signatures = vec![];
    for (a, signature) in handle_dd_sign(database, payload_set, seeds)? {
        transactions.push(MTransaction {
            content: a.content.clone(),
            ttype: TransactionType::Sign,
            author_info: Some(a.author_info.clone()),
            network_info: Some(a.network_info.clone().into()),
        });
        signatures.push((signature.signature().to_owned(), signature.signature_type()));
    }
    Ok(MSignedTransaction {
        transaction: transactions,
        signature: export_signatures_bulk(&signatures)?,
    })
}

/// Parse and sign dynamic derivation transaction
pub(crate) fn handle_dd_sign(
    database: &sled::Db,
    payload_set: &[String],
    seeds: HashMap<String, String>,
) -> Result<Vec<(TransactionSignAction, SignatureAndChecksum)>> {
    let mut signed_transactions = vec![];

    let mut actions = vec![];
    let mut checksum = 0;
    for t in payload_set.iter() {
        match parse_dd_transaction(database, t, &seeds) {
            Ok(TransactionAction::Sign {
                actions: a,
                checksum: c,
            }) => {
                actions.extend(a);
                checksum = c;
            }
            Ok(_) => return Err(Error::TxActionNotSign),
            Err(e) => return Err(e.into()),
        };
    }
    for (idx, sign_action) in actions.into_iter().enumerate() {
        let seed_phrase = seeds
            .get(&sign_action.author_info.address.seed_name)
            .ok_or(Error::NoSeedPhrase)?;

        let signature_and_checksum = create_signature(
            database,
            seed_phrase,
            "",
            "",
            checksum,
            idx,
            sign_action.network_info.specs.encryption,
        )?;
        checksum = signature_and_checksum.new_checksum();
        signed_transactions.push((sign_action, signature_and_checksum));
    }
    Ok(signed_transactions)
}

/// Get keys by seed name
pub fn keys_by_seed_name(database: &sled::Db, seed_name: &str) -> Result<MKeysNew> {
    Ok(db_handling::interface_signer::keys_by_seed_name(
        database, seed_name,
    )?)
}

pub fn sign_sufficient_content(
    database: &sled::Db,
    address_key: &AddressKey,
    sufficient_content: SufficientContent,
    seed_phrase: &str,
    pwd_entry: &str,
) -> Result<MSufficientCryptoReady> {
    let address_details = get_address_details(database, address_key)?;
    let multisigner = address_key.multi_signer();
    let (sufficient, content) = sign_content(
        database,
        multisigner,
        &address_details,
        sufficient_content,
        seed_phrase,
        pwd_entry,
    )?;
    let network_key = address_details
        .network_id
        .as_ref()
        .ok_or(Error::NoNetwork(address_details.path.clone()))?;
    let network_specs = db_handling::helpers::get_network_specs(database, network_key)?.specs;
    let base58 = print_multisigner_as_base58_or_eth(
        multisigner,
        Some(network_specs.base58prefix),
        network_specs.encryption,
    );
    Ok(MSufficientCryptoReady {
        author_info: MAddressCard {
            base58,
            address_key: hex::encode(address_key.key()),
            address: Address {
                path: address_details.path.clone(),
                has_pwd: address_details.has_pwd,
                identicon: make_identicon_from_multisigner(
                    multisigner,
                    address_details.identicon_style(),
                ),
                seed_name: address_details.seed_name,
                secret_exposed: address_details.secret_exposed,
            },
        },
        sufficient,
        content,
        network_logo: Some(network_specs.logo),
    })
}
