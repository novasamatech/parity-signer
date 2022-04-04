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
import org.json.JSONObject

/**
 * Settings screen; General purpose stuff like legal info, networks management
 * and history should be here. This is final point in navigation:
 * all subsequent interactions should be in modals or drop-down menus
 */
@Composable
fun SettingsScreen(
	screenData: JSONObject,
	button: (ButtonID) -> Unit,
	alertState: ShieldAlert?,
	strongbox: String,
	version: String,
	wipeToFactory: () -> Unit
) {
	var confirm by remember { mutableStateOf(false) }

	Column(
		verticalArrangement = Arrangement.spacedBy(4.dp)
	) {
		Row(Modifier.clickable { button(ButtonID.ManageNetworks) }) {
			SettingsCardTemplate(text = "Networks")
		}
		Row(Modifier.clickable {
			if (alertState == ShieldAlert.None)
				button(ButtonID.BackupSeed)
			else
				button(ButtonID.Shield)
		}) {
			SettingsCardTemplate(text = "Backup keys")
		}
		Column(
			Modifier
				.padding(12.dp)
				.clickable { button(ButtonID.ViewGeneralVerifier) }
		) {
			Row {
				Text(
					"Verifier certificate",
					style = MaterialTheme.typography.h1,
					color = MaterialTheme.colors.Text600
				)
				Spacer(Modifier.weight(1f))
			}

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
					Identicon(identicon = screenData.optString("identicon"))
					Spacer(Modifier.width(4.dp))
					Column {
						Text(
							screenData.optString("public_key").abbreviateString(8),
							style = CryptoTypography.body2,
							color = MaterialTheme.colors.Crypto400
						)
						Text(
							"encryption: " + screenData.optString("encryption"),
							style = CryptoTypography.body1,
							color = MaterialTheme.colors.Text400
						)
					}
				}
			}

		}
		Row(
			Modifier.clickable {
				confirm = true
			}
		) { SettingsCardTemplate(text = "Wipe signer", danger = true) }
		Row(Modifier.clickable { button(ButtonID.ShowDocuments) }) {
			SettingsCardTemplate(text = "About")
		}
		SettingsCardTemplate(
			"Hardware seed protection: $strongbox",
			withIcon = false,
			withBackground = false
		)
		SettingsCardTemplate(
			"Version: $version",
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
			wipeToFactory()
		},
		backText = "Cancel",
		forwardText = "Wipe"
	)
}
