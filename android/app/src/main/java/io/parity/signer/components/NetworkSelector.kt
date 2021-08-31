package io.parity.signer.components

import androidx.compose.material.*
import androidx.compose.runtime.*
import androidx.compose.runtime.livedata.observeAsState
import io.parity.signer.models.SignerDataModel
import io.parity.signer.ui.theme.Typography
import org.json.JSONObject

@Composable
fun NetworkSelector(signerDataModel: SignerDataModel) {

	var selectedNetwork = signerDataModel.selectedNetwork.observeAsState()
	var networks = signerDataModel.networks.observeAsState()

	var expanded by remember { mutableStateOf(false) }

	Button(
		colors = ButtonDefaults.buttonColors(
			backgroundColor = MaterialTheme.colors.background,
			contentColor = MaterialTheme.colors.onBackground,
		),
		onClick = { expanded = true }) {
		Text(selectedNetwork.value!!.get("title").toString())
	}
	DropdownMenu(expanded = expanded, onDismissRequest = { expanded = false }) {
		for (i in 0 .. (networks.value!!.length()-1)) {
			Button (
				colors = ButtonDefaults.buttonColors(
					backgroundColor = MaterialTheme.colors.background,
					contentColor = MaterialTheme.colors.onBackground,
				),
				onClick = {
				signerDataModel.selectNetwork(networks.value!!.getJSONObject(i))
				expanded = false
			}) {Text(networks.value!!.getJSONObject(i).get("title").toString())}
		}
	}
}
