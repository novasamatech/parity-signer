package io.parity.signer.models

import android.util.Log
import android.widget.Toast
import androidx.compose.runtime.MutableState
import androidx.compose.runtime.currentCompositionLocalContext
import org.json.JSONObject
import uniffi.signer.Action

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

enum class DeriveDestination {
	pin,
	pwd;
}

/**
 * Checker for derivation path
 */
class DerivationCheck(
	var buttonGood: MutableState<Boolean>,
	var whereTo: MutableState<DeriveDestination?>,
	var collision: MutableState<JSONObject?>,
	var checkCallback: (path: String) -> String
) {
	/**
	 * Call to perform on every path change
	 */
	fun check(path: String) {
		val checkResult = checkCallback(path)
		Log.d("checkResult", checkResult)
		fromJSON(JSONObject(checkResult).optJSONObject("derivation_check")?: JSONObject())
	}

	/**
	 * Gerenate check state
	 */
	fun fromJSON(input: JSONObject) {
		buttonGood.value = input.optBoolean("button_good", false)
		whereTo.value = try {
			DeriveDestination.valueOf(input.optString("where_to"))
		} catch (_: java.lang.Exception) {
			null
		}
		collision.value = input.optJSONObject("collision")
		input.optString("error").let {
			Log.d("collision checker error", it)
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
