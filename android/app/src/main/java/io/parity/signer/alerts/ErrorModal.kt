package io.parity.signer.alerts

import androidx.compose.runtime.Composable
import io.parity.signer.ButtonID
import io.parity.signer.components.AlertComponent
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.pushButton

@Composable
fun ErrorModal(error: String, button: (ButtonID) -> Unit) {
	AlertComponent(
		show = true,
		header = "Error!",
		text = error,
		back = { button(ButtonID.GoBack) },
		forward = {  },
		backText = "Dismiss",
		showForward = false
	)
}
