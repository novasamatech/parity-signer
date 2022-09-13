package io.parity.signer.bottomsheets

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.padding
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import io.parity.signer.alerts.AndroidCalledConfirm
import io.parity.signer.components.BigButton
import io.parity.signer.components.HeaderBar
import io.parity.signer.models.*
import io.parity.signer.ui.theme.Bg000
import io.parity.signer.ui.theme.modal
import io.parity.signer.uniffi.Action

@Composable
fun KeyDetailsAction(signerDataModel: SignerDataModel) {
	var confirmForget by remember { mutableStateOf(false) }
	var confirmExport by remember { mutableStateOf(false) }

	Column (
		Modifier.clickable { signerDataModel.navigate(Action.GO_BACK) }
		) {
		Spacer(Modifier.weight(1f))
		Surface(
			color = MaterialTheme.colors.Bg000,
			shape = MaterialTheme.shapes.modal
		) {
			Column(
				modifier = Modifier.padding(20.dp)
			) {
				HeaderBar(line1 = "KEY MENU", line2 = "Select action")
				if (FeatureFlags.isEnabled(FeatureOption.EXPORT_SECRET_KEY)) {
					BigButton(
						text = "Export Private Key",
						isShaded = true,
						isDangerous = false,
						action = {
							confirmExport = true
						}
					)
				}
				BigButton(
					text = "Forget this key forever",
					isShaded = true,
					isDangerous = true,
					action = {
						confirmForget = true
					}
				)
			}
		}
	}
	AndroidCalledConfirm(
		show = confirmForget,
		header = "Forget this key?",
		text = "This key will be removed for this network. Are you sure?",
		back = { confirmForget = false },
		forward = { signerDataModel.navigate(Action.REMOVE_KEY) },
		backText = "Cancel",
		forwardText = "Remove key"
	)
	AndroidCalledConfirm(
		show = confirmExport,
		header = "Export Private Key",
		text = "A private key can be used to sign transactions. This key will be marked as a hot key after export.",
		back = { confirmExport = false },
		forward = { signerDataModel.navigator.navigate(LocalNavRequest.ShowExportPrivateKey()) }, //TODO dmitry show PrivateKeyExportBottomSheet
		backText = "Cancel",
		forwardText = "Export Private Key"
	)
}
