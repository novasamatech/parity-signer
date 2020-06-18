import { Point, Size } from 'react-native-camera/types';

export interface TxRequestData {
	bounds: {
		width: number;
		height: number;
		/**
		 * @description For Android use `[Point<string>, Point<string>]`
		 * @description For iOS use `{ origin: Point<string>, size: Size<string> }`
		 */
		bounds:
			| [Point<string>, Point<string>]
			| { origin: Point<string>; size: Size<string> };
	};
	type: string;
	rawData: string;
	data: string;
	target?: number;
}

export type ParsedData = SubstrateParsedData | EthereumParsedData;

export type EthereumParsedData = {
	data: {
		data: string;
		account: string;
		rlp: string;
	};
	action: string | null; //"signTransaction"
};

export type SubstrateParsedData =
	| SubstrateMultiParsedData
	| SubstrateCompletedParsedData;

export type CompletedParsedData =
	| SubstrateCompletedParsedData
	| EthereumParsedData;

export type SubstrateCompletedParsedData =
	| SubstrateTransactionParsedData
	| SubstrateMessageParsedData;

export type SubstrateTransactionParsedData = {
	data: {
		account: string;
		crypto: 'ed25519' | 'sr25519' | null;
		data: Uint8Array;
		genesisHash: string;
	};
	action: 'signTransaction';
	oversized: boolean;
	isHash: false;
};

export type SubstrateMessageParsedData = {
	data: {
		account: string;
		crypto: 'ed25519' | 'sr25519' | null;
		data: string;
		genesisHash: string;
	};
	action: 'signData';
	oversized: boolean;
	isHash: true;
};

export type SubstrateMultiParsedData = {
	currentFrame: number;
	frameCount: number;
	isMultipart: boolean;
	partData: string;
};

export type SURIObject = {
	derivePath: string;
	password: string;
	phrase: string;
};

export function isEthereumCompletedParsedData(
	parsedData: ParsedData
): parsedData is EthereumParsedData {
	return (parsedData as EthereumParsedData).data.rlp !== undefined;
}

export function isSubstrateCompletedParsedData(
	parsedData: ParsedData | null
): parsedData is SubstrateCompletedParsedData {
	return (
		(parsedData as SubstrateCompletedParsedData)?.data?.crypto !== undefined
	);
}

export function isSubstrateMessageParsedData(
	parsedData: ParsedData | null
): parsedData is SubstrateMessageParsedData {
	return (
		(parsedData as SubstrateCompletedParsedData)?.data?.crypto !== undefined &&
		(parsedData as SubstrateCompletedParsedData)?.action === 'signData'
	);
}

export function isMultipartData(
	parsedData: ParsedData | null
): parsedData is SubstrateMultiParsedData {
	const hasMultiFrames =
		(parsedData as SubstrateMultiParsedData)?.frameCount !== undefined &&
		(parsedData as SubstrateMultiParsedData).frameCount > 1;
	return (
		(parsedData as SubstrateMultiParsedData)?.isMultipart || hasMultiFrames
	);
}
