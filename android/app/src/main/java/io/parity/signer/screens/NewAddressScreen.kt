package io.parity.signer.screens

import android.util.Log
import androidx.compose.foundation.layout.*
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.platform.LocalFocusManager
import androidx.compose.ui.unit.dp
import io.parity.signer.components.*
import io.parity.signer.models.*
import io.parity.signer.ui.theme.Text600
import io.parity.signer.uniffi.*

@Composable
fun NewAddressScreen(
	deriveKey: MDeriveKey,
	button: (Action, String) -> Unit,
	pathCheck: (String, String, String) -> String,
	addKey: (String, String) -> Unit
) {
	val derivationPath = remember { mutableStateOf("") }
	val seedName = deriveKey.seedName
	val networkSpecKey = deriveKey.networkSpecsKey
	var derivationState by remember {
		mutableStateOf(
			DerivationCheck(
				false,
				null,
				null,
				null
			)
		)
	}
	val focusManager = LocalFocusManager.current
	val focusRequester = remember { FocusRequester() }

	Column(
		horizontalAlignment = Alignment.CenterHorizontally,
		verticalArrangement = Arrangement.Top,
		modifier = Modifier
			.padding(20.dp)
			.fillMaxSize()
	) {
		Row {
			HeaderBar(line1 = "Create new key", line2 = "For seed $seedName")
			Spacer(Modifier.weight(1f))
		}
		// TODO: Another type conversion MDeriveKey -> network
		//  NetworkCard(deriveKey)
		SingleTextInput(
			content = derivationPath,
			update = {
				derivationPath.value = it
				derivationPath.value = pathCheck(it)
			},
			prefix = {
				Text(
					seedName.decode64(),
					style = MaterialTheme.typography.body2,
					color = MaterialTheme.colors.Text600
				)
			},
			isCrypto = true,
			isCryptoColor = true,
			capitalize = false,
			onDone = {
				focusManager.clearFocus()
				if (derivationState.buttonGood) {
					when (derivationState.whereTo) {
						DerivationDestination.PIN -> {
							addKey(
								derivationPath.value,
								seedName
							)
						}
						DerivationDestination.PWD -> {
							button(
								Action.CHECK_PASSWORD,
								derivationPath.value
							)
						}
						null -> {}
					}
				}
			},
			focusManager = focusManager,
			focusRequester = focusRequester
		)
		derivationState.collision.let {
			Column(
				Modifier.fillMaxWidth(1f)
			) {
				Text("This key already exists:")
				KeyCard(identity = it)
			}
		}
		Spacer(Modifier.height(20.dp))
		Row {
			BigButton(
				text = "Next",
				action = {
					when (derivationState.whereTo) {
						DerivationDestination.PIN -> {
							addKey(
								derivationPath.value,
								seedName
							)
						}
						DerivationDestination.PWD -> {
							button(
								Action.CHECK_PASSWORD,
								derivationPath.value
							)
						}
						null -> {}
					}
				},
				isDisabled = !derivationState.buttonGood
			)
		}
	}
	DisposableEffect(Unit) {
		if (deriveKey.keyboard) {
			focusRequester.requestFocus()
		}
		derivationPath.value = deriveKey.suggestedDerivation
		deriveKey.derivationCheck?.let {
			derivationState = it
		}
		onDispose { focusManager.clearFocus() }
	}
}
