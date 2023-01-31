package io.parity.signer.screens.keysetdetails

import androidx.compose.material.ExperimentalMaterialApi
import androidx.compose.material.ModalBottomSheetValue
import androidx.compose.material.rememberModalBottomSheetState
import androidx.compose.runtime.*
import androidx.navigation.NavController
import io.parity.signer.domain.*
import io.parity.signer.ui.BottomSheetWrapperContent
import kotlinx.coroutines.launch

@OptIn(ExperimentalMaterialApi::class)
@Composable
fun KeySetDetailsScreenFull(
	model: KeySetDetailsModel,
	navigator: Navigator,
	navController: NavController,
	networkState: State<NetworkState?>, //for shield icon
	onRemoveKeySet: Callback,
) {
	val bottomSheetState =
		rememberModalBottomSheetState(
			ModalBottomSheetValue.Hidden,
			confirmStateChange = {
				it != ModalBottomSheetValue.HalfExpanded
			}
		)
	val scope = rememberCoroutineScope()

	BottomSheetWrapperContent(
		bottomSheetState = bottomSheetState,
		bottomSheetContent = {
			KeySetDetailsMenu(
				navigator = navigator,
				networkState = networkState,
				removeSeed = onRemoveKeySet,
				onSelectKeysClicked = {
					scope.launch { bottomSheetState.hide() }
					navController.navigate(KeySetDetailsNavSubgraph.multiselect)
				},
				onBackupClicked = {
					scope.launch { bottomSheetState.hide() }
					navController.navigate(KeySetDetailsNavSubgraph.backup)
				},
			)
		},
		mainContent = {
			KeySetDetailsScreenView(
				model = model,
				navigator = navigator,
				networkState = networkState,
				onMenu = {
					scope.launch {
						bottomSheetState.animateTo(
							ModalBottomSheetValue.Expanded
						)
					}
				}
			)
		},
	)
}
