package io.parity.signer.alerts

import androidx.compose.runtime.Composable
import io.parity.signer.ButtonID
import io.parity.signer.components.AlertComponent
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.pushButton

/**
 * Confirmation alert called from backend navigation
 */
@Composable
fun Confirm(signerDataModel: SignerDataModel) {

	AlertComponent(
		show = true,
		back = { signerDataModel.pushButton(ButtonID.GoBack) },
		forward = { signerDataModel.pushButton(ButtonID.GoForward) }
	)

}
