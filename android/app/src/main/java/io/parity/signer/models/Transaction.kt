package io.parity.signer.models

import android.util.Log
import androidx.compose.ui.graphics.ImageBitmap
import io.parity.signer.TransactionState
import org.json.JSONArray
import org.json.JSONObject

//MARK: Transaction utils begin

/**
 * Send scanned QR to backend and rearrange cards nicely
 * We should probably simplify this once UI development is done
 */
internal fun SignerDataModel.parseTransaction() {
	_transactionState.value = TransactionState.Parsing
	try {
		val transactionString = substrateParseTransaction(payload, dbName)
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
		_transactionState.value = TransactionState.Preview
	} catch (e: java.lang.Exception) {
		Log.e("Transaction parsing failed", e.toString())
		_transactionState.value = TransactionState.None
	}
}

fun SignerDataModel.acceptTransaction() {
	if (action.getString("type") == "sign") {
		Log.d("authorcard", signingAuthor.toString())
		if (signingAuthor.getJSONObject("payload").getBoolean("has_password")) {
			_transactionState.value = TransactionState.Password
		} else {
			signTransaction("")
		}
	} else {
		performTransaction()
		clearTransaction()
	}
}

internal fun SignerDataModel.signTransaction(password: String) {
	authentication.authenticate(activity) {
		signature = substrateHandleSign(
			action.getString(
				"payload"), sharedPreferences.getString(
				signingAuthor.getJSONObject("payload").getString("seed"), ""
			) ?: "", password, "", dbName
		)
		_transactionState.value = TransactionState.Signed
	}
}

fun SignerDataModel.getSignedQR(): ImageBitmap {
	return signature.intoImageBitmap()
}

fun SignerDataModel.performTransaction() {
	try {
		substrateHandleStub(
			action.getString("payload"),
			dbName
		)
	} catch (e: java.lang.Exception) {
		Log.e("transaction failed", e.toString())
		_lastError.value = e.toString()
	}
}

/**
 * Clear all transaction progress side effects
 */
fun SignerDataModel.clearTransaction() {
	signature = ""
	action = JSONObject()
	signingAuthor = JSONObject()
	_transaction.value = JSONArray()
	_actionable.value = false
	_transactionState.value = TransactionState.None
	resetScan()
}

//MARK: Transaction utils end
