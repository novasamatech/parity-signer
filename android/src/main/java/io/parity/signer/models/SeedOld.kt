package io.parity.signer.models

import android.util.Log
import android.widget.Toast
import io.parity.signer.components.SeedBoxStatus
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.historySeedNameWasShown
import io.parity.signer.uniffi.initNavigation
import io.parity.signer.uniffi.updateSeedNames

/**
 * Refresh seed names list
 * should be called within authentication envelope
 * authentication.authenticate(activity) {refreshSeedNames()}
 * which is somewhat asynchronous
 */
internal fun SignerDataModel.refreshSeedNames(init: Boolean = false) {
	val allNames = sharedPreferences.all.keys.sorted().toTypedArray()
	if (init) {
		initNavigation(dbName, allNames.toList())
	} else {
		updateSeedNames(allNames.toList())
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

	// Check if seed name already exists
	if (seedNames.value.contains(seedName)) {
		return
	}

	// Run standard login prompt!
	ServiceLocator.authentication.authenticate(activity) {
		try {
			// First check for seed collision
			if (sharedPreferences.all.values.contains(seedPhrase)) {
				error("This seed phrase already exists")
			}

			// Encrypt and save seed
			with(sharedPreferences.edit()) {
				putString(seedName, seedPhrase)
				apply()
			}

			refreshSeedNames()
			navigate(
				button = Action.GO_FORWARD,
				details = if (createRoots) "true" else "false",
				seedPhrase = seedPhrase
			)
		} catch (e: java.lang.Exception) {
			Log.e("Seed creation error", e.toString())
		}
	}
}

/**
 * Fetch seed from strongbox; must be in unlocked scope
 */
internal fun SignerDataModel.getSeed(
	seedName: String,
	backup: Boolean = false
): String {
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
 * All logic required to remove seed from memory
 *
 * 1. Remover encrypted storage item
 * 2. Synchronizes list of seeds with rust
 * 3. Calls rust remove seed logic
 */
fun SignerDataModel.removeSeed(seedName: String) {
	ServiceLocator.authentication.authenticate(activity) {
		try {
			sharedPreferences.edit().remove(seedName).apply()
			refreshSeedNames()
			navigator.navigate(Action.REMOVE_SEED)
		} catch (e: java.lang.Exception) {
			Log.d("remove seed error", e.toString())
			Toast.makeText(context, e.toString(), Toast.LENGTH_SHORT).show()
		}
	}
}

