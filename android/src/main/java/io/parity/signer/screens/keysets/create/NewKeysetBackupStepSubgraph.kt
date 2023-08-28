package io.parity.signer.screens.keysets.create

import androidx.activity.compose.BackHandler
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
import io.parity.signer.screens.keysets.create.backupstepscreens.NewKeySetBackupBottomSheet
import io.parity.signer.screens.keysets.create.backupstepscreens.NewKeySetBackupScreen
import io.parity.signer.screens.keysets.create.backupstepscreens.NewKeySetSelectNetworkScreen
import io.parity.signer.ui.BottomSheetWrapperRoot


@Composable
fun NewKeysetStepSubgraph(
	navController: NavHostController,
) {

	//background
	Box(//todo remove when rust navigation not in place yet
		modifier = Modifier
			.fillMaxSize(1f)
			.statusBarsPadding()
			.background(MaterialTheme.colors.background)
	)

	val vm: NewKeysetNameViewModel = viewModel()
	var seedName by rememberSaveable() {
		mutableStateOf("")
	}
	val seedPhrase = rememberSaveable() {
		vm.createNewSeedPhrase() ?: run {
			navController.popBackStack()
			""
		}
	}

	NavHost(
		navController = navController,
		startDestination = NewKeySetBackupStepSubgraph.NewKeySetName,
	) {

		composable(NewKeySetBackupStepSubgraph.NewKeySetName) {
			NewKeySetNameScreen(
				prefilledName = seedName,
				onBack = { navController.popBackStack() },
				onNextStep = {
					seedName = it
					navController.navigate(
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
					navController.navigate(
						NewKeySetBackupStepSubgraph.NewKeySetBackupConfirmation
					)
				},
				onBack = { navController.popBackStack() },
				modifier = Modifier.statusBarsPadding(),
			)
		}
		composable(NewKeySetBackupStepSubgraph.NewKeySetBackupConfirmation) {
			NewKeySetBackupScreen(
				seedPhrase = seedPhrase,
				onProceed = {
					navController.navigate(
						NewKeySetBackupStepSubgraph.NewKeySetBackupConfirmation
					)
				},
				onBack = { navController.popBackStack() },
				modifier = Modifier.statusBarsPadding(),
			)
			BottomSheetWrapperRoot(onClosedAction = navController::popBackStack) {
				NewKeySetBackupBottomSheet(
					onProceed = {
						navController.navigate(NewKeySetBackupStepSubgraph.NewKeySetSelectNetworks) {
							popUpTo(NewKeySetBackupStepSubgraph.NewKeySetBackup)
						}
					},
					onCancel = navController::popBackStack,
				)
			}
			BackHandler(onBack = navController::popBackStack)
		}
		composable(NewKeySetBackupStepSubgraph.NewKeySetSelectNetworks) {
			NewKeySetSelectNetworkScreen(
				seedName = seedName,
				seedPhrase = seedPhrase,
				onSuccess = {
					navController.popBackStack(
						NewKeySetBackupStepSubgraph.NewKeySetName, true
					)
				},
				onBack = navController::popBackStack,
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
