import { NETWORK_LIST, SubstrateNetworkKeys } from '../src/constants';

export const signingTestIdentityPath = `//${NETWORK_LIST[SubstrateNetworkKeys.KUSAMA].pathID}//default`;

export const createDataSignRequest = address => ({
	rawData:
		'4' + // indicates data is binary encoded
		'37' + // byte length of data
		'00' + // is it multipart?
		'0001' + // how many parts in total?
		'0000' + // which frame are we on?
		'53' + // S for Substrate
		'01' + // sr25519
		'03' + // sign message
		address + // key
		'5448495320495320535041525441210' + // THIS IS SPARTA!
		'ec11ec11ec11ec'
});
