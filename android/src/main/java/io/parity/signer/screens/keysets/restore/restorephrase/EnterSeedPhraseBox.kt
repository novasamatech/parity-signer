package io.parity.signer.screens.keysets.restore.restorephrase

import android.content.res.Configuration
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.IntrinsicSize
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.defaultMinSize
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.text.BasicTextField
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.focus.focusRequester
import androidx.compose.ui.graphics.SolidColor
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.TextRange
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.text.input.TextFieldValue
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import com.google.accompanist.flowlayout.FlowMainAxisAlignment
import com.google.accompanist.flowlayout.FlowRow
import com.google.accompanist.flowlayout.SizeMode
import io.parity.signer.R
import io.parity.signer.components.base.ScanIconPlain
import io.parity.signer.domain.Callback
import io.parity.signer.domain.DisableScreenshots
import io.parity.signer.domain.KeepScreenOn
import io.parity.signer.domain.conditional
import io.parity.signer.screens.keysetdetails.backup.PhraseNumberStyle
import io.parity.signer.screens.keysetdetails.backup.PhraseWordStyle
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.fill6
import io.parity.signer.ui.theme.textDisabled
import io.parity.signer.ui.theme.textTertiary


@Composable
fun EnterSeedPhraseBox(
	enteredWords: List<String>,
	rawUserInput: String,
	modifier: Modifier = Modifier,
	onEnteredChange: (progressWord: String) -> Unit,
	onScanOpen: Callback,
) {
	val innerRound = dimensionResource(id = R.dimen.innerFramesCornerRadius)
	val innerShape = RoundedCornerShape(innerRound)

	val focusRequester = remember { FocusRequester() }

	//workaround for //https://issuetracker.google.com/issues/160257648 and https://issuetracker.google.com/issues/235576056 - update to new TextField
	//for now need to keep this intermediate state
	val seedWord = remember { mutableStateOf(TextFieldValue(" ")) }
	seedWord.value = TextFieldValue(
		text = rawUserInput,
		//to always keep position after artificially added " "
		selection = TextRange(rawUserInput.length)
	)

	FlowRow(
		mainAxisSize = SizeMode.Expand,
		mainAxisSpacing = 4.dp,
		mainAxisAlignment = FlowMainAxisAlignment.Start,
		crossAxisSpacing = 4.dp,
		modifier = modifier
			.background(MaterialTheme.colors.fill6, innerShape)
			.defaultMinSize(minHeight = 156.dp)
			.padding(8.dp),
	) {
		enteredWords.onEachIndexed { index, word ->
			EnterSeedPhraseWord(index = index + 1, word = word)
		}
		val shouldShowPlaceholder = enteredWords.isEmpty() && rawUserInput.isEmpty()
		BasicTextField(
			textStyle = TextStyle(color = MaterialTheme.colors.primary),
			value = seedWord.value, //as was before redesign, should been moved to rust but need to align with iOS
			onValueChange = {
				if (it.text != seedWord.value.text) {
					onEnteredChange(it.text)
				}
				seedWord.value = it
			},
			cursorBrush = SolidColor(MaterialTheme.colors.primary),
			modifier = Modifier
				.focusRequester(focusRequester)
				.padding(vertical = 8.dp, horizontal = 12.dp)
				.conditional(!shouldShowPlaceholder) {
					width(IntrinsicSize.Min)
				},
			decorationBox = @Composable { innerTextField ->
				innerTextField()
				if (shouldShowPlaceholder) {
					Text(
						text = stringResource(R.string.enter_seed_phease_box_placeholder),
						color = MaterialTheme.colors.textTertiary,
						style = SignerTypeface.BodyL,
					)
				}
			}
		)
		Box(
			//todo dmitry make sure it's at bottom right
			modifier = Modifier.fillMaxWidth(1f),
			contentAlignment = Alignment.BottomEnd
		) {
			ScanIconPlain(onClick = onScanOpen)
		}
	}

	DisableScreenshots()
	KeepScreenOn()
	LaunchedEffect(Unit) {
		focusRequester.requestFocus()
	}
}


@Composable
private fun EnterSeedPhraseWord(index: Int, word: String) {
	Row(
		Modifier
			.background(MaterialTheme.colors.fill6, RoundedCornerShape(16.dp))
			.defaultMinSize(minWidth = 40.dp, minHeight = 24.dp)
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
			color = MaterialTheme.colors.primary,
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
private fun PreviewSeedPhraseRestoreComponentEmptry() {
	SignerNewTheme {
		EnterSeedPhraseBox(emptyList(), "", Modifier, {}, {})
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
	val entered = listOf(
		"some", "workds", "used", "secret", "veryverylong", "special",
		"long", "text", "here", "how", "printed1234"
	)
	SignerNewTheme {
		EnterSeedPhraseBox(entered, "", Modifier, {}, {})
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
	val entered = listOf(
		"some", "workds", "used", "secret", "veryverylong", "special",
		"long", "text", "here", "how"
	)
	SignerNewTheme {
		EnterSeedPhraseBox(entered, "printed", Modifier, {}, {})
	}
}

