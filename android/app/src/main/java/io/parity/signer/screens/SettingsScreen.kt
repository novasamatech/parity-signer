package io.parity.signer.screens

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.material.Button
import androidx.compose.material.ButtonDefaults
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.ui.Modifier
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
				Row(
					Modifier.clickable { signerDataModel.wipe()
						signerDataModel.totalRefresh()}
				) { Text("Wipe Signer") }
				Row( Modifier.clickable{ signerDataModel.jailbreak() }
				) { Text("Wipe general certificate") }
			}
		}
		SettingsModal.History -> {
			HistoryModal(signerDataModel)
		}
	}
}
