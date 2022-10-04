package io.parity.signer.screens

import android.content.res.Configuration
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.components.SeedCard
import io.parity.signer.ui.helpers.PreviewData
import io.parity.signer.ui.theme.Bg200
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.uniffi.MSeeds
import io.parity.signer.uniffi.SeedNameCard

//old design screen was called SeedManager todo dmitry update
@Composable
fun KeySetsSelectScreen(
	model: KeySetsSelectViewModel,
) {
	val cards = model.keys

	LazyColumn(
		contentPadding = PaddingValues(horizontal = 12.dp),
		verticalArrangement = Arrangement.spacedBy(10.dp)
	) {
		items(cards.size) { item ->
			Row(
				Modifier
					// .padding(top = 3.dp, start = 12.dp, end = 12.dp)
					.background(MaterialTheme.colors.Bg200)
			) {
				Row(
					Modifier
						.clickable {
//							pushButton( todo dmitry
//								Action.SELECT_SEED,
//								cards[item].seedName
//							)
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

/**
 * Local copy of shared [MSeeds] class
 */
data class KeySetsSelectViewModel(val keys: List<KeySetViewModel>)

fun MSeeds.toKeySetsSelectViewModel() = KeySetsSelectViewModel(
	seedNameCards.map { it.toSeedViewModel() }
)

/**
 * Local copy of shared [SeedNameCard] class
 */
data class KeySetViewModel(
	val seedName: String,
	val identicon: List<UByte>,
	val derivedKeysCount: UInt
)

fun SeedNameCard.toSeedViewModel() =
	KeySetViewModel(seedName, identicon, derivedKeysCount)



@Preview(
	name = "light", group = "general", uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true, backgroundColor = 0xFFFFFFFF,
)
@Preview(
	name = "dark", group = "general",
	uiMode = Configuration.UI_MODE_NIGHT_YES,
	showBackground = true, backgroundColor = 0xFF000000,
)
@Composable
private fun PreviewKeySetsSelectScreen() {
	val mockModel =  KeySetsSelectViewModel(listOf(
		KeySetViewModel("first seed name", PreviewData.exampleIdenticon, 1.toUInt()),
		KeySetViewModel("second seed name", PreviewData.exampleIdenticon, 3.toUInt()),
	))
	SignerNewTheme {
		KeySetsSelectScreen(mockModel)
	}
}
