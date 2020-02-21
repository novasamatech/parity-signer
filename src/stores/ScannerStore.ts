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
import { GenericExtrinsicPayload } from '@polkadot/types';
import {
	hexStripPrefix,
	hexToU8a,
	isU8a,
	u8aToHex,
	u8aConcat
} from '@polkadot/util';
import { decodeAddress, encodeAddress } from '@polkadot/util-crypto';
import { Container } from 'unstated';
import { ExtrinsicPayload } from '@polkadot/types/interfaces';

import AccountsStore from './AccountsStore';

import {
	NETWORK_LIST,
	NetworkProtocols,
	SUBSTRATE_NETWORK_LIST
} from 'constants/networkSpecs';
import { isAscii } from 'utils/strings';
import {
	brainWalletSign,
	decryptData,
	keccak,
	ethSign,
	substrateSign
} from 'utils/native';
import { mod } from 'utils/numbers';
import transaction, { Transaction } from 'utils/transaction';
import {
	constructDataFromBytes,
	asciiToHex,
	encodeNumber
} from 'utils/decoders';
import { Account, FoundAccount } from 'types/identityTypes';
import { constructSURI } from 'utils/suri';
import { emptyAccount } from 'utils/account';
import {
	CompletedParsedData,
	EthereumParsedData,
	isEthereumCompletedParsedData,
	isMultipartData,
	SubstrateCompletedParsedData
} from 'types/scannerTypes';
import { NetworkProtocol } from 'types/networkSpecsTypes';

type TXRequest = Record<string, any>;

type SignedTX = {
	recipient: Account;
	sender: Account;
	txRequest: TXRequest;
};

type MultipartData = {
	[x: string]: Uint8Array;
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
	multipartData: MultipartData;
	multipartComplete: boolean;
	prehash: GenericExtrinsicPayload | null;
	recipient: FoundAccount | null;
	scanErrorMsg: string;
	sender: FoundAccount | null;
	signedData: string;
	signedTxList: SignedTX[];
	totalFrameCount: number;
	tx: Transaction | GenericExtrinsicPayload | string | Uint8Array | null;
	txRequest: TXRequest | null;
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
	multipartData: {},
	prehash: null,
	recipient: null,
	scanErrorMsg: '',
	sender: null,
	signedData: '',
	signedTxList: [],
	totalFrameCount: 0,
	tx: null,
	txRequest: null,
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
		accountsStore: AccountsStore,
		multipartComplete = false
	): Promise<void> {
		const parsedData = await constructDataFromBytes(
			strippedData,
			multipartComplete
		);

		if (isMultipartData(parsedData)) {
			this.setPartData(
				parsedData.currentFrame,
				parsedData.frameCount,
				parsedData.partData,
				accountsStore
			);
			return;
		}

		if (accountsStore.getAccountByAddress(parsedData.data.account)) {
			this.setState({
				unsignedData: parsedData
			});
		} else {
			// If the address is not found on device in its current encoding,
			// try decoding the public key and encoding it to all the other known network prefixes.
			const networks = Object.keys(SUBSTRATE_NETWORK_LIST);

			for (let i = 0; i < networks.length; i++) {
				const key = networks[i];
				const account = accountsStore.getAccountByAddress(
					encodeAddress(
						decodeAddress(parsedData.data.account),
						SUBSTRATE_NETWORK_LIST[key].prefix
					)
				);

				if (account) {
					parsedData.data.account = account.address;

					this.setState({
						unsignedData: parsedData
					});
					return;
				}
			}

			// if the account was not found, unsignedData was never set, alert the user appropriately.
			this.setErrorMsg(
				`No private key found for ${parsedData.data.account} in your signer key storage.`
			);
		}

		// set payload before it got hashed.
		// signature will be generated from the hash, but we still want to display it.
		if (parsedData.hasOwnProperty('preHash')) {
			this.setPrehashPayload(
				(parsedData as SubstrateCompletedParsedData).preHash
			);
		}
	}

	async setPartData(
		frame: number,
		frameCount: number,
		partData: string,
		accountsStore: AccountsStore
	): Promise<boolean | void | Uint8Array> {
		const {
			latestFrame,
			missedFrames,
			multipartComplete,
			multipartData,
			totalFrameCount
		} = this.state;

		// set it once only
		if (!totalFrameCount) {
			this.setState({
				totalFrameCount: frameCount
			});
		}

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

		const completedFramesCount = Object.keys(multipartData).length;

		if (
			completedFramesCount > 0 &&
			totalFrameCount > 0 &&
			completedFramesCount === totalFrameCount &&
			!multipartComplete
		) {
			// all the frames are filled

			this.setState({
				multipartComplete: true
			});

			// concatenate all the parts into one binary blob
			let concatMultipartData = Object.values(multipartData).reduce(
				(acc: Uint8Array, part: Uint8Array): Uint8Array => {
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
				encodeNumber(frame)
			);
			concatMultipartData = u8aConcat(frameInfo, concatMultipartData);

			// handle the binary blob as a single UOS payload
			this.setParsedData(concatMultipartData, accountsStore, true);
		} else if (completedFramesCount < totalFrameCount) {
			// we haven't filled all the frames yet
			const nextDataState = multipartData;
			nextDataState[frame] = partDataAsBytes;

			const missedFramesRange: number = latestFrame
				? mod(frame - latestFrame, totalFrameCount) - 1
				: 0;

			// we skipped at least one frame that we haven't already scanned before
			if (
				latestFrame &&
				missedFramesRange >= 1 &&
				!missedFrames.includes(frame)
			) {
				// enumerate all the frames between (current)frame and latestFrame
				const updatedMissedFrames = Array.from(
					new Array(missedFramesRange),
					(_, i) => mod(i + latestFrame, totalFrameCount)
				);

				const dedupMissedFrames = new Set([
					...this.state.missedFrames,
					...updatedMissedFrames
				]);

				this.setState({
					missedFrames: Array.from(dedupMissedFrames)
				});
			}

			// if we just filled a frame that was previously missed, remove it from the missedFrames list
			if (missedFrames && missedFrames.includes(frame - 1)) {
				missedFrames.splice(missedFrames.indexOf(frame - 1), 1);
			}

			this.setState({
				latestFrame: frame,
				multipartData: nextDataState
			});
		}

		this.setState({
			completedFramesCount
		});
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
			txRequest,
			type: 'transaction'
		});

		return true;
	}

	//seed is SURI on substrate and is seedPhrase on Ethereum
	async signData(seed: string): Promise<void> {
		const { dataToSign, isHash, sender } = this.state;

		if (!sender || !NETWORK_LIST.hasOwnProperty(sender.networkKey))
			throw new Error('Signing Error: sender could not be found.');

		const isEthereum =
			NETWORK_LIST[sender.networkKey].protocol === NetworkProtocols.ETHEREUM;

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

	async signDataWithSeedPhrase(
		seedPhrase: string,
		protocol: NetworkProtocol
	): Promise<void> {
		if (
			protocol === NetworkProtocols.SUBSTRATE ||
			protocol === NetworkProtocols.UNKNOWN
		) {
			const suri = constructSURI({
				derivePath: this.state.sender?.path,
				password: '',
				phrase: seedPhrase
			});
			await this.signData(suri);
		} else {
			await this.signData(seedPhrase);
		}
	}

	async signDataLegacy(pin = '1'): Promise<void> {
		const { sender } = this.state;
		if (!sender || !sender.encryptedSeed)
			throw new Error('Signing Error: sender could not be found.');
		const seed = await decryptData(sender.encryptedSeed, pin);
		await this.signData(seed);
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
			multipartData: {},
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

	getTXRequest(): TXRequest | null {
		return this.state.txRequest;
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

	setErrorMsg(scanErrorMsg: string): void {
		this.setState({ scanErrorMsg });
	}

	getErrorMsg(): string {
		return this.state.scanErrorMsg;
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
