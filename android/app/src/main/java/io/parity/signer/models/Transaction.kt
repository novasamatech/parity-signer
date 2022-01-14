package io.parity.signer.models

import android.util.Log
import androidx.compose.ui.graphics.ImageBitmap
import org.json.JSONArray
import org.json.JSONObject

/**
 * Send scanned QR to backend and rearrange cards nicely
 * We should probably simplify this once UI development is done
 */
internal fun SignerDataModel.parseTransaction() {
	try {
		val transactionString = "" //TODO
		Log.d("transaction string", transactionString)
		val transactionObject = JSONObject(transactionString)
		//TODO: something here
		val author = (transactionObject.optJSONArray("author") ?: JSONArray())
		val warnings = transactionObject.optJSONArray("warning") ?: JSONArray()
		val error = (transactionObject.optJSONArray("error") ?: JSONArray())
		val typesInfo =
			transactionObject.optJSONArray("types_info") ?: JSONArray()
		val method = (transactionObject.optJSONArray("method") ?: JSONArray())
		val extensions =
			(transactionObject.optJSONArray("extensions") ?: JSONArray())
		val newSpecs = (transactionObject.optJSONArray("new_specs") ?: JSONArray())
		val verifier = (transactionObject.optJSONArray("verifier") ?: JSONArray())
		action = transactionObject.optJSONObject("action") ?: JSONObject()
		_actionable.value = !action.isNull("type")
		if (action.optString("type") == "sign") {
			signingAuthor = author.getJSONObject(0)
		}
		Log.d("action", action.toString())
		_transaction.value =
			sortCards(
				concatJSONArray(
					warnings,
					error,
					typesInfo,
					method,
					extensions,
					newSpecs,
					verifier
				)
			)
	} catch (e: java.lang.Exception) {
		Log.e("Transaction parsing failed", e.toString())
	}
}

