import React, { ReactElement } from 'react';
import { View } from 'react-native';

import { PayloadCardType, PayloadCardContent } from 'types/payload';
import { CallCard, DefaultCard } from 'modules/sign/components/CardTemplates';

type PayloadCardProps = {
	indent: number;
	type: PayloadCardType;
	payload?: PayloadCardContent;
};

export default function PayloadCard({
	indent,
	type,
	payload
}: PayloadCardProps): ReactElement {
	if (type==='call') {
		return (
			<CallCard
				indent={indent}
				payload={payload}
			/>
		);
	}
	return (
		<DefaultCard 
			indent={indent}
			payload={payload}
		/>
	);
}
