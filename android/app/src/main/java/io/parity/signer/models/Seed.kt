package io.parity.signer.models

import android.util.Log
import android.widget.Toast
import io.parity.signer.ButtonID
import org.json.JSONArray

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
internal fun SignerDataModel.getSeed(seedName: String, backup: Boolean = false): String {
	return try {
		val seedPhrase = sharedPreferences.getString(seedName, "") ?: ""
		if (seedPhrase.isBlank()) {
			""
		} else {
			if (backup) {
				historySeedNameWasShown(seedName, dbName)
			}
			seedPhrase
		}
	} catch (e: java.lang.Exception) {
		Log.d("get seed failure", e.toString())
		Toast.makeText(context, "get seed failure: $e", Toast.LENGTH_LONG).show()
		""
	}

}

/**
 * Guess possible seed words from user input
 */
internal fun SignerDataModel.guessWord(word: String): List<String> {
	return JSONArray(substrateGuessWord(word)).toListOfStrings()
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

fun SignerDataModel.removeSeed(seedName: String) {
	authentication.authenticate(activity) {
		try {
			sharedPreferences.edit().remove(seedName).apply()
			refreshSeedNames()
			pushButton(ButtonID.RemoveSeed)
		} catch (e: java.lang.Exception) {
			Log.d("remove seed error", e.toString())
			Toast.makeText(context, e.toString(), Toast.LENGTH_SHORT).show()
		}
	}
}
