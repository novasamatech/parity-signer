package io.parity.signer.domain.usecases

import io.parity.signer.domain.backend.OperationResult
import io.parity.signer.uniffi.ErrorDisplayed
import io.parity.signer.uniffi.tryCreateAddress
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext

/**
 * Creates key set
 */
class CreateDerivationUseCase() {

	suspend fun createDerivation(
		seedName: String,
		seedPhrase: String,
		path: String,
		networkKey: String,
	): OperationResult<Unit, ErrorDisplayed> = withContext(Dispatchers.IO) {
		try {
			tryCreateAddress(
				seedName,
				seedPhrase,
				path,
				networkKey
			)
			OperationResult.Ok(Unit)
		} catch (e: ErrorDisplayed) {
			OperationResult.Err(e)
		}
	}
}
