package io.parity.signer.modals

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.material.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import io.parity.signer.ButtonID
import io.parity.signer.components.BigButton
import io.parity.signer.components.HeaderBar
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.pushButton
import io.parity.signer.ui.theme.Bg000

@Composable
fun KeyDetailsAction(signerDataModel: SignerDataModel) {
	var confirm by remember { mutableStateOf(false) }

	Column {
		Spacer(Modifier.weight(1f))
		Surface(color = Bg000, shape = MaterialTheme.shapes.large) {
			Column {
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
	if (confirm) {
		AlertDialog(
			onDismissRequest = { confirm = false },
			buttons = {
				Button(onClick = { confirm = false }) { Text("Cancel") }
				Button(onClick = { signerDataModel.pushButton(ButtonID.RemoveKey) }) {
					Text(
						"Remove key"
					)
				}
			},
			title = { Text("Forget this key?") },
			text = { Text("This key will be removed for this network. Are you sure?") }
		)
	}
}
