import { NETWORK_LIST, SubstrateNetworkKeys } from '../src/constants';

export const signingTestIdentityPath = `//${NETWORK_LIST[SubstrateNetworkKeys.KUSAMA].pathID}//default`;

export const createDataSignRequest = address => ({
	bounds: {
		height: 1440,
		origin: [],
		width: 1920
	},
	data: '',
	rawData:
		'49a00000100005301020a06171a5ad1be958012949789fb94a2ba91210a42e2a38b33ef5376b1c43450a40600ff94d3487c7d14e2a5c28abc8cc16cad9dfe5195562a594f82e0b290dd566d0b5c070010a5d4e875010000b90000000d667fd278ec412cd9fccdb066f09ed5b4cfd9c9afa9eb747213acb02b1e70bcda2680fb733de798a1b40472fb657afcb860537651f6b4fa5d884f3f20aeefff0ec11ec11ec11ec11ec11ec11ec11ec11ec11ec11ec11ec11ec11',
	target: 319,
	type: 'QR_CODE'
});
