package io.parity.signer.screens.keysetdetails.export

import androidx.compose.material.ExperimentalMaterialApi
import androidx.compose.material.ModalBottomSheetValue
import androidx.compose.material.rememberModalBottomSheetState
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import io.parity.signer.domain.Callback
import io.parity.signer.domain.KeySetDetailsModel
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
			confirmValueChange = {
				it != ModalBottomSheetValue.HalfExpanded
			},
			skipHalfExpanded = false
		)
	val scope = rememberCoroutineScope()

	val selected = remember { mutableStateOf(setOf<String>()) }

	BottomSheetWrapperContent(
		bottomSheetState = modalBottomSheetState,
		bottomSheetContent = {
			KeySetDetailsExportResultBottomSheet(
				selectedKeys = selected.value,
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
						modalBottomSheetState.show()
					}
				},
				onExportAll = {
					scope.launch {
						selected.value = model.keysAndNetwork.map { it.key.addressKey }.toSet()
						modalBottomSheetState.show()
					}
				},
			)
		},
	)
}
