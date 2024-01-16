package io.parity.signer.screens.keysets.restore.keysetname

import android.content.res.Configuration
import timber.log.Timber
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material.MaterialTheme
import androidx.compose.material.OutlinedTextField
import androidx.compose.material.Text
import androidx.compose.material.TextFieldDefaults
import androidx.compose.runtime.Composable
import androidx.compose.runtime.DisposableEffect
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.focus.focusRequester
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.ScreenHeaderProgressWithButton
import io.parity.signer.domain.Callback
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.textSecondary
import io.parity.signer.ui.theme.textTertiary


@Composable
fun KeysetRecoverNameScreen(
	initialKeySetName: String,
	onBack: Callback,
	onNext: (newName: String) -> Unit,
	seedNames: Array<String>,
	modifier: Modifier = Modifier,
) {
	var keySetName by rememberSaveable { mutableStateOf(initialKeySetName) }
	val focusRequester = remember { FocusRequester() }

	val canProceed = keySetName.isNotEmpty() && !seedNames.contains(keySetName)

	Column(
		modifier
			.fillMaxSize(1f)
			.background(MaterialTheme.colors.background),
	) {
		ScreenHeaderProgressWithButton(
			canProceed = canProceed,
			currentStep = 1,
			allSteps = 3,
			modifier = Modifier.padding(start = 8.dp),
			btnText = stringResource(R.string.button_next),
			onClose = onBack,
			onButton = {
				if (canProceed) {
					onNext(keySetName)
				}
			},
			backNotClose = false,
		)
		Text(
			text = stringResource(R.string.recovert_key_set_title),
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.TitleL,
			modifier = Modifier.padding(horizontal = 24.dp),
		)
		Text(
			text = stringResource(R.string.new_key_set_subtitle),
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.BodyL,
			modifier = Modifier
				.padding(horizontal = 24.dp)
				.padding(top = 8.dp, bottom = 20.dp),
		)
		OutlinedTextField(
			value = keySetName,
			onValueChange = { newStr -> keySetName = newStr },
			keyboardOptions = KeyboardOptions(
//				fixme #1749 recreation of options leading to first letter dissapearing on some samsung devices
				imeAction = ImeAction.Done
			),
			keyboardActions = KeyboardActions(
				onDone = {
					if (canProceed) {
						onNext(keySetName)
					}
				}
			),
			maxLines = 1,
			placeholder = {
				Text(
					text = stringResource(R.string.new_key_set_name_placeholder),
					color = MaterialTheme.colors.textTertiary,
					style = SignerTypeface.BodyL,
				)
			},
			singleLine = true,
			textStyle = SignerTypeface.LabelM,
			colors = TextFieldDefaults.textFieldColors(textColor = MaterialTheme.colors.primary),
			modifier = Modifier
				.focusRequester(focusRequester)
				.fillMaxWidth(1f)
				.padding(horizontal = 24.dp),
		)

		Text(
			text = stringResource(R.string.recovert_key_set_name_hint),
			color = MaterialTheme.colors.textSecondary,
			style = SignerTypeface.CaptionM,
			modifier = Modifier
				.padding(horizontal = 24.dp)
				.padding(top = 16.dp, bottom = 16.dp),
		)
	}

	LaunchedEffect(Unit) {
		focusRequester.requestFocus()
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
private fun PreviewKeysetRecoverNameScreen() {
	SignerNewTheme {
		KeysetRecoverNameScreen("", {}, {}, arrayOf())
	}
}

