package io.parity.signer.modals

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.selection.toggleable
import androidx.compose.material.Checkbox
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.geometry.RoundRect
import androidx.compose.ui.graphics.RectangleShape
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.unit.dp
import io.parity.signer.ButtonID
import io.parity.signer.components.BigButton
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.addSeed
import io.parity.signer.models.pushButton
import io.parity.signer.ui.theme.Action600
import io.parity.signer.ui.theme.Bg200
import io.parity.signer.ui.theme.Crypto400
import io.parity.signer.ui.theme.CryptoTypography

@Composable
fun NewSeedBackup(signerDataModel: SignerDataModel) {
	val confirmBackup = remember { mutableStateOf(false) }
	val createRoots = remember { mutableStateOf(true) }
	Surface(
		color = MaterialTheme.colors.Bg200,
		shape = MaterialTheme.shapes.large
	) {
		Column(Modifier.padding(horizontal = 8.dp)) {
			Text(
				signerDataModel.modalData.value?.optString("seed_phrase") ?: "",
				style = CryptoTypography.body1,
				color = MaterialTheme.colors.Crypto400
			)
			Row(Modifier.toggleable(
				value = confirmBackup.value,
				role = Role.Checkbox,
				onValueChange = { confirmBackup.value = it }
			)) {
				Checkbox(
					checked = confirmBackup.value,
					onCheckedChange = null)
				Text("I have written down my seed phrase")
			}
			Row(Modifier.toggleable(
				value = createRoots.value,
				role = Role.Checkbox,
				onValueChange = { createRoots.value = it }
			)) {
				Checkbox(
					checked = createRoots.value,
					onCheckedChange = { createRoots.value = it })
				Text("Create root keys")
			}
			BigButton(
				text = "Next",
				action = {
					signerDataModel.modalData.value?.let {modalData ->
						modalData.optString("seed").let { seedName ->
							modalData.optString("seed_phrase")
								.let { seedPhrase ->
									signerDataModel.addSeed(
										seedName = seedName,
										seedPhrase = seedPhrase,
										createRoots = createRoots.value
									)
								}
						}
					}
				}
			)
			Spacer(Modifier.fillMaxHeight())
		}
	}
}
