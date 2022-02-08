package io.parity.signer.modals

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material.Text
import androidx.compose.material.TextField
import androidx.compose.runtime.Composable
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.focus.focusRequester
import androidx.compose.ui.platform.LocalFocusManager
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.text.input.KeyboardCapitalization
import androidx.compose.ui.text.input.KeyboardType
import io.parity.signer.ButtonID
import io.parity.signer.components.BigButton
import io.parity.signer.components.HeaderBar
import io.parity.signer.components.SingleTextInput
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.pushButton

@Composable
fun LogComment(signerDataModel: SignerDataModel) {
	var comment = remember { mutableStateOf("") }
	val focusManager = LocalFocusManager.current
	val focusRequester = remember { FocusRequester() }

	Column(
		horizontalAlignment = Alignment.CenterHorizontally,
		verticalArrangement = Arrangement.Center,
		modifier = Modifier.fillMaxSize()
	) {
		HeaderBar(line1 = "COMMENT", line2 = "Enter text")
		SingleTextInput(
			content = comment,
			update = {
				comment.value = it
			},
			onDone = {
				signerDataModel.pushButton(ButtonID.GoForward, comment.value)
			},
			focusManager = focusManager,
			focusRequester = focusRequester
		)

	}
	DisposableEffect(Unit) {
		focusRequester.requestFocus()
		onDispose { focusManager.clearFocus() }
	}
}
