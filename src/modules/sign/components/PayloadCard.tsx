import React, { ReactElement } from 'react';
import { View } from 'react-native';

import { PayloadCardType, PayloadCardContent } from 'types/payload';
import { BlockHashCard, CallCard, EraNonceTipCard, IdCard, TxSpecCard, VariableNameCard, DefaultCard } from 'modules/sign/components/CardTemplates';

type PayloadCardProps = {
	type: PayloadCardType;
	payload?: PayloadCardContent;
};

export default function PayloadCard({
	type,
	payload
}: PayloadCardProps): ReactElement {
	if (type==='call') {
		return (
			<CallCard
				payload={payload}
			/>
		);
	} else if (type === 'varname') {
		return (
			<VariableNameCard 
				payload={payload}
			/>
		);
	} else if (type === 'enum_variant_name') {
		return (
			<VariableNameCard 
				payload={payload}
			/>
		);
	} else if (type === 'Id') {
		return (
			<IdCard
				payload={payload}
			/>
		);
	} else if (type === 'era_nonce_tip') {
		return (
			<EraNonceTipCard 
				payload={payload}
			/>
		);

	} else if (type === 'block_hash') {
		return (
			<BlockHashCard
				payload={payload}
			/>
		);

	} else if (type === 'tx_spec') {
		return (
			<TxSpecCard
				payload={payload}
			/>
		);

	} else {
		return (
			<DefaultCard 
				payload={payload}
			/>
		);
	}
}
