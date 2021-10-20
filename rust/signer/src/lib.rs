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

//used for identicon size; because JNI has no signed int
use std::convert::TryInto;

use plot_icon;
use db_handling;
use transaction_parsing;
use transaction_signing;
use qr_reader_phone;

mod export;

export! {
	@Java_io_parity_signer_models_SignerDataModel_substrateExportPubkey
	fn export_pubkey(
		address: &str,
        network: &str,
        dbname: &str
	) -> anyhow::Result<String, anyhow::Error> {
		db_handling::identities::export_identity(address, network, dbname)
	}

	@Java_io_parity_signer_models_SignerDataModel_qrparserGetPacketsTotal
	fn get_packets_total(
		data: &str,
        cleaned: bool
	) -> anyhow::Result<u32, anyhow::Error> {
        qr_reader_phone::get_length(data, cleaned)
	}

	@Java_io_parity_signer_models_SignerDataModel_qrparserTryDecodeQrSequence
	fn try_decode_qr_sequence(
		data: &str,
        cleaned: bool
	) -> anyhow::Result<String, anyhow::Error> {
        qr_reader_phone::decode_sequence(data, cleaned)
	}

    @Java_io_parity_signer_models_SignerDataModel_substrateParseTransaction
	fn parse_transaction(
		transaction: &str,
        dbname: &str
	) -> String {
        if transaction == "test all" {return transaction_parsing::test_all_cards::make_all_cards()}
        else {return transaction_parsing::produce_output(transaction, dbname)}
    }

    @Java_io_parity_signer_models_SignerDataModel_substrateHandleStub
	fn handle_stub(
		checksum: &str,
        dbname: &str
	) -> anyhow::Result<(), anyhow::Error> {
        transaction_signing::handle_stub(checksum, dbname)
    }

    @Java_io_parity_signer_models_SignerDataModel_substrateHandleSign
	fn handle_sign(
		checksum: &str,
        seed_phrase: &str,
        password: &str,
        user_comment: &str,
        dbname: &str
	) -> anyhow::Result<String, anyhow::Error> {
        transaction_signing::handle_sign(checksum, seed_phrase, password, user_comment, dbname)
    }

    @Java_io_parity_signer_models_SignerDataModel_substrateDevelopmentTest
	fn development_test(
		input: &str
	) -> anyhow::Result<String, anyhow::Error> {
        //let output = Ok(std::env::consts::OS.to_string());
        let picture = plot_icon::png_data_from_base58(input, 32)?;
        Ok(hex::encode(picture))
    }

    @Java_io_parity_signer_models_SignerDataModel_substrateBase58Identicon
	fn base58_identicon(
		base58: &str,
        size: u32
	) -> anyhow::Result<String, anyhow::Error> {
        //let output = Ok(std::env::consts::OS.to_string());
        let picture = plot_icon::png_data_from_base58(base58, size.try_into()?)?;
        Ok(hex::encode(picture))
    }

    @Java_io_parity_signer_models_SignerDataModel_substrateIdenticon
	fn identicon(
		key: &str,
        size: u32
	) -> anyhow::Result<String, anyhow::Error> {
        //let output = Ok(std::env::consts::OS.to_string());
        let picture = plot_icon::png_data_from_hex(key, size.try_into()?)?;
        Ok(hex::encode(picture))
    }

    @Java_io_parity_signer_models_SignerDataModel_dbGetNetwork
	fn get_network(
		genesis_hash: &str,
        dbname: &str
	) -> anyhow::Result<String, anyhow::Error> {
        db_handling::chainspecs::print_network(dbname, genesis_hash)
    }

    @Java_io_parity_signer_models_SignerDataModel_dbGetAllNetworksForNetworkSelector
	fn get_all_networks_for_network_selector(
        dbname: &str
    ) -> anyhow::Result<String, anyhow::Error> {
        db_handling::chainspecs::print_all_networks(dbname)
    }

    @Java_io_parity_signer_models_SignerDataModel_dbGetRelevantIdentities
	fn get_relevant_identities(
		seed_name: &str,
        genesis_hash: &str,
        dbname: &str
	) -> anyhow::Result<String, anyhow::Error> {
        db_handling::identities::print_relevant_identities(seed_name, genesis_hash, dbname)
    }
    
    @Java_io_parity_signer_models_SignerDataModel_dbGetAllIdentities
	fn get_all_identities(
        dbname: &str
	) -> anyhow::Result<String, anyhow::Error> {
        db_handling::identities::print_all_identities(dbname)
    }

    @Java_io_parity_signer_models_SignerDataModel_substrateTryCreateSeed
	fn try_create_seed(
        seed_name: &str,
        seed_phrase: &str,
        seed_length: u32,
		dbname: &str
	) -> anyhow::Result<String, anyhow::Error> {
        db_handling::identities::try_create_seed(seed_name, seed_phrase, seed_length, dbname)
    }

    @Java_io_parity_signer_models_SignerDataModel_substrateSuggestNPlusOne
	fn suggest_n_plus_one(
        path: &str,
        seed_name: &str,
        network_id_string: &str,
        dbname: &str
	) -> anyhow::Result<String, anyhow::Error> {
        db_handling::identities::suggest_n_plus_one(path, seed_name, network_id_string, dbname)
    }

    @Java_io_parity_signer_models_SignerDataModel_substrateCheckPath
	fn check_path(
        path: &str
	) -> anyhow::Result<bool, anyhow::Error> {
        db_handling::identities::check_derivation_format(path)
    }

    @Java_io_parity_signer_models_SignerDataModel_substrateTryCreateIdentity
	fn try_create_identity(
        id_name: &str,
        seed_name: &str,
        seed_phrase: &str,
        crypto: &str,
        path: &str,
        network: &str,
        has_password: bool,
		dbname: &str
	) -> anyhow::Result<(), anyhow::Error> {
        db_handling::identities::try_create_address(id_name, seed_name, seed_phrase, crypto, path, network, has_password, dbname)
    }

    @Java_io_parity_signer_models_SignerDataModel_substrateSuggestName
	fn suggest_name(
        path: &str
	) -> String {
        db_handling::identities::suggest_path_name(path)
    }

    @Java_io_parity_signer_models_SignerDataModel_substrateDeleteIdentity
	fn delete_identity(
        pub_key: &str,
        network: &str,
        dbname: &str
	) -> anyhow::Result<(), anyhow::Error> {
        db_handling::identities::delete_address(pub_key, network, dbname)
    }

    @Java_io_parity_signer_models_SignerDataModel_substrateGetNetworkSpecs
	fn get_network_specs(
        network: &str,
        dbname: &str
	) -> anyhow::Result<String, anyhow::Error> {
        db_handling::network_details::get_network_details_by_hex(network, dbname)
    }
    
    @Java_io_parity_signer_models_SignerDataModel_substrateRemoveNetwork
	fn remove_network(
        network: &str,
        dbname: &str
	) -> anyhow::Result<(), anyhow::Error> {
        db_handling::remove_network::remove_network_by_hex(network, dbname)
    }

    @Java_io_parity_signer_models_SignerDataModel_substrateRemoveMetadata
	fn remove_metadata(
        network_name: &str,
        network_version: u32,
        dbname: &str
	) -> anyhow::Result<(), anyhow::Error> {
        db_handling::remove_network::remove_metadata(network_name, network_version, dbname)
    }

    @Java_io_parity_signer_models_SignerDataModel_substrateRemoveSeed
	fn remove_seed(
        seed_name: &str,
        dbname: &str
	) -> anyhow::Result<(), anyhow::Error> {
        db_handling::identities::remove_identities_for_seed(seed_name, dbname)
    }

    @Java_io_parity_signer_models_SignerDataModel_historyPrintHistory
	fn print_history(
        dbname: &str
	) -> anyhow::Result<String, anyhow::Error> {
        db_handling::manage_history::print_history(dbname)
    }

    @Java_io_parity_signer_models_SignerDataModel_historyClearHistory
	fn clear_history(
        dbname: &str
	) -> anyhow::Result<(), anyhow::Error> {
        db_handling::manage_history::clear_history(dbname)
    }

    @Java_io_parity_signer_models_SignerDataModel_historyInitHistoryWithCert
	fn init_history_with_cert(
        dbname: &str
	) -> anyhow::Result<(), anyhow::Error> {
        db_handling::cold_default::signer_init_with_cert(dbname)
    }

    @Java_io_parity_signer_models_SignerDataModel_historyInitHistoryNoCert
	fn init_history_no_cert(
        dbname: &str
	) -> anyhow::Result<(), anyhow::Error> {
        db_handling::cold_default::signer_init_no_cert(dbname)
    }


    @Java_io_parity_signer_models_SignerDataModel_historyDeviceWasOnline
	fn device_was_online(
        dbname: &str
	) -> anyhow::Result<(), anyhow::Error> {
        db_handling::manage_history::device_was_online(dbname)
    }

    @Java_io_parity_signer_models_SignerDataModel_historyGetWarnings
	fn get_warnings(
        dbname: &str
	) -> anyhow::Result<bool, anyhow::Error> {
        db_handling::helpers::get_danger_status(dbname)
    }

    @Java_io_parity_signer_models_SignerDataModel_historyAcknowledgeWarnings
	fn acknowledge_warnings(
        dbname: &str
	) -> anyhow::Result<(), anyhow::Error> {
        db_handling::manage_history::reset_danger_status_to_safe(dbname)
    }

    @Java_io_parity_signer_models_SignerDataModel_historyHistoryEntryUser
	fn history_entry_user(
        entry: &str,
        dbname: &str
	) -> anyhow::Result<(), anyhow::Error> {
        db_handling::manage_history::history_entry_user(dbname, entry.to_string())
    }

    @Java_io_parity_signer_models_SignerDataModel_historyHistoryEntrySystem
	fn history_entry_system(
        entry: &str,
        dbname: &str
	) -> anyhow::Result<(), anyhow::Error> {
        db_handling::manage_history::history_entry_system(dbname, entry.to_string())
    }

    @Java_io_parity_signer_models_SignerDataModel_historySeedNameWasShown
	fn seed_name_was_shown(
        seed_name: &str,
        dbname: &str
	) -> anyhow::Result<(), anyhow::Error> {
        db_handling::manage_history::seed_name_was_shown(dbname, seed_name.to_string())
    }

    @Java_io_parity_signer_models_SignerDataModel_signerSignTypes
	fn sign_load_types(
        public_key: &str,
        encryption: &str,
        seed_phrase: &str,
        password: &str,
        dbname: &str
	) -> anyhow::Result<String, anyhow::Error> {
        transaction_signing::sign_message::sufficient_crypto_load_types(
            public_key, encryption, dbname, seed_phrase, password
        )
    }

    @Java_io_parity_signer_models_SignerDataModel_signerSignMetadata
	fn sign_load_metadata(
        network: &str,
        version: u32,
        public_key: &str,
        encryption: &str,
        seed_phrase: &str,
        password: &str,
        dbname: &str
	) -> anyhow::Result<String, anyhow::Error> {
        transaction_signing::sign_message::sufficient_crypto_load_metadata(
            network, version, public_key, encryption, dbname, seed_phrase, password
        )
    }

    @Java_io_parity_signer_models_SignerDataModel_signerSignSpecs
	fn sign_load_specs(
        network: &str,
        public_key: &str,
        encryption: &str,
        seed_phrase: &str,
        password: &str,
        dbname: &str
	) -> anyhow::Result<String, anyhow::Error> {
        transaction_signing::sign_message::sufficient_crypto_add_specs(
            network, public_key, encryption, dbname, seed_phrase, password
        )
    }

}

ffi_support::define_string_destructor!(signer_destroy_string);

#[cfg(test)]
mod tests {
	//use super::*;
}
