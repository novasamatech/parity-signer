package io.parity.signer.screens.scan.bananasplitcreate.show

import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.NavController
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import io.parity.signer.domain.Callback
import io.parity.signer.screens.error.handleErrorAppState
import io.parity.signer.ui.BottomSheetWrapperRoot


@Composable
fun BananaSplitShowFull(
	coreNavController: NavController,
	seedName: String,
) {
	val menuNavController = rememberNavController()

	val vm: ShowBananaSplitViewModel = viewModel()
	val qrCodes = remember {
		vm.getBananaSplit(seedName).handleErrorAppState(coreNavController)
			?: emptyList()
	}

	BananaSplitExportScreen(
		qrCodes = qrCodes,
		onMenu = { menuNavController.navigate(BananaSplitShowMenu.ShowBsMenu) },
		onClose = { coreNavController.popBackStack() },
		modifier = Modifier.statusBarsPadding(),
	)

	NavHost(
		navController = menuNavController,
		startDestination = BananaSplitShowMenu.Empty,
	) {
		val closeAction: Callback = {
			menuNavController.popBackStack()
		}
		composable(BananaSplitShowMenu.Empty) {
			//no menu - Spacer element so when other part shown there won't
			// be an appearance animation from top left part despite there shouldn't be
			Spacer(modifier = Modifier.fillMaxSize(1f))
		}
		composable(BananaSplitShowMenu.ShowBsMenu) {
			BottomSheetWrapperRoot(onClosedAction = closeAction) {
				BananaSplitExportMenuBottomSheet(
					onCancel = closeAction,
					onShowPassphrase = {
						menuNavController.navigate(BananaSplitShowMenu.ShowBsSeePassword) {
							popUpTo(0)
						}
					},
					onRemoveBackup = {
						menuNavController.navigate(BananaSplitShowMenu.ShowBSConfirmRemove) {
							popUpTo(0)
						}
					},
				)
			}
		}
		composable(BananaSplitShowMenu.ShowBSConfirmRemove) {
			BottomSheetWrapperRoot(onClosedAction = closeAction) {
				BananaSplitExportRemoveConfirmBottomSheet(
					onCancel = closeAction,
					onRemoveKeySet = {
						vm.removeBS(seedName)
						//todo dmitry handle result
						coreNavController.popBackStack()
					},
				)
			}
		}
		composable(BananaSplitShowMenu.ShowBsSeePassword) {
			BottomSheetWrapperRoot(onClosedAction = closeAction) {
				BananaSplitShowPassphraseMenu(
					onClose = closeAction,
					password = vm.getPassword(seedName),
				)
			}
		}
	}
}

private object BananaSplitShowMenu {
	const val Empty = "show_bs"
	const val ShowBSConfirmRemove = "show_bs_confirm_remove"
	const val ShowBsMenu = "show_bs_menu"
	const val ShowBsSeePassword = "show_bs_password_show"
}
