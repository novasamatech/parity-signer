export function assertNever(x: never): never {
	throw new Error('Unexpected value: ' + x);
}

export interface ValidSeed {
	accountRecoveryAllowed?: boolean;
	bip39: boolean;
	reason: string | null;
	valid: boolean;
}
