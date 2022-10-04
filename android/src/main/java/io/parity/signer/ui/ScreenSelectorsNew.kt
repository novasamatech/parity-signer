package io.parity.signer.ui

import androidx.compose.runtime.Composable
import androidx.compose.runtime.State
import io.parity.signer.bottomsheets.*
import io.parity.signer.models.*
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.ModalData

@Composable
fun BottomSheetSelector(
	modalData: ModalData?,
	localNavAction: LocalNavAction?,
	alertState: State<AlertState?>,
	button: (Action, String, String) -> Unit,
	signerDataModel: SignerDataModel
) {
	val button1: (Action) -> Unit = { action -> button(action, "", "") }
	val button2: (Action, String) -> Unit =
		{ action, details -> button(action, details, "") }
	if (localNavAction != null && localNavAction != LocalNavAction.None) {
		SignerNewTheme {
			when (localNavAction) {
				is LocalNavAction.ShowExportPrivateKey -> {
					BottomSheetWrapper {
						PrivateKeyExportBottomSheet(
							model = localNavAction.model,
							navigator = localNavAction.navigator
						)
					}
				}
				LocalNavAction.None -> { }
			}
		}
	} else {
	}
}

