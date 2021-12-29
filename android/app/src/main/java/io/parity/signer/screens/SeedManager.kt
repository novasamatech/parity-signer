package io.parity.signer.modals

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.runtime.Composable
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import io.parity.signer.ButtonID
import io.parity.signer.components.SeedCard
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.pushButton
import io.parity.signer.ui.theme.Bg200
import org.json.JSONArray

@Composable
fun SeedManager(signerDataModel: SignerDataModel) {
	val cards = signerDataModel.screenData.value?.getJSONArray("seedNameCards") ?: JSONArray()

	LazyColumn {
		//keys should be defined already, can't panic
		items(cards.length()) { item ->
			Row(
				Modifier
					.padding(top = 3.dp, start = 12.dp, end = 12.dp)
					.background(Bg200)
			) {
				Row(
					Modifier
						.clickable {
							signerDataModel.pushButton(ButtonID.SelectSeed, details = cards.getString(item))
						}
						.weight(1f, true)
				) {
					SeedCard(
						seedName = cards.getJSONObject(item).getString("seed_name"),
						identicon = cards.getJSONObject(item).getString("identicon"),
						signerDataModel = signerDataModel
					)
				}
			}
		}
	}
}
