import { NETWORK_LIST, SubstrateNetworkKeys } from '../src/constants';

export const signingTestIdentityPath = `//${NETWORK_LIST[SubstrateNetworkKeys.KUSAMA].pathID}//default`;

// const setRemarkExtrinsicKusama = "47900000100005301023c36776005aec2f32a34c109dc791a82edef980eec3be80da938ac9bcc68217220170000010c11111185020000fa030000e3777fa922cafbff200cadeaea1a76bd7898ad5b89f7848999058b50e715f63610ed3df3dd943fd93c0a7eda2b7712d25e77ecb153e1ff0dc1f388d028e31fc40ec";
const transaction =
	'49a00000100005301020a06171a5ad1be958012949789fb94a2ba91210a42e2a38b33ef5376b1c43450a40600ffd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d070010a5d4e8f5030000b90000000d667fd278ec412cd9fccdb066f09ed5b4cfd9c9afa9eb747213acb02b1e70bca31fbad00ea7110bbf760cfa85f9d585f6651179d4ee698ef2c4794ff9dd51ac0ec11ec11ec11ec11ec11ec11ec11ec11ec11ec11ec11ec11ec11';

export const createMockSignRequest = () => ({
	bounds: {
		height: 1440,
		origin: [],
		width: 1920
	},
	data: '',
	rawData: transaction,
	target: 319,
	type: 'QR_CODE'
});
