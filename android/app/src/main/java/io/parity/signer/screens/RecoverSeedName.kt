package io.parity.signer.screens

import androidx.compose.foundation.layout.*
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
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
import io.parity.signer.components.HeadingOverline
import io.parity.signer.components.SingleTextInput
import io.parity.signer.ui.theme.Text600
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.MRecoverSeedName

@Composable
fun RecoverSeedName(
	recoverSeedName: MRecoverSeedName,
	button: (action: Action, details: String) -> Unit,
	seedNames: Array<String>
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
			},
			onDone = {
				if (seedName.value.isNotBlank() && !seedNames.contains(
						seedName.value
					)
				) {
					button(Action.GO_FORWARD, seedName.value)
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
				button(Action.GO_FORWARD, seedName.value)
			},
			isDisabled = seedName.value.isBlank() || seedNames.contains(
				seedName.value
			)
		)
	}

	DisposableEffect(Unit) {
		if (recoverSeedName.keyboard) {
			focusRequester.requestFocus()
		}
		seedName.value =
			recoverSeedName.seedName
		onDispose { focusManager.clearFocus() }
	}
}
