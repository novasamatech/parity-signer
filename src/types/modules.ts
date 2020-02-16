declare module 'react-native-secure-storage' {
	export interface SecureStoreParams {
		keychainService: string;
		sharedPreferencesName: string;
	}

	export function getAllItems(store: SecureStoreParams): any;

	export function getItem(itemLabel: string, store: SecureStoreParams): any;

	export function setItem(
		itemLabel: string,
		item: any,
		store: SecureStoreParams
	): void;

	export function deleteItem(itemLabel: string, store: SecureStoreParams): void;
}
