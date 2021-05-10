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

import { GenericExtrinsicPayload } from '@polkadot/types';
import {
	compactFromU8a,
	hexStripPrefix,
	hexToU8a,
	isU8a,
	u8aConcat,
	u8aToHex
} from '@polkadot/util';
import React, { useReducer } from 'react';

import { AccountsContextState } from 'stores/AccountsContext';
import { ETHEREUM_NETWORK_LIST } from 'constants/networkSpecs';
import { GetNetwork, NetworksContextState } from 'stores/NetworkContext';
import { Account, FoundAccount } from 'types/identityTypes';
import {
	isEthereumNetworkParams,
	SubstrateNetworkParams
} from 'types/networkTypes';
import {
	CompletedParsedData,
	EthereumParsedData,
	isEthereumCompletedParsedData,
	isSubstrateMessageParsedData,
	MessageQRInfo,
	MultiFramesInfo,
	QrInfo,
	SubstrateCompletedParsedData,
	SubstrateMessageParsedData,
	SubstrateTransactionParsedData,
	TxQRInfo
} from 'types/scannerTypes';
import { emptyAccount } from 'utils/account';
import {
	asciiToHex,
	constructDataFromBytes,
	encodeNumber
} from 'utils/decoders';
import {
	brainWalletSign,
	decryptData,
	ethSign,
	keccak,
	substrateSign
} from 'utils/native';
import { TryBrainWalletSignFunc, TrySignFunc } from 'utils/seedRefHooks';
import { isAscii } from 'utils/strings';
import transaction, { Transaction } from 'utils/transaction';

type TXRequest = Record<string, any>;

type SignedTX = {
	recipient: Account;
	sender: Account;
	txRequest: TXRequest;
};

type ScannerStoreState = {
	busy: boolean;
	completedFramesCount: number;
	dataToSign: string | Uint8Array;
	isHash: boolean;
	isOversized: boolean;
	latestFrame: number | null;
	message: string | null;
	missedFrames: Array<number>;
	multipartData: null | Array<Uint8Array | null>;
	multipartComplete: boolean;
	rawPayload: Uint8Array | string | null;
	recipient: FoundAccount | null;
	sender: FoundAccount | null;
	signedData: string;
	signedTxList: SignedTX[];
	totalFrameCount: number;
	tx: Transaction | GenericExtrinsicPayload | string | Uint8Array | null;
	type: 'transaction' | 'message' | null;
};

export type ScannerContextState = {
	cleanup: () => void;
	clearMultipartProgress: () => void;
	setBusy: () => void;
	setReady: () => void;
	state: ScannerStoreState;
	setPartData: (
		currentFrame: number,
		frameCount: number,
		partData: string,
		networkContext: NetworksContextState
	) => Promise<MultiFramesInfo | SubstrateCompletedParsedData>;
	setData: (
		accountsStore: AccountsContextState,
		unsignedData: CompletedParsedData,
		networkContext: NetworksContextState
	) => Promise<QrInfo>;
	signEthereumData: (
		signFunction: TryBrainWalletSignFunc,
		qrInfo: QrInfo
	) => Promise<void>;
	signSubstrateData: (
		signFunction: TrySignFunc,
		suriSuffix: string,
		qrInfo: QrInfo,
		networks: Map<string, SubstrateNetworkParams>
	) => Promise<void>;
	signDataLegacy: (pin: string, getNetwork: GetNetwork) => Promise<void>;
};

const DEFAULT_STATE: ScannerStoreState = {
	busy: false,
	completedFramesCount: 0,
	dataToSign: '',
	isHash: false,
	isOversized: false,
	latestFrame: null,
	message: null,
	missedFrames: [],
	multipartComplete: false,
	multipartData: null,
	rawPayload: null,
	recipient: null,
	sender: null,
	signedData: '',
	signedTxList: [],
	totalFrameCount: 0,
	tx: null,
	type: null
};

const MULTIPART = new Uint8Array([0]); // always mark as multipart for simplicity's sake. Consistent with @polkadot/react-qr

// const SIG_TYPE_NONE = new Uint8Array();
// const SIG_TYPE_ED25519 = new Uint8Array([0]);
const SIG_TYPE_SR25519 = new Uint8Array([1]);
// const SIG_TYPE_ECDSA = new Uint8Array([2]);

export function useScannerContext(): ScannerContextState {
	const initialState = DEFAULT_STATE;

	const reducer = (
		state: ScannerStoreState,
		delta: Partial<ScannerStoreState>
	): ScannerStoreState => ({
		...state,
		...delta
	});
	const [state, setState] = useReducer(reducer, initialState);

	/**
	 * @dev sets a lock on writes
	 */
	function setBusy(): void {
		setState({
			busy: true
		});
	}

	async function _integrateMultiPartData(
		multipartData: Array<Uint8Array | null>,
		totalFrameCount: number,
		networkContext: NetworksContextState
	): Promise<SubstrateCompletedParsedData> {
		// concatenate all the parts into one binary blob
		let concatMultipartData = multipartData!.reduce(
			(acc: Uint8Array, part: Uint8Array | null): Uint8Array => {
				if (part === null) throw new Error('part data is not completed');
				const c = new Uint8Array(acc.length + part.length);
				c.set(acc);
				c.set(part, acc.length);
				return c;
			},
			new Uint8Array(0)
		);

		// unshift the frame info
		const frameInfo = u8aConcat(
			MULTIPART,
			encodeNumber(totalFrameCount),
			encodeNumber(0)
		);
		concatMultipartData = u8aConcat(frameInfo, concatMultipartData);

		const parsedData = (await constructDataFromBytes(
			concatMultipartData,
			true,
			networkContext.networks
		)) as SubstrateCompletedParsedData;

		return parsedData;
	}

	async function setPartData(
		currentFrame: number,
		frameCount: number,
		partData: string,
		networkContext: NetworksContextState
	): Promise<MultiFramesInfo | SubstrateCompletedParsedData> {
		const newArray = new Array(frameCount).fill(null);
		const totalFrameCount = frameCount;
		// set it once only
		const multipartData = !state.totalFrameCount
			? newArray
			: state.multipartData;
		const { completedFramesCount, multipartComplete } = state;
		const partDataAsBytes = new Uint8Array(partData.length / 2);

		for (let i = 0; i < partDataAsBytes.length; i++) {
			partDataAsBytes[i] = parseInt(partData.substr(i * 2, 2), 16);
		}

		if (
			currentFrame === 0 &&
			(partDataAsBytes[0] === new Uint8Array([0x00])[0] ||
				partDataAsBytes[0] === new Uint8Array([0x7b])[0])
		) {
			// part_data for frame 0 MUST NOT begin with byte 00 or byte 7B.
			throw new Error('Error decoding invalid part data.');
		}
		if (completedFramesCount < totalFrameCount) {
			// we haven't filled all the frames yet
			const nextDataState = multipartData!;
			nextDataState[currentFrame] = partDataAsBytes;

			const nextMissedFrames = nextDataState.reduce(
				(acc: number[], current: Uint8Array | null, index: number) => {
					if (current === null) acc.push(index + 1);
					return acc;
				},
				[]
			);
			const nextCompletedFramesCount =
				totalFrameCount - nextMissedFrames.length;
			setState({
				completedFramesCount: nextCompletedFramesCount,
				latestFrame: currentFrame,
				missedFrames: nextMissedFrames,
				multipartData: nextDataState,
				totalFrameCount
			});
			if (
				totalFrameCount > 0 &&
				nextCompletedFramesCount === totalFrameCount &&
				!multipartComplete
			) {
				// all the frames are filled
				return await _integrateMultiPartData(
					nextDataState,
					totalFrameCount,
					networkContext
				);
			}
			return {
				completedFramesCount: nextCompletedFramesCount,
				missedFrames: nextMissedFrames,
				totalFrameCount
			};
		}
		return {
			completedFramesCount: totalFrameCount,
			missedFrames: [],
			totalFrameCount
		};
	}

	/**
	 * @dev allow write operations
	 */
	function setReady(): void {
		setState({
			busy: false
		});
	}

	async function _setTXRequest(
		txRequest: EthereumParsedData | SubstrateTransactionParsedData,
		accountsStore: AccountsContextState,
		networkContext: NetworksContextState
	): Promise<TxQRInfo> {
		setBusy();

		const { getNetwork } = networkContext;
		const isOversized =
			(txRequest as SubstrateCompletedParsedData)?.oversized || false;

		const isEthereum = isEthereumCompletedParsedData(txRequest);

		if (
			isEthereum &&
			!(
				txRequest.data &&
				(txRequest as EthereumParsedData).data!.rlp &&
				txRequest.data.account
			)
		) {
			throw new Error('Scanned QR contains no valid extrinsic');
		}
		let tx, networkKey, recipientAddress, dataToSign;
		if (isEthereumCompletedParsedData(txRequest)) {
			tx = await transaction(txRequest.data.rlp);
			networkKey = tx.ethereumChainId;
			recipientAddress = tx.action;
			// For Eth, always sign the keccak hash.
			// For Substrate, only sign the blake2 hash if payload bytes length > 256 bytes (handled in decoder.js).
			dataToSign = await keccak(txRequest.data.rlp);
		} else {
			const payloadU8a = txRequest.data.data;
			const [offset] = compactFromU8a(payloadU8a);
			tx = payloadU8a;
			networkKey = txRequest.data.genesisHash;
			recipientAddress = txRequest.data.account;
			dataToSign = payloadU8a.subarray(offset);
		}

		const sender = await accountsStore.getById(
			txRequest.data.account,
			networkKey,
			networkContext
		);

		const networkTitle = getNetwork(networkKey).title;

		if (!sender) {
			throw new Error(
				`No private key found for account ${txRequest.data.account} found in your signer key storage for the ${networkTitle} chain.`
			);
		}

		const recipient =
			(await accountsStore.getById(
				recipientAddress,
				networkKey,
				networkContext
			)) || emptyAccount(recipientAddress, networkKey);

		const qrInfo: TxQRInfo = {
			dataToSign: dataToSign,
			isHash: false,
			isOversized,
			recipient: recipient as FoundAccount,
			sender,
			tx,
			type: 'transaction'
		};

		setState({ ...qrInfo, rawPayload: txRequest.data.data });
		return qrInfo;
	}

	async function _setDataToSign(
		signRequest: SubstrateMessageParsedData | EthereumParsedData,
		accountsStore: AccountsContextState,
		networkContext: NetworksContextState
	): Promise<MessageQRInfo> {
		setBusy();

		const address = signRequest.data.account;
		let message = '';
		let isHash = false;
		let isOversized = false;
		let dataToSign = '';

		if (isSubstrateMessageParsedData(signRequest)) {
			if (signRequest.data.crypto !== 'sr25519')
				throw new Error('currently Parity Signer only support sr25519');
			isHash = signRequest.isHash;
			isOversized = signRequest.oversized;
			dataToSign = signRequest.data.data;
			message = dataToSign;
		} else {
			message = signRequest.data.data;
			dataToSign = await ethSign(message.toString());
		}

		const sender = accountsStore.getAccountByAddress(address, networkContext);
		if (!sender) {
			throw new Error(
				`No private key found for ${address} in your signer key storage.`
			);
		}

		const qrInfo: MessageQRInfo = {
			dataToSign,
			isHash: isHash,
			isOversized: isOversized,
			message: message!.toString(),
			sender: sender!,
			type: 'message'
		};

		setState(qrInfo);

		return qrInfo;
	}

	async function setData(
		accountsStore: AccountsContextState,
		unsignedData: CompletedParsedData,
		networkContext: NetworksContextState
	): Promise<QrInfo> {
		if (unsignedData !== null) {
			switch (unsignedData.action) {
				case 'signTransaction':
					return await _setTXRequest(
						unsignedData,
						accountsStore,
						networkContext
					);
				case 'signData':
					return await _setDataToSign(
						unsignedData,
						accountsStore,
						networkContext
					);
				default:
					throw new Error(
						'Scanned QR should contain either extrinsic or a message to sign'
					);
			}
		} else {
			throw new Error(
				'Scanned QR should contain either extrinsic or a message to sign'
			);
		}
	}

	// signing ethereum data with seed reference
	async function signEthereumData(
		signFunction: TryBrainWalletSignFunc,
		qrInfo: QrInfo
	): Promise<void> {
		const { dataToSign, sender } = qrInfo;
		if (!sender || !ETHEREUM_NETWORK_LIST.hasOwnProperty(sender.networkKey))
			throw new Error('Signing Error: sender could not be found.');
		const signedData = await signFunction(dataToSign as string);
		setState({ signedData });
	}

	// signing substrate data with seed reference
	async function signSubstrateData(
		signFunction: TrySignFunc,
		suriSuffix: string,
		qrInfo: QrInfo,
		networks: Map<string, SubstrateNetworkParams>
	): Promise<void> {
		const { dataToSign, isHash, sender } = qrInfo;
		if (!sender || !networks.has(sender.networkKey))
			throw new Error('Signing Error: sender could not be found.');
		let signable;

		if (dataToSign instanceof GenericExtrinsicPayload) {
			signable = u8aToHex(dataToSign.toU8a(true), -1, false);
		} else if (isHash) {
			console.log('sign substrate data type is', typeof dataToSign);
			signable = hexStripPrefix(dataToSign.toString());
		} else if (isU8a(dataToSign)) {
			signable = hexStripPrefix(u8aToHex(dataToSign));
		} else if (isAscii(dataToSign)) {
			signable = hexStripPrefix(asciiToHex(dataToSign));
		} else {
			throw new Error('Signing Error: cannot signing message');
		}
		let signed = await signFunction(suriSuffix, signable);
		signed = '0x' + signed;
		// TODO: tweak the first byte if and when sig type is not sr25519
		const sig = u8aConcat(SIG_TYPE_SR25519, hexToU8a(signed));
		const signedData = u8aToHex(sig, -1, false); // the false doesn't add 0x
		setState({ signedData });
	}

	// signing data with legacy account.
	async function signDataLegacy(
		pin = '1',
		getNetwork: GetNetwork
	): Promise<void> {
		const { sender, dataToSign, isHash } = state;
		if (!sender || !sender.encryptedSeed)
			throw new Error('Signing Error: sender could not be found.');
		const networkParams = getNetwork(sender.networkKey);
		const isEthereum = isEthereumNetworkParams(networkParams);
		const seed = await decryptData(sender.encryptedSeed, pin);
		let signedData;
		if (isEthereum) {
			signedData = await brainWalletSign(seed, dataToSign as string);
		} else {
			let signable;

			if (dataToSign instanceof GenericExtrinsicPayload) {
				signable = u8aToHex(dataToSign.toU8a(true), -1, false);
			} else if (isHash) {
				console.log('sign legacy data type is', typeof dataToSign);
				signable = hexStripPrefix(dataToSign.toString());
			} else if (isU8a(dataToSign)) {
				signable = hexStripPrefix(u8aToHex(dataToSign));
			} else if (isAscii(dataToSign)) {
				signable = hexStripPrefix(asciiToHex(dataToSign));
			} else {
				throw new Error('Signing Error: cannot signing message');
			}
			let signed = await substrateSign(seed, signable);
			signed = '0x' + signed;
			// TODO: tweak the first byte if and when sig type is not sr25519
			const sig = u8aConcat(SIG_TYPE_SR25519, hexToU8a(signed));
			signedData = u8aToHex(sig, -1, false); // the false doesn't add 0x
		}
		setState({ signedData });
	}

	function clearMultipartProgress(): void {
		setState({
			completedFramesCount: DEFAULT_STATE.completedFramesCount,
			latestFrame: DEFAULT_STATE.latestFrame,
			missedFrames: DEFAULT_STATE.missedFrames,
			multipartComplete: DEFAULT_STATE.multipartComplete,
			multipartData: null,
			totalFrameCount: DEFAULT_STATE.totalFrameCount
		});
	}

	function cleanup(): void {
		setState({
			...DEFAULT_STATE
		});
	}

	return {
		cleanup,
		clearMultipartProgress,
		setBusy,
		setData,
		setPartData,
		setReady,
		signDataLegacy,
		signEthereumData,
		signSubstrateData,
		state
	};
}

export const ScannerContext = React.createContext({} as ScannerContextState);
