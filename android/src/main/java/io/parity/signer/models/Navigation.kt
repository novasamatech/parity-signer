package io.parity.signer.models

import android.util.Log
import android.widget.Toast
import io.parity.signer.BuildConfig
import io.parity.signer.bottomsheets.PrivateKeyExportModel
import io.parity.signer.components.NetworkCardModel
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.ScreenData
import io.parity.signer.uniffi.backendAction
import io.parity.signer.uniffi.generateSecretKeyQr
import java.lang.RuntimeException


@Deprecated("obsolete, for backwards compatibility, use SignerNavigator class")
fun SignerDataModel.navigate(
	button: Action,
	details: String = "",
	seedPhrase: String = ""
) {
	SignerNavigator(this).navigate(button, details, seedPhrase)
}


interface Navigator {
	fun navigate(
		button: Action,
		details: String = "",
		seedPhrase: String = ""
	)

	fun navigate(action: LocalNavRequest)
}

class SignerNavigator(private val singleton: SignerDataModel): Navigator {

	override fun navigate(button: Action, details: String, seedPhrase: String) {
		try {
			val navigationAction = backendAction(button, details, seedPhrase)
			//Workaround while Rust state machine is keeping state inside as it's needed for exporting private key in different screen
			if (navigationAction?.screenData is ScreenData.KeyDetails) {
				singleton.lastOpenedKeyDetails = (navigationAction.screenData as ScreenData.KeyDetails).f
			}
			singleton._actionResult.value = navigationAction
		} catch (e: java.lang.Exception) {
			Log.e("Navigation error", e.toString())
			Toast.makeText(singleton.context, e.toString(), Toast.LENGTH_SHORT).show()
		}
	}

	override fun navigate(action: LocalNavRequest) {
		when (action) {
			is LocalNavRequest.ShowExportPrivateKey -> {
				// `publicKey`: String, - take from MKeyDetails todo dmitry
				// `expectedSeedName`: String,
				// `networkSpecsKey`: String, // - take from MDeriveKey it's speca_key_hex
				// `seedPhrase`: String,
				// `keyPassword`:

				val keyDetails = singleton.lastOpenedKeyDetails
				if (keyDetails == null || keyDetails.pubkey != action.publicKey) {
					Toast.makeText(singleton.context, "Invalid navigation state - cannot export key. You should never see it. ${keyDetails == null}",
						Toast.LENGTH_LONG).show()
					if (BuildConfig.DEBUG) throw RuntimeException("Invalid navigation state - cannot export key. You should never see it. ${keyDetails == null}")
					return
				}
				//password only from
				val secretKeyDetailsQR = generateSecretKeyQr(dbname = singleton.dbName,
					publicKey = action.publicKey, expectedSeedName = keyDetails.address.seedName,
					networkSpecsKey = keyDetails.networkInfo.networkSpecsKey,
					seedPhrase = singleton.getSeed(keyDetails.address.seedName), keyPassword = null)
				val viewModel = PrivateKeyExportModel(qrImage = secretKeyDetailsQR.qr,
					address = secretKeyDetailsQR.address, NetworkCardModel(secretKeyDetailsQR.networkInfo))
				singleton._localNavigationAction.value = LocalNavAction.ShowExportPrivateKey(
					viewModel, singleton.navigator)
			}
		}
	}
}

class EmptyNavigator : Navigator {
	override fun navigate(button: Action, details: String, seedPhrase: String) {
		//do nothing
	}

	override fun navigate(action: LocalNavRequest) {
		//do nothing
	}
}


sealed class LocalNavAction {
	object None : LocalNavAction()
	class ShowExportPrivateKey(val model: PrivateKeyExportModel, val navigator: SignerNavigator): LocalNavAction()
}

sealed class LocalNavRequest {
	class ShowExportPrivateKey(val publicKey: String): LocalNavRequest()
}
