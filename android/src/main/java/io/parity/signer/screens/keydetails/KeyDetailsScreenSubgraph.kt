package io.parity.signer.screens.keydetails

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.navigation.NavHostController
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import io.parity.signer.domain.Callback
import io.parity.signer.domain.backend.mapError
import io.parity.signer.domain.toKeyDetailsModel
import io.parity.signer.screens.keydetails.exportprivatekey.ConfirmExportPrivateKeyMenu
import io.parity.signer.screens.keydetails.exportprivatekey.PrivateKeyExportBottomSheet
import io.parity.signer.ui.BottomSheetWrapperRoot
import kotlinx.coroutines.runBlocking


@Composable
fun KeyDetailsScreenSubgraph(
	navController: NavHostController,
	keyAddr: String,
	keySpec: String,
) {

	val vm = KeyDetailsScreenViewModel()
	//todo dmitry fix
	val model = remember {
		runBlocking {
			vm.fetchModel(keyAddr, keySpec)
		}.mapError()!!.toKeyDetailsModel()
	}
	val menuNavController = rememberNavController()

	Box(modifier = Modifier.statusBarsPadding()) {
		KeyDetailsPublicKeyScreen(
			model = model,
			onBack = { navController.popBackStack() },
			onMenu = {
				menuNavController.navigate(
					KeyPublicDetailsMenuSubgraph.keyMenuGeneral
				)
			},
		)
	}



	NavHost(
		navController = menuNavController,
		startDestination = KeyPublicDetailsMenuSubgraph.empty,
	) {
		val closeAction: Callback = {
			menuNavController.popBackStack()
		}
		composable(KeyPublicDetailsMenuSubgraph.empty) {}//no menu
		composable(KeyPublicDetailsMenuSubgraph.keyMenuGeneral) {
			BottomSheetWrapperRoot(onClosedAction = closeAction) {
				KeyDetailsGeneralMenu(
					closeMenu = closeAction,
					onExportPrivateKey = {
						state.value = KeyDetailsMenuState.PRIVATE_KEY_CONFIRM
					},
					onDelete = {
						state.value = KeyDetailsMenuState.DELETE_CONFIRM
					},
				)
			}
		}
		composable(KeyPublicDetailsMenuSubgraph.keyMenuDelete) {
			BottomSheetWrapperRoot(onClosedAction = closeAction) {
				KeyDetailsDeleteConfirmBottomSheet(
					onCancel = closeAction,
					onRemoveKey = { navigator.navigate(Action.REMOVE_KEY) },
				)
			}
		}
		composable(KeyPublicDetailsMenuSubgraph.keyMenuExportConfirmation) {
			BottomSheetWrapperRoot(onClosedAction = closeAction) {
				ConfirmExportPrivateKeyMenu(
					onExportPrivate = {
						//todo dmitry fix
						navigator.navigate(LocalNavRequest.ShowExportPrivateKey(keyDetails!!.pubkey))
					},
					onClose = closeAction,
				)
			}
		}
		composable(KeyPublicDetailsMenuSubgraph.keyMenuExportResult) {
			BottomSheetWrapperRoot(onClosedAction = closeAction) {
				//todo dmitry implement
				PrivateKeyExportBottomSheet(
					model = localNavAction.model,
					onClose = {},
				)
			}
		}
		composable(KeyPublicDetailsMenuSubgraph.keyMenuPasswordForExport) {
			//todo dmitry handle  keyMenuExportResult
		}
	}
}


private object KeyPublicDetailsMenuSubgraph {
	const val empty = "key_menu_empty"
	const val keyMenuGeneral = "key_menu_general"
	const val keyMenuDelete = "key_menu_delete"
	const val keyMenuExportConfirmation = "key_menu_export"
	const val keyMenuExportResult = "key_private_export_result"
	const val keyMenuPasswordForExport = "key_private_export_password"
}
