package io.parity.signer.models

import android.util.Log
import io.parity.signer.ButtonID

/**
 * Add key to database; uses phone crypto to fetch seeds!
 */
fun SignerDataModel.addKey(path: String, seedName: String) {
	authentication.authenticate(activity) {
		try {
			val seedPhrase = getSeed(seedName)
			if (seedPhrase.isNotBlank()) {
				pushButton(ButtonID.GoForward, path, seedPhrase)
			}
		} catch (e: java.lang.Exception) {
			Log.e("Add key error", e.toString())
			_lastError.value = e.toString()
		}
	}
}

/**
 * Checker for derivation path
 */
class DerivationState(
	var isValid: Boolean = true,
	var hasPassword: Boolean = false
)

fun SignerDataModel.checkAsDerivation(path: String): DerivationState {
	return try {
		DerivationState(isValid = true, hasPassword = substrateCheckPath(path))
	} catch (e: java.lang.Exception) {
		DerivationState(isValid = false, hasPassword = false)
	}
}
