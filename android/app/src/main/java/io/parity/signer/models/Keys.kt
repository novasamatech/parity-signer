package io.parity.signer.models

import android.util.Log
import io.parity.signer.uniffi.Action

/**
 * Add key to database; uses phone crypto to fetch seeds!
 */
fun SignerDataModel.addKey(path: String, seedName: String) {
	authentication.authenticate(activity) {
		try {
			val seedPhrase = getSeed(seedName)
			if (seedPhrase.isNotBlank()) {
				pushButton(Action.GO_FORWARD, path, seedPhrase)
			}
		} catch (e: java.lang.Exception) {
			Log.e("Add key error", e.toString())
			_lastError.value = e.toString()
		}
	}
}

fun SignerDataModel.increment(number: Int, seedName: String) {
	authentication.authenticate(activity) {
		try {
			val seedPhrase = getSeed(seedName)
			if (seedPhrase.isNotBlank()) {
				pushButton(Action.INCREMENT, number.toString())
			}
		} catch (e: java.lang.Exception) {
			Log.e("Add key error", e.toString())
			_lastError.value = e.toString()
		}
	}
}
