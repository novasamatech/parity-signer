package io.parity.signer.models


suspend fun SignerDataModel.getSeedPhraseForBackup(seedName: String,
): String? {
	return when (authentication.authenticate(activity)) {
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
