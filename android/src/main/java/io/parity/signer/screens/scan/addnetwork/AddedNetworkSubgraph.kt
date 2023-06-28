package io.parity.signer.screens.scan.addnetwork

import android.widget.Toast
import androidx.activity.compose.BackHandler
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.MutableState
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.platform.LocalContext
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import io.parity.signer.R
import io.parity.signer.domain.Callback
import io.parity.signer.domain.NetworkModel
import io.parity.signer.domain.submitErrorState
import io.parity.signer.ui.BottomSheetWrapperRoot


@Composable
fun AddedNetworkSheetsSubgraph(
	networkNameAdded: String,
	onClose: Callback
) {
	val viewModel: AddedNetworkViewModel = viewModel()

	val addedNetwork: MutableState<NetworkModel?> =
		remember { mutableStateOf(null) }
	LaunchedEffect(key1 = networkNameAdded) {
		addedNetwork.value = viewModel.getNetworkByName(networkNameAdded) ?: run {
			onClose()
			null
		}
	}

	addedNetwork.value?.let { addedNetwork ->
		val navController = rememberNavController()
		NavHost(
			navController = navController,
			startDestination = AddedNetworkNavigationSubgraph.AddedNetworkNavigationQuestion,
		) {
			composable(AddedNetworkNavigationSubgraph.AddedNetworkNavigationQuestion) {
				BottomSheetWrapperRoot(onClosedAction = onClose) {
					AddNetworkToKeysetQuestionBottomSheet(
						networkModel = addedNetwork,
						onConfirm = {
							navController.navigate(AddedNetworkNavigationSubgraph.AddedNetworkNavigationAllKeysets) {
								popUpTo(0)
							}
						},
						onCancel = onClose
					)
				}
				BackHandler(onBack = onClose)
			}
			composable(AddedNetworkNavigationSubgraph.AddedNetworkNavigationAllKeysets) {
				val context = LocalContext.current
				BottomSheetWrapperRoot(onClosedAction = onClose) {
					AddNetworkAddKeysBottomSheet(
						networkTitle = addedNetwork.title,
						seeds = viewModel.getSeedList(),
						onCancel = onClose,
						onDone = { seeds ->
							viewModel.processAddNetworkToSeeds(
								network = addedNetwork,
								seeds = seeds,
								onAfterProcess = { isSuccess ->
									if (isSuccess) {
										Toast.makeText(
											context,
											context.getString(R.string.add_network_add_keys_success_message),
											Toast.LENGTH_SHORT
										).show()
										onClose()
									} else {
										submitErrorState("Error in add networks - this is unexpected")
									}
								},
							)
						},
					)
				}
				BackHandler(onBack = onClose)
			}
		}
	}
}

private object AddedNetworkNavigationSubgraph {
	const val AddedNetworkNavigationQuestion = "added_network_question"
	const val AddedNetworkNavigationAllKeysets = "added_network_all_networks"
}
