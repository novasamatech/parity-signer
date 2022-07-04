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
import io.parity.signer.uniffi.MLogRight

@Composable
fun LogMenu(
	logRight: MLogRight,
	button: (Action) -> Unit
) {
	val checksum = logRight.checksum
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
				HeaderBar(line1 = "LOG", line2 = "Checksum: $checksum")
				BigButton(
					text = "Add note",
					action = {
						button(Action.CREATE_LOG_COMMENT)
					})
				BigButton(
					text = "Clear log",
					action = { confirm = true },
					isDangerous = true,
					isShaded = true
				)
			}
		}
	}

	AndroidCalledConfirm(
		show = confirm,
		header = "Clear log?",
		back = { confirm = false },
		forward = { button(Action.CLEAR_LOG) })
}
