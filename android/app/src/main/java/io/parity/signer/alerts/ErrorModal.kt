package io.parity.signer.modals

import androidx.compose.material.*
import androidx.compose.runtime.Composable
import io.parity.signer.ButtonID
import io.parity.signer.components.AlertComponent
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.pushButton

@Composable
fun ErrorModal(error: String, signerDataModel: SignerDataModel) {
	AlertComponent(
		show = true,
		header = "Error!",
		text = error,
		back = { signerDataModel.pushButton(ButtonID.GoBack) },
		forward = {  },
		backText = "Dismiss",
		showForward = false
	)

}
