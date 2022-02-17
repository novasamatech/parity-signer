package io.parity.signer.screens

import androidx.compose.foundation.layout.*
import androidx.compose.material.*
import androidx.compose.runtime.*
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.platform.LocalFocusManager
import androidx.compose.ui.unit.dp
import io.parity.signer.ButtonID
import io.parity.signer.components.BigButton
import io.parity.signer.components.SingleTextInput
import io.parity.signer.models.SignerDataModel
import io.parity.signer.ui.theme.SignalDanger
import io.parity.signer.ui.theme.Text600

@Composable
fun RecoverSeedName(
	button: (button: ButtonID, details: String) -> Unit,
	signerDataModel: SignerDataModel
) {
	val seedName = remember { mutableStateOf("") }
	val lastError = signerDataModel.lastError.observeAsState()
	val focusManager = LocalFocusManager.current
	val focusRequester = remember { FocusRequester() }

	Column(
		horizontalAlignment = Alignment.CenterHorizontally,
		verticalArrangement = Arrangement.Center,
		modifier = Modifier
			.fillMaxSize(1f)
			.padding(20.dp)
	) {
		Text(
			"DISPLAY NAME",
			style = MaterialTheme.typography.overline,
			color = MaterialTheme.colors.Text600
		)
		Text(
			lastError.value.toString(),
			style = MaterialTheme.typography.caption,
			color = MaterialTheme.colors.SignalDanger
		)
		SingleTextInput(
			content = seedName,
			update = {
				seedName.value = it
				signerDataModel.clearError()
			},
			onDone = {
				if (seedName.value.isNotBlank() && !seedName.value.contains(",")) {
					button(ButtonID.GoForward, seedName.value)
				}
			},
			capitalize = true,
			focusManager = focusManager,
			focusRequester = focusRequester
		)

		Text(
			"Display name visible only to you",
			style = MaterialTheme.typography.caption,
			color = MaterialTheme.colors.Text600
		)
		BigButton(
			text = "Next",
			action = {
				focusManager.clearFocus()
				button(ButtonID.GoForward, seedName.value)
			},
			isDisabled = seedName.value.isBlank() || seedName.value.contains(",")
		)


	}
	DisposableEffect(Unit) {
		if (signerDataModel.screenData.value?.optBoolean("keyboard") == true) {
			focusRequester.requestFocus()
		}
		seedName.value =
			signerDataModel.screenData.value?.optString("seed_name") ?: ""
		onDispose { focusManager.clearFocus() }
	}
}
