package io.parity.signer.modals

import android.widget.ImageButton
import androidx.compose.foundation.Image
import androidx.compose.ui.graphics.Color
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material.Icon
import androidx.compose.material.Surface
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Check
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import io.parity.signer.ButtonID
import io.parity.signer.components.HeaderBar
import io.parity.signer.components.NetworkCard
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.pushButton
import io.parity.signer.ui.theme.Action400
import io.parity.signer.ui.theme.Bg000
import io.parity.signer.ui.theme.Bg200
import org.json.JSONArray

@Composable
fun NetworkSelector(signerDataModel: SignerDataModel) {
	val modalData = signerDataModel.modalData.value!!
	val networks = signerDataModel.modalData.value?.optJSONArray("networks") ?: JSONArray()
	Surface(
		color = Color(0x00000000)
	) {
		Column {
			Spacer(Modifier.weight(2f).clickable { signerDataModel.pushButton(ButtonID.GoBack) })
			Column(
				Modifier.weight(1f).background(Bg000)
			) {
				HeaderBar(line1 = "NETWORK", line2 = "Select network")
				LazyColumn {
					items(networks.length()) {item ->
						Row(Modifier.clickable {
							signerDataModel.pushButton(ButtonID.ChangeNetwork, networks.getJSONObject(item).optString("key"))
						}) {
							NetworkCard(network = networks.getJSONObject(item))
							Spacer(Modifier.weight(1f))
							if (networks.getJSONObject(item).optBoolean("selected", false)) {
								Icon(Icons.Default.Check, "this network is selected", tint = Action400)
							}
						}
					}
				}
			}
		}
	}
}
