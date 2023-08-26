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
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import io.parity.signer.domain.Callback
import io.parity.signer.screens.keysets.create.backupstepscreens.NewKeySetBackupBottomSheet
import io.parity.signer.screens.keysets.create.backupstepscreens.NewKeySetBackupScreen
import io.parity.signer.screens.keysets.create.backupstepscreens.NewKeySetSelectNetworkScreen
import io.parity.signer.ui.BottomSheetWrapperRoot


@Composable
fun NewKeysetStepSubgraph(
	onExitFlow: Callback,
) {

	//background
	Box(//todo remove when rust navigation not in place yet
		modifier = Modifier
			.fillMaxSize(1f)
			.statusBarsPadding()
			.background(MaterialTheme.colors.background)
	)

	var seedName by rememberSaveable() {
		mutableStateOf("")
	}
	val seedPhrase = rememberSaveable() {
		"some value" //todo dmitry generate it from view model
	}

	val navController = rememberNavController()
	NavHost(
		navController = navController,
		startDestination = NewKeySetBackupStepSubgraph.NewKeySetName,
	) {

		composable(NewKeySetBackupStepSubgraph.NewKeySetName) {
			NewKeySetNameScreen(
				onBack = { navController.popBackStack() },
				onNextStep = {
					seedName = it   //todo dmitry pass it back as well
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
				onBack = onExitFlow,
				modifier = Modifier.statusBarsPadding(),
			)
			BackHandler(onBack = onExitFlow)
		}
		composable(NewKeySetBackupStepSubgraph.NewKeySetBackupConfirmation) {
			NewKeySetBackupScreen(
				seedPhrase = seedPhrase,
				onProceed = {
					navController.navigate(
						NewKeySetBackupStepSubgraph.NewKeySetBackupConfirmation
					)
				},
				onBack = onExitFlow,
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
				onSuccess = onExitFlow,
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
