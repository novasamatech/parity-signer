package io.parity.signer.screens.keysets.create

import androidx.compose.material.ExperimentalMaterialApi
import androidx.compose.material.ModalBottomSheetValue
import androidx.compose.material.rememberModalBottomSheetState
import androidx.compose.runtime.Composable
import androidx.compose.runtime.rememberCoroutineScope
import io.parity.signer.models.Callback
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
	onCreateKeySet: (String, String, Boolean) -> Unit
) {
	val modalBottomSheetState =
		rememberModalBottomSheetState(
			ModalBottomSheetValue.Hidden,
			confirmStateChange = {
				it != ModalBottomSheetValue.HalfExpanded
			}
		)
	val scope = rememberCoroutineScope()

	BottomSheetWrapperContent(
		bottomSheetState = modalBottomSheetState,
		bottomSheetContent = {
			NewKeySetBackupBottomSheet(
				onProceed = {
					val alwaysCreateRootKeys = true
					onCreateKeySet(model.seed, model.seedPhrase, alwaysCreateRootKeys)
				},
				onCancel = { scope.launch { modalBottomSheetState.hide() } },
			)
		},
		mainContent = {
			NewKeySetBackupScreen(
				model = model,
				onProceed = {
					scope.launch {
						modalBottomSheetState.animateTo(
							ModalBottomSheetValue.Expanded
						)
					}
				},
				onBack = onBack,
			)
		},
	)
}
