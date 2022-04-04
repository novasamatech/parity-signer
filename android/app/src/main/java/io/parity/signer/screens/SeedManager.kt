package io.parity.signer.screens

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import io.parity.signer.ButtonID
import io.parity.signer.components.SeedCard
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.pushButton
import io.parity.signer.ui.theme.Bg200
import org.json.JSONArray
import org.json.JSONObject

@Composable
fun SeedManager(
	screenData: JSONObject,
	button: (ButtonID, String) -> Unit
) {
	val cards = screenData.getJSONArray("seedNameCards")
		?: JSONArray()

	LazyColumn(
		contentPadding = PaddingValues(horizontal = 12.dp),
		verticalArrangement = Arrangement.spacedBy(10.dp)
	) {
		items(cards.length()) { item ->
			Row(
				Modifier
					//.padding(top = 3.dp, start = 12.dp, end = 12.dp)
					.background(MaterialTheme.colors.Bg200)
			) {
				Row(
					Modifier
						.clickable {
							button(
								ButtonID.SelectSeed,
								cards
									.getJSONObject(item)
									.getString("seed_name")
							)
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
