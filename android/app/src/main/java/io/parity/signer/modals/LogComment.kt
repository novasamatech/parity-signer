package io.parity.signer.modals

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
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
import androidx.compose.ui.unit.dp
import io.parity.signer.ButtonID
import io.parity.signer.components.BigButton
import io.parity.signer.components.HeaderBar
import io.parity.signer.components.SingleTextInput
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.pushButton
import io.parity.signer.ui.theme.Bg000
import io.parity.signer.ui.theme.modal

@Composable
fun LogComment(signerDataModel: SignerDataModel) {
	var comment = remember { mutableStateOf("") }
	val focusManager = LocalFocusManager.current
	val focusRequester = remember { FocusRequester() }

	Surface(
		shape = MaterialTheme.shapes.modal,
		color = MaterialTheme.colors.Bg000,
		modifier = Modifier.fillMaxSize(1f).padding(8.dp)
	) {
		Column(
			horizontalAlignment = Alignment.CenterHorizontally,
			verticalArrangement = Arrangement.Center,
			modifier = Modifier.fillMaxSize().padding(20.dp)
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
	}

	DisposableEffect(Unit) {
		focusRequester.requestFocus()
		onDispose { focusManager.clearFocus() }
	}
}
