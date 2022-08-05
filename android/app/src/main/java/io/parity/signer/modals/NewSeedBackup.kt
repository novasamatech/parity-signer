package io.parity.signer.modals

import androidx.compose.foundation.layout.*
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import io.parity.signer.components.BigButton
import io.parity.signer.components.CheckboxTemplate
import io.parity.signer.components.HeaderBar
import io.parity.signer.components.SeedBox
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.addSeed
import io.parity.signer.ui.theme.Bg200
import io.parity.signer.ui.theme.modal
import io.parity.signer.uniffi.MNewSeedBackup

@Composable
fun NewSeedBackup(
	newSeedBackup: MNewSeedBackup,
	addSeed: (String, String, Boolean) -> Unit
) {
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
			HeaderBar(
				"BACKUP SEED PHRASE",
				newSeedBackup.seed
			)
			SeedBox(
				seedPhrase = newSeedBackup.seedPhrase
			)
			CheckboxTemplate(
				checked = confirmBackup.value,
				onValueChange = { confirmBackup.value = it },
				text = "I have written down my seed phrase"
			)
			CheckboxTemplate(
				checked = createRoots.value,
				onValueChange = { createRoots.value = it },
				text = "Create seed keys"
			)

			BigButton(
				text = "Next",
				action =
				{
					addSeed(
						newSeedBackup.seed,
						newSeedBackup.seedPhrase,
						createRoots.value
					)
				},
				isDisabled = !confirmBackup.value
			)
			Spacer(Modifier.fillMaxHeight())
		}
	}
}
