package io.parity.signer.screens.keysets.restore

import android.content.res.Configuration
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
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.focus.focusRequester
import androidx.compose.ui.platform.LocalFocusManager
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.ScreenHeaderWithButton
import io.parity.signer.domain.EmptyNavigator
import io.parity.signer.domain.Navigator
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.textSecondary
import io.parity.signer.uniffi.Action


@Composable
fun KeysetRecoverPhraseScreen(
	rootNavigator: Navigator,
	addSeed: (
		seedName: String,
		seedPhrase: String,
	) -> Unit,
) {
	var keySetName by remember { mutableStateOf("") }
	val focusManager = LocalFocusManager.current
	val focusRequester = remember { FocusRequester() }

	val canProceed = keySetName.isNotEmpty() //&& !seedNames.contains(keySetName)

	Column(
		Modifier
			.fillMaxSize(1f)
			.background(MaterialTheme.colors.background),
	) {

		ScreenHeaderWithButton(
			canProceed = canProceed,
			title = stringResource(R.string.recovert_key_set_title),
			btnText = stringResource(R.string.generic_done),
			onDone = {
//				posted.value = true
//				onDone(note.value)
			},
			onClose = { rootNavigator.backAction() },
			backNotClose = true,
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
						rootNavigator.navigate(Action.GO_FORWARD, keySetName)
						focusManager.clearFocus(true)
					}
				}
			),
			placeholder = { Text(text = stringResource(R.string.new_key_set_name_placeholder)) },
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

	DisposableEffect(Unit) {
		focusRequester.requestFocus()
		onDispose { focusManager.clearFocus() }
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
private fun PreviewKeysetRecoverPhraseScreen() {
	SignerNewTheme {
		KeysetRecoverPhraseScreen(EmptyNavigator(), {_,_ ->})
	}
}

