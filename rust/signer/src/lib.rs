// Copyright 2015-2021 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

use pixelate::{Color, Image, BLACK};
use qrcodegen::{QrCode, QrCodeEcc};

use db_handling;
use transaction_parsing;
use transaction_signing;

mod export;
mod metadata;
mod qr;
mod result;

fn base64png(png: &[u8]) -> String {
	static HEADER: &str = "data:image/png;base64,";
	let mut out = String::with_capacity(png.len() + png.len() / 2 + HEADER.len());
	out.push_str(HEADER);
	base64::encode_config_buf(png, base64::STANDARD, &mut out);
	out
}

fn qrcode_bytes(data: &[u8]) -> std::result::Result<String, Box<dyn std::error::Error>> {
	let qr = QrCode::encode_binary(data, QrCodeEcc::Medium)?;
	let palette = &[Color::Rgba(255, 255, 255, 0), BLACK];
	let mut pixels = Vec::with_capacity((qr.size() * qr.size()) as usize);
	for y in 0..qr.size() {
		for x in 0..qr.size() {
			pixels.push(qr.get_module(x, y) as u8);
		}
	}
	let mut result = Vec::new();
	let image = Image {
		palette,
		pixels: &pixels,
		width: qr.size() as usize,
		scale: 16,
	};
	match image.render(&mut result) {
    	Ok(_) => Ok(base64png(&result)),
        Err(_) => return Err(Box::from("Pixelation failed")),
    }
}

export! {
	@Java_io_parity_signer_SubstrateSignModule_ethkeyQrCode
	fn qrcode(
		data: &str
	) -> std::result::Result<String, Box<dyn std::error::Error>> {
		qrcode_bytes(data.as_bytes())
	}

	@Java_io_parity_signer_SubstrateSignModule_qrparserTryDecodeQrSequence
	fn try_decode_qr_sequence(
        size: u32,
        chunk_size: u32,
		data: &str
	) -> std::result::Result<String, Box<dyn std::error::Error>> {
        let data_vec: Vec<&str> = qr::deserialize(data);
        let answer = qr::parse_goblet(size as u64, chunk_size as u16, data_vec);
        Ok(answer)
	}

    @Java_io_parity_signer_SubstrateSignModule_metadataGenerateMetadataHandle
	fn generate_metadata_handle(
		metadata: &str
	) -> String {
        metadata::meta_to_json(metadata)
	}

    @Java_io_parity_signer_SubstrateSignModule_substrateParseTransaction
	fn parse_transaction(
		transaction: &str,
        dbname: &str
	) -> String {
        if transaction == "test all" {return transaction_parsing::test_all_cards::make_all_cards()}
        else {return transaction_parsing::produce_output(transaction, dbname)}
    }

    @Java_io_parity_signer_SubstrateSignModule_substrateSignTransaction
	fn sign_transaction(
		action: &str,
        seed_phrase: &str,
        password: &str,
        dbname: &str
	) -> std::result::Result<String, Box<dyn std::error::Error>> {
        transaction_signing::handle_action(action, seed_phrase, password, dbname)
    }

    @Java_io_parity_signer_SubstrateSignModule_substrateDevelopmentTest
	fn development_test(
		_input: &str
	) -> std::result::Result<String, Box<dyn std::error::Error>> {
        let output = Ok(std::env::consts::OS.to_string());
        return output;
    }

    @Java_io_parity_signer_SubstrateSignModule_dbGetNetwork
	fn get_network(
		genesis_hash: &str,
        dbname: &str
	) -> std::result::Result<String, Box<dyn std::error::Error>> {
        let spec = db_handling::chainspecs::get_network(dbname, genesis_hash)?;
        Ok(String::from(format!("{{\"color\":\"{}\",\"logo\":\"{}\",\"secondaryColor\":\"{}\",\"title\":\"{}\"}}",
            spec.color, 
            spec.logo,
            spec.secondary_color,
            spec.title)))
    }

    @Java_io_parity_signer_SubstrateSignModule_dbGetAllNetworksForNetworkSelector
	fn get_all_networks_for_network_selector(
        dbname: &str
    ) -> std::result::Result<String, Box<dyn std::error::Error>> {
        let specs = db_handling::chainspecs::get_all_networks(dbname)?;
        //TODO: gentler formatting, or serde-json?
        let mut output = "[".to_owned();
        for spec in specs {
            output.push_str(&format!("{{\"key\":\"{}\",\"color\":\"{}\",\"logo\":\"{}\",\"order\":\"{}\",\"secondaryColor\":\"{}\",\"title\":\"{}\"}},",
                hex::encode(spec.genesis_hash),
                spec.color, 
                spec.logo, 
                spec.order,
                spec.secondary_color,
                spec.title))
        }
        result::return_json_array(output)
    }

    @Java_io_parity_signer_SubstrateSignModule_dbAddNetwork
	fn add_network(
		_network_json: &str,
        _dbname: &str
	) -> std::result::Result<(), Box<dyn std::error::Error>> {
        
        Ok(())
    }

    @Java_io_parity_signer_SubstrateSignModule_dbRemoveNetwork
	fn remove_network(
		_genesis_hash: &str,
        _dbname: &str
	) -> std::result::Result<(), Box<dyn std::error::Error>> {
        
        Ok(())
    }

    @Java_io_parity_signer_SubstrateSignModule_dbGetRelevantIdentities
	fn get_relevant_identities(
		seed_name: &str,
        genesis_hash: &str,
        dbname: &str
	) -> std::result::Result<String, Box<dyn std::error::Error>> {
        let relevant_identities = db_handling::identities::get_relevant_identities(seed_name, genesis_hash, dbname)?;
        let prefix = db_handling::chainspecs::get_network(dbname, genesis_hash)?.base58prefix;
        let mut output = "[".to_owned();
        for (pubkey, identity) in relevant_identities.iter() {
            output.push_str(&format!("{{\"publicKey\":\"{}\",\"ss58\":\"{}\",\"path\":\"{}\",\"hasPassword\":\"{}\",\"name\":\"{}\"}},",
                hex::encode(pubkey),
                transaction_parsing::utils_base58::vec_to_base(pubkey, prefix),
                identity.path,
                identity.has_pwd,
                identity.name))
        }
        result::return_json_array(output)
    }

    @Java_io_parity_signer_SubstrateSignModule_dbAckUserAgreement
	fn ack_user_agreement(
		dbname: &str
	) -> std::result::Result<(), Box<dyn std::error::Error>> {
        db_handling::settings::ack_user_agreement(dbname)
    }

    @Java_io_parity_signer_SubstrateSignModule_dbCheckUserAgreement
	fn check_user_agreement(
		dbname: &str
	) -> std::result::Result<bool, Box<dyn std::error::Error>> {
        db_handling::settings::check_user_agreement(dbname)
    }
    
    @Java_io_parity_signer_SubstrateSignModule_substrateTryCreateSeed
	fn try_create_seed(
        seed_name: &str,
        crypto: &str,
        seed_phrase: &str,
        seed_length: u32,
		dbname: &str
	) -> std::result::Result<String, Box<dyn std::error::Error>> {
        db_handling::identities::try_create_seed(seed_name, crypto, seed_phrase, seed_length, dbname)
    }

    @Java_io_parity_signer_SubstrateSignModule_substrateSuggestNPlusOne
	fn suggest_n_plus_one(
        path: &str,
        seed_name: &str,
        network_id_string: &str,
        dbname: &str
	) -> std::result::Result<String, Box<dyn std::error::Error>> {
        db_handling::identities::suggest_n_plus_one(path, seed_name, network_id_string, dbname)
    }

    @Java_io_parity_signer_SubstrateSignModule_substrateCheckPath
	fn check_path(
        path: &str
	) -> std::result::Result<bool, Box<dyn std::error::Error>> {
        db_handling::identities::check_derivation_format(path)
    }

    @Java_io_parity_signer_SubstrateSignModule_substrateTryCreateIdentity
	fn try_create_identity(
        id_name: &str,
        seed_name: &str,
        seed_phrase: &str,
        crypto: &str,
        path: &str,
        network: &str,
        has_password: bool,
		dbname: &str
	) -> std::result::Result<(), Box<dyn std::error::Error>> {
        db_handling::identities::try_create_address(id_name, seed_name, seed_phrase, crypto, path, network, has_password, dbname)
    }

    @Java_io_parity_signer_SubstrateSignModule_substrateSuggestName
	fn suggest_name(
        path: &str
	) -> String {
        db_handling::identities::suggest_path_name(path)
    }

    @Java_io_parity_signer_SubstrateSignModule_substrateDeleteIdentity
	fn delete_identity(
        pub_key: &str,
        network: &str,
        dbname: &str
	) -> std::result::Result<(), Box<dyn std::error::Error>> {
        db_handling::identities::delete_address(pub_key, network, dbname)
    }

    @Java_io_parity_signer_SubstrateSignModule_substrateGetNetworkSpecs
	fn get_network_specs(
        network: &str,
        dbname: &str
	) -> std::result::Result<String, Box<dyn std::error::Error>> {
        db_handling::network_details::get_network_details_by_hex(network, dbname)
    }

}

ffi_support::define_string_destructor!(signer_destroy_string);

#[cfg(test)]
mod tests {
	use super::*;
}
