package io.parity.signer.screens

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import io.parity.signer.ButtonID
import io.parity.signer.ShieldAlert
import io.parity.signer.alerts.AndroidCalledConfirm
import io.parity.signer.components.Identicon
import io.parity.signer.components.SettingsCardTemplate
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.abbreviateString
import io.parity.signer.models.pushButton
import io.parity.signer.ui.theme.*

/**
 * Settings screen; General purpose stuff like legal info, networks management
 * and history should be here. This is final point in navigation:
 * all subsequent interactions should be in modals or drop-down menus
 */
@Composable
fun SettingsScreen(signerDataModel: SignerDataModel) {
	var confirm by remember { mutableStateOf(false) }

	Column(
		verticalArrangement = Arrangement.spacedBy(4.dp)
	) {
		Row(Modifier.clickable { signerDataModel.pushButton(ButtonID.ManageNetworks) }) {
			SettingsCardTemplate(text = "Networks")
		}
		Row(Modifier.clickable {
			if (signerDataModel.alertState.value == ShieldAlert.None)
				signerDataModel.pushButton(ButtonID.BackupSeed)
			else
				signerDataModel.pushButton(ButtonID.Shield)
		}) {
			SettingsCardTemplate(text = "Backup keys")
		}
		Column(
			Modifier
				.padding(12.dp)
				.clickable { signerDataModel.pushButton(ButtonID.ViewGeneralVerifier) }
		) {
			Row {
				Text(
					"Verifier certificate",
					style = MaterialTheme.typography.h1,
					color = MaterialTheme.colors.Text600
				)
				Spacer(Modifier.weight(1f))
			}
			signerDataModel.screenData.value?.let {
				Surface(
					shape = MaterialTheme.shapes.small,
					color = MaterialTheme.colors.Bg200,
					modifier = Modifier.padding(8.dp)
				) {
					Row(
						verticalAlignment = Alignment.CenterVertically,
						modifier = Modifier
							.padding(8.dp)
							.fillMaxWidth(1f)
					) {
						Identicon(identicon = it.optString("identicon"))
						Spacer(Modifier.width(4.dp))
						Column {
							Text(
								it.optString("public_key").abbreviateString(8),
								style = CryptoTypography.body2,
								color = MaterialTheme.colors.Crypto400
							)
							Text(
								"encryption: " + it.optString("encryption"),
								style = CryptoTypography.body1,
								color = MaterialTheme.colors.Text400
							)
						}
					}
				}
			}
		}
		Row(
			Modifier.clickable {
				confirm = true
			}
		) { SettingsCardTemplate(text = "Wipe signer", danger = true) }
		Row(Modifier.clickable { signerDataModel.pushButton(ButtonID.ShowDocuments) }) {
			SettingsCardTemplate(text = "About")
		}
		SettingsCardTemplate(
			"Hardware seed protection: " + signerDataModel.isStrongBoxProtected()
				.toString(),
			withIcon = false,
			withBackground = false
		)
		SettingsCardTemplate(
			"Version: " + signerDataModel.getAppVersion(),
			withIcon = false,
			withBackground = false
		)
	}

	AndroidCalledConfirm(
		show = confirm,
		header = "Wipe ALL data?",
		text = "Factory reset the Signer app. This operation can not be reverted!",
		back = { confirm = false },
		forward = {
			signerDataModel.authentication.authenticate(signerDataModel.activity) {
				signerDataModel.wipe()
				signerDataModel.totalRefresh()
			}
		},
		backText = "Cancel",
		forwardText = "Wipe"
	)
}
