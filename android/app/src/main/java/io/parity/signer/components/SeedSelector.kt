package io.parity.signer.components

import androidx.compose.material.*
import androidx.compose.runtime.*
import androidx.compose.runtime.livedata.observeAsState
import io.parity.signer.models.SignerDataModel

@Composable
fun SeedSelector(signerDataModel: SignerDataModel) {
	var selectedSeed = signerDataModel.selectedSeed.observeAsState()
	var seedNames = signerDataModel.seedNames.observeAsState()

	var expanded by remember { mutableStateOf(false) }

	Button(
		colors = ButtonDefaults.buttonColors(
			backgroundColor = MaterialTheme.colors.background,
			contentColor = MaterialTheme.colors.onBackground,
		),
		onClick = { expanded = true }) {
		Text(if (selectedSeed.value?.isEmpty() as Boolean) "Select seed" else selectedSeed.value!!)
	}
	DropdownMenu(expanded = expanded, onDismissRequest = { expanded = false }) {
		//NPE: this should only fail before lateInit of signerDataModel which is impossible
		for (seedName in seedNames.value?: arrayOf<String>()) {
			Button (
				colors = ButtonDefaults.buttonColors(
					backgroundColor = MaterialTheme.colors.background,
					contentColor = MaterialTheme.colors.onBackground,
				),
				onClick = {
					signerDataModel.selectSeed(seedName)
					expanded = false
				}) { Text(seedName) }
		}
		Button (
			colors = ButtonDefaults.buttonColors(
				backgroundColor = MaterialTheme.colors.background,
				contentColor = MaterialTheme.colors.onBackground,
			),
			onClick = {
				signerDataModel.selectSeed("")
				expanded = false
			}) { Text("Show all") }
	}
}
