import { useState, useEffect } from 'React';

import { getMetadata } from 'utils/db';
import { MetadataHandle } from 'types/metadata';

export function useFullMetadataHook( metadataHandle, MetadataHandle ): [boolean, string] {
	const [ metadataReady, setMetadataReady ] = useState<boolean>(false);
	metadata = await getMetadata(metadataHandle);
	setMetadataReady(true);
	return [metadataReady, metadata];
}
