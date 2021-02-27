const strings = {
	ERROR_ADDRESS_MESSAGE:
		'Please create a transaction using software such as MyCrypto so that Stylo can sign it.',
	ERROR_NO_NETWORK:
		'Signer does not currently recognize a chain with genesis hash, please add the network first',
	ERROR_NO_RAW_DATA: 'There is no raw data from the request',
	ERROR_NO_SENDER_FOUND: 'There is no related account in the app',
	ERROR_NO_SENDER_IDENTITY: 'There is no related identity in the app',
	ERROR_TITLE: 'Unable to parse QR data',
	ERROR_WRONG_RAW:
		'Frames number is too big, the QR seems not to be a recognized extrinsic raw data',
	INFO_ETH_TX: 'You are about to send the following amount',
	INFO_MULTI_PART:
		'You are about to send the following extrinsic. We will sign the hash of the payload as it is oversized.',
	SUCCESS_ADD_NETWORK: 'Successfully updated new network: ',
	SUCCESS_TITLE: 'Success'
};

export default strings;
