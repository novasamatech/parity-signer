package io.parity.signer.alerts

import androidx.compose.runtime.Composable
import io.parity.signer.components.AlertComponent
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.ShieldAlert

@Composable
fun ShieldAlert(
	shieldAlert: ShieldAlert?,
	button: (Action) -> Unit,
	acknowledgeWarning: () -> Unit
) {
	when (shieldAlert) {
		null -> {
			AlertComponent(
				show = true,
				header = "Signer is secure",
				back = { button(Action.GO_BACK) },
				forward = { },
				backText = "Ok",
				showForward = false
			)
		}
		ShieldAlert.ACTIVE -> {
			AlertComponent(
				show = true,
				header = "Network connected!",
				text = "Signer detects currently connected network; please enable airplane mode, disconnect all cables and handle security breach according with your security protocol.",
				back = { button(Action.GO_BACK) },
				forward = { },
				backText = "Dismiss",
				showForward = false
			)
		}
		ShieldAlert.PAST -> {
			AlertComponent(
				show = true,
				header = "Network was connected!",
				text = "Your Signer device has connected to a WiFi, tether or Bluetooth network since your last acknowledgement and should be considered unsafe to use. Please follow your security protocol",
				back = { button(Action.GO_BACK) },
				forward = {
					acknowledgeWarning()
					button(Action.GO_BACK)
				},
				backText = "Back",
				forwardText = "Acknowledge and reset"
			)
		}
	}
}
