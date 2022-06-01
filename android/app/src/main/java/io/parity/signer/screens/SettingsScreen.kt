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
import io.parity.signer.alerts.AndroidCalledConfirm
import io.parity.signer.components.Identicon
import io.parity.signer.components.SettingsCardTemplate
import io.parity.signer.models.AlertState
import io.parity.signer.models.abbreviateString
import io.parity.signer.ui.theme.*
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.MSettings

/**
 * Settings screen; General purpose stuff like legal info, networks management
 * and history should be here. This is final point in navigation:
 * all subsequent interactions should be in modals or drop-down menus
 */
@Composable
fun SettingsScreen(
	settings: MSettings,
	button1: (Action) -> Unit,
	isStrongBoxProtected: () -> Boolean,
	getAppVersion: () -> String,
	wipeToFactory: () -> Unit,
	alertState: State<AlertState?>
) {
	var confirm by remember { mutableStateOf(false) }

	Column(
		verticalArrangement = Arrangement.spacedBy(4.dp)
	) {
		Row(Modifier.clickable { button1(Action.MANAGE_NETWORKS) }) {
			SettingsCardTemplate(text = "Networks")
		}
		Row(
			Modifier.clickable {
				if (alertState.value == AlertState.None)
					button1(Action.BACKUP_SEED)
				else
					button1(Action.SHIELD)
			}
		) {
			SettingsCardTemplate(text = "Backup keys")
		}
		Column(
			Modifier
				.padding(12.dp)
				.clickable { button1(Action.VIEW_GENERAL_VERIFIER) }
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
					Identicon(identicon = settings.identicon ?: listOf())
					Spacer(Modifier.width(4.dp))
					Column {
						Text(
							settings.publicKey ?: "".abbreviateString(8),
							style = CryptoTypography.body2,
							color = MaterialTheme.colors.Crypto400
						)
						Text(
							"encryption: " + settings.encryption,
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
		Row(Modifier.clickable { button1(Action.SHOW_DOCUMENTS) }) {
			SettingsCardTemplate(text = "About")
		}
		SettingsCardTemplate(
			"Hardware seed protection: " + isStrongBoxProtected()
				.toString(),
			withIcon = false,
			withBackground = false
		)
		SettingsCardTemplate(
			"Version: " + getAppVersion(),
			withIcon = false,
			withBackground = false
		)
	}

	AndroidCalledConfirm(
		show = confirm,
		header = "Wipe ALL data?",
		text = "Factory reset the Signer app. This operation can not be reverted!",
		back = { confirm = false },
		forward = { wipeToFactory() },
		backText = "Cancel",
		forwardText = "Wipe"
	)
}
