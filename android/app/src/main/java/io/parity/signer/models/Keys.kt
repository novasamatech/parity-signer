package io.parity.signer.models

import android.util.Log
import org.json.JSONObject

/**
 * Add key to database; uses phone crypto to fetch seeds!
 */
fun SignerDataModel.addKey(path: String, password: String) {
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
			//TODO: this should result in navigation event
			TODO()
		} catch (e: java.lang.Exception) {
			Log.e("Add key error", e.toString())
			_lastError.value = e.toString()
		}
	}
}
