package io.parity.signer.screens

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import io.parity.signer.components.NetworkCard
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.pushButton
import org.json.JSONArray
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.MManageNetworks

@Composable
fun ManageNetworks(
	manageNetworks: MManageNetworks,
	button: (Action, String) -> Unit
) {
	val networks = manageNetworks.networks
	LazyColumn(
		contentPadding = PaddingValues(horizontal = 8.dp),
		verticalArrangement = Arrangement.spacedBy(10.dp)
	) {
		items(networks.size) { index ->
			val thisNetwork = networks[index]
			Row(Modifier.clickable {
				button(
					Action.GO_FORWARD,
					thisNetwork.key
				)
			}) {
				/* TODO: MNetwork -> MDeriveKey
				NetworkCard(deriveKey = thisNetwork)
				*/
			}
		}
	}
}
