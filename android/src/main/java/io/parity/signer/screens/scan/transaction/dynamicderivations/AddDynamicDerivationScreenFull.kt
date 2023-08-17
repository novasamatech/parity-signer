package io.parity.signer.screens.scan.transaction.dynamicderivations

import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import io.parity.signer.domain.Callback
import io.parity.signer.ui.BottomSheetWrapperRoot
import io.parity.signer.uniffi.DdPreview


@Composable
fun AddDynamicDerivationScreenFull(
	model: DdPreview,
	onBack: Callback,
	onDone: Callback,
) {

	val confirmState = remember { mutableStateOf(false) }

	AddDerivedKeysScreen(
		model = model,
		modifier = Modifier.statusBarsPadding(),
		onBack = onBack,
		onDone = { confirmState.value = true },
	)

	if (confirmState.value) {
		BottomSheetWrapperRoot(onClosedAction = { confirmState.value = false }) {
			AddDDConfirmBottomSheet(
				onConfirm = onDone,
				onCancel = { confirmState.value = false },
			)
		}
	}
}
