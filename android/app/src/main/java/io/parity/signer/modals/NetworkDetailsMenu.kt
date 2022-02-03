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
import io.parity.signer.models.removeSeed
import io.parity.signer.ui.theme.Bg000

@Composable
fun NetworkDetailsMenu(signerDataModel: SignerDataModel) {
	var confirm by remember { mutableStateOf(false) }

	Column {
		Spacer(Modifier.weight(1f))
		Surface(color = MaterialTheme.colors.Bg000, shape = MaterialTheme.shapes.large) {
			Column {
				HeaderBar(line1 = "MANAGE NETWORK", line2 = "Select action")
				BigButton(
					text = "Sign network specs",
					isShaded = true,
					isCrypto = true,
					action = { signerDataModel.pushButton(ButtonID.SignNetworkSpecs) })
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
	if (confirm) {
		AlertDialog(
			onDismissRequest = { confirm = false },
			buttons = {
				Button(onClick = { confirm = false }) { Text("Cancel") }
				Button(onClick = { signerDataModel.pushButton(ButtonID.RemoveNetwork) }) {
					Text(
						"Remove network"
					)
				}
			},
			title = { Text("Remove network?") },
			text = { Text("This network will be removed for whole device") }
		)
	}
}
