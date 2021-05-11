import { GenericExtrinsicPayload, GenericCall, Struct } from '@polkadot/types';
import type { Call, ExtrinsicEra } from '@polkadot/types/interfaces';
import {
	AnyJson,
	AnyU8a,
	Codec,
	IExtrinsicEra,
	IMethod
} from '@polkadot/types/types';

export type FrameMethod = {
	method: string;
	pallet: string;
};

export type SanitizedArgs = {
	[key: string]: unknown;
	call?: SanitizedCall;
	calls?: SanitizedCall[];
};

export type SanitizedCall = {
	[key: string]: unknown;
	args: SanitizedArgs;
	callIndex?: Uint8Array | string;
	method: string | FrameMethod;
};
