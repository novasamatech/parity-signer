export function deepCopyMap<T>(original: Map<string, T>): Map<string, T> {
	const originalEntries = original.entries();
	const copiedEntries = Array.from(originalEntries);

	return new Map(copiedEntries);
}
