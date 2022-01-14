package io.parity.signer.modals

import androidx.compose.material.*
import androidx.compose.runtime.Composable
import io.parity.signer.ButtonID
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.pushButton

@Composable
fun ErrorModal(error: String, signerDataModel: SignerDataModel) {
	AlertDialog(
		onDismissRequest = {
			signerDataModel.pushButton(ButtonID.GoBack)
		},
		buttons = {
			Button(
				colors = ButtonDefaults.buttonColors(
					backgroundColor = MaterialTheme.colors.background,
					contentColor = MaterialTheme.colors.onBackground,
				),
				onClick = {
					signerDataModel.pushButton(ButtonID.GoBack)
				}
			) {
				Text("Dismiss")
			}
		},
		title = { Text("Error!") },
		text = { Text(error) }
		//TODO: special message for roots!
	)
}
