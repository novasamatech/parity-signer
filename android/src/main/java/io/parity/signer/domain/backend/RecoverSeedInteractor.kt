package io.parity.signer.domain.backend

import io.parity.signer.uniffi.ErrorDisplayed
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext


class RecoverSeedInteractor {

	suspend fun seedPhraseGuessWords(
		userInput: String
	): UniffiResult<List<String>> =
		withContext(Dispatchers.IO) {
			try {
				val transactionResult =
					io.parity.signer.uniffi.seedPhraseGuessWords(userInput)
				UniffiResult.Success(transactionResult)
			} catch (e: ErrorDisplayed) {
				UniffiResult.Error(e)
			}
		}

	suspend fun validateSeedPhrase(
		seedPhrase: String
	): UniffiResult<Boolean> =
		withContext(Dispatchers.IO) {
			try {
				val transactionResult =
					io.parity.signer.uniffi.validateSeedPhrase(seedPhrase)
				UniffiResult.Success(transactionResult)
			} catch (e: ErrorDisplayed) {
				UniffiResult.Error(e)
			}
		}
}
