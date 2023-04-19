package io.parity.signer.domain

import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.storage.getSeed


suspend fun SharedViewModel.getSeedPhraseForBackup(
	seedName: String,
): String? {
	val authenticator = ServiceLocator.authentication
	return when (authenticator.authenticate(activity)) {
		AuthResult.AuthSuccess -> {
			val seedPhrase = getSeed(seedName, showInLogs = true)
			seedPhrase
		}
		AuthResult.AuthError,
		AuthResult.AuthFailed,
		AuthResult.AuthUnavailable -> {
			null
		}
	}
}

/**
 * This later should be moved to proper component with activity's lifecycle
 * and available in ServiceLocator
 */
suspend fun getSeedPhraseForBackup(
	seedName: String,
): String? {
	val authenticator = ServiceLocator.authentication
	val seedStorage = ServiceLocator.seedStorage
	val activity = ServiceLocator.activityScope!!.activity

	return when (authenticator.authenticate(activity)) {
		AuthResult.AuthSuccess -> {
			try {
				seedStorage.getSeed(seedName, showInLogs = true)
			} catch (e: Exception) {
				null
			}
		}
		AuthResult.AuthError,
		AuthResult.AuthFailed,
		AuthResult.AuthUnavailable -> {
			null
		}
	}
}
