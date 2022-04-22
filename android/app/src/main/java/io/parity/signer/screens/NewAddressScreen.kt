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
import io.parity.signer.models.DerivationCheck
import io.parity.signer.ui.theme.Text600
import io.parity.signer.uniffi.*
import org.json.JSONObject

@Composable
fun NewAddressScreen(
	deriveKey: MDeriveKey,
	signerDataModel: SignerDataModel,
	increment: Boolean
) {
	val derivationPath = remember { mutableStateOf("") }
	val buttonGood = remember { mutableStateOf(false) }
	val whereTo = remember { mutableStateOf<DerivationDestination?>(null) }
	val collision = remember { mutableStateOf<Address?>(null) }
	val seedName = deriveKey.seedName
	val networkSpecKey = deriveKey.networkSpecsKey
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
		// TODO: Another type conversion MDeriveKey -> network
		//  NetworkCard(deriveKey)
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
						DerivationDestination.PIN -> {
							signerDataModel.addKey(
								path = derivationPath.value,
								seedName = seedName
							)
						}
						DerivationDestination.PWD -> {
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
						DerivationDestination.PIN -> {
							signerDataModel.addKey(
								path = derivationPath.value,
								seedName = seedName
							)
						}
						DerivationDestination.PWD -> {
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
		if (deriveKey.keyboard) {
			focusRequester.requestFocus()
		}
		derivationPath.value = deriveKey.suggestedDerivation
		deriveKey.derivationCheck?.let {
			derivationState.fromFFI(it)
		}
		onDispose { focusManager.clearFocus() }
	}
}
