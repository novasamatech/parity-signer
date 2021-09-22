package io.parity.signer.components

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material.Button
import androidx.compose.material.ButtonDefaults
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.ui.Modifier
import io.parity.signer.models.SignerDataModel
import org.json.JSONObject

@Composable
fun KeySelector(signerDataModel: SignerDataModel) {
	val identities = signerDataModel.identities.observeAsState()
	val selectedIdentity = signerDataModel.selectedIdentity.observeAsState()

	LazyColumn {
		//keys should be defined already, can't panic
		items(identities.value!!.length()) { item ->
			Column {
				Button(
					colors = ButtonDefaults.buttonColors(
						backgroundColor = MaterialTheme.colors.secondary,
						contentColor = MaterialTheme.colors.onSecondary,
					),
					onClick = {
						if (identities.value!!.getJSONObject(item) == selectedIdentity.value) {
							signerDataModel.selectKey(JSONObject())
						} else {
							signerDataModel.selectKey(identities.value!!.getJSONObject(item))
						}
					}
				) {
					KeyCard(
						identities.value!!.getJSONObject(item),
						signerDataModel = signerDataModel
					)
				}
				if (identities.value!!.getJSONObject(item) == selectedIdentity.value) {
					Row(
						horizontalArrangement = Arrangement.SpaceBetween,
						modifier = Modifier.fillMaxWidth()
					) {
						Button(
							colors = ButtonDefaults.buttonColors(
								backgroundColor = MaterialTheme.colors.secondary,
								contentColor = MaterialTheme.colors.onSecondary,
							),
							onClick = { signerDataModel.deleteKeyConfirmation() }
						) { Text("delete") }
						Button(
							colors = ButtonDefaults.buttonColors(
								backgroundColor = MaterialTheme.colors.secondary,
								contentColor = MaterialTheme.colors.onSecondary,
							),
							onClick = {
								signerDataModel.exportPublicKeyEngage()
							}
						) { Text("export") }
						Button(
							colors = ButtonDefaults.buttonColors(
								backgroundColor = MaterialTheme.colors.secondary,
								contentColor = MaterialTheme.colors.onSecondary,
							),
							onClick = { signerDataModel.newKeyScreenEngage() /*TODO*/ }
						) { Text("N+1") }
						Button(
							colors = ButtonDefaults.buttonColors(
								backgroundColor = MaterialTheme.colors.secondary,
								contentColor = MaterialTheme.colors.onSecondary,
							),
							onClick = { signerDataModel.newKeyScreenEngage() }
						) { Text("new") }
					}
					//TODO: Relevant history
				}
			}
		}
	}
}
