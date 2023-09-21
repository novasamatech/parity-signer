package io.parity.signer.screens.keydetails

import android.widget.Toast
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.MutableState
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.NavHostController
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import io.parity.signer.domain.Callback
import io.parity.signer.domain.backend.OperationResult
import io.parity.signer.domain.toKeyDetailsModel
import io.parity.signer.screens.error.handleErrorAppState
import io.parity.signer.screens.keydetails.exportprivatekey.ConfirmExportPrivateKeyMenu
import io.parity.signer.screens.keydetails.exportprivatekey.PrivateKeyExportBottomSheet
import io.parity.signer.screens.keydetails.exportprivatekey.PrivateKeyExportModel
import io.parity.signer.ui.BottomSheetWrapperRoot
import kotlinx.coroutines.launch
import kotlinx.coroutines.runBlocking


@Composable
fun KeyDetailsScreenSubgraph(
	navController: NavHostController,
	keyAddr: String,
	keySpec: String,
) {

	val vm: KeyDetailsScreenViewModel = viewModel()
	val model = remember(keyAddr, keySpec) {
		runBlocking {
			vm.fetchModel(keyAddr, keySpec)
		}.handleErrorAppState(navController)?.toKeyDetailsModel()
	} ?: return
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
						menuNavController.navigate(KeyPublicDetailsMenuSubgraph.keyMenuExportConfirmation) {
							popUpTo(KeyPublicDetailsMenuSubgraph.empty)
						}
					},
					onDelete = {
						menuNavController.navigate(KeyPublicDetailsMenuSubgraph.keyMenuDelete) {
							popUpTo(KeyPublicDetailsMenuSubgraph.empty)
						}
					},
				)
			}
		}
		composable(KeyPublicDetailsMenuSubgraph.keyMenuDelete) {
			val context = rememberCoroutineScope()
			BottomSheetWrapperRoot(onClosedAction = closeAction) {
				KeyDetailsDeleteConfirmBottomSheet(
					onCancel = closeAction,
					onRemoveKey = {
						context.launch {
							vm.removeDerivedKey(keyAddr, keySpec)
								.handleErrorAppState(navController)?.let {
									closeAction()
									navController.popBackStack()
								}
						}
					},
				)
			}
		}
		composable(KeyPublicDetailsMenuSubgraph.keyMenuExportConfirmation) {
			BottomSheetWrapperRoot(onClosedAction = closeAction) {
				ConfirmExportPrivateKeyMenu(
					onExportPrivate = {
						menuNavController.navigate(KeyPublicDetailsMenuSubgraph.keyMenuExportResult) {
							popUpTo(KeyPublicDetailsMenuSubgraph.empty)
						}
					},
					onClose = closeAction,
				)
			}
		}
		composable(KeyPublicDetailsMenuSubgraph.keyMenuExportResult) {
			val privateModel: MutableState<OperationResult<PrivateKeyExportModel, Any>?> =
				remember(model) {
					mutableStateOf(null)
				}

			//don't forget to pass password in this parameter in future
			LaunchedEffect(key1 = model) {
				privateModel.value = vm.getPrivateExportKey(model)
			}

			when (val model = privateModel.value) {
				is OperationResult.Err -> {
					// #1533
					// navigate to KeyPublicDetailsMenuSubgraph.keyMenuPasswordForExport
					val context = LocalContext.current
					Toast.makeText(
						context,
						"For passworded keys export not yet supported, ${model.error}",
						Toast.LENGTH_LONG
					).show()
					closeAction()
				}

				is OperationResult.Ok -> {
					BottomSheetWrapperRoot(onClosedAction = closeAction) {
						PrivateKeyExportBottomSheet(
							model = model.result,
							onClose = closeAction,
						)
					}
				}

				null -> {}
			}
		}
		composable(KeyPublicDetailsMenuSubgraph.keyMenuPasswordForExport) {
			//todo handle keyMenuExportResult #1533 issue
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
