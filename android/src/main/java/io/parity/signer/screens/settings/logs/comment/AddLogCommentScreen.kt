package io.parity.signer.screens.settings.logs.comment

import android.content.res.Configuration
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material.MaterialTheme
import androidx.compose.material.OutlinedTextField
import androidx.compose.material.Text
import androidx.compose.material.TextFieldDefaults
import androidx.compose.runtime.Composable
import androidx.compose.runtime.DisposableEffect
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.focus.focusRequester
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalFocusManager
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.lifecycle.viewmodel.compose.viewModel
import io.parity.signer.R
import io.parity.signer.components.base.ScreenHeaderWithButton
import io.parity.signer.domain.Callback
import io.parity.signer.screens.settings.logs.LogsViewModel
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface


@Composable
internal fun AddLogCommentScreen(onBack: Callback) {
	val viewModel: LogsViewModel = viewModel<LogsViewModel>()
	val context = LocalContext.current

	AddLogCommentInternal(
		onBack = onBack,
		onDone = { note -> }//todo dmitry do it
	)
}

@Composable
private fun AddLogCommentInternal(
	onBack: Callback,
	onDone: (note: String) -> Unit,
) {
	val note = remember { mutableStateOf("") }
	val canProceed = note.value.isNotBlank()

	val focusManager = LocalFocusManager.current
	val focusRequester = remember { FocusRequester() }

	Column(
		Modifier
			.statusBarsPadding()
			.fillMaxSize(1f)
	) {
		ScreenHeaderWithButton(
			canProceed = canProceed,
			title = stringResource(R.string.add_log_comment_title),
			onDone = { onDone(note.value) },
			onClose = onBack,
		)
		OutlinedTextField(
			value = note.value, //hide password, add hint
			onValueChange = { note.value = it },
			placeholder = { Text(text = stringResource(R.string.add_log_note_placeholder)) },
			keyboardOptions = KeyboardOptions(
				imeAction = ImeAction.Done
			),
			keyboardActions = KeyboardActions(onDone = {
				if (canProceed) {
					onDone(note.value)
				}
			}),
			singleLine = false,
			minLines = 3,
			textStyle = SignerTypeface.LabelM,
			colors = TextFieldDefaults.textFieldColors(
				textColor = MaterialTheme.colors.primary,
				errorCursorColor = MaterialTheme.colors.primary,
			),
			modifier = Modifier
				.focusRequester(focusRequester)
				.fillMaxWidth(1f)
//				.defaultMinSize(minHeight = 120.dp)
				.padding(horizontal = 24.dp, vertical = 24.dp)
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
private fun PreviewAddLogCommentInternal() {
	SignerNewTheme {
		AddLogCommentInternal(
			{},
			{},
		)
	}
}
