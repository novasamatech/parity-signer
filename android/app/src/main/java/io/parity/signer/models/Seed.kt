package io.parity.signer.models

import android.util.Log
import io.parity.signer.ButtonID
import org.json.JSONArray
import org.json.JSONObject

//MARK: Seed management begin

/**
 * Refresh seed names list
 * should be called within authentication envelope
 * authentication.authenticate(activity) {refreshSeedNames()}
 * which is somewhat asynchronous
 */
internal fun SignerDataModel.refreshSeedNames(init: Boolean = false) {
	clearError()
	val allNames = sharedPreferences.all.keys.sorted().toTypedArray()
	if (init) {
		initNavigation(dbName, allNames.joinToString(","))
	} else {
		updateSeedNames(allNames.joinToString(separator = ","))
	}
	_seedNames.value = allNames
}

/**
 * Add seed, encrypt it, and create default accounts
 */
fun SignerDataModel.addSeed(
	seedName: String,
	seedPhrase: String,
	createRoots: Boolean
) {

	//Check if seed name already exists
	if (seedNames.value?.contains(seedName) as Boolean) {
		_lastError.value = "Seed with this name already exists!"
	}

	//Run standard login prompt!
	authentication.authenticate(activity) {
		try {
			//First check for seed collision
			if (sharedPreferences.all.values.contains(seedPhrase)) {
				error("This seed phrase already exists")
			}

			//Encrypt and save seed
			with(sharedPreferences.edit()) {
				putString(seedName, seedPhrase)
				apply()
			}

			refreshSeedNames()
			pushButton(
				button = ButtonID.GoForward,
				details = if (createRoots) "true" else "false",
				seedPhrase = seedPhrase
			)
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

/**
 * Guess possible seed words from user input
 */
internal fun SignerDataModel.guessWord(word: String): MutableList<String> {
	Log.d("Guess query", "[" + word + "]")
	return JSONArray(substrateGuessWord(word)).toListOfStrings() as MutableList<String>
}

/**
 * Check if provided seed phrase is valid
 */
internal fun SignerDataModel.validatePhrase(seedPhrase: String): String? {
	var errorMessage: String? = null
	try {
		substrateValidateSeedphrase(seedPhrase)
	} catch (e: java.lang.Exception) {
		errorMessage = e.toString()
	}
	return errorMessage
}
