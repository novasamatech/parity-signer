package io.parity.signer.screens.error.wrongversion

import androidx.activity.compose.BackHandler
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.NavController
import io.parity.signer.domain.Callback
import io.parity.signer.domain.storage.toOperationResult
import io.parity.signer.screens.error.handleErrorAppState
import io.parity.signer.screens.settings.backup.SeedBackupFullOverlayBottomSheet

@Composable
fun FallbackRecoverPhraseScreenFull(
	onBack: Callback,
	navController: NavController
) {
	var selectedSeed by remember { mutableStateOf<String?>(null) }

	BackHandler(enabled = selectedSeed != null) {
		selectedSeed = null
	}

	val viewModel: FallbackRecoverPhraseViewModel = viewModel()

	//screen
	Box(modifier = Modifier.statusBarsPadding()) {
		FallbackRecoverPhraseScreen(
			seedList = viewModel.getSeedsList(),
			onSelected = { selectedSeed = it },
			onBack = onBack,
		)
	}

	// bottomsheet and it is full screen
	selectedSeed?.let { selectedSeedValue ->
		SeedBackupFullOverlayBottomSheet(
			seedName = selectedSeedValue,
			getSeedPhraseForBackup = { name ->
				viewModel.getSeedPhrase(name)
					.toOperationResult()
					.handleErrorAppState(navController)
			},
			onClose = { selectedSeed = null }
		)
	}
}
