package io.parity.signer.screens.scan.bananasplitcreate.show

import androidx.compose.foundation.layout.imePadding
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.navigation.NavController
import androidx.navigation.NavHostController
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import io.parity.signer.domain.backend.AuthOperationResult
import io.parity.signer.domain.popUpToTop
import io.parity.signer.screens.error.handleErrorAppState
import io.parity.signer.screens.keysets.create.backupstepscreens.NewKeySetBackupBottomSheet
import io.parity.signer.screens.keysets.create.backupstepscreens.NewKeySetBackupScreen
import io.parity.signer.screens.keysets.create.backupstepscreens.NewKeySetSelectNetworkScreen
import io.parity.signer.ui.BottomSheetWrapperRoot
import io.parity.signer.ui.mainnavigation.CoreUnlockedNavSubgraph


@Composable
fun BananaSplitShowFull(
	coreNavController: NavController,
) {

	val subgraphNavController = rememberNavController()

//	val vm: NewKeysetNameViewModel = viewModel()
//	var seedName by rememberSaveable() {
//		mutableStateOf("")
//	}
//	val seedPhrase = rememberSaveable() {
//		vm.createNewSeedPhrase().toOperationResult().handleErrorAppState(coreNavController) ?: ""
//	}

//	NavHost(
//		navController = subgraphNavController,
//		startDestination = NewKeySetBackupStepSubgraph.NewKeySetName,
//	) {
//		composable(NewKeySetBackupStepSubgraph.NewKeySetName) {
//			NewKeySetNameScreen(
//				prefilledName = seedName,
//				onBack = { coreNavController.popBackStack() },
//				onNextStep = {
//					seedName = it
//					subgraphNavController.navigate(
//						NewKeySetBackupStepSubgraph.NewKeySetBackup
//					)
//				},
//				modifier = Modifier
//					.statusBarsPadding()
//					.imePadding(),
//			)
//		}
//		composable(NewKeySetBackupStepSubgraph.NewKeySetBackup) {
//			NewKeySetBackupScreen(
//				seedPhrase = seedPhrase,
//				onProceed = {
//					subgraphNavController.navigate(
//						NewKeySetBackupStepSubgraph.NewKeySetBackupConfirmation
//					)
//				},
//				onBack = { subgraphNavController.popBackStack() },
//				modifier = Modifier.statusBarsPadding(),
//			)
//		}
//		composable(NewKeySetBackupStepSubgraph.NewKeySetBackupConfirmation) {
//			NewKeySetBackupScreen(
//				seedPhrase = seedPhrase,
//				onProceed = {
//					subgraphNavController.navigate(
//						NewKeySetBackupStepSubgraph.NewKeySetBackupConfirmation
//					)
//				},
//				onBack = { subgraphNavController.popBackStack() },
//				modifier = Modifier.statusBarsPadding(),
//			)
//			BottomSheetWrapperRoot(onClosedAction = subgraphNavController::popBackStack) {
//				NewKeySetBackupBottomSheet(
//					onProceed = {
//						subgraphNavController.navigate(NewKeySetBackupStepSubgraph.NewKeySetSelectNetworks) {
//							popUpTo(NewKeySetBackupStepSubgraph.NewKeySetBackup)
//						}
//					},
//					onCancel = subgraphNavController::popBackStack,
//				)
//			}
//		}
//		composable(NewKeySetBackupStepSubgraph.NewKeySetSelectNetworks) {
//			val context = LocalContext.current
//			NewKeySetSelectNetworkScreen(
//				seedName = seedName,
//				seedPhrase = seedPhrase,
//				onSuccess = {
//					coreNavController.navigate(
//						CoreUnlockedNavSubgraph.KeySet.destination(
//							seedName
//						)
//					) {
//							popUpToTop(coreNavController)
//					}
//				},
//				showError = { error: AuthOperationResult ->
//					error.handleErrorAppState(coreNavController, context)
//				},
//				onBack = subgraphNavController::popBackStack,
//			)
//		}
//	}

}

internal object BananaSplitShowSubgraph {
	const val ShowBS = "show_bs"
	const val ShowBSConfirmRemove = "show_bs_confirm_remove"
	const val ShowBsMenu = "show_bs_menu"
	const val ShowBsSeePassword = "show_bs_password_show"
}
