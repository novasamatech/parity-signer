package io.parity.signer.components

import androidx.compose.material.Button
import androidx.compose.material.DropdownMenu
import androidx.compose.material.Text
import androidx.compose.runtime.*
import androidx.compose.runtime.livedata.observeAsState
import io.parity.signer.models.SignerDataModel
import org.json.JSONObject

@Composable
fun NetworkSelector(signerDataModel: SignerDataModel) {

	var selectedNetwork = signerDataModel.selectedNetwork.observeAsState()
	var networks = signerDataModel.networks.observeAsState()

	var expanded by remember { mutableStateOf(false) }

	Button(onClick = { expanded = true }) {
		Text(selectedNetwork.value!!.get("title").toString())
	}
	DropdownMenu(expanded = expanded, onDismissRequest = { expanded = false }) {
		for (i in 0 .. (networks.value!!.length()-1)) {
			Button (onClick = {
				signerDataModel.selectNetwork(networks.value!!.getJSONObject(i))
				expanded = false
			}) {Text(networks.value!!.getJSONObject(i).get("title").toString())}
		}
	}
}
