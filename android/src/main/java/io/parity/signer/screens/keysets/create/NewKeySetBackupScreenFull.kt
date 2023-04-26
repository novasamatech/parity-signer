package io.parity.signer.screens.keysets.create

import androidx.compose.material.ExperimentalMaterialApi
import androidx.compose.material.ModalBottomSheetValue
import androidx.compose.material.rememberModalBottomSheetState
import androidx.compose.runtime.Composable
import androidx.compose.runtime.rememberCoroutineScope
import io.parity.signer.domain.Callback
import io.parity.signer.ui.BottomSheetWrapperContent
import kotlinx.coroutines.launch

/**
 * 2/2 step of new key set creation
 */
@OptIn(ExperimentalMaterialApi::class)
@Composable
fun NewKeySetBackupScreenFull(
	model: NewSeedBackupModel,
	onBack: Callback,
	onCreateKeySet: (String, String) -> Unit
) {
	val modalBottomSheetState =
		rememberModalBottomSheetState(ModalBottomSheetValue.Hidden,
			skipHalfExpanded = true,
			confirmValueChange = {
				it != ModalBottomSheetValue.HalfExpanded
			}
		)
	val scope = rememberCoroutineScope()

	BottomSheetWrapperContent(
		bottomSheetState = modalBottomSheetState,
		bottomSheetContent = {
			NewKeySetBackupBottomSheet(
				onProceed = {
					onCreateKeySet(model.seed, model.seedPhrase)
				},
				onCancel = { scope.launch { modalBottomSheetState.hide() } },
			)
		},
		mainContent = {
			NewKeySetBackupScreen(
				model = model,
				onProceed = {
					scope.launch {
						modalBottomSheetState.show()
					}
				},
				onBack = onBack,
			)
		},
	)
}
