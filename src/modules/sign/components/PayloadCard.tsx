import React, { ReactElement } from 'react';
import { View } from 'react-native';

import { PayloadCardType, PayloadCardContent } from 'types/payloads';
import {
	LoadingCard,
	ErrorCard,
	DefaultCard,
	BlockHashCard,
	CallCard,
	EraNonceTipCard,
	IdCard,
	TxSpecCard,
	VariableNameCard
} from 'modules/sign/components/CardTemplates';

type PayloadCardProps = {
	type: PayloadCardType;
	payload?: PayloadCardContent;
};

export default function PayloadCard({
	type,
	payload
}: PayloadCardProps): ReactElement {
	if (type === 'loading') {
		return <LoadingCard payload={payload} />;
	} else if (type === 'error') {
		return <ErrorCard payload={payload} />;
	} else if (type === 'call') {
		return <CallCard payload={payload} />;
	} else if (type === 'varname') {
		return <VariableNameCard payload={payload} />;
	} else if (type === 'enum_variant_name') {
		return <VariableNameCard payload={payload} />;
	} else if (type === 'Id') {
		return <IdCard payload={payload} />;
	} else if (type === 'era_nonce_tip') {
		return <EraNonceTipCard payload={payload} />;
	} else if (type === 'block_hash') {
		return <BlockHashCard payload={payload} />;
	} else if (type === 'tx_spec') {
		return <TxSpecCard payload={payload} />;
	} else {
		return <DefaultCard payload={payload} />;
	}
}
