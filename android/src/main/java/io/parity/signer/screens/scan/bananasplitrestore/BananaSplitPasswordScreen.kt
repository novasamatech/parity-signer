package io.parity.signer.screens.scan.bananasplitrestore

import android.annotation.SuppressLint
import android.content.res.Configuration
import androidx.compose.foundation.background
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
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import io.parity.signer.R
import io.parity.signer.components.base.ScreenHeaderWithButton
import io.parity.signer.domain.Callback
import io.parity.signer.ui.theme.*


@Composable
fun BananaSplitPasswordScreen(
	onClose: Callback,
	onDone: Callback,
	bananaViewModel: BananaSplitViewModel,
	modifier: Modifier = Modifier,
) {

	val name = bananaViewModel.seedName.collectAsStateWithLifecycle()
	val password = bananaViewModel.password.collectAsStateWithLifecycle()
	val nameCollision = bananaViewModel.seedCollision.collectAsStateWithLifecycle()
	val wrongPassword = bananaViewModel.wrongPasswordCurrent.collectAsStateWithLifecycle()


	BananaSplitPasswordInternal(
		onClose = onClose,
		name = name,
		nameCollision = nameCollision,
		password = password,
		wrongPassword = wrongPassword,
		onChangePassword = bananaViewModel::updatePassword,
		onChangeSeedName = bananaViewModel::updateSeedName,
		onDoneTap = onDone,
		modifier = modifier,
	)
}

@Composable
private fun BananaSplitPasswordInternal(
	onDoneTap: Callback,
	onClose: Callback,
	onChangeSeedName: (String) -> Unit,
	onChangePassword: (String) -> Unit,
	name: State<String>,
	nameCollision: State<Boolean>,
	password: State<String>,
	wrongPassword: State<Boolean>,
	modifier: Modifier = Modifier,
) {

	val pathFocusRequester = remember { FocusRequester() }
	val passwordFocusRequester = remember { FocusRequester() }

	val canProceed = name.value.isNotEmpty() && password.value.isNotEmpty()
		&& !nameCollision.value && !wrongPassword.value

	var passwordVisible by remember { mutableStateOf(false) }

	Column(
		modifier
			.background(MaterialTheme.colors.backgroundPrimary)
			.fillMaxSize(1f)
	) {
		ScreenHeaderWithButton(
			canProceed = canProceed,
			title = "",
			modifier = Modifier.padding(start = 8.dp),
			onClose = onClose,
			onDone = onDoneTap,
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
					.padding(bottom = 14.dp)
			)
			Text(
				text = stringResource(R.string.banana_split_password_name_header),
				color = MaterialTheme.colors.primary,
				style = SignerTypeface.BodyL,
				modifier = Modifier.padding(vertical = 6.dp),
			)

			OutlinedTextField(
				value = name.value,
				onValueChange = { newStr -> onChangeSeedName(newStr) },
				keyboardOptions = KeyboardOptions(
					//				fixme #1749 recreation of options leading to first letter dissapearing on some samsung devices
					imeAction = ImeAction.Go
				),
				keyboardActions = KeyboardActions(onGo = {
					if (name.value.isNotEmpty() && !nameCollision.value) {
						passwordFocusRequester.requestFocus()
					}
				}),
				placeholder = { Text(text = stringResource(R.string.banana_split_password_name_label)) },
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
					text = stringResource(R.string.banana_split_password_name_error_collision),
					color = MaterialTheme.colors.red500,
					style = SignerTypeface.CaptionM,
				)
			}
			Spacer(modifier = Modifier.padding(bottom = 16.dp))

			//password
			Text(
				text = stringResource(R.string.banana_split_password_password_header),
				color = MaterialTheme.colors.primary,
				style = SignerTypeface.BodyL,
				modifier = Modifier.padding(vertical = 6.dp),
			)
			OutlinedTextField(
				value = password.value,
				onValueChange = { password -> onChangePassword(password) },
				modifier = Modifier
					.focusRequester(passwordFocusRequester)
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
							onDoneTap()
						}
					}
				),
				placeholder = { Text(text = stringResource(R.string.banana_split_password_password_label)) },
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
					text = stringResource(R.string.banana_split_password_password_error_wrong),
					color = MaterialTheme.colors.red500,
					style = SignerTypeface.CaptionM,
				)
			}
			Spacer(modifier = Modifier.padding(bottom = 24.dp))
		}
	}

	LaunchedEffect(Unit) {
		pathFocusRequester.requestFocus()
	}
}


@SuppressLint("UnrememberedMutableState")
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
private fun PreviewBananaSplitPasswordScreenEmpty() {
	SignerNewTheme {
		BananaSplitPasswordInternal(
			onDoneTap = {},
			onClose = {},
			onChangeSeedName = {},
			onChangePassword = {},
			name = mutableStateOf(""),
			nameCollision = mutableStateOf(false),
			password = mutableStateOf(""),
			wrongPassword = mutableStateOf(false),
		)
	}
}

@SuppressLint("UnrememberedMutableState")
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
private fun PreviewBananaSplitPasswordScreenFull() {
	SignerNewTheme {
		BananaSplitPasswordInternal(
			onDoneTap = {},
			onClose = {},
			onChangeSeedName = {},
			onChangePassword = {},
			name = mutableStateOf("Seed"),
			nameCollision = mutableStateOf(true),
			password = mutableStateOf("special"),
			wrongPassword = mutableStateOf(true),
		)
	}
}
