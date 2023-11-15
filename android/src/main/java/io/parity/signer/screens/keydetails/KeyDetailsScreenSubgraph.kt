package io.parity.signer.screens.keydetails

import android.widget.Toast
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.DisposableEffect
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.MutableState
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.lifecycle.viewModelScope
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.NavHostController
import androidx.navigation.NavType
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import androidx.navigation.navArgument
import io.parity.signer.R
import io.parity.signer.bottomsheets.password.EnterPassword
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
		composable(KeyPublicDetailsMenuSubgraph.empty) {
			//no menu - Spacer element so when other part shown there won't
			// be an appearance animation from top left part despite there shouldn't be
			Spacer(modifier = Modifier.fillMaxSize(1f))
		}
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
						if (model.address.cardBase.hasPassword) {
							menuNavController.navigate(KeyPublicDetailsMenuSubgraph.keyMenuPasswordForExport) {
								popUpTo(KeyPublicDetailsMenuSubgraph.empty)
							}
						} else {
							menuNavController.navigate(
								KeyPublicDetailsMenuSubgraph.KeyMenuExportResult.destination(
									null
								)
							) {
								popUpTo(KeyPublicDetailsMenuSubgraph.empty)
							}
						}
					},
					onClose = closeAction,
				)
			}
		}
		composable(
			KeyPublicDetailsMenuSubgraph.KeyMenuExportResult.route,
			arguments = listOf(
				navArgument(KeyPublicDetailsMenuSubgraph.KeyMenuExportResult.password) {
					type = NavType.StringType
					nullable = true
				}
			)
		) {
			val password =
				it.arguments?.getString(KeyPublicDetailsMenuSubgraph.KeyMenuExportResult.password)

			val privateModel: MutableState<OperationResult<PrivateKeyExportModel, Any>?> =
				remember(model) {
					mutableStateOf(null)
				}

			LaunchedEffect(key1 = model, key2 = password) {
				privateModel.value = vm.getPrivateExportKey(
					model = model,
					password = password
				)
			}

			DisposableEffect(key1 = model, key2 = password) {
				onDispose { vm.clearExportResultState() }
			}

			when (val model = privateModel.value) {
				is OperationResult.Err -> {
					val context = LocalContext.current
					Toast.makeText(
						context,
						"Error, ${model.error}",
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
			val passwordModel =
				remember { mutableStateOf(vm.createPasswordModel(model)) }
			val context = LocalContext.current
			BottomSheetWrapperRoot(onClosedAction = closeAction) {
				EnterPassword(
					data = passwordModel.value,
					proceed = { password ->
						vm.viewModelScope.launch {
							when (val reply =
								vm.tryPassword(model, passwordModel.value, password)) {
								ExportTryPasswordReply.ErrorAttemptsExceeded -> {
									Toast.makeText(
										context,
										context.getString(R.string.attempts_exceeded_title),
										Toast.LENGTH_LONG
									).show()
									closeAction()
								}

								ExportTryPasswordReply.ErrorAuthWrong -> {
									Toast.makeText(
										context,
										context.getString(R.string.auth_failed_message),
										Toast.LENGTH_LONG
									).show()
									closeAction()
								}

								is ExportTryPasswordReply.OK -> {
									menuNavController.navigate(
										KeyPublicDetailsMenuSubgraph.KeyMenuExportResult.destination(
											reply.password
										)
									) {
										popUpTo(KeyPublicDetailsMenuSubgraph.empty)
									}
								}

								is ExportTryPasswordReply.UpdatePassword -> {
									passwordModel.value = reply.model
								}
							}
						}
					},
					onClose = closeAction
				)
			}
		}
	}
}


private object KeyPublicDetailsMenuSubgraph {
	const val empty = "key_menu_empty"
	const val keyMenuGeneral = "key_menu_general"
	const val keyMenuDelete = "key_menu_delete"
	const val keyMenuExportConfirmation = "key_menu_export"

	object KeyMenuExportResult {
		private const val baseRoute = "key_private_export_result"
		internal const val password = "password_arg" //optional
		const val route = "$baseRoute?$password={$password}"
		fun destination(passwordValue: String?): String {
			val result =
				if (passwordValue == null) baseRoute else "${baseRoute}?${password}=${passwordValue}"
			return result
		}
	}

	const val keyMenuPasswordForExport = "key_private_export_password"
}
