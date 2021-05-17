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
