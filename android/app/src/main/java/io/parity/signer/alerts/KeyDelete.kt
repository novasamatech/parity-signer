package io.parity.signer.modals

import androidx.compose.material.*
import androidx.compose.runtime.Composable
import io.parity.signer.models.SignerDataModel

@Composable
fun KeyDelete(signerDataModel: SignerDataModel) {

	AlertDialog(
		onDismissRequest = {  },
		buttons = {
			Button(
				colors = ButtonDefaults.buttonColors(
					backgroundColor = MaterialTheme.colors.background,
					contentColor = MaterialTheme.colors.onBackground,
				),
				onClick = {

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
				}
			) {
				Text("Delete")
			}
		},
		title = { Text("Delete key?") },
		text = { Text("Are you sure you want to remove key?") }
	//TODO: special message for roots!
	)
}
