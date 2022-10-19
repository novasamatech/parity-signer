package io.parity.signer.ui

import androidx.compose.runtime.Composable
import androidx.compose.runtime.State
import io.parity.signer.bottomsheets.KeyDetailsMenuAction
import io.parity.signer.bottomsheets.exportprivatekey.PrivateKeyExportBottomSheet
import io.parity.signer.models.AlertState
import io.parity.signer.models.LocalNavAction
import io.parity.signer.models.Navigator
import io.parity.signer.models.SignerDataModel
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.uniffi.ModalData

@Composable
fun BottomSheetSelector(
	modalData: ModalData?,
	localNavAction: LocalNavAction?,
	alertState: State<AlertState?>,
	signerDataModel: SignerDataModel,
	navigator: Navigator,
) {
	SignerNewTheme {

		if (localNavAction != null && localNavAction != LocalNavAction.None) {

			when (localNavAction) {
				is LocalNavAction.ShowExportPrivateKey -> {
					BottomSheetWrapperRoot {
						PrivateKeyExportBottomSheet(
							model = localNavAction.model,
							navigator = localNavAction.navigator
						)
					}
				}
				LocalNavAction.None -> {}
			}

		} else {
			when (modalData) {
				is ModalData.KeyDetailsAction ->
					BottomSheetWrapperRoot(onClosedAction = {
						navigator.backAction()
					} ) {
						KeyDetailsMenuAction(
							navigator = navigator,
							keyDetails = signerDataModel.lastOpenedKeyDetails
						)
					}
				else -> {}
			}
		}
	}
}

