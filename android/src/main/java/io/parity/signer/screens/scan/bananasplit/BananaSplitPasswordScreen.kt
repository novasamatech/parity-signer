package io.parity.signer.screens.scan.bananasplit

import android.content.res.Configuration
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Visibility
import androidx.compose.material.icons.filled.VisibilityOff
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.focus.focusRequester
import androidx.compose.ui.platform.LocalFocusManager
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.text.input.PasswordVisualTransformation
import androidx.compose.ui.text.input.VisualTransformation
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.lifecycle.viewmodel.compose.viewModel
import io.parity.signer.R
import io.parity.signer.components.base.ScreenHeaderWithButton
import io.parity.signer.models.Callback
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.red500
import io.parity.signer.uniffi.updateSeedNames


@Composable
fun BananaSplitPasswordScreen(
	qrData: List<String>,
	onClose: Callback,
	onDone: Callback,
	onErrorWrongPassword: Callback,
	onCustomError: (String) -> Unit,
) {

	val bananaViewModel: BananaSplitViewModel = viewModel()

	val name = bananaViewModel.seedName.collectAsState()
	val password = bananaViewModel.password.collectAsState()
	val nameCollision = bananaViewModel.seedCollision.collectAsState()
	val wrongPassword = bananaViewModel.wrongPasswordCurrent.collectAsState()


	val canProceed = name.value.isNotEmpty() && password.value.isNotEmpty()

	val focusManager = LocalFocusManager.current
	val pathFocusRequester = remember { FocusRequester() }
	val passwordFocusRequester = remember { FocusRequester() }

	var passwordVisible by remember { mutableStateOf(false) }


	DisposableEffect(Unit) {
		bananaViewModel.initState(qrData)
		pathFocusRequester.requestFocus()
		onDispose { focusManager.clearFocus() }
	}

	Column() {
		ScreenHeaderWithButton(
			canProceed = canProceed,
			title = "",
			onClose = onClose,
			onDone = onDone, //todo banana
		)

		Column(
			modifier = Modifier
				.padding(horizontal = 24.dp)
				.verticalScroll(rememberScrollState())
		) {

			Text(
				text = stringResource(R.string.banana_split_password_title),
				color = MaterialTheme.colors.primary,
				style = SignerTypeface.TitleL,
				modifier = Modifier
					.padding(bottom = 20.dp)
			)
			Text(
				text = stringResource(R.string.banana_split_password_name_header),
				color = MaterialTheme.colors.primary,
				style = SignerTypeface.BodyL,
			)

			OutlinedTextField(
				value = name.value,
				onValueChange = { newStr -> (bananaViewModel::updateSeedName)(newStr) },
				keyboardOptions = KeyboardOptions(
					imeAction = if (canProceed) ImeAction.Done else ImeAction.None
				),
				keyboardActions = KeyboardActions(onDone = {
					passwordFocusRequester.requestFocus()
				}),
				label = { Text(text = stringResource(R.string.banana_split_password_name_label)) },
				isError = nameCollision.value,
				singleLine = true,
				textStyle = SignerTypeface.LabelM,
				colors = TextFieldDefaults.textFieldColors(
					textColor = MaterialTheme.colors.primary,
					errorCursorColor = MaterialTheme.colors.primary,
				),
				modifier = Modifier
					.focusRequester(pathFocusRequester)
					.fillMaxWidth(1f)
			)
			if (nameCollision.value) {
				Text(
					text = "Given display name already exists.",
					color = MaterialTheme.colors.red500,
					style = SignerTypeface.CaptionM,
				)
			}
			Spacer(modifier = Modifier.padding(bottom = 20.dp))

			//password
			Text(
				text = stringResource(R.string.banana_split_password_password_header),
				color = MaterialTheme.colors.primary,
				style = SignerTypeface.BodyL,
				modifier = Modifier
			)
			OutlinedTextField(
				value = password.value,
				onValueChange = { password -> (bananaViewModel::updatePassword)(password) },
				modifier = Modifier
					.focusRequester(passwordFocusRequester)
					.fillMaxWidth(1f),
				visualTransformation = if (passwordVisible) VisualTransformation.None else PasswordVisualTransformation(),
				keyboardOptions = KeyboardOptions(
					keyboardType = KeyboardType.Password,
					imeAction = if (canProceed) ImeAction.Done else ImeAction.None
				),
				keyboardActions = KeyboardActions(
					onDone = {
						if (canProceed) {
//						proceed(password)//todo banana
//						focusManager.clearFocus(true)
						}
					}
				),
				label = { Text(text = stringResource(R.string.banana_split_password_password_label)) },
				isError = wrongPassword.value,
				singleLine = true,
				textStyle = SignerTypeface.LabelM,
				colors = TextFieldDefaults.textFieldColors(
					textColor = MaterialTheme.colors.primary,
					errorCursorColor = MaterialTheme.colors.primary,
				),
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
			if (wrongPassword.value) {
				Text(
					text = "The password you entered is incorrect",
					color = MaterialTheme.colors.red500,
					style = SignerTypeface.CaptionM,
				)
			}
			Spacer(modifier = Modifier.padding(bottom = 24.dp))
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
private fun PreviewBananaSplitPasswordScreen() {
	SignerNewTheme {
		BananaSplitPasswordScreen(
			qrData = emptyList(),
			onClose = {},
			onDone = {},
			onCustomError = { _ -> },
			onErrorWrongPassword = {},
		)
	}
}
