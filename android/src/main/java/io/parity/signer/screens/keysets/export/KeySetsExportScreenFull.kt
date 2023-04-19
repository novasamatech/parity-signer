package io.parity.signer.screens.keysets.export

import androidx.compose.material.ExperimentalMaterialApi
import androidx.compose.material.ModalBottomSheetValue
import androidx.compose.material.rememberModalBottomSheetState
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import io.parity.signer.domain.Callback
import io.parity.signer.domain.KeySetModel
import io.parity.signer.domain.KeySetsSelectModel
import io.parity.signer.ui.BottomSheetWrapperContent
import kotlinx.coroutines.launch

@OptIn(ExperimentalMaterialApi::class)
@Composable
fun KeySetsExportScreenFull(
	model: KeySetsSelectModel,
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

	val selected = remember { mutableStateOf(setOf<KeySetModel>()) }

	BottomSheetWrapperContent(
		bottomSheetState = modalBottomSheetState,
		bottomSheetContent = {
			KeySetExportResultBottomSheet(
				seeds = selected.value,
				onClose = { scope.launch { modalBottomSheetState.hide() }},
			)
		},
		mainContent = {
			KeySetsSelectExportScreenContent(
				model = model,
				selected = selected,
				onClose = onClose,
				onExportSelected = {
					scope.launch {
						modalBottomSheetState.show()
					}
				},
				onExportAll = {
					scope.launch {
						selected.value = model.keys.toSet()
						modalBottomSheetState.show()
					}
				},
			)
		},
	)
}
