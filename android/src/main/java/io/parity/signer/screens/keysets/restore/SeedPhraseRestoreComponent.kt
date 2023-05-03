package io.parity.signer.screens.keysets.restore

import android.content.res.Configuration
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.defaultMinSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.text.BasicTextField
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import com.google.accompanist.flowlayout.FlowMainAxisAlignment
import com.google.accompanist.flowlayout.FlowRow
import com.google.accompanist.flowlayout.SizeMode
import io.parity.signer.R
import io.parity.signer.domain.DisableScreenshots
import io.parity.signer.domain.KeepScreenOn
import io.parity.signer.screens.keysetdetails.backup.PhraseNumberStyle
import io.parity.signer.screens.keysetdetails.backup.PhraseWordStyle
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.fill6
import io.parity.signer.ui.theme.textDisabled
import io.parity.signer.ui.theme.textSecondary


@Composable
fun SeedPhraseRestoreComponent(phrase: String) {
	val innerRound = dimensionResource(id = R.dimen.innerFramesCornerRadius)
	val innerShape =
		RoundedCornerShape(innerRound, innerRound, innerRound, innerRound)
	FlowRow(
		mainAxisSize = SizeMode.Expand,
		mainAxisAlignment = FlowMainAxisAlignment.SpaceBetween,
		crossAxisSpacing = 4.dp,
		modifier = Modifier
			.padding(horizontal = 16.dp)
			.padding(top = 8.dp, bottom = 16.dp)
			.background(MaterialTheme.colors.fill6, innerShape)
			.padding(16.dp),
	) {

		val editedWord = phrase.substringAfterLast(' ', "")
		val words =
			phrase.split(' ').filter() { it.isNotEmpty() }//todo fix last word?

		for (index in 0..words.lastIndex) {
			SeedPhraseRestoreWord(index = index + 1, word = words[index])
		}
		BasicTextField(
			value = editedWord,
			onValueChange = {},//todo dmitry
		)
	}

	DisableScreenshots()
	KeepScreenOn()
}


@Composable
private fun SeedPhraseRestoreWord(index: Int, word: String) {
	Row(
		Modifier
			.background(MaterialTheme.colors.fill6, RoundedCornerShape(16.dp))
			.defaultMinSize(minWidth = 100.dp, minHeight = 24.dp)
			.padding(vertical = 8.dp, horizontal = 12.dp)
	) {
		Text(
			text = index.toString(),
			color = MaterialTheme.colors.textDisabled,
			style = PhraseNumberStyle,
			textAlign = TextAlign.End,
		)
		Spacer(Modifier.padding(start = 6.dp))
		Text(
			text = word,
			color = MaterialTheme.colors.textSecondary,
			style = PhraseWordStyle,
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
private fun PreviewSeedPhraseRestoreComponentFinished() {
	SignerNewTheme {
		SeedPhraseRestoreComponent("some workds used for secret veryverylong special long text here to see how printed ")
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
private fun PreviewSeedPhraseRestoreComponentInProgress() {
	SignerNewTheme {
		SeedPhraseRestoreComponent("some workds used for secret veryverylong special long text here to see how printed")
	}
}
