import React, { ReactElement } from 'react';
import { View } from 'react-native';

import { PayloadCardType, PayloadCardContent } from 'types/payloads';
import {
	LoadingCard,
	ErrorCard,
	WarningCard,
	DefaultCard,
	AuthorCard,
	BalanceCard,
	BlockHashCard,
	CallCard,
	EraNonceCard,
	IdCard,
	TipCard,
	TxSpecCard,
	VariableNameCard
} from 'components/CardTemplates';

type PayloadCardProps = {
	type: PayloadCardType;
	payload?: PayloadCardContent;
};

export default function PayloadCard({
	type,
	payload
}: PayloadCardProps): ReactElement {
	switch (type) {
		case 'loading':
			return <LoadingCard payload={payload} />;
		case 'error':
			return <ErrorCard payload={payload} />;
		case 'warning':
			return <WarningCard payload={payload} />;
		case 'author':
			return <AuthorCard payload={payload} />;
		case 'balance':
			return <BalanceCard payload={payload} />;
		case 'call':
			return <CallCard payload={payload} />;
		case 'varname':
			return <VariableNameCard payload={payload} />;
		case 'enum_variant_name':
			return <VariableNameCard payload={payload} />;
		case 'Id':
			return <IdCard payload={payload} />;
		case 'era_nonce':
			return <EraNonceCard payload={payload} />;
		case 'tip':
			return <TipCard payload={payload} />;
		case 'block_hash':
			return <BlockHashCard payload={payload} />;
		case 'tx_spec':
			return <TxSpecCard payload={payload} />;
		default:
			return <DefaultCard payload={payload} />;
	}
}
