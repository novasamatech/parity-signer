import { MetadataHandle } from 'types/metadata';
import { generateMetadataHandle } from 'utils/native';

export async function getMetadataHandleFromRaw(
	metadataRaw: string | null
): Promise<MetadataHandle> {
	const metadataHandle = await generateMetadataHandle(metadataRaw + '');
	return metadataHandle;
}
