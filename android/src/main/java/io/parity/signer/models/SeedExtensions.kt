package io.parity.signer.models


suspend fun SignerDataModel.getSeedPhraseForBackup(seedName: String,
): String? {
	return when (authentication.authenticate(activity)) {
		Authentication.AuthResult.AuthSuccess -> {
			val seedPhrase = getSeed(seedName, backup = true)
			seedPhrase
		}
		Authentication.AuthResult.AuthError,
		Authentication.AuthResult.AuthFailed,
		Authentication.AuthResult.AuthUnavailable -> {
			null
		}
	}
}
