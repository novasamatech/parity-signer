package io.parity.signer.screens.scan.bananasplitcreate.show

import android.widget.Toast
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.MutableState
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.lifecycle.viewModelScope
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.NavController
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import io.parity.signer.R
import io.parity.signer.domain.Callback
import io.parity.signer.domain.backend.OperationResult
import io.parity.signer.screens.error.handleErrorAppState
import io.parity.signer.screens.scan.bananasplitcreate.BananaSplitCreateDestination
import io.parity.signer.ui.BottomSheetWrapperRoot
import io.parity.signer.ui.mainnavigation.CoreUnlockedNavSubgraph
import kotlinx.coroutines.launch


@Composable
fun BananaSplitShowFull(
	coreNavController: NavController,
	seedName: String,
) {
	val menuNavController = rememberNavController()

	val vm: ShowBananaSplitViewModel = viewModel()
	val qrCodes = remember {
		val qrCodes = vm.getBananaSplitQrs(seedName)
		if (qrCodes == null) {
			//no BS for this seed - open create screen
			coreNavController.navigate(BananaSplitCreateDestination.CreateBsCreateScreen) {
				popUpTo(CoreUnlockedNavSubgraph.CreateBananaSplit.route)
			}
		}
		qrCodes ?: emptyList()
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
							popUpTo(BananaSplitShowMenu.Empty)
						}
					},
					onRemoveBackup = {
						menuNavController.navigate(BananaSplitShowMenu.ShowBSConfirmRemove) {
							popUpTo(BananaSplitShowMenu.Empty)
//							todo dmitry find out popupto(0) and if navigation is broken there, test this - if close is working
						}
					},
				)
			}
		}
		composable(BananaSplitShowMenu.ShowBSConfirmRemove) {
			val context = LocalContext.current
			BottomSheetWrapperRoot(onClosedAction = closeAction) {
				BananaSplitExportRemoveConfirmBottomSheet(
					onCancel = closeAction,
					onRemoveKeySet = {
						vm.viewModelScope.launch {
							val result =
								vm.removeBS(seedName).handleErrorAppState(coreNavController)

							result?.let {
								//Show toast
								Toast.makeText(
									context,
									context.getString(R.string.banana_split_backup_removed_ok),
									Toast.LENGTH_SHORT
								).show()
								coreNavController.popBackStack()
							}
						}
					},
				)
			}
		}
		composable(BananaSplitShowMenu.ShowBsSeePassword) {
			val password: MutableState<String?> = remember { mutableStateOf(null) }
			LaunchedEffect(key1 = seedName) {
				val result =
					vm.getPassword(seedName)
				when (result) {
					is OperationResult.Err -> {
						menuNavController.popBackStack()
						(result as OperationResult).handleErrorAppState(coreNavController)
					}

					is OperationResult.Ok -> {
						password.value = result.result
					}
				}
			}
			//todo dmitry close action not working?
			password.value?.let { password ->
				BottomSheetWrapperRoot(onClosedAction = closeAction) {
					BananaSplitShowPassphraseMenu(
						onClose = closeAction,
						password = password,
					)
				}
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
