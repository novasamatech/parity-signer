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
	//Here we just list all possible arguments coming from backend
	try {
		Log.w("AACTION", "$details $button")
		val actionResult = backendAction(button, details, seedPhrase)
		_screenLabel.value = actionResult.screenLabel
		_back.value = actionResult.back
		_footer.value = actionResult.footer
		_footerButton.value = actionResult.footerButton
		_rightButton.value = actionResult.rightButton
		_screenNameType.value = actionResult.screenNameType
		_alert.value = SignerAlert.valueOf(actionResult.alert)
		_screenData.value = actionResult.screenData
		_modalData.value = actionResult.modalData
		_alertData.value = JSONObject(actionResult.alertData)
		Log.d("screen", _screenData.value.toString())
		Log.d("modal", _modalData.value.toString())
	} catch (e: java.lang.Exception) {
		Log.e("Navigation error", e.toString())
		Toast.makeText(context, e.toString(), Toast.LENGTH_SHORT).show()
	}
}
