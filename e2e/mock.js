import { NETWORK_LIST, SubstrateNetworkKeys } from '../src/constants';

export const signingTestIdentityPath = `//${NETWORK_LIST[SubstrateNetworkKeys.KUSAMA].pathID}//default`;

const setRemarkExtrinsicKusama =
	'47900000100005301023c36776005aec2f32a34c109dc791a82edef980eec3be80da938ac9bcc68217220170000010c11111165030000fa030000e3777fa922cafbff200cadeaea1a76bd7898ad5b89f7848999058b50e715f636dbb5aefb451e26bd64faf476301f980437d87c0d88dec1a8c7a3eb3cc82e9bbb0ec';

export const createMockSignRequest = () => ({
	bounds: {
		height: 1440,
		origin: [],
		width: 1920
	},
	data: '',
	rawData: setRemarkExtrinsicKusama,
	target: 319,
	type: 'QR_CODE'
});
