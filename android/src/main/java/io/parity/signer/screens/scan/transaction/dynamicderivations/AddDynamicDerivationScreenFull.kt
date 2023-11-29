package io.parity.signer.screens.scan.transaction.dynamicderivations

import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.ui.Modifier
import io.parity.signer.domain.Callback
import io.parity.signer.ui.BottomSheetWrapperRoot
import io.parity.signer.uniffi.DdPreview


@Composable
fun AddDynamicDerivationScreenFull(
	model: DdPreview,
	onClose: Callback,
	onDone: Callback,
) {

	val confirmCloseState = rememberSaveable { mutableStateOf(false) }

	AddDerivedKeysScreen(
		model = model,
		modifier = Modifier.statusBarsPadding(),
		onBack = { confirmCloseState.value = true },
		onDone = onDone,
	)

	if (confirmCloseState.value) {
		BottomSheetWrapperRoot(onClosedAction = { confirmCloseState.value = false }) {
			AddDDConfirmCloseBottomSheet(
				onConfirm = onClose,
				onCancel = { confirmCloseState.value = false },
			)
		}
	}
}
