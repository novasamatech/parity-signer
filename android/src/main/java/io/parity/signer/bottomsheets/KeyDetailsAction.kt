package io.parity.signer.bottomsheets

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.padding
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.alerts.AndroidCalledConfirm
import io.parity.signer.components.BigButton
import io.parity.signer.components.HeaderBar
import io.parity.signer.models.LocalNavRequest
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.navigate
import io.parity.signer.ui.theme.Bg000
import io.parity.signer.ui.theme.modal
import io.parity.signer.uniffi.Action

@Composable
fun KeyDetailsAction(signerDataModel: SignerDataModel) {
	var confirmForget by remember { mutableStateOf(false) }
	var confirmExport by remember { mutableStateOf(false) }

	Column(
		Modifier.clickable { signerDataModel.navigator.backAction() }
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
				// Don't show `Export Private Key` if intermediate state is broken or when key is password protected
				if (signerDataModel.lastOpenedKeyDetails?.address?.hasPwd == false) {
					BigButton(
						text = stringResource(R.string.menu_option_export_private_key),
						isShaded = true,
						isDangerous = false,
						action = {
							confirmExport = true
						}
					)
				}
				BigButton(
					text = stringResource(R.string.menu_option_forget_delete_key),
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
		header = stringResource(R.string.remove_key_confirm_title),
		text = stringResource(R.string.remove_key_confirm_text),
		back = { confirmForget = false },
		forward = { signerDataModel.navigate(Action.REMOVE_KEY) },
		backText = stringResource(R.string.generic_cancel),
		forwardText = stringResource(R.string.remove_key_confirm_cta)
	)
	AndroidCalledConfirm(
		show = confirmExport,
		header = stringResource(R.string.export_private_key_confirm_title),
		text = stringResource(R.string.export_private_key_confirm_text),
		back = { confirmExport = false },
		forward = {
			confirmExport = false
			signerDataModel.navigator.navigate(
				LocalNavRequest.ShowExportPrivateKey(signerDataModel.lastOpenedKeyDetails!!.pubkey)
			)
		},
		backText = stringResource(R.string.generic_cancel),
		forwardText = stringResource(R.string.export_private_key_confirm_cta)
	)
}
