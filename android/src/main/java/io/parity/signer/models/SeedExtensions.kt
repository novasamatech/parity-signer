package io.parity.signer.models

import io.parity.signer.dependencygraph.ServiceLocator


suspend fun SignerDataModel.getSeedPhraseForBackup(seedName: String,
): String? {
	return when (ServiceLocator.authentication.authenticate(activity)) {
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
