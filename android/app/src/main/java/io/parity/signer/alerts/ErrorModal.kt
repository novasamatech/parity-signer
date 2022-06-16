package io.parity.signer.alerts

import androidx.compose.runtime.Composable
import io.parity.signer.components.AlertComponent
import io.parity.signer.uniffi.Action

@Composable
fun ErrorModal(error: String, button: (Action) -> Unit) {
	AlertComponent(
		show = true,
		header = "Error!",
		text = error,
		back = { button(Action.GO_BACK) },
		forward = { },
		backText = "Dismiss",
		showForward = false
	)
}
