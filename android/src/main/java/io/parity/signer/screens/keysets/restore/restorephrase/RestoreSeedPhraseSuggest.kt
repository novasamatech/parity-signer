package io.parity.signer.screens.keysets.restore.restorephrase

import android.content.res.Configuration
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyRow
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.compositeOver
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.backgroundDanger
import io.parity.signer.ui.theme.fill12
import io.parity.signer.ui.theme.pink300

@Composable
fun RestoreSeedPhraseSuggest(
	guessWord: List<String>,
	onClicked: (word: String) -> Unit,
) {
	LazyRow(
		contentPadding = PaddingValues(horizontal = 8.dp),
		horizontalArrangement = Arrangement.spacedBy(4.dp)
	) {
		this.items(
			items = guessWord,
			key = {
				it
			}
		) { word ->
			SeedPhraseButton(word = word, select = { onClicked(word) })
		}
	}
}

/**
 * Suggest buttons for seed phrase recovery screen
 */
@Composable
private fun SeedPhraseButton(word: String, select: () -> Unit) {
	Surface(
		shape = RoundedCornerShape(16.dp),
		color = MaterialTheme.colors.backgroundDanger.compositeOver(
			MaterialTheme.colors.fill12
		),
		modifier = Modifier.clickable(onClick = select)
	) {
		Text(
			text = word,
			style = SignerTypeface.LabelS,
			color = MaterialTheme.colors.pink300,
			modifier = Modifier.padding(horizontal = 12.dp, vertical = 8.dp)
		)
	}
}


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
private fun PreviewRestoreSeedPhraseSuggest() {
	SignerNewTheme {
		RestoreSeedPhraseSuggest(listOf("word1", "word2", "word3"), {})
	}
}
