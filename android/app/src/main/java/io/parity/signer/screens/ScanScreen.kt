package io.parity.signer.screens

import androidx.compose.runtime.Composable
import androidx.compose.runtime.livedata.observeAsState
import io.parity.signer.modals.*
import io.parity.signer.models.SignerDataModel

/**
 * This is a simple screen with a single button that
 * triggers transaction sequence starting with camera
 */
@Composable
fun ScanScreen(signerDataModel: SignerDataModel) {
	CameraModal(signerDataModel)
}

