package io.parity.signer.screens.keysetdetails.backup

import android.content.res.Configuration
import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.grid.GridCells
import androidx.compose.foundation.lazy.grid.LazyVerticalGrid
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.ui.theme.*


@Composable
fun BackupPhraseBox(seedPhrase: String) {
	val innerRoun = dimensionResource(id = R.dimen.innerFramesCornerRadius)
	val innerShape =
		RoundedCornerShape(innerRoun, innerRoun, innerRoun, innerRoun)
	LazyVerticalGrid(
		columns = GridCells.Adaptive(128.dp),
		modifier = Modifier
			.padding(8.dp)
			.background(MaterialTheme.colors.fill6, innerShape),
		contentPadding = PaddingValues(
			start = 12.dp,
			top = 12.dp,
			end = 12.dp,
			bottom = 12.dp,
		),
	) {
		val words = seedPhrase.split(" ")
		items(words.size) { index ->
			BackupPhraseItem(index = index + 1, word = words[index])
		}
	}
}


@Composable
private fun BackupPhraseItem(index: Int, word: String) {
	Row() {
		Text(
			text = index.toString(),
			color = MaterialTheme.colors.textDisabled,
			style = TypefaceNew.CaptionM,
			textAlign = TextAlign.End,
			modifier = Modifier.defaultMinSize(minWidth = 16.dp)
		)
		Spacer(Modifier.padding(start = 6.dp))
		Text(
			text = word,
			color = MaterialTheme.colors.textSecondary,
			style = TypefaceNew.CaptionM,
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
private fun PreviewBackupPhraseBox() {
	SignerNewTheme {
		BackupPhraseBox("some workds used for secret special long text here to see how printed")
	}
}
