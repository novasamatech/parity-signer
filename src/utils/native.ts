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

import { NativeModules } from 'react-native';

import { MetadataHandle } from 'types/metadata';
import { PayloadCardsSet } from 'types/payloads';

const { SubstrateSign } = NativeModules || {};

export async function rustTest(input: string): Promise<string> {
	console.log('###########################');
	console.log('RUST INTERFACE TEST INVOKED');
	console.log('###########################');
	console.log(input);
	console.log(typeof input);
	let output = '';
	try {
		output = await SubstrateSign.developmentTest(input);
	} catch (e) {
		output = e;
	}
	console.log('###########################');
	console.log(output);
	console.log('###########################');
	console.log('RUST INTERFACE TEST SUCCESS');
	console.log('###########################');
	return output;
}

// Creates a QR code for the UTF-8 representation of a string
export function qrCode(data: string): Promise<string> {
	return SubstrateSign.qrCode(data);
}

//This is called when terms and conditions are acknowledged; the Signer is not usable before db init.
export async function dbInit(): Promise<void> {
	try {
		await SubstrateSign.dbInit();
		console.log('db created!');
	} catch (e) {
		console.log(e);
	}
}

export async function tryCreateSeed(
	seedName: string,
	cryptoType: string
): Promise<void> {
	console.log(seedName);
	console.log(cryptoType);
	const spitOut = await SubstrateSign.tryCreateSeed(seedName, cryptoType, 24);
	console.log('seed creation');
	console.log(spitOut);
}

export async function tryRecoverSeed(
	seedName: string,
	cryptoType: string,
	seedPhrase: string
): Promise<void> {
	await SubstrateSign.tryRecoverSeed(seedName, cryptoType, seedPhrase);
}

export async function getSeedPhraseForBackup(
	seedName: string,
	pin: string
): Promise<string> {
	const seedPhrase = await SubstrateSign.fetchSeed(seedName, pin);
	console.log('it is now smeared all over memory');
	console.log(seedPhrase);
	return seedPhrase;
}

export async function suggestNPlusOne(
	path: string,
	seedName: string,
	network: string
): Promise<string> {
	try {
		const suggest = await SubstrateSign.suggestNPlusOne(
			path,
			seedName,
			network
		);
		return suggest;
	} catch (e) {
		console.warn(e);
		return '';
	}
}

export async function suggestSeedName(path: string): Promise<string> {
	const suggest = await SubstrateSign.suggestPathName(path);
	console.log(suggest);
	return suggest;
}

export async function tryCreateIdentity(
	idName: string,
	seedName: string,
	cryptoType: string,
	path: string,
	networkId: string
): Promise<void> {
	console.log('creating identity...');
	if (idName == "") throw new Error("Seed name should not be blank");
	await SubstrateSign.tryCreateIdentity(
		idName,
		seedName,
		cryptoType,
		path,
		networkId
	);
	console.log('identity created');
}

export async function deleteIdentity(
	pubKey: string,
	networkId: string
): Promise<void> {
	try {
		console.log('deleting identity');
		await SubstrateSign.deleteIdentity(pubKey, networkId);
		console.log('deleting successful');
	} catch (e) {
		console.warn(e);
	}
}

//Try to decode fountain packages
export async function tryDecodeQr(
	data: Array<string>,
	size: number,
	packetSize: number
): Promise<string> {
	const preparedData = data.join(',');
	const localSizeCopy = size;
	const localPacketSizeCopy = packetSize;
	const decoded = await SubstrateSign.tryDecodeQrSequence(
		localSizeCopy,
		localPacketSizeCopy,
		preparedData
	);
	return decoded;
}

//Generate metadata handle from metadata
export async function generateMetadataHandle(
	metadata: string
): Promise<MetadataHandle> {
	const handleJSON = await SubstrateSign.generateMetadataHandle(metadata);
	const handle = JSON.parse(handleJSON);
	const metadataHandle: MetadataHandle = {
		hash: handle[2].toString() as string,
		specName: handle[0] ? (handle[0] as string) : '',
		specVersion: handle[1] ? parseInt(handle[1], 10) : 0
	};
	return metadataHandle;
}

//Generate payload info
//TODO: replace altogether with arbitrary payload parsing finction
export async function makeTransactionCardsContents(
	payload: string
): Promise<PayloadCardsSet> {
	const parsedJSON = await SubstrateSign.parseTransaction(payload);
	console.log(parsedJSON);
	const parsed = JSON.parse(parsedJSON);
	return parsed;
}

//Perform action requiring use of secret
//Typically sign a transaction
export async function sign(
	action: string,
	seedName: string,
	password: string
): Promise<string> {
	const signedPayload = await SubstrateSign.signTransaction(
		action,
		seedName,
		password
	);
	return signedPayload;
}

//Perform action not requiring use of secret
//Typically updates
export async function accept(action: string): Promise<string> {
	try {
		const acceptResult = await SubstrateSign.handleTransaction(action);
		console.log(acceptResult);
		return acceptResult;
	} catch (e) {
		console.log(e);
		return e.toString();
	}
}

/**
 * Functions to fill UI
 */

//Get info to fill screen with list of networks
export async function getAllNetworks(): Promise<[Network] | []> {
	try {
		const allNetworksJSON = await SubstrateSign.getAllNetworksForNetworkSelector();
		const allNetworks = JSON.parse(allNetworksJSON);
		return allNetworks;
	} catch (e) {
		console.log(e);
		return [];
	}
}

//Get relevant showable info on one network
export async function getNetwork(networkKey: string): Promise<Network> {
	try {
		const networkJSON = await SubstrateSign.getNetwork(networkKey);
		const network = JSON.parse(networkJSON);
		return network;
	} catch (e) {
		console.log(e);
		return {};
	}
}

//Get list of identities under current seed
export async function getIdentitiesForSeed(
	seedName: string,
	genesisHash: string
): Promise<Identitieslist> {
	try {
		const relevantIdentitiesJSON = await SubstrateSign.getRelevantIdentities(
			seedName,
			genesisHash
		);
		const relevantIdentities = JSON.parse(relevantIdentitiesJSON);
		console.log(relevantIdentities);
		return relevantIdentities;
	} catch (e) {
		console.log(e);
		return [];
	}
}

//Get list of seedphrase identifiers
export async function getAllSeedNames(): Promise<[string] | []> {
	try {
		const allSeedsJSON = await SubstrateSign.getAllSeedNames();
		console.log('something returned for seeds');
		console.log(allSeedsJSON);
		const allSeeds = JSON.parse(allSeedsJSON);
		return allSeeds;
	} catch (e) {
		console.log(e);
		return [];
	}
}

//Network management screen
export async function getNetworkSpecs(networkKey: string): Promise<object> {
	const specsJSON = await SubstrateSign.getNetworkSpecs(networkKey);
	console.log(specsJSON);
	const specs = JSON.parse(specsJSON);
	return specs;
}

//Network deletion
export async function removeNetwork(networkKey: string): Promise<void> {
	return;
}

export async function removeMetadata(specVersion: string, specName: string): Promise<void> {
	return;
}
