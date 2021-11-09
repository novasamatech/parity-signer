package io.parity.signer.components

import androidx.compose.foundation.gestures.detectTapGestures
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.CheckCircle
import androidx.compose.runtime.Composable
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.unit.dp
import io.parity.signer.models.SignerDataModel
import org.json.JSONObject

@Composable
fun KeySelector(signerDataModel: SignerDataModel) {
	val identities = signerDataModel.identities.observeAsState()
	val selectedIdentity = signerDataModel.selectedIdentity.observeAsState()

	LazyColumn {
		//keys should be defined already, can't panic
		items(identities.value!!.length()) { item ->
			if (identities.value!!.getJSONObject(item) != signerDataModel.getRootIdentity(
					signerDataModel.selectedSeed.value ?: ""
				)
			) {
				Row( verticalAlignment = Alignment.CenterVertically) {
					Row(verticalAlignment = Alignment.CenterVertically, modifier = Modifier.pointerInput(Unit) {
						detectTapGestures(
							onTap = {
								signerDataModel.selectKey(identities.value!!.getJSONObject(item)?: JSONObject())
								signerDataModel.exportPublicKeyEngage()
							},
							onLongPress = {

							}
						)
					}
						.padding(horizontal = 8.dp)
					) {
						KeyCard(
							identities.value!!.getJSONObject(item),
							signerDataModel = signerDataModel
						)
						Spacer(modifier = Modifier.weight(1f, true))
						Icon(Icons.Default.CheckCircle, "Address selected")
					}

				}
			}
		}
	}
}

/*
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
					}


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
					)
 */
