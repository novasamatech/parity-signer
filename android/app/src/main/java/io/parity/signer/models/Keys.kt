package io.parity.signer.models

import android.util.Log
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.substratePathCheck
import io.parity.signer.uniffi.DerivationCheck as DerivationCheckFFI

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

fun SignerDataModel.pathCheck(
	seedName: String,
	path: String,
	network: Network
): DerivationCheckFFI {
	return substratePathCheck(
		seedName = seedName,
		path = path,
		network = network.toString(),
		dbname = dbName
	)
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
