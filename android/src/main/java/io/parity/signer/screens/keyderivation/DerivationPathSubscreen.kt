package io.parity.signer.screens.keyderivation

import android.content.res.Configuration
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material.MaterialTheme
import androidx.compose.material.OutlinedTextField
import androidx.compose.material.TextFieldDefaults
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.focus.focusRequester
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalFocusManager
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.CloseIcon
import io.parity.signer.components.base.DotsIndicator
import io.parity.signer.components.base.PrimaryButtonGreyDisabled
import io.parity.signer.models.Callback
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface


@Composable
fun DerivationPathScreen(
	onClose: Callback,
	onDone: Callback,
) {
	val canProceed = true
	val path = remember {
		mutableStateOf("")
	}

	val focusManager = LocalFocusManager.current
	val focusRequester = remember { FocusRequester() }

	val onDoneLocal = {
		onDone()
		focusManager.clearFocus(true)
	}

	Column() {
		DerivationPathHeader(
			canProceed = canProceed,
			onClose = onClose,
			onDone = onDoneLocal,
		)
		OutlinedTextField(
			value = path.value, //hide password, add hint
			onValueChange = { newStr -> path.value = newStr },
			keyboardOptions = KeyboardOptions(
				imeAction = if (canProceed) ImeAction.Done else ImeAction.None
			),
			visualTransformation = DerivationPathVisualTransformation(
				LocalContext.current,
				MaterialTheme.colors
			),
			keyboardActions = KeyboardActions(
				onDone = {
					if (canProceed) {
						onDoneLocal()
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
	}
}


/**
 * io/parity/signer/screens/keysets/create/NewKeySetNameScreen.kt:107
 */
@Composable
private fun DerivationPathHeader(
	canProceed: Boolean,
	onClose: Callback,
	onDone: Callback,
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
				onCloseClicked = onClose,
			)
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
				label = stringResource(R.string.generic_done),
				isEnabled = canProceed,
			) {
				if (canProceed) {
					onDone()
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
private fun PreviewDerivationPathScreen() {
	SignerNewTheme {
		DerivationPathScreen({}, {})
	}
}
