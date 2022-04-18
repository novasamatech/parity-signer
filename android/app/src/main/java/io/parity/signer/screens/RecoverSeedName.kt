package io.parity.signer.screens

import android.app.Notification
import androidx.compose.foundation.layout.*
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.DisposableEffect
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.platform.LocalFocusManager
import androidx.compose.ui.unit.dp
import io.parity.signer.components.BigButton
import io.parity.signer.components.HeadingOverline
import io.parity.signer.components.SingleTextInput
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.decode64
import io.parity.signer.models.encode64
import io.parity.signer.ui.theme.Text600
import io.parity.signer.uniffi.Action

@Composable
fun RecoverSeedName(
	button: (action: Action, details: String) -> Unit,
	signerDataModel: SignerDataModel
) {
	val seedName = remember { mutableStateOf("") }
	val focusManager = LocalFocusManager.current
	val focusRequester = remember { FocusRequester() }

	Column(
		horizontalAlignment = Alignment.CenterHorizontally,
		verticalArrangement = Arrangement.Top,
		modifier = Modifier
			.fillMaxSize(1f)
			.padding(20.dp)
	) {
		Row {
			HeadingOverline("DISPLAY NAME")
			Spacer(Modifier.weight(1f))
		}
		SingleTextInput(
			content = seedName,
			update = {
				seedName.value = it
				signerDataModel.clearError()
			},
			onDone = {
				if (seedName.value.isNotBlank() && signerDataModel.seedNames.value?.contains(seedName.value.encode64()) == false) {
					button(Action.GO_FORWARD, seedName.value.encode64())
				}
			},
			isCrypto = true,
			capitalize = true,
			focusManager = focusManager,
			focusRequester = focusRequester
		)

		Text(
			"Display name visible only on this device",
			style = MaterialTheme.typography.caption,
			color = MaterialTheme.colors.Text600
		)
		Spacer(Modifier.height(20.dp))
		BigButton(
			text = "Next",
			action = {
				focusManager.clearFocus()
				button(Action.GO_FORWARD, seedName.value.encode64())
			},
			isDisabled = seedName.value.isBlank() || (signerDataModel.seedNames.value?.contains(seedName.value.encode64()) != false)
		)
	}

	DisposableEffect(Unit) {
		if (signerDataModel.screenData.value?.optBoolean("keyboard") == true) {
			focusRequester.requestFocus()
		}
		seedName.value =
			signerDataModel.screenData.value?.optString("seed_name")?.decode64() ?: ""
		onDispose { focusManager.clearFocus() }
	}
}
