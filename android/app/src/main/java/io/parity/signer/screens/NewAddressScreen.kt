package io.parity.signer.screens

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
import org.json.JSONObject
import uniffi.signer.Action
import uniffi.signer.substratePathCheck

@Composable
fun NewAddressScreen(signerDataModel: SignerDataModel, increment: Boolean) {
	val screenData = signerDataModel.screenData.value ?: JSONObject()
	val derivationPath = remember { mutableStateOf("") }
	val buttonGood = remember { mutableStateOf(false) }
	val whereTo = remember { mutableStateOf<DeriveDestination?>(null) }
	val collision = remember { mutableStateOf<JSONObject?>(null) }
	val seedName = signerDataModel.screenData.value?.optString("seed_name") ?: ""
	val networkSpecKey =
		signerDataModel.screenData.value?.optString("network_specs_key") ?: ""
	var derivationState by remember(buttonGood, whereTo, collision) {
		mutableStateOf(DerivationCheck(
			buttonGood,
			whereTo,
			collision
		) {
			substratePathCheck(
				seedName,
				it,
				networkSpecKey,
				signerDataModel.dbName
			)
		})
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
		NetworkCard(network = screenData)
		SingleTextInput(
			content = derivationPath,
			update = {
				derivationPath.value = it
				derivationState.check(it)
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
				if (derivationState.buttonGood.value) {
					when (derivationState.whereTo.value) {
						DeriveDestination.pin -> {
							signerDataModel.addKey(
								path = derivationPath.value,
								seedName = seedName
							)
						}
						DeriveDestination.pwd -> {
							signerDataModel.pushButton(
								Action.CHECK_PASSWORD,
								details = derivationPath.value
							)
						}
						null -> {}
					}
				}
			},
			focusManager = focusManager,
			focusRequester = focusRequester
		)
		collision.value?.let {
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
					when (derivationState.whereTo.value) {
						DeriveDestination.pin -> {
							signerDataModel.addKey(
								path = derivationPath.value,
								seedName = seedName
							)
						}
						DeriveDestination.pwd -> {
							signerDataModel.pushButton(
								Action.CHECK_PASSWORD,
								details = derivationPath.value
							)
						}
						null -> {}
					}
				},
				isDisabled = !derivationState.buttonGood.value
			)
		}
	}
	DisposableEffect(Unit) {
		if (signerDataModel.screenData.value?.optBoolean("keyboard") == true) {
			focusRequester.requestFocus()
		}
		derivationPath.value =
			signerDataModel.screenData.value?.optString("suggested_derivation")
				?: ""
		derivationState.fromJSON(signerDataModel.screenData.value ?: JSONObject())
		onDispose { focusManager.clearFocus() }
	}
}
