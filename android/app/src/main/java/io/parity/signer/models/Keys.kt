package io.parity.signer.models

import android.util.Log
import androidx.compose.ui.graphics.ImageBitmap
import org.json.JSONArray
import org.json.JSONObject

//MARK: Key management begin

/**
 * Refresh keys relevant for other parameters
 */
internal fun SignerDataModel.fetchKeys() {
	try {
		Log.d("selectedNetwork", selectedNetwork.value.toString())
		Log.d("Selected seed", selectedSeed.value.toString())
		_identities.value = JSONArray(
			dbGetRelevantIdentities(
				selectedSeed.value ?: "",
				selectedNetwork.value?.get("key").toString(),
				dbName
			)
		)
	} catch (e: java.lang.Exception) {
		Log.e("fetch keys error", e.toString())
	}
}

/**
 * Just set key for filtering
 */
fun SignerDataModel.selectKey(key: JSONObject) {
	_selectedIdentity.value = key
}

/**
 * Add key to database; uses phone crypto to fetch seeds!
 */
fun SignerDataModel.addKey(path: String, password: String) {
	if (selectedSeed.value?.isEmpty() as Boolean) selectSeed(
		selectedIdentity.value?.get(
			"seed_name"
		).toString()
	)
	var fullPath = path
	val hasPassword = password.isNotEmpty()
	if (hasPassword) fullPath += "///$password"
	try {
		if (substrateCheckPath(path) != hasPassword) {
			_lastError.value =
				"The sequence /// is not allowed in path; use password field to include password (omitting ///)"
			Log.e("Add key preparation error", "password in path field")
			return
		}
	} catch (e: java.lang.Exception) {
		_lastError.value = e.toString()
		Log.e("Add key path check error", e.toString())
	}
	authentication.authenticate(activity) {
		try {
			substrateTryCreateIdentity(
				selectedSeed.value!!,
				getSeed(),
				"sr25519",
				path,
				selectedNetwork.value?.get("key").toString(),
				hasPassword,
				dbName
			)
			fetchKeys()
			clearKeyManagerScreen()
		} catch (e: java.lang.Exception) {
			Log.e("Add key error", e.toString())
			_lastError.value = e.toString()
		}
	}
}

/**
 * delete selected key for selected network
 */
fun SignerDataModel.deleteKey() {
	try {
		substrateDeleteIdentity(
			selectedIdentity.value?.get("public_key").toString(),
			selectedNetwork.value?.get("key").toString(),
			dbName
		)
		fetchKeys()
		clearKeyManagerScreen()
	} catch (e: java.lang.Exception) {
		Log.e("key deletion error", e.toString())
	}
}

fun SignerDataModel.proposeDerivePath(): String {
	return if (selectedIdentity.value?.isNull("path") as Boolean)
		"//"
	else
		selectedIdentity.value?.get("path").toString()
}

fun SignerDataModel.proposeIncrement(): String {
	if (selectedIdentity.value?.isNull("path") as Boolean)
		return ""
	else {
		return try {
			substrateSuggestNPlusOne(
				selectedIdentity.value?.get("path").toString(),
				selectedSeed.value.toString(),
				selectedNetwork.value?.get("key").toString(),
				dbName
			)
		} catch (e: java.lang.Exception) {
			_lastError.value = e.toString()
			Log.e("Increment error", e.toString())
			""
		}
	}
}

fun SignerDataModel.exportPublicKey(): ImageBitmap {
	return try {
		substrateExportPubkey(
			selectedIdentity.value?.get("public_key").toString(),
			selectedNetwork.value?.get("key").toString(),
			dbName
		).intoImageBitmap()
	} catch (e: java.lang.Exception) {
		Log.d("QR export error", e.toString())
		_lastError.value = e.toString()
		ImageBitmap(1, 1)
	}
}

fun SignerDataModel.selectNext() {

}

fun SignerDataModel.selectPrefious() {

}

fun SignerDataModel.multiselect() {

}

fun SignerDataModel.isMultiselect() {

}

//MARK: Key management end
