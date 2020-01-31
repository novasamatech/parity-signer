// Copyright 2018 @polkadot/react-identicon authors & contributors
// This software may be modified and distributed under the terms
// of the Apache-2.0 license. See the LICENSE file for details.

export interface BaseProps {
	className?: string;
	style?: Record<string, any>;
}

export interface Props extends BaseProps {
	address: string;
	publicKey: string;
	size: number;
}
