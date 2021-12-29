package io.parity.signer.models

import android.util.Log
import org.json.JSONObject

//MARK: Seed management begin

/**
 * Refresh seed names list
 * should be called within authentication envelope
 * authentication.authenticate(activity) {refreshSeedNames()}
 * which is somewhat asynchronous
 */
internal fun SignerDataModel.refreshSeedNames() {
	clearError()
	_seedNames.value = sharedPreferences.all.keys.toTypedArray()
}

/**
 * Add seed, encrypt it, and create default accounts
 */
fun SignerDataModel.addSeed(seedName: String, seedPhrase: String) {

	//Check if seed name already exists
	if (seedNames.value?.contains(seedName) as Boolean) {
		_lastError.value = "Seed with this name already exists!"
	}

	//Run standard login prompt!
	authentication.authenticate(activity) {
		try {

			TODO() //create keys etc

			//Encrypt and save seed
			with(sharedPreferences.edit()) {
				putString(seedName, seedPhrase)
				apply()
			}

			//Refresh model
			refreshSeedNames()

			//TODO: shis should result in navigation event
		} catch (e: java.lang.Exception) {
			_lastError.value = e.toString()
			Log.e("Seed creation error", e.toString())
		}
	}
}

/**
 * Fetch seed from strongbox; must be in unlocked scope
 */
internal fun SignerDataModel.getSeed(seedName: String): String {
	return sharedPreferences.getString(seedName, "") ?: ""
}
