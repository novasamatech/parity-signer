package io.parity.signer.alerts

import androidx.compose.runtime.Composable
import androidx.compose.runtime.State
import io.parity.signer.components.AlertComponent
import io.parity.signer.models.AlertState
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.ShieldAlert

@Composable
fun ShieldAlert(
	// shieldAlert: ShieldAlert, // TODO: use this instead of alertstate
	alertState: State<AlertState?>,
	button: (Action) -> Unit,
	acknowledgeWarning: () -> Unit
) {
	when (alertState.value) {
		AlertState.Active -> {
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
		AlertState.Past -> AlertComponent(
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
		AlertState.None -> AlertComponent(
			show = true,
			header = "Signer is secure",
			back = { button(Action.GO_BACK) },
			forward = { },
			backText = "Ok",
			showForward = false
		)
		else -> {
			AlertComponent(
				show = true,
				header = "Network detector failure",
				text = "Please report this error",
				back = { button(Action.GO_BACK) },
				forward = { },
				backText = "Dismiss",
				showForward = false
			)
		}
	}
}
