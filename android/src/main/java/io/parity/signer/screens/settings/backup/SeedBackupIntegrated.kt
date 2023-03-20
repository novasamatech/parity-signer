package io.parity.signer.screens.settings.backup

import androidx.activity.compose.BackHandler
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember


@Composable
fun SeedBackupIntegratedScreen() {
	val selectedSeed = remember { mutableStateOf<String?>(null) }

	BackHandler(enabled = selectedSeed.value != null) {
		selectedSeed.value = null
	}

	// content

	// bottomsheet

}
