package io.parity.signer.screens.keysetdetails.backup

import android.content.res.Configuration
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.padding
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.ui.theme.*


@Composable
fun BackupPhraseBox(seedPhrase: String) {

}


@Composable
private fun BackupPhraseItem(index: Int, word: String) {
	Row() {
		Text(
			text = index.toString(),
			color = MaterialTheme.colors.textDisabled,
			style = TypefaceNew.CaptionM,
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
