package io.parity.signer.screens

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material.Button
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import io.parity.signer.ButtonID
import io.parity.signer.components.NetworkCard
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.pushButton
import org.json.JSONArray

@Composable
fun ManageNetworks(signerDataModel: SignerDataModel) {
	val networks =
		signerDataModel.screenData.value?.optJSONArray("networks") ?: JSONArray()
	LazyColumn(
		contentPadding = PaddingValues(horizontal = 8.dp),
		verticalArrangement = Arrangement.spacedBy(10.dp)
	) {
		items(networks.length()) { index ->
			val thisNetwork = networks.getJSONObject(index)
			Row(Modifier.clickable {
				signerDataModel.pushButton(
					ButtonID.GoForward,
					details = thisNetwork.optString("key")
				)
			}) {
				NetworkCard(network = thisNetwork)
			}
		}
	}
}
