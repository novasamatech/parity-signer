package io.parity.signer.modals

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
fun NetworkDetailsMenu(button: (Action) -> Unit) {
	var confirm by remember { mutableStateOf(false) }

	Column {
		Spacer(Modifier.weight(1f))
		Surface(
			color = MaterialTheme.colors.Bg000,
			shape = MaterialTheme.shapes.modal
		) {
			Column(
				modifier = Modifier.padding(20.dp)
			) {
				HeaderBar(line1 = "MANAGE NETWORK", line2 = "Select action")
				BigButton(
					text = "Sign network specs",
					isShaded = true,
					isCrypto = true,
					action = { button(Action.SIGN_NETWORK_SPECS) })
				BigButton(
					text = "Delete network",
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
		header = "Remove network?",
		text = "This network will be removed for whole device",
		back = { confirm = false },
		forward = { button(Action.REMOVE_NETWORK) },
		backText = "Cancel",
		forwardText = "Remove network"
	)
}
