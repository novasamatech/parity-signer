package io.parity.signer.screens.scan.bananasplitcreate.show

import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.navigation.NavController
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import io.parity.signer.domain.Callback
import io.parity.signer.ui.BottomSheetWrapperRoot


@Composable
fun BananaSplitShowFull(
	coreNavController: NavController,
) {

	val menuNavController = rememberNavController()

//	val vm: NewKeysetNameViewModel = viewModel()
//	var seedName by rememberSaveable() {
//		mutableStateOf("")
//	}
//	val seedPhrase = rememberSaveable() {
//		vm.createNewSeedPhrase().toOperationResult().handleErrorAppState(coreNavController) ?: ""
//	}

	BananaSplitExportScreen(
		qrCodes = emptyList(),// todo dmitry
		onMenu = { /*TODO*/ },
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
					onShowPassphrase = {}, //todo dmitry
					onRemoveBackup = {}, //todo dmitry
				)
			}
		}
		composable(BananaSplitShowMenu.ShowBSConfirmRemove) {
			BottomSheetWrapperRoot(onClosedAction = closeAction) {
				BananaSplitExportRemoveConfirmBottomSheet(
					onCancel = closeAction,
					onRemoveKeySet = {}, //todo dmitry
				)
			}
		}
		composable(BananaSplitShowMenu.ShowBsSeePassword) {
			BottomSheetWrapperRoot(onClosedAction = closeAction) {
				BananaSplitShowPassphraseMenu(
					onClose = closeAction,
					password = "", //todo dmitry implement
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
