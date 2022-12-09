package io.parity.signer.models

import android.util.Log
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.DerivationCheck
import io.parity.signer.uniffi.substratePathCheck

/**
 * Add key to database; uses phone crypto to fetch seeds!
 */
fun SignerDataModel.addKey(path: String, seedName: String) {
	ServiceLocator.authentication.authenticate(activity) {
		try {
			val seedPhrase = getSeed(seedName)
			if (seedPhrase.isNotBlank()) {
				navigate(Action.GO_FORWARD, path, seedPhrase)
			}
		} catch (e: java.lang.Exception) {
			Log.e("Add key error", e.toString())
		}
	}
}

fun SignerDataModel.increment(number: Int, seedName: String) {
	ServiceLocator.authentication.authenticate(activity) {
		try {
			val seedPhrase = getSeed(seedName)
			if (seedPhrase.isNotBlank()) {
				navigate(Action.INCREMENT, number.toString())
			}
		} catch (e: java.lang.Exception) {
			Log.e("Add key error", e.toString())
		}
	}
}

fun SignerDataModel.checkPath(
	seedName: String,
	path: String,
	network: String
): DerivationCheck {
	return substratePathCheck(
		seedName = seedName,
		path = path,
		network = network,
		dbname = dbName
	)
}
