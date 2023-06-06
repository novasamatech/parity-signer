package io.parity.signer.screens.keysets.create

import androidx.compose.runtime.Composable
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import io.parity.signer.domain.Callback
import io.parity.signer.domain.NetworkModel
import io.parity.signer.screens.keysets.create.backupstepscreens.NewKeySetBackupScreen
import io.parity.signer.screens.keysets.create.backupstepscreens.NewKeySetSelectNetworkScreen
import io.parity.signer.screens.keysets.create.backupstepscreens.NewSeedBackupModel


@Composable
fun NewKeySetBackupStepSubgraph(
	model: NewSeedBackupModel,
	onBackExit: Callback,
	onCreateKeySet: (String, String) -> Unit
) {
	val navController = rememberNavController()
	NavHost(
		navController = navController,
		startDestination = NewKeySetBackupStepSubgraph.NewKeySetBackup,
	) {
		composable(NewKeySetBackupStepSubgraph.NewKeySetBackup) {
			NewKeySetBackupScreen(
				model = model,
				onProceed = {
					navController.navigate(NewKeySetBackupStepSubgraph.NewKeySetBackupConfirmation)
				},
				onBack = onBackExit,
			)
			//todo dmitry handle back
		}
		composable(NewKeySetBackupStepSubgraph.NewKeySetBackupConfirmation) {
			NewKeySetBackupScreen(
				model = model,
				onProceed = {
					navController.navigate(NewKeySetBackupStepSubgraph.NewKeySetSelectNetworks)
				},
				onBack = { navController.popBackStack() },
			)
		}
		composable(NewKeySetBackupStepSubgraph.NewKeySetSelectNetworks) {
			NewKeySetSelectNetworkScreen(
				networks = emptyList<NetworkModel>(),//todo dmitry check it out as in create derivation
				onProceed = {}, //todo dmitry implement
				onBack = { navController.popBackStack() },
			)
		}
	}
}

internal object NewKeySetBackupStepSubgraph {
	const val NewKeySetBackup = "new_keyset_backup_main"
	const val NewKeySetBackupConfirmation = "new_keyset_backup_confirmation"
	const val NewKeySetSelectNetworks = "new_keyset_select_networkis"
}
