// Copyright 2015-2020 Parity Technologies (UK) Ltd.
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
	hexStripPrefix,
	hexToU8a,
	isU8a,
	u8aToHex,
	u8aConcat
} from '@polkadot/util';
import { Container } from 'unstated';
import { ExtrinsicPayload } from '@polkadot/types/interfaces';

import AccountsStore from 'stores/AccountsStore';
import { NETWORK_LIST, NetworkProtocols } from 'constants/networkSpecs';
import { TryBrainWalletSignFunc, TrySignFunc } from 'utils/seedRefHooks';
import { isAscii } from 'utils/strings';
import {
	brainWalletSign,
	decryptData,
	keccak,
	ethSign,
	substrateSign
} from 'utils/native';
import transaction, { Transaction } from 'utils/transaction';
import {
	constructDataFromBytes,
	asciiToHex,
	encodeNumber
} from 'utils/decoders';
import { Account, FoundAccount } from 'types/identityTypes';
import { emptyAccount } from 'utils/account';
import {
	CompletedParsedData,
	EthereumParsedData,
	isEthereumCompletedParsedData,
	isMultipartData,
	SubstrateCompletedParsedData
} from 'types/scannerTypes';

type TXRequest = Record<string, any>;

type SignedTX = {
	recipient: Account;
	sender: Account;
	txRequest: TXRequest;
};

type ScannerState = {
	busy: boolean;
	completedFramesCount: number;
	dataToSign: string | GenericExtrinsicPayload;
	isHash: boolean;
	isOversized: boolean;
	latestFrame: number | null;
	message: string | null;
	missedFrames: Array<number>;
	multipartData: null | Array<Uint8Array | null>;
	multipartComplete: boolean;
	prehash: GenericExtrinsicPayload | null;
	recipient: FoundAccount | null;
	sender: FoundAccount | null;
	signedData: string;
	signedTxList: SignedTX[];
	totalFrameCount: number;
	tx: Transaction | GenericExtrinsicPayload | string | Uint8Array | null;
	type: 'transaction' | 'message' | null;
	unsignedData: CompletedParsedData | null;
};

const DEFAULT_STATE = Object.freeze({
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
	prehash: null,
	recipient: null,
	sender: null,
	signedData: '',
	signedTxList: [],
	totalFrameCount: 0,
	tx: null,
	type: null,
	unsignedData: null
});

const MULTIPART = new Uint8Array([0]); // always mark as multipart for simplicity's sake. Consistent with @polkadot/react-qr

// const SIG_TYPE_NONE = new Uint8Array();
// const SIG_TYPE_ED25519 = new Uint8Array([0]);
const SIG_TYPE_SR25519 = new Uint8Array([1]);
// const SIG_TYPE_ECDSA = new Uint8Array([2]);

export default class ScannerStore extends Container<ScannerState> {
	state: ScannerState = DEFAULT_STATE;

	async setUnsigned(data: string): Promise<void> {
		this.setState({
			unsignedData: JSON.parse(data)
		});
	}

	/*
	 * @param strippedData: the rawBytes from react-native-camera, stripped of the ec11 padding to fill the frame size. See: decoders.js
	 * N.B. Substrate oversized/multipart payloads will already be hashed at this point.
	 */

	async setParsedData(
		strippedData: Uint8Array,
		multipartComplete = false
	): Promise<void> {
		const parsedData = await constructDataFromBytes(
			strippedData,
			multipartComplete
		);
		if (isMultipartData(parsedData)) {
			await this.setPartData(
				parsedData.currentFrame,
				parsedData.frameCount,
				parsedData.partData
			);
			return;
		}

		await this.setState({
			unsignedData: parsedData
		});

		// set payload before it got hashed.
		// signature will be generated from the hash, but we still want to display it.
		if (parsedData.hasOwnProperty('preHash')) {
			this.setPrehashPayload(
				(parsedData as SubstrateCompletedParsedData).preHash
			);
		}
	}

	async integrateMultiPartData(): Promise<void> {
		const { multipartData, totalFrameCount } = this.state;

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

		await this.setState({
			multipartComplete: true
		});
		// handle the binary blob as a single UOS payload
		await this.setParsedData(concatMultipartData, true);
	}

	async setPartData(
		currentFrame: number,
		frameCount: number,
		partData: string
	): Promise<boolean | void | Uint8Array> {
		// set it once only
		if (!this.state.totalFrameCount) {
			const newArray = new Array(frameCount).fill(null);
			await this.setState({
				multipartData: newArray,
				totalFrameCount: frameCount
			});
		}
		const {
			completedFramesCount,
			multipartComplete,
			multipartData,
			totalFrameCount
		} = this.state;

		const partDataAsBytes = new Uint8Array(partData.length / 2);

		for (let i = 0; i < partDataAsBytes.length; i++) {
			partDataAsBytes[i] = parseInt(partData.substr(i * 2, 2), 16);
		}

		if (
			partDataAsBytes[0] === new Uint8Array([0x00])[0] ||
			partDataAsBytes[0] === new Uint8Array([0x7b])[0]
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
			await this.setState({
				completedFramesCount: nextCompletedFramesCount,
				latestFrame: currentFrame,
				missedFrames: nextMissedFrames,
				multipartData: nextDataState
			});

			if (
				totalFrameCount > 0 &&
				nextCompletedFramesCount === totalFrameCount &&
				!multipartComplete
			) {
				// all the frames are filled
				await this.integrateMultiPartData();
			}
		}
	}

	async setData(accountsStore: AccountsStore): Promise<boolean | void> {
		const { unsignedData } = this.state;
		if (!isMultipartData(unsignedData) && unsignedData !== null) {
			switch (unsignedData.action) {
				case 'signTransaction':
					return await this.setTXRequest(unsignedData, accountsStore);
				case 'signData':
					return await this.setDataToSign(unsignedData, accountsStore);
				default:
					return;
			}
		} else {
			throw new Error(
				'Scanned QR should contain either transaction or a message to sign'
			);
		}
	}

	async setDataToSign(
		signRequest: CompletedParsedData,
		accountsStore: AccountsStore
	): Promise<boolean> {
		this.setBusy();

		const address = signRequest.data.account;
		const message = signRequest.data.data;
		const crypto = (signRequest as SubstrateCompletedParsedData).data?.crypto;
		const isHash =
			(signRequest as SubstrateCompletedParsedData)?.isHash || false;
		const isOversized =
			(signRequest as SubstrateCompletedParsedData)?.oversized || false;

		let dataToSign = '';
		const messageString = message?.toString();
		if (messageString === undefined)
			throw new Error('No message data to sign.');

		if (crypto === 'sr25519' || crypto === 'ed25519') {
			// only Substrate payload has crypto field
			dataToSign = message!.toString();
		} else {
			dataToSign = await ethSign(message!.toString());
		}

		const sender = accountsStore.getAccountByAddress(address);
		if (!sender) {
			throw new Error(
				`No private key found for ${address} in your signer key storage.`
			);
		}

		this.setState({
			dataToSign,
			isHash: isHash,
			isOversized: isOversized,
			message: message!.toString(),
			sender: sender!,
			type: 'message'
		});

		return true;
	}

	async setTXRequest(
		txRequest: CompletedParsedData,
		accountsStore: AccountsStore
	): Promise<boolean> {
		this.setBusy();

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
			throw new Error('Scanned QR contains no valid transaction');
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
			tx = txRequest.data.data;
			networkKey = (txRequest.data
				.data as ExtrinsicPayload)?.genesisHash.toHex();
			recipientAddress = txRequest.data.account;
			dataToSign = txRequest.data.data;
		}

		const sender = await accountsStore.getById({
			address: txRequest.data.account,
			networkKey
		});

		const networkTitle = NETWORK_LIST[networkKey].title;

		if (!sender) {
			throw new Error(
				`No private key found for account ${txRequest.data.account} found in your signer key storage for the ${networkTitle} chain.`
			);
		}

		const recipient =
			(await accountsStore.getById({
				address: recipientAddress,
				networkKey
			})) || emptyAccount(recipientAddress, networkKey);

		this.setState({
			dataToSign: dataToSign as string,
			isOversized,
			recipient: recipient as FoundAccount,
			sender,
			tx,
			type: 'transaction'
		});

		return true;
	}

	// signing ethereum data with seed reference
	async signEthereumData(signFunction: TryBrainWalletSignFunc): Promise<void> {
		const { dataToSign, sender } = this.state;
		if (!sender || !NETWORK_LIST.hasOwnProperty(sender.networkKey))
			throw new Error('Signing Error: sender could not be found.');
		const signedData = await signFunction(dataToSign as string);
		this.setState({ signedData });
	}

	// signing substrate data with seed reference
	async signSubstrateData(
		signFunction: TrySignFunc,
		suriSuffix: string
	): Promise<void> {
		const { dataToSign, isHash, sender } = this.state;
		if (!sender || !NETWORK_LIST.hasOwnProperty(sender.networkKey))
			throw new Error('Signing Error: sender could not be found.');
		let signable;

		if (dataToSign instanceof GenericExtrinsicPayload) {
			signable = u8aToHex(dataToSign.toU8a(true), -1, false);
		} else if (isHash) {
			signable = hexStripPrefix(dataToSign);
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
		this.setState({ signedData });
	}

	// signing data with legacy account.
	async signDataLegacy(pin = '1'): Promise<void> {
		const { sender, dataToSign, isHash } = this.state;
		if (!sender || !sender.encryptedSeed)
			throw new Error('Signing Error: sender could not be found.');
		const isEthereum =
			NETWORK_LIST[sender.networkKey].protocol === NetworkProtocols.ETHEREUM;
		const seed = await decryptData(sender.encryptedSeed, pin);
		let signedData;
		if (isEthereum) {
			signedData = await brainWalletSign(seed, dataToSign as string);
		} else {
			let signable;

			if (dataToSign instanceof GenericExtrinsicPayload) {
				signable = u8aToHex(dataToSign.toU8a(true), -1, false);
			} else if (isHash) {
				signable = hexStripPrefix(dataToSign);
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
		this.setState({ signedData });
	}

	async cleanup(): Promise<void> {
		await this.setState({
			...DEFAULT_STATE
		});
		this.clearMultipartProgress();
	}

	clearMultipartProgress(): void {
		this.setState({
			completedFramesCount: DEFAULT_STATE.completedFramesCount,
			latestFrame: DEFAULT_STATE.latestFrame,
			missedFrames: DEFAULT_STATE.missedFrames,
			multipartComplete: DEFAULT_STATE.multipartComplete,
			multipartData: null,
			totalFrameCount: DEFAULT_STATE.totalFrameCount,
			unsignedData: DEFAULT_STATE.unsignedData
		});
	}

	/**
	 * @dev signing payload type can be either transaction or message
	 */
	getType(): 'transaction' | 'message' | null {
		return this.state.type;
	}

	/**
	 * @dev sets a lock on writes
	 */
	setBusy(): void {
		this.setState({
			busy: true
		});
	}

	/**
	 * @dev allow write operations
	 */
	setReady(): void {
		this.setState({
			busy: false
		});
	}

	isBusy(): boolean {
		return this.state.busy;
	}

	isMultipartComplete(): boolean {
		return this.state.multipartComplete;
	}

	/**
	 * @dev is the payload a hash
	 */
	getIsHash(): boolean {
		return this.state.isHash;
	}

	/**
	 * @dev is the payload size greater than 256 (in Substrate chains)
	 */
	getIsOversized(): boolean {
		return this.state.isOversized;
	}

	/**
	 * @dev returns the number of completed frames so far
	 */
	getCompletedFramesCount(): number {
		return this.state.completedFramesCount;
	}

	/**
	 * @dev returns the number of frames to fill in total
	 */
	getTotalFramesCount(): number {
		return this.state.totalFrameCount;
	}

	getSender(): FoundAccount | null {
		return this.state.sender;
	}

	getRecipient(): FoundAccount | null {
		return this.state.recipient;
	}

	getMessage(): string | null {
		return this.state.message;
	}

	/**
	 * @dev unsigned data, not yet formatted as signable payload
	 */
	getUnsigned(): CompletedParsedData | null {
		return this.state.unsignedData;
	}

	getTx(): GenericExtrinsicPayload | Transaction | string | Uint8Array | null {
		return this.state.tx;
	}

	/**
	 * @dev unsigned date, formatted as signable payload
	 */
	getDataToSign(): string | GenericExtrinsicPayload {
		return this.state.dataToSign;
	}

	getSignedTxData(): string {
		return this.state.signedData;
	}

	getMissedFrames(): number[] {
		return this.state.missedFrames;
	}

	getPrehashPayload(): GenericExtrinsicPayload | null {
		return this.state.prehash;
	}

	setPrehashPayload(prehash: GenericExtrinsicPayload): void {
		this.setState({
			prehash
		});
	}
}
