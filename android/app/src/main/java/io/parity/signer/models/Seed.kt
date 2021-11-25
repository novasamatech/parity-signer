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
			//Create relevant keys - should make sure this works before saving key
			val finalSeedPhrase =
				substrateTryCreateSeed(seedName, seedPhrase, 24, dbName)

			//Encrypt and save seed
			with(sharedPreferences.edit()) {
				putString(seedName, finalSeedPhrase)
				apply()
			}

			//Refresh model
			refreshSeedNames()
			selectSeed(seedName)
			_backupSeedPhrase.value = finalSeedPhrase
			//TODO: shis should result in navigation event
		} catch (e: java.lang.Exception) {
			_lastError.value = e.toString()
			Log.e("Seed creation error", e.toString())
		}
	}
}

/**
 * Seed selector; does not check if seedname is valid
 * TODO: check that all related operations are done
 */
fun SignerDataModel.selectSeed(seedName: String) {
	_selectedSeed.value = seedName
	fetchKeys()
}

/**
 * Fetch seed from strongbox; must be in unlocked scope
 */
internal fun SignerDataModel.getSeed(): String {
	return sharedPreferences.getString(selectedSeed.value, "") ?: ""
}

/**
 * Selects seed key, if available
 */
fun SignerDataModel.getRootIdentity(seedName: String): JSONObject {
	for (i in 0 until identities.value!!.length()) {
		val identity = identities.value!!.getJSONObject(i)
		if (identity.getString("seed_name") == seedName && identity.getString("path") == "" && identity.getString(
				"has_password"
			) == "false"
		) {
			return identity
		}
	}
	return JSONObject()
}

fun SignerDataModel.removeSeed() {

}
//MARK: Seed management end
