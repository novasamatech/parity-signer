// Copyright 2015-2019 Parity Technologies (UK) Ltd.
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

const testIDs = {
	AccountListScreen: {
		accountList: 'accountList'
	},
	AccountNetworkChooser: {
		addCustomNetworkButton: 'anc_add_custom_button',
		addNewNetworkButton: 'anc_add_new_button',
		chooserScreen: 'anc_chooser_screen',
		createButton: 'anc_create_button',
		networkButton: 'anc_network_button',
		noAccountScreen: 'anc_no_account_screen',
		recoverButton: 'anc_recover_button',
		showExistedButton: 'anc_show_existed'
	},
	Header: {
		headerBackButton: 'header_back_button'
	},
	IdentitiesSwitch: {
		addIdentityButton: 'identities_switch_add_identity',
		manageIdentityButton: 'identities_switch_manager_button',
		modal: 'identity_switch_modal',
		toggleButton: 'identities_switch_toggle_button'
	},
	IdentityBackup: {
		nextButton: 'identity_backup_next',
		seedText: 'identity_backup_seed'
	},
	IdentityManagement: {
		deleteButton: 'identity_management_delete_button',
		popupMenuButton: 'identity_management_popup_menu'
	},
	IdentityNew: {
		createButton: 'identity_new_create_button',
		nameInput: 'identity_new_name_input',
		recoverButton: 'identity_new_recover_button',
		seedInput: 'identity_new_seed_input'
	},
	IdentityPin: {
		confirmPin: 'identity_pin_confirm',
		scrollScreen: 'identity_pin_scroll',
		setPin: 'identity_pin_set',
		submitButton: 'identity_submit_button',
		unlockPinButton: 'identity_unlock_pin_button',
		unlockPinInput: 'identity_unlock_pin_input'
	},
	MessageDetails: {
		scrollScreen: 'message_details_scroll',
		signButton: 'message_details_sign_button'
	},
	PathDerivation: {
		deriveButton: 'path_derivation_derive_button',
		nameInput: 'path_derivation_name_input',
		pathInput: 'path_derivation_path_input'
	},
	PathDetail: {
		deleteButton: 'path_detail_delete_button',
		popupMenuButton: 'path_detail_popup_menu_button',
		screen: 'path_detail_screen'
	},
	PathsList: {
		deriveButton: 'path_list_derive_button',
		pathCard: 'path_list_path_card',
		scanButton: 'path_list_scan_button',
		screen: 'path_list_screen'
	},
	SecurityHeader: {
		scanButton: 'security_header_scan_button'
	},
	SignedMessage: {
		qrView: 'signed_message_qr_view'
	},
	SignedTx: {
		qrView: 'signed_tx_qr_view'
	},
	TacScreen: {
		agreePrivacyButton: 'tac_privacy',
		agreeTacButton: 'tac_agree',
		nextButton: 'tac_next',
		tacView: 'tac_view'
	},
	TxDetails: {
		scrollScreen: 'tx_details_scroll',
		signButton: 'tx_details_sign_button'
	}
};

export default testIDs;
