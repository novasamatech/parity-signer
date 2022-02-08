package io.parity.signer.components

import android.util.Log
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.lazy.LazyRow
import androidx.compose.foundation.lazy.LazyListScope
import androidx.compose.foundation.lazy.items
import androidx.compose.runtime.Composable
import androidx.compose.runtime.MutableState
import androidx.compose.ui.unit.dp

@Composable
fun RestoreSeedPhraseSuggest(guessWord: MutableState<List<String>>, push: (word: String) -> Unit) {

	LazyRow(
		contentPadding = PaddingValues(horizontal = 8.dp),
		horizontalArrangement = Arrangement.spacedBy(12.dp)
	) {
		this.items(
			items = guessWord.value,
			key = {
				it
			}
		) { word ->
			SeedPhraseButton(word = word, select = {
				push(word)
			})
		}
	}
}
