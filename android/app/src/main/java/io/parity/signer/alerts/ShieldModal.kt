package io.parity.signer.modals

import androidx.compose.material.*
import androidx.compose.runtime.Composable
import androidx.compose.runtime.livedata.observeAsState
import io.parity.signer.ButtonID
import io.parity.signer.ShieldAlert
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.pushButton

@Composable
fun ShieldModal(signerDataModel: SignerDataModel) {
	val alert = signerDataModel.alert.observeAsState()
	when (alert.value) {
		 ShieldAlert.None -> {
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
						Text("Ok")
					}
				},
				title = { Text("Signer is safe to use") },
				text = { Text("Safe to use") }
			)
		}
		ShieldAlert.Active -> {
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
				title = { Text("Signer is online!") },
				text = { Text("Turn off network") }
			)
		}
		ShieldAlert.Past -> {
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
					Button(
						colors = ButtonDefaults.buttonColors(
							backgroundColor = MaterialTheme.colors.background,
							contentColor = MaterialTheme.colors.onBackground,
						),
						onClick = {
							signerDataModel.pushButton(ButtonID.GoBack)
						}
					) {
						Text("Acknowledge")
					}
				},
				title = { Text("Network was connected") },
				text = { Text("Not safe to use") }
			)
		}
	}
}
