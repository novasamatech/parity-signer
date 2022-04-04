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
fun Confirm(button: (ButtonID) -> Unit) {

	AlertComponent(
		show = true,
		back = { button(ButtonID.GoBack) },
		forward = { button(ButtonID.GoForward) }
	)

}
