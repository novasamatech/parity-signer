package io.parity.signer.alerts

import androidx.compose.material.*
import androidx.compose.runtime.Composable
import androidx.compose.runtime.MutableState
import io.parity.signer.ButtonID
import io.parity.signer.components.AlertComponent
import io.parity.signer.models.pushButton

/**
 * Unified alert that is called by Android
 * (navigation has no idea about it).
 *
 * TODO: completely replace by navigation-driven one
 */
@Composable
fun AndroidCalledConfirm(
	show: Boolean,
	header: String = "Are you sure?",
	text: String? = null, //null is preferred for UX
	back: () -> Unit,
	forward: () -> Unit,
	backText: String = "Cancel",
	forwardText: String = "Confirm"
) {
	AlertComponent(show, header, text, back, forward, backText, forwardText)
}
