package io.parity.signer.modals

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.selection.toggleable
import androidx.compose.material.*
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.unit.dp
import io.parity.signer.components.*
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.addSeed
import io.parity.signer.ui.theme.Bg200
import io.parity.signer.ui.theme.modal

@Composable
fun NewSeedBackup(signerDataModel: SignerDataModel) {
	val confirmBackup = remember { mutableStateOf(false) }
	val createRoots = remember { mutableStateOf(true) }
	Surface(
		color = MaterialTheme.colors.Bg200,
		shape = MaterialTheme.shapes.modal
	) {
		Column(
			verticalArrangement = Arrangement.spacedBy(8.dp),
			modifier = Modifier.padding(20.dp)
		) {
			HeaderBar("BACKUP SEED PHRASE", signerDataModel.modalData.value?.optString("seed")?: "")
			SeedBox(
				seedPhrase = signerDataModel.modalData.value?.optString("seed_phrase")
					?: ""
			)
			CheckboxTemplate(
				checked = confirmBackup.value,
				onValueChange = { confirmBackup.value = it },
				text = "I have written down my seed phrase"
			)
			CheckboxTemplate(
				checked = createRoots.value,
				onValueChange = { createRoots.value = it },
				text = "Create root keys"
			)

			BigButton(
				text = "Next",
				action = {
					signerDataModel.modalData.value?.let { modalData ->
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
				},
				isDisabled = !confirmBackup.value
			)
			Spacer(Modifier.fillMaxHeight())
		}
	}
}
