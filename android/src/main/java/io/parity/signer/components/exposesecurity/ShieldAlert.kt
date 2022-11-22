package io.parity.signer.components.exposesecurity

import androidx.compose.runtime.Composable
import androidx.compose.runtime.State
import io.parity.signer.components.AlertComponent
import io.parity.signer.models.AlertState
import io.parity.signer.models.Callback
import io.parity.signer.ui.BottomSheetWrapperRoot

@Composable
fun ShieldAlert(
	// shieldAlert: ShieldAlert, // TODO: use this instead of alertstate
	alertState: State<AlertState?>,
	navigateBack: Callback,
	acknowledgeWarning: Callback,
) {
	when (alertState.value) {
		AlertState.Active -> {
			BottomSheetWrapperRoot(onClosedAction = { navigateBack() }) {
				ExposedNowBottomSheet(close = navigateBack)
			}
		}
		AlertState.Past -> {
			BottomSheetWrapperRoot(onClosedAction = { navigateBack() }) {
				ExposedPastBottomSheet(
					close = navigateBack,
					acknowledgeWarning = acknowledgeWarning
				)
			}
		}
		AlertState.None -> AlertComponent(
			show = true,
			header = "Signer is secure",
			back = navigateBack,
			forward = { },
			backText = "Ok",
			showForward = false
		)
		else -> {
			AlertComponent(
				show = true,
				header = "Network detector failure",
				text = "Please report this error",
				back = navigateBack,
				forward = { },
				backText = "Dismiss",
				showForward = false
			)
		}
	}
}
