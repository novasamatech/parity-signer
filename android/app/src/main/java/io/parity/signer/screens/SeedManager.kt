package io.parity.signer.screens

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import io.parity.signer.components.SeedCard
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.pushButton
import io.parity.signer.ui.theme.Bg200
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.MSeeds

@Composable
fun SeedManager(seed: MSeeds, signerDataModel: SignerDataModel) {
	val cards = seed.seedNameCards

	LazyColumn(
		contentPadding = PaddingValues(horizontal = 12.dp),
		verticalArrangement = Arrangement.spacedBy(10.dp)
	) {
		items(cards.size) { item ->
			Row(
				Modifier
					//.padding(top = 3.dp, start = 12.dp, end = 12.dp)
					.background(MaterialTheme.colors.Bg200)
			) {
				Row(
					Modifier
						.clickable {
							signerDataModel.pushButton(
								Action.SELECT_SEED,
								details = cards[item].seedName
							)
						}
						.weight(1f, true)
				) {
					SeedCard(
						seedName = cards[item].seedName,
						identicon = cards[item].identicon,
					)
				}
			}
		}
	}
}
