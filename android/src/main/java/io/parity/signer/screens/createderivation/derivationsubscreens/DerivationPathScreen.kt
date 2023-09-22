package io.parity.signer.screens.createderivation.derivationsubscreens

import android.content.res.Configuration
import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Visibility
import androidx.compose.material.icons.filled.VisibilityOff
import androidx.compose.material.icons.outlined.HelpOutline
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.focus.focusRequester
import androidx.compose.ui.graphics.compositeOver
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.SpanStyle
import androidx.compose.ui.text.TextRange
import androidx.compose.ui.text.buildAnnotatedString
import androidx.compose.ui.text.input.*
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.ScreenHeaderWithButton
import io.parity.signer.domain.Callback
import io.parity.signer.screens.createderivation.DerivationCreateViewModel
import io.parity.signer.screens.createderivation.DerivationPathAnalyzer
import io.parity.signer.screens.createderivation.DerivationPathVisualTransformation
import io.parity.signer.ui.theme.*


@Composable
fun DerivationPathScreen(
	initialPath: String,
	onDerivationHelp: Callback,
	onClose: Callback,
	onDone: (String) -> Unit,
	validator: (String) -> DerivationCreateViewModel.DerivationPathValidity,
	modifier: Modifier = Modifier,
) {
	val path = remember {
		mutableStateOf(
			TextFieldValue(
				text = initialPath,
				selection = TextRange(initialPath.length),
			)
		)
	}
	val pathValidity = validator(path.value.text)
	val canProceed =
		pathValidity == DerivationCreateViewModel.DerivationPathValidity.ALL_GOOD
	val password = remember { mutableStateOf("") }
	var passwordVisible by remember { mutableStateOf(false) }
	var passwordNotMatch by remember { mutableStateOf(false) }

	val hasPassword = DerivationPathAnalyzer.getPassword(path.value.text) != null
	if (!hasPassword) {
		passwordNotMatch = false
	} else if (DerivationPathAnalyzer.getPassword(path.value.text) == password.value) {
		passwordNotMatch = false
	}

	val pathFocusRequester = remember { FocusRequester() }
	val passwordFocusRequester = remember { FocusRequester() }

	val onDoneLocal = {
		val pathValue = path.value.text
		if (hasPassword && DerivationPathAnalyzer.getPassword(pathValue) != password.value) {
			passwordNotMatch = true
		} else {
			onDone(path.value.text)
		}
	}

	Column(modifier = modifier) {
		ScreenHeaderWithButton(
			canProceed = canProceed,
			title = stringResource(R.string.create_derivation_title),
			subtitle = stringResource(R.string.screen_step_2_2),
			backNotClose = true,
			onClose = onClose,
			onDone = onDoneLocal,
		)
		Text(
			text = stringResource(R.string.derivation_path_screen_title),
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.BodyL,
			modifier = Modifier
				.padding(horizontal = 24.dp)
		)
		Spacer(modifier = Modifier.padding(top = 16.dp))
		OutlinedTextField(
			value = path.value, //hide password, add hint
			onValueChange = { newStr -> path.value = newStr },
			keyboardOptions = KeyboardOptions(
//				fixme #1749 recreation of options leading to first letter dissapearing on some samsung devices
				imeAction = ImeAction.Done
			),
			visualTransformation = DerivationPathVisualTransformation(
				context = LocalContext.current,
				themeColors = MaterialTheme.colors,
				hidePassword = !passwordVisible,
			),
			keyboardActions = KeyboardActions(onDone = {
				if (hasPassword) {
					passwordFocusRequester.requestFocus()
				} else {
					onDoneLocal()
				}
			}),
			isError = pathValidity != DerivationCreateViewModel.DerivationPathValidity.ALL_GOOD,
			singleLine = true,
			textStyle = SignerTypeface.LabelM,
			colors = TextFieldDefaults.textFieldColors(
				textColor = MaterialTheme.colors.primary,
				errorCursorColor = MaterialTheme.colors.primary,
			),
			modifier = Modifier
				.focusRequester(pathFocusRequester)
				.fillMaxWidth(1f)
				.padding(horizontal = 24.dp)
		)
		val errorForPath = when (pathValidity) {
			DerivationCreateViewModel.DerivationPathValidity.ALL_GOOD -> null
			DerivationCreateViewModel.DerivationPathValidity.WRONG_PATH -> stringResource(
				R.string.derivation_path_screen_error_wrong_path
			)
			DerivationCreateViewModel.DerivationPathValidity.COLLISION_PATH -> stringResource(
				R.string.derivation_path_screen_error_collision_path
			)
			DerivationCreateViewModel.DerivationPathValidity.EMPTY_PASSWORD -> stringResource(
				R.string.derivation_path_screen_error_password_empty
			)
			DerivationCreateViewModel.DerivationPathValidity.CONTAIN_SPACES -> stringResource(
				R.string.derivation_path_screen_error_contain_spaces
			)
		}
		errorForPath?.let { error ->
			Text(
				text = error,
				color = MaterialTheme.colors.red500,
				style = SignerTypeface.CaptionM,
				modifier = Modifier
					.padding(horizontal = 24.dp)
					.padding(top = 8.dp)
			)
		}


		if (!hasPassword) {
			//suggestion slashes
			val hintBackground =
				MaterialTheme.colors.fill6.compositeOver(MaterialTheme.colors.backgroundDanger)
			Row(
				modifier = Modifier
					.padding(top = 8.dp, bottom = 16.dp)
					.padding(horizontal = 24.dp),
				horizontalArrangement = Arrangement.spacedBy(4.dp)
			) {
				Surface(
					modifier = Modifier
						.clickable {
							val newText = path.value.text + "/"
							path.value = TextFieldValue(
								text = newText,
								selection = TextRange(newText.length),
							)
						}
						.background(hintBackground, RoundedCornerShape(24.dp))
						.padding(vertical = 8.dp, horizontal = 20.dp),
				) {
					Text(
						text = "/",
						color = MaterialTheme.colors.pink300,
						style = SignerTypeface.LabelS,
					)
				}
				Surface(
					modifier = Modifier
						.clickable {
							val newText = path.value.text + "//"
							path.value = TextFieldValue(
								text = newText,
								selection = TextRange(newText.length),
							)
						}
						.background(hintBackground, RoundedCornerShape(24.dp))
						.padding(vertical = 8.dp, horizontal = 20.dp),
				) {
					Text(
						text = "//",
						color = MaterialTheme.colors.pink300,
						style = SignerTypeface.LabelS,
					)
				}
				Surface(
					modifier = Modifier
						.clickable {
							val newText = path.value.text + "///"
							path.value = TextFieldValue(
								text = newText,
								selection = TextRange(newText.length),
							)
						}
						.background(hintBackground, RoundedCornerShape(24.dp))
						.padding(vertical = 8.dp, horizontal = 20.dp),
				) {
					Text(
						text = stringResource(R.string.derivation_path_screen_password_input_button),
						color = MaterialTheme.colors.pink300,
						style = SignerTypeface.LabelS,
					)
				}
			}
		}
		Text(
			text = stringResource(R.string.derivation_path_screen_subinput_hint),
			color = MaterialTheme.colors.textTertiary,
			style = SignerTypeface.CaptionM,
			modifier = Modifier
				.padding(horizontal = 24.dp)
				.padding(vertical = 8.dp)
		)

		if (hasPassword) {
			Text(
				text = stringResource(R.string.enter_password_title),
				color = MaterialTheme.colors.primary,
				style = SignerTypeface.BodyL,
				modifier = Modifier.padding(horizontal = 24.dp, vertical = 6.dp)
			)
			OutlinedTextField(
				value = password.value,
				onValueChange = { password.value = it },
				modifier = Modifier
					.padding(horizontal = 24.dp, vertical = 8.dp)
					.focusRequester(passwordFocusRequester)
					.fillMaxWidth(1f),
				visualTransformation = if (passwordVisible) VisualTransformation.None else PasswordVisualTransformation(),
				keyboardOptions = KeyboardOptions(
					keyboardType = KeyboardType.Password,
					imeAction = ImeAction.Done
				),
				keyboardActions = KeyboardActions(onDone = {
					onDoneLocal()
				}),
				isError = passwordNotMatch,
				placeholder = { Text(text = stringResource(R.string.derivation_path_screen_password_empty_hint)) },
				singleLine = true,
				textStyle = SignerTypeface.LabelM,
				colors = TextFieldDefaults.textFieldColors(
					textColor = MaterialTheme.colors.primary,
					errorCursorColor = MaterialTheme.colors.primary,
				),
				trailingIcon = {
					val image = if (passwordVisible) Icons.Filled.Visibility
					else Icons.Filled.VisibilityOff

					val description =
						if (passwordVisible) stringResource(R.string.password_hide_password) else stringResource(
							R.string.password_show_password
						)

					IconButton(onClick = { passwordVisible = !passwordVisible }) {
						Icon(
							imageVector = image,
							contentDescription = description,
							tint = MaterialTheme.colors.textTertiary,
						)
					}
				},
			)
			if (passwordNotMatch) {
				Text(
					text = stringResource(R.string.derivation_path_screen_password_error_not_match),
					color = MaterialTheme.colors.red500,
					style = SignerTypeface.CaptionM,
					modifier = Modifier.padding(horizontal = 24.dp)
				)
			}
		}
		DerivationAlarm(
			Modifier
				.padding(top = 8.dp, bottom = 8.dp)
				.padding(horizontal = 24.dp)
				.clickable(onClick = {
					onDerivationHelp()
				})
		)
	}

	LaunchedEffect(Unit) {
		pathFocusRequester.requestFocus()
	}
}

@Composable
private fun DerivationAlarm(modifier: Modifier = Modifier) {
	val innerShape =
		RoundedCornerShape(dimensionResource(id = R.dimen.innerFramesCornerRadius))
	Row(
		modifier = modifier
			.padding(vertical = 8.dp)
			.border(
				BorderStroke(1.dp, MaterialTheme.colors.appliedStroke),
				innerShape
			)

	) {

		val derivationAlarmText = buildAnnotatedString {
			val str =
				stringResource(R.string.derivation_path_screen_help_tile_message)
			val boldStr =
				stringResource(R.string.derivation_path_screen_help_tile_message_highlited_part)
			val startIndex = str.indexOf(boldStr)
			val endIndex = startIndex + boldStr.length
			append(str)
			addStyle(
				style = SpanStyle(color = MaterialTheme.colors.pink300),
				start = startIndex,
				end = endIndex
			)
		}
		Text(
			text = derivationAlarmText,
			color = MaterialTheme.colors.textTertiary,
			style = SignerTypeface.BodyM,
			modifier = Modifier
				.weight(1f)
				.padding(start = 16.dp, top = 16.dp, bottom = 16.dp)
		)
		Icon(
			imageVector = Icons.Outlined.HelpOutline,
			contentDescription = null,
			tint = MaterialTheme.colors.pink300,
			modifier = Modifier
				.align(Alignment.CenterVertically)
				.padding(start = 18.dp, end = 18.dp)
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
private fun PreviewDerivationPathScreen() {
	SignerNewTheme {
		DerivationPathScreen(
			initialPath = "//",
			{},
			{},
			{},
			{ _ -> DerivationCreateViewModel.DerivationPathValidity.ALL_GOOD })
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
private fun PreviewDerivationPathPassworded() {
	SignerNewTheme {
		DerivationPathScreen(
			initialPath = "//seed//1///ggg",
			{},
			{},
			{},
			{ _ -> DerivationCreateViewModel.DerivationPathValidity.ALL_GOOD })
	}
}
