package io.parity.signer.models

import android.util.Log
import android.widget.Toast
import io.parity.signer.*
import org.json.JSONObject
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.backendAction

/**
 * This pretty much offloads all navigation to backend!
 */
fun SignerDataModel.pushButton(
	button: Action,
	details: String = "",
	seedPhrase: String = ""
) {
	Log.w("SIGNER_RUST_LOG", "action $button")
	//Here we just list all possible arguments coming from backend
	try {
		_actionResult.value = backendAction(button, details, seedPhrase)
		_alertState.value = _actionResult.value?.alertData
		Log.w("SIGNER_RUST_LOG", "VALUE ${_actionResult.value}")
	} catch (e: java.lang.Exception) {
		Log.e("Navigation error", e.toString())
		Toast.makeText(context, e.toString(), Toast.LENGTH_SHORT).show()
	}
}
