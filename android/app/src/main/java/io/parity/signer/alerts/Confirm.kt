package io.parity.signer.alerts

import androidx.compose.runtime.Composable
import io.parity.signer.components.AlertComponent
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.pushButton
import uniffi.signer.Action

/**
 * Confirmation alert called from backend navigation
 */
@Composable
fun Confirm(signerDataModel: SignerDataModel) {

	AlertComponent(
		show = true,
		back = { signerDataModel.pushButton(Action.GO_BACK) },
		forward = { signerDataModel.pushButton(Action.GO_FORWARD) }
	)

}
