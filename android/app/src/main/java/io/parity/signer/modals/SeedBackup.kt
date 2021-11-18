package io.parity.signer.modals

import androidx.compose.foundation.layout.Column
import androidx.compose.material.ButtonDefaults
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.livedata.observeAsState
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.acknowledgeBackup

/**
 * Modal to show seed phrase. Dangerous place.
 * TODO: make sure seed phrase is cleared at all times!!!
 */
@Composable
fun SeedBackup(signerDataModel: SignerDataModel) {
	val selectedSeed = signerDataModel.selectedSeed.observeAsState()
	val backupSeedPhrase = signerDataModel.backupSeedPhrase.observeAsState()

	Column {
		Text("Please back up seed phrase")
		Text(selectedSeed.value?:"")
		Text(backupSeedPhrase.value?:"")
		TextButton(
			colors = ButtonDefaults.buttonColors(
				backgroundColor = MaterialTheme.colors.background,
				contentColor = MaterialTheme.colors.onBackground
			),
			onClick = { signerDataModel.acknowledgeBackup() }
		) {
			Text("Done")
		}
	}
}
