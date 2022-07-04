package io.parity.signer.modals

import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Lock
import androidx.compose.runtime.Composable
import androidx.compose.runtime.DisposableEffect
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.platform.LocalFocusManager
import androidx.compose.ui.unit.dp
import io.parity.signer.components.BigButton
import io.parity.signer.components.HeaderBar
import io.parity.signer.components.SingleTextInput
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.addKey
import io.parity.signer.ui.theme.Bg200
import io.parity.signer.ui.theme.modal
import io.parity.signer.uniffi.MPasswordConfirm

@Composable
fun PasswordConfirm(
	passwordConfirm: MPasswordConfirm,
	addKey: (String, String) -> Unit
) {
	val passwordCheck = remember { mutableStateOf("") }
	val pwd = passwordConfirm.pwd
	val croppedPath = passwordConfirm.croppedPath
	val seedName = passwordConfirm.seedName
	val focusManager = LocalFocusManager.current
	val focusRequester = remember { FocusRequester() }

	Surface(
		color = MaterialTheme.colors.Bg200,
		shape = MaterialTheme.shapes.modal,
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
			SingleTextInput(
				content = passwordCheck,
				update = {
					passwordCheck.value = it
				},
				onDone = {
					if (passwordCheck.value == pwd) {
						addKey(
							"$croppedPath///$pwd",
							seedName
						)
					}
				},
				prefix = { Text("///") },
				focusManager = focusManager,
				focusRequester = focusRequester
			)

			BigButton(
				text = "Next", action = {
					addKey(
						"$croppedPath///$pwd",
						seedName
					)
				},
				isDisabled = passwordCheck.value != pwd
			)
		}
	}
	DisposableEffect(Unit) {
		// if (signerDataModel.modalData.value?.optBoolean("keyboard") == true) {
		focusRequester.requestFocus()
		// }
		onDispose { focusManager.clearFocus() }
	}
}
