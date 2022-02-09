package io.parity.signer.modals

import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.material.TextField
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Lock
import androidx.compose.runtime.*
import androidx.compose.runtime.livedata.observeAsState
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
import io.parity.signer.models.addKey
import io.parity.signer.models.checkAsDerivation
import io.parity.signer.models.pushButton
import io.parity.signer.ui.theme.Bg200

@Composable
fun PasswordConfirm(signerDataModel: SignerDataModel) {
	var passwordCheck = remember { mutableStateOf("") }
	val pwd = signerDataModel.modalData.value?.optString("pwd")
	val croppedPath = signerDataModel.modalData.value?.optString("cropped_path")
	val seedName = signerDataModel.modalData.value?.optString("seed_name") ?: ""
	val lastError = signerDataModel.lastError.observeAsState()
	val focusManager = LocalFocusManager.current
	val focusRequester = remember { FocusRequester() }

	Surface(
		color = MaterialTheme.colors.Bg200,
		shape = MaterialTheme.shapes.large,
		modifier = Modifier.fillMaxSize(1f)
	) {
		Column(
			horizontalAlignment = Alignment.CenterHorizontally,
			modifier = Modifier.padding(20.dp)
		) {
			HeaderBar(line1 = "Confirm secret path", line2 = "")
			Row {
				Text("$croppedPath///")
				Image(Icons.Default.Lock, contentDescription = "Locked account")
			}
			Text(lastError.value.toString())
			SingleTextInput(
				content = passwordCheck,
				update = {
					passwordCheck.value = it
					signerDataModel.clearError()
				},
				onDone = {
					if (passwordCheck.value == pwd) {
						signerDataModel.addKey(
							path = "$croppedPath///$pwd",
							seedName = seedName
						)
					}
				},
				prefix = { Text("///") },
				focusManager = focusManager,
				focusRequester = focusRequester
			)

			BigButton(
				text = "Next", action = {
					signerDataModel.addKey(
						path = "$croppedPath///$pwd",
						seedName = seedName
					)
				},
				isDisabled = passwordCheck.value != pwd
			)
		}
	}
	DisposableEffect(Unit) {
		//if (signerDataModel.modalData.value?.optBoolean("keyboard") == true) {
		focusRequester.requestFocus()
		//}
		onDispose { focusManager.clearFocus() }
	}
}
