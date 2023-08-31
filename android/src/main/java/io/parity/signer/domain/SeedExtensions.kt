package io.parity.signer.domain

import android.util.Log
import android.widget.Toast
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.storage.getSeed


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
				Log.d("get seed failure", e.toString())
				Toast.makeText(activity, "get seed failure: $e", Toast.LENGTH_LONG).show()
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
