package io.parity.signer.modals

import androidx.compose.material.*
import androidx.compose.runtime.Composable
import io.parity.signer.models.SignerDataModel

@Composable
fun KeyDelete(signerDataModel: SignerDataModel) {
	val keyName = signerDataModel.selectedIdentity.value?.get("name").toString()
	val seed = signerDataModel.selectedIdentity.value?.get("seed_name").toString()
	val path = signerDataModel.selectedIdentity.value?.get("path").toString()
	val networkName = signerDataModel.selectedNetwork.value?.get("title").toString()

	AlertDialog(
		onDismissRequest = { signerDataModel.clearKeyManagerScreen() },
		buttons = {
			Button(
				colors = ButtonDefaults.buttonColors(
					backgroundColor = MaterialTheme.colors.background,
					contentColor = MaterialTheme.colors.onBackground,
				),
				onClick = {
					signerDataModel.clearKeyManagerScreen()
				}
			) {
				Text("Cancel")
			}
			Button(
				colors = ButtonDefaults.buttonColors(
					backgroundColor = MaterialTheme.colors.background,
					contentColor = MaterialTheme.colors.onBackground,
				),
				onClick = {
					signerDataModel.deleteKey()
					signerDataModel.clearKeyManagerScreen()
				}
			) {
				Text("Delete")
			}
		},
		title = { Text("Delete key?") },
		text = { Text("Are you sure you want to remove key $keyName with path $seed $path for network $networkName?") }
	//TODO: special message for roots!
	)
}
