package io.parity.signer.models

import android.util.Log
import android.widget.Toast
import io.parity.signer.*
import org.json.JSONObject

/**
 * This pretty much offloads all navigation to backend!
 */
fun SignerDataModel.pushButton(button: ButtonID) {
	Log.d("push button", button.toString())
	val actionResult =
		backendAction(button.name, "")
	Log.d("action result", actionResult)
	//Here we just list all possible arguments coming from backend
	try {
		val actionResultObject = JSONObject(actionResult)
		actionResultObject.optString("screen").let { screen ->
			_signerScreen.value = SignerScreen.valueOf(screen)
			actionResultObject.getString("screenLabel").let {
				_screenName.value = it
			}
			actionResultObject.getBoolean("back").let {
				_backButton.value = it
			}
		}
		screenInfo = actionResultObject.getJSONObject("content")
	} catch (e: java.lang.Exception) {
		Log.e("Navigation error", e.toString())
		Toast.makeText(context, actionResult, Toast.LENGTH_SHORT).show()
	}
}

/**
 * This happens when backup seed acknowledge button is pressed in seed creation screen.
 * TODO: This might misfire - replace with explicit getter and lifetime bound thing
 */
fun SignerDataModel.acknowledgeBackup() {
	_backupSeedPhrase.value = ""
}
