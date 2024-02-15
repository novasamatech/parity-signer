package io.parity.signer.screens.keysetdetails.backup

import android.content.res.Configuration
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.ExperimentalLayoutApi
import androidx.compose.foundation.layout.FlowRow
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.defaultMinSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.text.font.Font
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import io.parity.signer.R
import io.parity.signer.domain.KeepScreenOn
import io.parity.signer.domain.DisableScreenshots
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.fill6
import io.parity.signer.ui.theme.textDisabled
import io.parity.signer.ui.theme.textSecondary

private val RobotoFontFamily = FontFamily(
	Font(R.font.robotomono_regular, FontWeight.Medium),
	Font(R.font.robotomono_bold, FontWeight.Bold),
)
internal val PhraseNumberStyle: TextStyle = TextStyle(
	fontFamily = RobotoFontFamily,
	fontWeight = FontWeight.Normal,
	fontSize = 13.sp
)
internal val PhraseWordStyle: TextStyle = TextStyle(
	fontFamily = RobotoFontFamily,
	fontWeight = FontWeight.Bold,
	fontSize = 13.sp
)

@OptIn(ExperimentalLayoutApi::class)
@Composable
fun SeedPhraseBox(seedPhrase: String) {
	val innerRound = dimensionResource(id = R.dimen.innerFramesCornerRadius)
	val innerShape =
		RoundedCornerShape(innerRound, innerRound, innerRound, innerRound)
	FlowRow(
		modifier = Modifier
			.padding(horizontal = 16.dp)
			.padding(top = 8.dp, bottom = 16.dp)
			.background(MaterialTheme.colors.fill6, innerShape)
			.padding(16.dp),
	) {
		val words = seedPhrase.split(" ")
		for (index in 0..words.lastIndex) {
			SeedPhraseItem(index = index + 1, word = words[index])
		}
	}

	DisableScreenshots()
	KeepScreenOn()
}


@Composable
private fun SeedPhraseItem(index: Int, word: String) {
	Row(Modifier
		.defaultMinSize(minWidth = 100.dp, minHeight = 24.dp)
		.padding(vertical = 2.dp)
	) {
		Text(
			text = index.toString(),
			color = MaterialTheme.colors.textDisabled,
			style = PhraseNumberStyle,
			textAlign = TextAlign.End,
			modifier = Modifier.defaultMinSize(minWidth = 16.dp)
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
private fun PreviewSeedPhraseBox() {
	SignerNewTheme {
		SeedPhraseBox("some workds used for secret veryverylong special long text here to see how printed")
	}
}
