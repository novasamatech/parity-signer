package io.parity.signer.screens.settings.verifiercert

import androidx.compose.material.ExperimentalMaterialApi
import androidx.compose.material.ModalBottomSheetValue
import androidx.compose.material.rememberModalBottomSheetState
import androidx.compose.runtime.Composable
import androidx.compose.runtime.rememberCoroutineScope
import io.parity.signer.domain.Callback
import io.parity.signer.domain.Navigator
import io.parity.signer.domain.VerifierDetailsModels
import io.parity.signer.ui.BottomSheetWrapperContent
import kotlinx.coroutines.launch

@OptIn(ExperimentalMaterialApi::class)
@Composable
fun VerifierScreenFull(
	verifierDetails: VerifierDetailsModels,
	wipe: Callback,
	navigator: Navigator,
) {
	val bottomSheetState =
		rememberModalBottomSheetState(
			ModalBottomSheetValue.Hidden,
			confirmValueChange = {
				it != ModalBottomSheetValue.HalfExpanded
			}
		)
	val scope = rememberCoroutineScope()

	BottomSheetWrapperContent(
		bottomSheetState = bottomSheetState,
		bottomSheetContent = {
			ConfirmRemoveCertificateBottomSheet(
				onCancel = { scope.launch { bottomSheetState.hide() } },
				onRemoveCertificate = wipe,
			)
		},
		mainContent = {
			VerifierScreen(
				verifierDetails = verifierDetails,
				onBack = navigator::backAction,
				onRemove = { scope.launch { bottomSheetState.show() } },
			)
		},
	)
}


