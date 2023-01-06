package io.parity.signer.screens.keysets.create

import android.content.res.Configuration
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.FocusManager
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.focus.focusRequester
import androidx.compose.ui.platform.LocalFocusManager
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.CloseIcon
import io.parity.signer.components.base.DotsIndicator
import io.parity.signer.components.base.PrimaryButtonGreyDisabled
import io.parity.signer.models.EmptyNavigator
import io.parity.signer.models.Navigator
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.textSecondary
import io.parity.signer.uniffi.Action

/**
 * 1/2 stage to create new key set
 * second it NewKeySetBackup
 */
@Composable
fun NewKeySetNameScreen(
	rootNavigator: Navigator,
	seedNames: Array<String>
) {
	var keySetName by remember { mutableStateOf("") }
	val focusManager = LocalFocusManager.current
	val focusRequester = remember { FocusRequester() }

	val canProceed = keySetName.isNotEmpty() && !seedNames.contains(keySetName)

	Column(
		Modifier
			.fillMaxSize(1f)
			.background(MaterialTheme.colors.background),
	) {
		NewSeedHeader(rootNavigator, canProceed, keySetName, focusManager)
		Text(
			text = stringResource(R.string.new_key_set_title),
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.TitleL,
			modifier = Modifier.padding(horizontal = 24.dp),
		)
		Text(
			text = stringResource(R.string.new_key_set_subtitle),
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.LabelM,
			modifier = Modifier
				.padding(horizontal = 24.dp)
				.padding(top = 8.dp, bottom = 20.dp),
		)

		OutlinedTextField(
			value = keySetName,
			onValueChange = { newStr -> keySetName = newStr },
			keyboardOptions = KeyboardOptions(
				imeAction = if (canProceed) ImeAction.Done else ImeAction.None
			),
			keyboardActions = KeyboardActions(
				onDone = {
					if (canProceed) {
						rootNavigator.navigate(Action.GO_FORWARD, keySetName)
						focusManager.clearFocus(true)
					}
				}
			),
			singleLine = true,
			textStyle = SignerTypeface.LabelM,
			colors = TextFieldDefaults.textFieldColors(textColor = MaterialTheme.colors.primary),
			modifier = Modifier
				.focusRequester(focusRequester)
				.fillMaxWidth(1f)
				.padding(horizontal = 24.dp),
		)

		Text(
			text = stringResource(R.string.new_key_set_description),
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

@Composable
private fun NewSeedHeader(
	rootNavigator: Navigator,
	canProceed: Boolean,
	keySetName: String,
	focusManager: FocusManager,
) {
	Box(
		modifier = Modifier
			.padding(start = 24.dp, end = 8.dp, top = 8.dp, bottom = 8.dp),
		contentAlignment = Alignment.Center,
	) {
		Box(
			modifier = Modifier.fillMaxWidth(1f),
			contentAlignment = Alignment.CenterStart,
		) {
			CloseIcon(
				modifier = Modifier.padding(vertical = 20.dp),
				noBackground = true,
			) {
				rootNavigator.backAction()
			}
		}
		Box(
			modifier = Modifier.fillMaxWidth(1f),
			contentAlignment = Alignment.Center,
		) {
			DotsIndicator(totalDots = 2, selectedIndex = 1)
		}
		Box(
			modifier = Modifier.fillMaxWidth(1f),
			contentAlignment = Alignment.CenterEnd,
		) {
			PrimaryButtonGreyDisabled(
				label = stringResource(R.string.button_next),
				isEnabled = canProceed,
			) {
				if (canProceed) {
					rootNavigator.navigate(Action.GO_FORWARD, keySetName)
					focusManager.clearFocus(true)
				}
			}
		}
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
private fun PreviewNewKeySetScreen() {
	SignerNewTheme {
		NewKeySetNameScreen(EmptyNavigator(), arrayOf())
	}
}
