// Type for all operations on metadata not involving metadata itself

export type MetadataHandle = {
	hash: string;
	specName: string;
	specVersion: number;
};

// Type to store serialized metadata. Should be accessed only
// on first init or by rust-native, but for now legacy getMetadata should
// will use it passing MetadataHandle instead of networkKey

export type MetadataRecord = {
	serialized: string;
};
