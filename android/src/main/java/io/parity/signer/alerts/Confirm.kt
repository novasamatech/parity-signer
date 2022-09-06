package io.parity.signer.alerts

import androidx.compose.runtime.Composable
import io.parity.signer.components.AlertComponent
import io.parity.signer.uniffi.Action

/**
 * Confirmation alert called from backend navigation
 */
@Composable
fun Confirm(button: (Action) -> Unit) {
	AlertComponent(
		show = true,
		back = { button(Action.GO_BACK) },
		forward = { button(Action.GO_FORWARD) }
	)
}
