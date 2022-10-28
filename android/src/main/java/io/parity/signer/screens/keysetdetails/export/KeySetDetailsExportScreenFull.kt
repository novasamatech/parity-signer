package io.parity.signer.screens.keysetdetails.export

import androidx.compose.material.ExperimentalMaterialApi
import androidx.compose.material.ModalBottomSheetValue
import androidx.compose.material.rememberModalBottomSheetState
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import io.parity.signer.models.Callback
import io.parity.signer.models.KeySetDetailsModel
import io.parity.signer.ui.BottomSheetWrapperContent
import kotlinx.coroutines.launch

@OptIn(ExperimentalMaterialApi::class)
@Composable
fun KeySetDetailsExportScreenFull(
	model: KeySetDetailsModel,
	onClose: Callback,
) {
	val modalBottomSheetState =
		rememberModalBottomSheetState(
			ModalBottomSheetValue.Hidden,
			confirmStateChange = {
				it != ModalBottomSheetValue.HalfExpanded
			}
		)
	val scope = rememberCoroutineScope()

	val selected = remember { mutableStateOf(setOf<String>()) }

	BottomSheetWrapperContent(
		bottomSheetState = modalBottomSheetState,
		bottomSheetContent = {
			KeySetDetailsExportResultBottomSheet(
				seeds = selected.value,
				model = model,
				onClose = { scope.launch { modalBottomSheetState.hide() }},
			)
		},
		mainContent = {
			KeySetDetailsMultiselectScreen(
				model = model,
				selected = selected,
				onClose = onClose,
				onExportSelected = {
					scope.launch {
						modalBottomSheetState.animateTo(
							ModalBottomSheetValue.Expanded
						)
					}
				},
				onExportAll = {
					scope.launch {
						selected.value = //todo dmitry compare with ios
							model.keys.map { it.addressKey }.toSet() + model.root.addressKey
						modalBottomSheetState.animateTo(
							ModalBottomSheetValue.Expanded
						)
					}
				},
			)
		},
	)
}
