package io.parity.signer.modals

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import io.parity.signer.ButtonID
import io.parity.signer.components.SeedCard
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.getSeed
import io.parity.signer.models.pushButton
import io.parity.signer.ui.theme.Bg100
import io.parity.signer.ui.theme.Bg200
import io.parity.signer.ui.theme.modal
import org.json.JSONArray

@Composable
fun SelectSeed(signerDataModel: SignerDataModel) {
	val cards = signerDataModel.screenData.value?.getJSONArray("seedNameCards")
		?: JSONArray()

	Surface(
		color = MaterialTheme.colors.Bg100,
		shape = MaterialTheme.shapes.modal
	) {
		LazyColumn(
			modifier = Modifier.padding(20.dp)
		) {
			items(cards.length()) { item ->
				Row(
					Modifier
						.padding(top = 3.dp, start = 12.dp, end = 12.dp)
						.background(MaterialTheme.colors.Bg200)
				) {
					Row(
						Modifier
							.clickable {
								signerDataModel.authentication.authenticate(signerDataModel.activity) {
									val seedName =
										cards
											.getJSONObject(item)
											.optString("seed_name")
									val seedPhrase = signerDataModel.getSeed(seedName)
									if (seedPhrase.isNotBlank()) {
										signerDataModel.pushButton(
											ButtonID.GoForward,
											seedName,
											seedPhrase
										)
									}
								}
							}
							.weight(1f, true)
					) {
						SeedCard(
							seedName = cards.getJSONObject(item).getString("seed_name"),
							identicon = cards.getJSONObject(item).getString("identicon")
						)
					}
				}
			}
		}
	}
}
