package io.parity.signer.screens

import androidx.compose.foundation.layout.Column
import androidx.compose.material.Button
import androidx.compose.material.ButtonDefaults
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.livedata.observeAsState
import io.parity.signer.SettingsModal
import io.parity.signer.modals.HistoryModal
import io.parity.signer.models.SignerDataModel

/**
 * Settings screen; General purpose stuff like legal info, networks management
 * and history should be here. This is final point in navigation:
 * all subsequent interactions should be in modals or drop-down menus
 */
@Composable
fun SettingsScreen(signerDataModel: SignerDataModel) {
	val settingsModal = signerDataModel.settingsModal.observeAsState()

	when (settingsModal.value) {
		SettingsModal.None -> {
			Column {
				Text(text = "Settings")
				Button(
					colors = ButtonDefaults.buttonColors(
						backgroundColor = MaterialTheme.colors.secondary,
						contentColor = MaterialTheme.colors.onSecondary,
					),
					onClick = {
						signerDataModel.wipe()
						signerDataModel.totalRefresh()
					}
				) { Text("Wipe Signer") }
				Button(
					colors = ButtonDefaults.buttonColors(
						backgroundColor = MaterialTheme.colors.secondary,
						contentColor = MaterialTheme.colors.onSecondary,
					),
					onClick = { signerDataModel.jailbreak() }
				) { Text("Wipe general certificate") }
			}
		}
		SettingsModal.History -> {
			HistoryModal(signerDataModel)
		}
	}
}
