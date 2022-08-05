package io.parity.signer.modals

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.padding
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import io.parity.signer.alerts.AndroidCalledConfirm
import io.parity.signer.components.BigButton
import io.parity.signer.components.HeaderBar
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.pushButton
import io.parity.signer.ui.theme.Bg000
import io.parity.signer.ui.theme.modal
import io.parity.signer.uniffi.Action

@Composable
fun KeyDetailsAction(
	button: (Action) -> Unit
) {
	var confirm by remember { mutableStateOf(false) }

	Column(
		Modifier.clickable { button(Action.GO_BACK) }
	) {
		Spacer(Modifier.weight(1f))
		Surface(
			color = MaterialTheme.colors.Bg000,
			shape = MaterialTheme.shapes.modal
		) {
			Column(
				modifier = Modifier.padding(20.dp)
			) {
				HeaderBar(line1 = "KEY MENU", line2 = "Select action")
				BigButton(
					text = "Forget this key forever",
					isShaded = true,
					isDangerous = true,
					action = {
						confirm = true
					}
				)
			}
		}
	}
	AndroidCalledConfirm(
		show = confirm,
		header = "Forget this key?",
		text = "This key will be removed for this network. Are you sure?",
		back = { confirm = false },
		forward = { button(Action.REMOVE_KEY) },
		backText = "Cancel",
		forwardText = "Remove key"
	)
}
