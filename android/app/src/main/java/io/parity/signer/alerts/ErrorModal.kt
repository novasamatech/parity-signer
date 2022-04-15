package io.parity.signer.alerts

import androidx.compose.runtime.Composable
import io.parity.signer.components.AlertComponent
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.pushButton
import uniffi.signer.Action

@Composable
fun ErrorModal(error: String, signerDataModel: SignerDataModel) {
	AlertComponent(
		show = true,
		header = "Error!",
		text = error,
		back = { signerDataModel.pushButton(Action.GO_BACK) },
		forward = {  },
		backText = "Dismiss",
		showForward = false
	)
}
