//List of valid renderable card templates
export type PayloadCardType =
	| 'call'
	| 'varname'
	| 'enum_variant_name'
	| 'Id'
	| 'default'
	| 'era_nonce_tip'
	| 'block_hash'
	| 'tx_spec';

export type PayloadCardContent = {
	[key: string]: unknown;
};

//Renderable unit records for transaction details screen
//index: key for proper sorting
//indent: the card should be shifted to indicate hierarchy
//type: template for rendering
//payload: actual info to show
export type PayloadCardData = {
	index: number;
	indent: number;
	type: PayloadCardType;
	payload?: PayloadCardContent;
};

//Object to store all parsed transaction information
export type PayloadCardsSet = {
	[key: string]: unknown;
	method: PayloadCardData[];
	extrinsics: PayloadCardData[];
};
