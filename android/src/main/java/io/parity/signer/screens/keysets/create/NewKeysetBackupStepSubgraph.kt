package io.parity.signer.screens.keysets.create

import androidx.activity.compose.BackHandler
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.material.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import io.parity.signer.domain.Navigator
import io.parity.signer.screens.keysets.create.backupstepscreens.NewKeySetBackupBottomSheet
import io.parity.signer.screens.keysets.create.backupstepscreens.NewKeySetBackupScreen
import io.parity.signer.screens.keysets.create.backupstepscreens.NewKeySetSelectNetworkScreen
import io.parity.signer.screens.keysets.create.backupstepscreens.NewSeedBackupModel
import io.parity.signer.ui.BottomSheetWrapperRoot


@Composable
fun NewKeySetBackupStepSubgraph(
	model: NewSeedBackupModel,
	rootNavigator: Navigator,
) {

	//background
	Box(modifier = Modifier.fillMaxSize(1f).statusBarsPadding()
		.background(MaterialTheme.colors.background))


	val navController = rememberNavController()
	NavHost(
		navController = navController,
		startDestination = NewKeySetBackupStepSubgraph.NewKeySetBackup,
	) {
		val onProceedFromBackupInitial = { //to cache so screen can be taked from caches during navigation
			navController.navigate(
				NewKeySetBackupStepSubgraph.NewKeySetBackupConfirmation
			)
		}
		composable(NewKeySetBackupStepSubgraph.NewKeySetBackup) {
			NewKeySetBackupScreen(
				model = model,
				onProceed = onProceedFromBackupInitial,
				onBack = rootNavigator::backAction,
				modifier = Modifier.statusBarsPadding(),
			)
			BackHandler(onBack = rootNavigator::backAction)
		}
		composable(NewKeySetBackupStepSubgraph.NewKeySetBackupConfirmation) {
			NewKeySetBackupScreen(
				model = model,
				onProceed = onProceedFromBackupInitial,
				onBack = rootNavigator::backAction,
				modifier = Modifier.statusBarsPadding(),
			)
			BottomSheetWrapperRoot(onClosedAction = navController::popBackStack) {
				NewKeySetBackupBottomSheet(
					onProceed = {
						navController.navigate(NewKeySetBackupStepSubgraph.NewKeySetSelectNetworks)
					},
					onCancel = navController::popBackStack,
				)
			}
			BackHandler(onBack = navController::popBackStack)
		}
		composable(NewKeySetBackupStepSubgraph.NewKeySetSelectNetworks) {
			NewKeySetSelectNetworkScreen(
				model = model,
				navigator = rootNavigator,
				onBack = navController::popBackStack,
				modifier = Modifier.statusBarsPadding(),
			)
		}
	}
}

internal object NewKeySetBackupStepSubgraph {
	const val NewKeySetBackup = "new_keyset_backup_main"
	const val NewKeySetBackupConfirmation = "new_keyset_backup_confirmation"
	const val NewKeySetSelectNetworks = "new_keyset_select_networkis"
}
