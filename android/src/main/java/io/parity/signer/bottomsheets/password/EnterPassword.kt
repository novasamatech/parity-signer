package io.parity.signer.bottomsheets.password

import android.content.res.Configuration
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Visibility
import androidx.compose.material.icons.filled.VisibilityOff
import androidx.compose.runtime.*
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.focus.focusRequester
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.text.input.PasswordVisualTransformation
import androidx.compose.ui.text.input.VisualTransformation
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.CloseIcon
import io.parity.signer.components.base.PrimaryButtonGreyDisabled
import io.parity.signer.components.sharedcomponents.KeyCardModelBase
import io.parity.signer.components.sharedcomponents.KeyCardPassword
import io.parity.signer.domain.Callback
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.red500
import io.parity.signer.ui.theme.textTertiary
import io.parity.signer.uniffi.MEnterPassword


@Composable
fun EnterPassword(
	data: EnterPasswordModel,
	proceed: (String) -> Unit,
	onClose: Callback,
) {
	var password by rememberSaveable { mutableStateOf("") }
	var passwordVisible by rememberSaveable { mutableStateOf(false) }

	val focusRequester = remember { FocusRequester() }

	val canProceed = password.isNotBlank()

	Column(
		modifier = Modifier.imePadding()
	) {
		EnterPasswordHeader(
			onClose = onClose,
			onProceed = {
				proceed(password)
			},
			isEnabled = canProceed
		)
		Column(
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

			Spacer(modifier = Modifier.padding(top = 16.dp))

			OutlinedTextField(
				value = password,
				onValueChange = { password = it },
				modifier = Modifier
					.focusRequester(focusRequester)
					.fillMaxWidth(1f),
				visualTransformation = if (passwordVisible) VisualTransformation.None else PasswordVisualTransformation(),
				keyboardOptions = KeyboardOptions(
					keyboardType = KeyboardType.Password,
//				fixme #1749 recreation of options leading to first letter dissapearing on some samsung devices
					imeAction = ImeAction.Done
				),
				keyboardActions = KeyboardActions(
					onDone = {
						if (canProceed) {
							proceed(password)
						}
					}
				),
				placeholder = { Text(text = stringResource(R.string.enter_password_input_label)) },
				singleLine = true,
				textStyle = SignerTypeface.LabelM,
				colors = TextFieldDefaults.textFieldColors(textColor = MaterialTheme.colors.primary),
				trailingIcon = {
					val image = if (passwordVisible)
						Icons.Filled.Visibility
					else Icons.Filled.VisibilityOff

					val description =
						if (passwordVisible) stringResource(R.string.password_hide_password) else stringResource(
							R.string.password_show_password
						)

					IconButton(onClick = { passwordVisible = !passwordVisible }) {
						Icon(imageVector = image, description)
					}
				},
			)

			if (data.showError) {
				Text(
					text = stringResource(R.string.password_error_wrong_password),
					color = MaterialTheme.colors.red500,
					style = SignerTypeface.CaptionM,
					modifier = Modifier.padding(top = 8.dp, bottom = 8.dp),
				)
			}

			Text(
				text = stringResource(R.string.enter_password_description),
				color = MaterialTheme.colors.textTertiary,
				style = SignerTypeface.CaptionM,
				modifier = Modifier.padding(top = 8.dp, bottom = 30.dp),
			)
		}
	}

	LaunchedEffect(Unit) {
		focusRequester.requestFocus()
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
data class EnterPasswordModel(
	val keyCard: KeyCardModelBase,
	val showError: Boolean,
	val attempt: Int,
) {
	companion object {
		fun createStub(): EnterPasswordModel = EnterPasswordModel(
			keyCard = KeyCardModelBase.createStub().copy(hasPassword = true),
			showError = true,
			attempt = 2,
		)
	}
}

fun MEnterPassword.toEnterPasswordModel(withShowError: Boolean = false) = EnterPasswordModel(
	keyCard = KeyCardModelBase.fromAddress(authorInfo, networkInfo?.networkLogo),
	showError = withShowError,
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
