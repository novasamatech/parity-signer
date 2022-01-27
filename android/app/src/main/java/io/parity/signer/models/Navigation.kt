package io.parity.signer.models

import android.util.Log
import android.widget.Toast
import io.parity.signer.*
import org.json.JSONObject

/**
 * This pretty much offloads all navigation to backend!
 */
fun SignerDataModel.pushButton(
	button: ButtonID,
	details: String = "",
	seedPhrase: String = ""
) {
	Log.d("push button", button.toString())
	val actionResult =
		backendAction(button.name, details, seedPhrase)
	Log.d("action result", actionResult)
	//Here we just list all possible arguments coming from backend
	try {
		val actionResultObject = JSONObject(actionResult)
		actionResultObject.optString("screen").let { screen ->
			_screen.value = SignerScreen.valueOf(screen)
			actionResultObject.getString("screenLabel").let {
				_screenLabel.value = it
			}
			actionResultObject.getBoolean("back").let {
				_back.value = it
			}
			actionResultObject.getBoolean("footer").let {
				_footer.value = it
			}
			actionResultObject.getString("footerButton").let {
				_footerButton.value = it
			}
			actionResultObject.getString("rightButton").let {
				_rightButton.value = it
			}
			actionResultObject.getString("screenNameType").let {
				_screenNameType.value = it
			}
		}
		_modal.value = SignerModal.valueOf(actionResultObject.getString("modal"))
		_alert.value = SignerAlert.valueOf(actionResultObject.getString("alert"))
		_screenData.value = actionResultObject.getJSONObject("screenData")
		_modalData.value = actionResultObject.getJSONObject("modalData")
		_alertData.value = actionResultObject.getJSONObject("alertData")
	} catch (e: java.lang.Exception) {
		Log.e("Navigation error", e.toString())
		Toast.makeText(context, actionResult, Toast.LENGTH_SHORT).show()
	}
}
