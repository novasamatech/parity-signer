package io.parity.signer.alerts

import androidx.compose.runtime.Composable
import androidx.compose.runtime.livedata.observeAsState
import io.parity.signer.ShieldAlert
import io.parity.signer.components.AlertComponent
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.pushButton
import io.parity.signer.uniffi.Action

@Composable
fun ShieldAlert(signerDataModel: SignerDataModel) {
	val alert = signerDataModel.alertState.observeAsState()
	when (alert.value) {
		ShieldAlert.None -> {
			AlertComponent(
				show = true,
				header = "Signer is secure",
				back = { signerDataModel.pushButton(Action.GO_BACK) },
				forward = { },
				backText = "Ok",
				showForward = false
			)
		}
		ShieldAlert.Active -> {
			AlertComponent(
				show = true,
				header = "Network connected!",
				text = "Signer detects currently connected network; please enable airplane mode, disconnect all cables and handle security breach according with your security protocol.",
				back = { signerDataModel.pushButton(Action.GO_BACK) },
				forward = { },
				backText = "Dismiss",
				showForward = false
			)
		}
		ShieldAlert.Past -> {
			AlertComponent(
				show = true,
				header = "Network was connected!",
				text = "Your Signer device has connected to a WiFi, tether or Bluetooth network since your last acknowledgement and should be considered unsafe to use. Please follow your security protocol",
				back = { signerDataModel.pushButton(Action.GO_BACK) },
				forward = {
					signerDataModel.acknowledgeWarning()
					signerDataModel.pushButton(Action.GO_BACK)
				},
				backText = "Back",
				forwardText = "Acknowledge and reset"
			)
		}
		null -> {}
	}
}
