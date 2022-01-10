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

use db_handling;
use qr_reader_phone;
use navigator;
use definitions;

mod export;

export! {
    @Java_io_parity_signer_models_SignerDataModel_backendAction
    fn act(
        action: &str,
        details: &str,
        seed_phrase: &str
    ) -> String {
        navigator::do_action(action, details, seed_phrase)
    }

    @Java_io_parity_signer_models_SignerDataModel_initNavigation
    fn init_navigation(
        dbname: &str,
        seed_names: &str
    ) -> () {
        navigator::init_navigation(dbname, seed_names)
    }

    @Java_io_parity_signer_models_SignerDataModel_updateSeedNames
    fn update_seed_names(
        seed_names: &str
    ) -> () {
        navigator::update_seed_names(seed_names)
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

     @Java_io_parity_signer_models_SignerDataModel_substrateGuessWord
    fn guess_word (
        part: &str
    ) -> String {
        db_handling::identities::guess(part)
    }

    @Java_io_parity_signer_models_SignerDataModel_substrateCheckPath
	fn check_path(
        path: &str
	) -> anyhow::Result<bool, anyhow::Error> {
        db_handling::identities::check_derivation_format(path).map_err(|e| e.anyhow())
    }

    @Java_io_parity_signer_models_SignerDataModel_substrateValidateSeedphrase
    fn validate_phrase(
        seed_phrase: &str
    ) -> anyhow::Result<(), anyhow::Error> {
        db_handling::identities::validate_phrase(seed_phrase).map_err(|e| e.anyhow())
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
        db_handling::helpers::display_danger_status(dbname)
    }

    @Java_io_parity_signer_models_SignerDataModel_historyAcknowledgeWarnings
	fn acknowledge_warnings(
        dbname: &str
	) -> anyhow::Result<(), anyhow::Error> {
        db_handling::manage_history::reset_danger_status_to_safe(dbname)
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

    @Java_io_parity_signer_models_SignerDataModel_testGetAllTXCards
	fn get_all_tx_cards() -> String {
        if let transaction_parsing::Action::Read(content) = transaction_parsing::test_all_cards::make_all_cards() {
            format!("{}", content)
        } else {
            "".to_string()
        }
    }

    @Java_io_parity_signer_models_SignerDataModel_testGetAllLogCards
	fn get_all_log_cards() -> String {
        definitions::history::print_all_events()
    }

}

ffi_support::define_string_destructor!(signer_destroy_string);

#[cfg(test)]
mod tests {
	//use super::*;
}
