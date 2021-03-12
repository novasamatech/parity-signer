
export const metadataHandleToKey = (
	metadataHandle: MetadataHandle
): string => {
	const metadataKey = metadataHandle.spec_name + '_v' + metadataHandle.spec_version;
	return metadataKey;
};

