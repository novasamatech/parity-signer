package io.parity.signer.modals

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.padding
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import io.parity.signer.ButtonID
import io.parity.signer.components.BigButton
import io.parity.signer.components.HeaderBar
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.pushButton
import io.parity.signer.ui.theme.Bg000
import io.parity.signer.ui.theme.modal

@Composable
fun LogMenu(signerDataModel: SignerDataModel) {
	val checksum = signerDataModel.modalData.value?.optString("checksum")

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
				BigButton(text = "Add note",
					action = {
						signerDataModel.pushButton(ButtonID.CreateLogComment)
					})
				BigButton(
					text = "Clear log",
					action = { signerDataModel.pushButton(ButtonID.ClearLog) },
					isDangerous = true,
					isShaded = true
				)
			}
		}
	}
}
