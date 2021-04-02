// Copyright 2015-2020 Parity Technologies (UK) Ltd.
// Copyright 2021 Commonwealth Labs, Inc.
// This file is part of Layer Wallet.

// Layer Wallet is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Layer Wallet is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Layer Wallet. If not, see <http://www.gnu.org/licenses/>.

import {
	EthereumNetworkKeys,
	SUBSTRATE_NETWORK_LIST,
	SubstrateNetworkKeys
} from 'constants/networkSpecs';
import {
	launchWithScanRequest,
	pinCode,
	tapBack,
	testExist,
	testRecoverIdentity,
	testScrollAndTap,
	testTap,
	testUnlockPin,
	testVisible
} from 'e2e/utils';
import { ScanTestRequest } from 'e2e/mockScanRequests';
import testIDs from 'e2e/testIDs';

const {
	DetailsTx,
	Main,
	PathDetail,
	PathsList,
	SecurityHeader,
	SignedTx
} = testIDs;

const testSignedTx = async (): Promise<void> => {
	await testTap(SecurityHeader.scanButton);
	await testTap(DetailsTx.signButton);
	await testUnlockPin(pinCode);
	await testExist(SignedTx.qrView);
};

describe('Signing ane exporting test', () => {
	testRecoverIdentity();

	describe('Kusama Signing Test', () => {
		it('is able to export the signing account', async () => {
			await testTap(PathsList.pathCard + '//kusama');
			await testTap(PathDetail.popupMenuButton);
			await testTap(PathDetail.exportButton);
			await testExist(
				'secret:0xdf46d55a2d98695e9342b67edae6669e5c0b4e1a3895f1adf85989565b9ab827:0xb0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe:Kusama root'
			);
			await tapBack();
			await testVisible(PathDetail.screen);
		});

		it('should sign the set remarks request', async () => {
			await launchWithScanRequest(ScanTestRequest.SetRemarkExtrinsicKusama);
			await testSignedTx();
		});

		it('does not need sign again after pin input', async () => {
			await tapBack();
			await tapBack();
			await testTap(SecurityHeader.scanButton);
			await testTap(DetailsTx.signButton);
			await testExist(SignedTx.qrView);
		});

		it('should sign transfer request', async () => {
			await launchWithScanRequest(ScanTestRequest.TransferExtrinsicKusama);
			await testSignedTx();
		});
	});

	describe('Polkadot Signing Test', () => {
		it('generate Polkadot account', async () => {
			await tapBack();
			await tapBack();
			const PolkadotNetworkButtonIndex =
				Main.networkButton +
				SUBSTRATE_NETWORK_LIST[SubstrateNetworkKeys.POLKADOT].pathId;
			await testTap(testIDs.Main.addNewNetworkButton);
			await testScrollAndTap(
				PolkadotNetworkButtonIndex,
				testIDs.Main.chooserScreen
			);
			await testVisible(PathDetail.screen);
		});

		it('should sign the set remarks request', async () => {
			await launchWithScanRequest(ScanTestRequest.SetRemarkExtrinsicPolkadot);
			await testSignedTx();
		});

		it('should sign transfer request', async () => {
			await launchWithScanRequest(ScanTestRequest.TransferExtrinsicPolkadot);
			await testSignedTx();
		});
	});

	describe('Ethereum Signing Test', () => {
		it('generate Kovan account', async () => {
			await tapBack();
			await tapBack();
			const kovanNetworkButtonIndex =
				Main.networkButton + EthereumNetworkKeys.KOVAN;
			await testTap(testIDs.Main.addNewNetworkButton);
			await testScrollAndTap(
				kovanNetworkButtonIndex,
				testIDs.Main.chooserScreen
			);
			await testVisible(PathDetail.screen);
			await tapBack();
			await testExist(Main.chooserScreen);
		});

		it('should sign transactions', async () => {
			await launchWithScanRequest(ScanTestRequest.EthereumTransaction);
			await testSignedTx();
		});
	});
});
