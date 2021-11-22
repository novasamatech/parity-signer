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
 * The home screen with history; should show detailed history record on tap
 */
@Composable
fun HistoryScreen(signerDataModel: SignerDataModel) {
	//val settingsModal = signerDataModel.settingsModal.observeAsState()

	HistoryModal(signerDataModel)
}
