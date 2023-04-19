package io.parity.signer.components.exposesecurity

import androidx.compose.runtime.Composable
import androidx.compose.runtime.State
import io.parity.signer.components.AlertComponent
import io.parity.signer.domain.Callback
import io.parity.signer.domain.NetworkState
import io.parity.signer.ui.BottomSheetWrapperRoot
import io.parity.signer.ui.theme.SignerNewTheme

@Composable
fun ShieldAlert(
	// shieldAlert: ShieldAlert, // TODO: use this instead of alertstate
	networkState: State<NetworkState?>,
	navigateBack: Callback,
	acknowledgeWarning: Callback,
) {
	when (networkState.value) {
		NetworkState.Active -> {
			SignerNewTheme() {
				BottomSheetWrapperRoot(onClosedAction = { navigateBack() }) {
					ExposedNowBottomSheet(close = navigateBack)
				}
			}
		}
		NetworkState.Past -> {
			SignerNewTheme() {
				BottomSheetWrapperRoot(onClosedAction = { navigateBack() }) {
					ExposedPastBottomSheet(
						close = navigateBack,
						acknowledgeWarning = acknowledgeWarning
					)
				}
			}
		}
		NetworkState.None -> AlertComponent(
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
