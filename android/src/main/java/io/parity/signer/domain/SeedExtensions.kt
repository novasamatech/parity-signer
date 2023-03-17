package io.parity.signer.domain

import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.storage.getSeed


suspend fun SharedViewModel.getSeedPhraseForBackup(
	seedName: String,
): String? {
	val authenticator = ServiceLocator.authentication
	return when (authenticator.authenticate(activity)) {
		AuthResult.AuthSuccess -> {
			val seedPhrase = getSeed(seedName, backup = true)
			seedPhrase
		}
		AuthResult.AuthError,
		AuthResult.AuthFailed,
		AuthResult.AuthUnavailable -> {
			null
		}
	}
}
