package io.parity.signer.modals

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
import io.parity.signer.ButtonID
import io.parity.signer.components.HeaderBar
import io.parity.signer.components.NetworkCard
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.pushButton
import io.parity.signer.ui.theme.Bg000
import io.parity.signer.ui.theme.modal
import org.json.JSONArray

@Composable
fun NetworkSelector(signerDataModel: SignerDataModel) {
	val networks =
		signerDataModel.modalData.value?.optJSONArray("networks") ?: JSONArray()
	Surface(
		color = Color(0x00000000),
		modifier = Modifier.clickable { signerDataModel.pushButton(ButtonID.GoBack) }
	) {
		Column {
			Spacer(
				Modifier
					.weight(1f)
					)
			Surface(
				shape = MaterialTheme.shapes.modal,
				color = MaterialTheme.colors.Bg000,
				modifier = Modifier.weight(1f)
			) {
				Column(
					modifier = Modifier.padding(horizontal = 20.dp)
				) {
					HeaderBar(
						line1 = "NETWORK",
						line2 = "Select network",
						modifier = Modifier.padding(10.dp)
					)

					LazyColumn(
						contentPadding = PaddingValues(horizontal = 8.dp),
						verticalArrangement = Arrangement.spacedBy(10.dp)
					) {
						items(networks.length()) { item ->
							Row(Modifier.clickable {
								signerDataModel.pushButton(
									ButtonID.ChangeNetwork,
									networks.getJSONObject(item).optString("key")
								)
							}) {
								NetworkCard(
									network = networks.getJSONObject(item),
									selected = networks.getJSONObject(item)
										.optBoolean("selected", false)
								)
							}
						}
					}
				}
			}
		}
	}
}
