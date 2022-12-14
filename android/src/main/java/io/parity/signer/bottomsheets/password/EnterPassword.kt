package io.parity.signer.bottomsheets.password

import android.content.res.Configuration
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.TextField
import androidx.compose.material.TextFieldDefaults
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.focus.focusRequester
import androidx.compose.ui.platform.LocalFocusManager
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.CloseIcon
import io.parity.signer.components.base.PrimaryButtonGreyDisabled
import io.parity.signer.components.sharedcomponents.KeyCardModelBase
import io.parity.signer.components.sharedcomponents.KeyCardPassword
import io.parity.signer.models.Callback
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.textTertiary
import io.parity.signer.uniffi.MEnterPassword

@Composable
fun EnterPassword(
	data: EnterPasswordModel,
	proceed: (String) -> Unit,
	onClose: Callback,
) {
	val password = remember {
		mutableStateOf("")
	}

	val focusManager = LocalFocusManager.current
	val focusRequester = remember { FocusRequester() }

	val canProceed = password.value.isNotBlank()

	Column(
		horizontalAlignment = Alignment.CenterHorizontally,
		modifier = Modifier.imePadding()
	) {
		EnterPasswordHeader(
			onClose = onClose,
			onProceed = {
				proceed(password.value)
				focusManager.clearFocus(true)
			},
			isEnabled = canProceed
		)
		Column(
			horizontalAlignment = Alignment.CenterHorizontally,
			modifier = Modifier
                .padding(horizontal = 24.dp)
                .verticalScroll(
                    rememberScrollState()
                )
		) {
			Text(
				text = stringResource(R.string.enter_password_title),
				color = MaterialTheme.colors.primary,
				style = SignerTypeface.TitleL,
			)
			Spacer(modifier = Modifier.padding(top = 16.dp))
			KeyCardPassword(model = data.keyCard)
//		if (enterPassword.counter > 0u) {
//			Text("Attempt " + enterPassword.counter.toString() + " of 3")
//		}

			//todo https://stackoverflow.com/questions/65304229/toggle-password-field-jetpack-compose
			//todo placeholder for mark
			Spacer(modifier = Modifier.padding(top = 16.dp))

			TextField(
				value = password.value,
				onValueChange = { password.value = it },
				keyboardOptions = KeyboardOptions(
					imeAction = if (canProceed) ImeAction.Done else ImeAction.None
				),
				keyboardActions = KeyboardActions(
					onDone = {
						if (canProceed) {
							proceed(password.value)
							focusManager.clearFocus(true)
						}
					}
				),
				label = { Text(text = "Enter password") },
				singleLine = true,
				textStyle = SignerTypeface.LabelM,
				colors = TextFieldDefaults.textFieldColors(textColor = MaterialTheme.colors.primary),
				modifier = Modifier
                    .focusRequester(focusRequester)
                    .fillMaxWidth(1f),
			)

			Text(
				text = stringResource(R.string.enter_password_description),
				color = MaterialTheme.colors.textTertiary,
				style = SignerTypeface.CaptionM,
				modifier = Modifier.padding(top = 8.dp, bottom = 30.dp),
			)
		}
	}
}


@Composable
private fun EnterPasswordHeader(
	onClose: Callback,
	onProceed: Callback,
	isEnabled: Boolean,
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
				noBackground = true,
			) {
				onClose()
			}
		}
		Spacer(modifier = Modifier.fillMaxWidth(1f))
		Box(
			modifier = Modifier.fillMaxWidth(1f),
			contentAlignment = Alignment.CenterEnd,
		) {
			PrimaryButtonGreyDisabled(
				label = stringResource(R.string.generic_done),
				isEnabled = isEnabled,
			) {
				if (isEnabled) {
					onProceed()
				}
			}
		}
	}
}

/**
 * Local copy of shared [MEnterPassword] class
 */
class EnterPasswordModel(
	val keyCard: KeyCardModelBase,
	val attempt: Int,
) {
	companion object {
		fun createStub(): EnterPasswordModel = EnterPasswordModel(
			keyCard = KeyCardModelBase.createStub(),
			attempt = 2,
		)
	}
}

fun MEnterPassword.toEnterPasswordModel() = EnterPasswordModel(
	keyCard = KeyCardModelBase.fromAddress(authorInfo),
	attempt = counter.toInt(),
)


@Preview(
	name = "day",
	uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true,
)
@Preview(
	name = "dark theme",
	uiMode = Configuration.UI_MODE_NIGHT_YES,
	backgroundColor = 0xFFFFFFFF
)
@Composable
private fun PreviewEnterPassword() {
	SignerNewTheme {
		EnterPassword(
			EnterPasswordModel.createStub(),
			{},
			{},
		)
	}
}
