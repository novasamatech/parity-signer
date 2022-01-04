package io.parity.signer.modals

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.material.Checkbox
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import io.parity.signer.models.SignerDataModel
import io.parity.signer.ui.theme.Crypto400
import io.parity.signer.ui.theme.CryptoTypography

@Composable
fun NewSeedBackup(signerDataModel: SignerDataModel) {
	val confirmBackup = remember { mutableStateOf(false) }
	val createRoots = remember { mutableStateOf(true) }
	Column {
		Text(
			signerDataModel.modalData.value?.optString("seed_phrase") ?: "",
			style = CryptoTypography.body1,
			color = Crypto400
		)
		Row {
			Checkbox(
				checked = confirmBackup.value,
				onCheckedChange = { confirmBackup.value = it })
			Text ("I have written down my seed phrase")
		}
		Row {
			Checkbox(
				checked = createRoots.value,
				onCheckedChange = { createRoots.value = it })
			Text ("Create root keys")
		}
	}
}
