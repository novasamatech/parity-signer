package io.parity.signer.components

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.lazy.LazyRow
import androidx.compose.foundation.lazy.items
import androidx.compose.runtime.Composable
import androidx.compose.ui.unit.dp
import io.parity.signer.uniffi.Action

@Composable
fun RestoreSeedPhraseSuggest(
	guessWord: List<String>,
	button: (action: Action, details: String) -> Unit,
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
				button(Action.PUSH_WORD, word)
			})
		}
	}
}
