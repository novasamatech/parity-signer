package io.parity.signer.components

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.lazy.LazyRow
import androidx.compose.foundation.lazy.items
import androidx.compose.runtime.Composable
import androidx.compose.runtime.MutableState
import androidx.compose.ui.unit.dp
import io.parity.signer.ButtonID

@Composable
fun RestoreSeedPhraseSuggest(
	guessWord: List<String>,
	button: (button: ButtonID, details: String) -> Unit,
) {
	LazyRow(
		contentPadding = PaddingValues(horizontal = 8.dp),
		horizontalArrangement = Arrangement.spacedBy(12.dp)
	) {
		this.items(
			items = guessWord,
			key = {
				it
			}
		) { word ->
			SeedPhraseButton(word = word, select = {
				button(ButtonID.PushWord, word)
			})
		}
	}
}
