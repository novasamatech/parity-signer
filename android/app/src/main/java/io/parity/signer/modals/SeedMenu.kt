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
import io.parity.signer.models.removeSeed
import io.parity.signer.ui.theme.Bg000
import io.parity.signer.ui.theme.modal

@Composable
fun SeedMenu(signerDataModel: SignerDataModel) {
	Column {
		Spacer(Modifier.weight(1f))
		Surface(
			color = MaterialTheme.colors.Bg000,
			shape = MaterialTheme.shapes.modal
		) {
			Column(
				modifier = Modifier.padding(20.dp)
			) {
				HeaderBar(line1 = "SEED MENU", line2 = "Select action")
				BigButton(
					text = "Backup",
					action = { signerDataModel.pushButton(ButtonID.BackupSeed) })
				BigButton(
					text = "Derive new key",
					action = { signerDataModel.pushButton(ButtonID.NewKey) },
					isShaded = true,
					isCrypto = true
				)
				BigButton(
					text = "Forget this seed forever",
					isShaded = true,
					isDangerous = true,
					action = {
						val seedName =
							signerDataModel.modalData.value?.optString("seed") ?: ""
						signerDataModel.removeSeed(seedName)
					}
				)
			}
		}
	}
}
