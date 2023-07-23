package io.parity.signer.screens.keysetdetails.export

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.material.ExperimentalMaterialApi
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.navigation.NavType
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import androidx.navigation.navArgument
import io.parity.signer.bottomsheets.PublicKeyBottomSheetView
import io.parity.signer.domain.Callback
import io.parity.signer.domain.KeySetDetailsModel
import io.parity.signer.domain.submitErrorState
import io.parity.signer.ui.BottomSheetWrapperRoot

@OptIn(ExperimentalMaterialApi::class)
@Composable
fun KeySetDetailsExportScreenFull(
	model: KeySetDetailsModel,
	onClose: Callback,
) {

	val menuNavController = rememberNavController()
	val selected = remember { mutableStateOf(setOf<String>()) }

	Box(Modifier.statusBarsPadding()) {
		KeySetDetailsMultiselectScreen(
			model = model,
			selected = selected,
			onClose = onClose,
			onExportSelected = {
				menuNavController.navigate(KeySetDetailsExportMenuSubgraph.export_result)
			},
			onExportAll = {
				selected.value = model.keysAndNetwork.map { it.key.addressKey }.toSet()
				menuNavController.navigate(KeySetDetailsExportMenuSubgraph.export_result)
			},
			onShowPublicKey = { title: String, key: String ->
				menuNavController.navigate("${KeySetDetailsExportMenuSubgraph.public_key}/$title/$key")
			},
		)
	}

	NavHost(
		navController = menuNavController,
		startDestination = KeySetDetailsExportMenuSubgraph.empty,
	) {
		val closeAction: Callback = {
			menuNavController.popBackStack()
		}
		composable(KeySetDetailsExportMenuSubgraph.empty) {} //no menu
		composable(KeySetDetailsExportMenuSubgraph.export_result) {
			BottomSheetWrapperRoot(onClosedAction = closeAction) {
				KeySetDetailsExportResultBottomSheet(
					selectedKeys = selected.value,
					model = model,
					onClose = closeAction,
				)
			}
		}
		composable(
			route = "${KeySetDetailsExportMenuSubgraph.public_key}/{${ARGUMENT_PUBLIC_KEY_TITLE}}/{${ARGUMENT_PUBLIC_KEY_VALUE}}",
			arguments = listOf(
				navArgument(ARGUMENT_PUBLIC_KEY_TITLE) {
					type = NavType.StringType
				},
				navArgument(ARGUMENT_PUBLIC_KEY_VALUE) {
					type = NavType.StringType
				}
			)) { backStackEntry ->
			val keyName =
				backStackEntry.arguments?.getString(ARGUMENT_PUBLIC_KEY_TITLE) ?: run {
					submitErrorState("mandatory parameter missing for KeySetDetailsExportMenuSubgraph")
					""
				}
			val keyValue =
				backStackEntry.arguments?.getString(ARGUMENT_PUBLIC_KEY_VALUE) ?: run {
					submitErrorState("mandatory parameter missing for KeySetDetailsExportMenuSubgraph")
					""
				}

			BottomSheetWrapperRoot(onClosedAction = closeAction) {
				PublicKeyBottomSheetView(
					name = keyName,
					key = keyValue,
					onClose = closeAction,
				)
			}
		}
	}
}


private object KeySetDetailsExportMenuSubgraph {
	const val empty = "keyset_export_menu_empty"
	const val export_result = "keyset_export_menu_export_result"
	const val public_key = "keyset_export_menu_public_key"
}

private const val ARGUMENT_PUBLIC_KEY_TITLE = "ARGUMENT_PUBLIC_KEY_TITLE"
private const val ARGUMENT_PUBLIC_KEY_VALUE = "ARGUMENT_PUBLIC_KEY_VALUE"
