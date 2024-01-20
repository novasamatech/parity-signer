package io.parity.signer.screens.keysets.create

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.imePadding
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.material.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.NavHostController
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import io.parity.signer.domain.backend.AuthOperationResult
import io.parity.signer.domain.backend.toOperationResult
import io.parity.signer.domain.popUpToTop
import io.parity.signer.screens.error.handleErrorAppState
import io.parity.signer.screens.keysets.create.backupstepscreens.NewKeySetBackupBottomSheet
import io.parity.signer.screens.keysets.create.backupstepscreens.NewKeySetBackupScreen
import io.parity.signer.screens.keysets.create.backupstepscreens.NewKeySetSelectNetworkScreen
import io.parity.signer.ui.BottomSheetWrapperRoot
import io.parity.signer.ui.mainnavigation.CoreUnlockedNavSubgraph


@Composable
fun NewKeysetSubgraph(
	coreNavController: NavHostController,
) {

	val subgraphNavController = rememberNavController()

	val vm: NewKeysetNameViewModel = viewModel()
	var seedName by rememberSaveable() {
		mutableStateOf("")
	}
	val seedPhrase = rememberSaveable() {
		vm.createNewSeedPhrase().toOperationResult().handleErrorAppState(coreNavController) ?: ""
	}

	NavHost(
		navController = subgraphNavController,
		startDestination = NewKeySetBackupStepSubgraph.NewKeySetName,
	) {
		composable(NewKeySetBackupStepSubgraph.NewKeySetName) {
			NewKeySetNameScreen(
				prefilledName = seedName,
				onBack = { coreNavController.popBackStack() },
				onNextStep = {
					seedName = it
					subgraphNavController.navigate(
						NewKeySetBackupStepSubgraph.NewKeySetBackup
					)
				},
				modifier = Modifier
					.statusBarsPadding()
					.imePadding(),
			)
		}
		composable(NewKeySetBackupStepSubgraph.NewKeySetBackup) {
			NewKeySetBackupScreen(
				seedPhrase = seedPhrase,
				onProceed = {
					subgraphNavController.navigate(
						NewKeySetBackupStepSubgraph.NewKeySetBackupConfirmation
					)
				},
				onBack = { subgraphNavController.popBackStack() },
				modifier = Modifier.statusBarsPadding(),
			)
		}
		composable(NewKeySetBackupStepSubgraph.NewKeySetBackupConfirmation) {
			NewKeySetBackupScreen(
				seedPhrase = seedPhrase,
				onProceed = {
					subgraphNavController.navigate(
						NewKeySetBackupStepSubgraph.NewKeySetBackupConfirmation
					)
				},
				onBack = { subgraphNavController.popBackStack() },
				modifier = Modifier.statusBarsPadding(),
			)
			BottomSheetWrapperRoot(onClosedAction = subgraphNavController::popBackStack) {
				NewKeySetBackupBottomSheet(
					onProceed = {
						subgraphNavController.navigate(NewKeySetBackupStepSubgraph.NewKeySetSelectNetworks) {
							popUpTo(NewKeySetBackupStepSubgraph.NewKeySetBackup)
						}
					},
					onCancel = subgraphNavController::popBackStack,
				)
			}
		}
		composable(NewKeySetBackupStepSubgraph.NewKeySetSelectNetworks) {
			NewKeySetSelectNetworkScreen(
				seedName = seedName,
				seedPhrase = seedPhrase,
				onSuccess = {
					coreNavController.navigate(
						CoreUnlockedNavSubgraph.KeySet.destination(
							seedName
						)
					) {
							popUpToTop(coreNavController)
					}
				},
				showError = { error: AuthOperationResult ->
					error.handleErrorAppState(coreNavController)
				},
				onBack = subgraphNavController::popBackStack,
			)
		}
	}
}

internal object NewKeySetBackupStepSubgraph {
	const val NewKeySetName = "new_keyset_name_select"
	const val NewKeySetBackup = "new_keyset_backup_main"
	const val NewKeySetBackupConfirmation = "new_keyset_backup_confirmation"
	const val NewKeySetSelectNetworks = "new_keyset_select_networkis"
}
