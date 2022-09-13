package io.parity.signer.models

import android.util.Log
import android.widget.Toast
import io.parity.signer.bottomsheets.PrivateKeyExportModel
import io.parity.signer.components.NetworkCardModel
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.backendAction
import io.parity.signer.uniffi.generateSecretKeyQr


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

	fun navigate(action: LocalNavAction)
}

class SignerNavigator(private val singleton: SignerDataModel): Navigator {

	override fun navigate(button: Action, details: String, seedPhrase: String) {
		try {
			singleton._actionResult.value = backendAction(button, details, seedPhrase)
		} catch (e: java.lang.Exception) {
			Log.e("Navigation error", e.toString())
			Toast.makeText(singleton.context, e.toString(), Toast.LENGTH_SHORT).show()
		}
	}

	override fun navigate(action: LocalNavAction) {
		when (action) {
			is LocalNavAction.ShowExportPrivateKey -> {
				// `publicKey`: String, - take from MKeyDetails todo dmitry
				// `expectedSeedName`: String,
				// `networkSpecsKey`: String, // - take from MDeriveKey it's speca_key_hex
				// `seedPhrase`: String,
				// `keyPassword`: String?
				val keyDetails = generateSecretKeyQr(dbname = singleton.dbName, publicKey = action.publicKey,
					expectedSeedName = action.expectedSeedName, networkSpecsKey = action.networkSpecsKey,
					seedPhrase =  action.seedPhrase, keyPassword = action.keyPassword)
				val viewModel = PrivateKeyExportModel(qrImage = keyDetails.qr,
					address = keyDetails.address, NetworkCardModel(keyDetails.networkInfo))
			}
		}
	}
}

class EmptyNavigator : Navigator {
	override fun navigate(button: Action, details: String, seedPhrase: String) {
		//do nothing
	}

	override fun navigate(action: LocalNavAction) {
		//do nothing
	}
}


sealed class LocalNavAction {
	class ShowExportPrivateKey(val publicKey: String, val expectedSeedName: String,val networkSpecsKey: String,
														 val seedPhrase: String,val keyPassword: String? = null): LocalNavAction()
}
