package io.parity.signer.screens.keysets.details

import androidx.compose.material.ExperimentalMaterialApi
import androidx.compose.material.ModalBottomSheetValue
import androidx.compose.material.rememberModalBottomSheetState
import androidx.compose.runtime.*
import io.parity.signer.models.*
import io.parity.signer.screens.keysets.KeySetDetailsScreenView
import io.parity.signer.ui.BottomSheetWrapperContent
import kotlinx.coroutines.launch

@OptIn(ExperimentalMaterialApi::class)
@Composable
fun KeySetDetailsScreenFull(
	model: KeySetDetailsModel,
	navigator: Navigator,
	alertState: State<AlertState?>, //for shield icon
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
				alertState = alertState,
				removeSeed = onRemoveKeySet,
			)
		},
		mainContent = {
			KeySetDetailsScreenView(
				model = model,
				navigator = navigator,
				alertState = alertState,
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
