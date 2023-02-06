package io.parity.signer.screens.scan.importderivations

import android.util.Log
import io.parity.signer.screens.scan.transaction.sortedValueCards
import io.parity.signer.uniffi.*


// Rust error state for import derived keys is different comparing to UI requirements,
// hence we need this support function to find out what is the proper UI error to show
// if there are no importable keys left
fun List<MTransaction>.dominantImportError(): DerivedKeyError? {
	Log.e("LOG", "checking dominant error, has keys is ${hasImportableKeys()}")
	if (hasImportableKeys()) return null

	val allErrors: List<DerivedKeyError> = flatMap { it.allImportDerivedKeys() }
		.flatMap { it.derivedKeys }
		.map { it.status }
		.filterIsInstance<DerivedKeyStatus.Invalid>()
		.flatMap { it.errors }

	Log.e("LOG", "checking dominant error, all errors is ${allErrors}")
	val mostCommonError = allErrors
		.groupBy { it }
		.maxByOrNull { entry -> entry.value.size }
		?.key

	Log.e("LOG", "most common error is ${mostCommonError}")
	return mostCommonError
}

// Informs whether there are any valid keys to be imported in `[MTransaction]` payload
fun List<MTransaction>.hasImportableKeys(): Boolean {
	return any { it.hasImportableKeys() }
}

// Informs whether there are any valid keys to be imported in `MTransaction` payload
internal fun MTransaction.hasImportableKeys(): Boolean {
	return when (ttype) {
		TransactionType.IMPORT_DERIVATIONS -> {
			val importedKey = sortedValueCards
				.asSequence()
				.map { it.card }
				.filterIsInstance<Card.DerivationsCard>()
				.flatMap { it.f }
				.flatMap { it.derivedKeys }
				.firstOrNull { it.status == DerivedKeyStatus.Importable }
			importedKey != null
		}
		else -> {
			false
		}
	}
}

// Extracts list of all `SeedKeysPreview` that are within given `MTransaction`
internal fun MTransaction.allImportDerivedKeys(): List<SeedKeysPreview> {
	return when (ttype) {
		TransactionType.IMPORT_DERIVATIONS -> {
			sortedValueCards
				.map { it.card }
				.filterIsInstance<Card.DerivationsCard>()
				.flatMap { it.f }
		}
		else -> {
			emptyList<SeedKeysPreview>()
		}
	}
}


internal fun MTransaction.importableKeysCount(): Int {
	return allImportDerivedKeys().size
}


// Extracts list of importable `SeedKeysPreview` that are within given `MTransaction`
internal fun MTransaction.importableSeedKeysPreviews(): List<SeedKeysPreview> {
	return when (ttype) {
		TransactionType.IMPORT_DERIVATIONS -> {
			sortedValueCards
				.asSequence()
				.map { it.card }
				.filterIsInstance<Card.DerivationsCard>()
				.flatMap { it.f }
				.filter { it.isImportable }
				.toList()
		}
		else -> {
			listOf()
		}
	}
}


private val SeedKeysPreview.importableKeysCount: Int
	get() = derivedKeys.count { it.status == DerivedKeyStatus.Importable }

private val SeedKeysPreview.isImportable: Boolean
	get() = importableKeysCount > 0

