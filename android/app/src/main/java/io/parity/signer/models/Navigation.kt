package io.parity.signer.models

import android.util.Log
import android.widget.Toast
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
	try {
		_actionResult.value = backendAction(button, details, seedPhrase)
	} catch (e: java.lang.Exception) {
		Log.e("Navigation error", e.toString())
		Toast.makeText(context, e.toString(), Toast.LENGTH_SHORT).show()
	}
}
