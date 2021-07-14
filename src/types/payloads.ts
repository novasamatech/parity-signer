//List of valid renderable card templates
export type PayloadCardType =
	| 'call'
	| 'varname'
	| 'enum_variant_name'
	| 'Id'
	| 'default'
	| 'era_nonce'
	| 'block_hash'
	| 'loading'
	| 'error'
	| 'warning'
	| 'author'
	| 'balance'
	| 'tip'
	| 'none'
	| 'identity_field'
	| 'bitvec'
	| 'field_name'
	| 'field_number'
	| 'tx_spec';

export type PayloadCardContentDefault = {
	[key: string]: unknown;
};

export type PayloadCardContent = PayloadCardContentDefault;

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

//Whet should the only button on payload preview screen do
export type Action = {
	type: string;
	payload: unknown;
};

//Object to store all parsed transaction information
export type PayloadCardsSet = {
	[key: string]: unknown;
	action?: Action;
	author?: PayloadCardData[];
	error?: PayloadCardData[];
	extrinsics?: PayloadCardData[];
	method?: PayloadCardData[];
	warning?: PayloadCardData[];
};
