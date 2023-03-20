package io.parity.signer.screens.settings.backup

import androidx.activity.compose.BackHandler
import androidx.compose.runtime.*
import io.parity.signer.domain.Navigator


@Composable
fun SeedBackupIntegratedScreen(rootNavigator: Navigator) {
	var selectedSeed by remember { mutableStateOf<String?>(null) }

	BackHandler(enabled = selectedSeed != null) {
		selectedSeed = null
	}

	// content
	SeedBackupScreen(rootNavigator) { seed ->
		selectedSeed = seed
	}

	//todo dmitry make it full screen and paddings for all other settigns screens
	// bottomsheet
	selectedSeed?.let { selectedSeedValue ->
		SeedBackupFullOverlayBottomSheet(
			seedName = selectedSeedValue,
			getSeedPhraseForBackup = {_ -> null}, //todo dmitry
			onClose = { selectedSeed = null }
		)
	}
}
