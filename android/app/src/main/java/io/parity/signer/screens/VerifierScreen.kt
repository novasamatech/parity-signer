package io.parity.signer.screens

import android.app.AlertDialog
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.material.*
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.*
import io.parity.signer.alerts.AndroidCalledConfirm
import io.parity.signer.components.BigButton
import io.parity.signer.components.Identicon
import io.parity.signer.models.SignerDataModel
import org.json.JSONObject

@Composable
fun VerifierScreen(signerDataModel: SignerDataModel) {
	val verifierDetails = signerDataModel.screenData.value ?: JSONObject()
	var jailbreakAlert by remember { mutableStateOf(false) }

	Column {
		Row {
			Identicon(identicon = verifierDetails.optString("identicon"))
			Column {
				Text("General verifier certificate")
				Text(verifierDetails.optString("hex"))
				Text("encryption: " + verifierDetails.optString("encryption"))
			}
		}
		BigButton(
			text = "Remove general certificate",
			action = { jailbreakAlert = true },
			isDangerous = true,
			isShaded = true
		)
	}

	AndroidCalledConfirm(
		show = jailbreakAlert,
		header = "Wipe ALL data?",
		text = "Remove all data and set general verifier blank so that it could be set later. This operation can not be reverted. Do not proceed unless you absolutely know what you are doing, there is no need to use this procedure in most cases. Misusing this feature may lead to loss of funds!",
		back = { jailbreakAlert = false },
		forward = { signerDataModel.jailbreak() },
		backText = "Cancel",
		forwardText = "I understand"
	)
}
