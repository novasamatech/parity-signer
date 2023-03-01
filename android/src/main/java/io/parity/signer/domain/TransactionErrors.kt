package io.parity.signer.domain

import io.parity.signer.uniffi.ErrorDisplayed

/**
 * Those errors are generic at the moment as state machine can throw them from anywhere,
 * but they mainly caused by transaction issues.
 */
sealed class TransactionError {
	data class Generic(val message: String) : TransactionError()
	data class MetadataForUnknownNetwork(val name: String) : TransactionError()
	data class NetworkAlreadyAdded(val name: String, val encryption: String) :
		TransactionError()

	data class MetadataAlreadyAdded(val name: String, val version: UInt) :
		TransactionError()

	data class OutdatedMetadata(
		val name: String,
		val currentVersion: UInt,
		val expectedVersion: UInt
	) : TransactionError()

	data class UnknownNetwork(val genesisHash: String, val encryption: String) :
		TransactionError()

	data class NoMetadataForNetwork(val name: String) : TransactionError()
}


private tailrec fun findErrorDisplayed(exception: Throwable): ErrorDisplayed? {
	if (exception is ErrorDisplayed) {
		return exception
	}
	val cause = exception.cause
	return if (cause != null) {
		findErrorDisplayed(cause)
	} else {
		null
	}
}


fun ErrorDisplayed.toTransactionError(): TransactionError {
	return when (this) {
		is ErrorDisplayed.DbNotInitialized -> TransactionError.Generic("")
		is ErrorDisplayed.LoadMetaUnknownNetwork -> TransactionError.MetadataForUnknownNetwork(
			name
		)
		is ErrorDisplayed.MetadataKnown -> TransactionError.MetadataAlreadyAdded(
			name = name,
			version = version
		)
		is ErrorDisplayed.MetadataOutdated -> TransactionError.OutdatedMetadata(
			name = name,
			currentVersion = have,
			expectedVersion = want
		)
		is ErrorDisplayed.MutexPoisoned -> TransactionError.Generic("")
		is ErrorDisplayed.NoMetadata -> TransactionError.NoMetadataForNetwork(name)
		is ErrorDisplayed.SpecsKnown -> TransactionError.NetworkAlreadyAdded(
			name,
			encryption.name
		)
		is ErrorDisplayed.Str -> TransactionError.Generic(s)
		is ErrorDisplayed.UnknownNetwork -> TransactionError.UnknownNetwork(
			genesisHash = genesisHash.toUByteArray().toByteArray().encodeHex(),
			encryption = encryption.name
		)
	}
}
