package io.parity.signer.models

import android.util.Log
import android.widget.Toast
import io.parity.signer.BuildConfig
import io.parity.signer.screens.keydetails.exportprivatekey.PrivateKeyExportModel
import io.parity.signer.components.NetworkCardModel
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.ScreenData
import io.parity.signer.uniffi.backendAction
import io.parity.signer.uniffi.generateSecretKeyQr
import io.parity.signer.uniffi.MAddressCard
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
	/**
	 * For old Rust-backed navigation actions
	 */
	fun navigate(
		button: Action,
		details: String = "",
		seedPhrase: String = ""
	)

	fun navigate(action: LocalNavRequest)

	fun backAction()
}

class SignerNavigator(private val singleton: SignerDataModel) : Navigator {

	override fun navigate(button: Action, details: String, seedPhrase: String) {
		if (singleton.localNavAction.value != LocalNavAction.None) {
			//if state machine navigation triggered - remove platform layers on top
			singleton._localNavAction.value = LocalNavAction.None
		}

		if (button == Action.NAVBAR_SCAN) {
			//swop rust-side navigation call to a local one so back navigation would work and move us back to where we came from
			navigate(LocalNavRequest.ShowScan)
			return
		}

		try {
			val navigationAction = backendAction(button, details, seedPhrase)
			//Workaround while Rust state machine is keeping state inside as it's needed for exporting private key in different screen
			if (navigationAction.screenData is ScreenData.KeyDetails) {
				singleton.lastOpenedKeyDetails =
					(navigationAction.screenData as ScreenData.KeyDetails).f
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
				val keyDetails = singleton.lastOpenedKeyDetails
				if (keyDetails == null || keyDetails.pubkey != action.publicKey) {
					Toast.makeText(
						singleton.context,
						"Invalid navigation state - cannot export key. You should never see it. ${keyDetails == null}",
						Toast.LENGTH_LONG
					).show()
					if (BuildConfig.DEBUG) throw RuntimeException("Invalid navigation state - cannot export key. You should never see it. ${keyDetails == null}")
					return
				}
				val secretKeyDetailsQR = generateSecretKeyQr(
					dbname = singleton.dbName,
					publicKey = action.publicKey,
					expectedSeedName = keyDetails.address.seedName,
					networkSpecsKey = keyDetails.networkInfo.networkSpecsKey,
					seedPhrase = singleton.getSeed(keyDetails.address.seedName),
					keyPassword = null
				)
				val model = PrivateKeyExportModel(
					qrData = secretKeyDetailsQR.qr.getData(),
					keyCard = KeyCardModel.fromAddress(
						address_card = MAddressCard(
							address = secretKeyDetailsQR.address,
							base58 = secretKeyDetailsQR.base58,
							addressKey = ""// not used there anyway
						),
						networkTitle = secretKeyDetailsQR.networkInfo.networkTitle
					),
					NetworkCardModel(secretKeyDetailsQR.networkInfo)
				)
				navigate(Action.GO_BACK) // close bottom sheet from rust stack
				singleton._localNavAction.value =
					LocalNavAction.ShowExportPrivateKey(
						model, singleton.navigator
					)
			}
			LocalNavRequest.ShowScan -> singleton._localNavAction.value =
				LocalNavAction.ShowScan
		}
	}

	override fun backAction() {
		if (singleton.localNavAction.value !is LocalNavAction.None) {
			singleton._localNavAction.value = LocalNavAction.None
		} else {
			backRustNavigation()
		}
	}

	private fun backRustNavigation() {
		val lastRustNavAction = singleton.actionResult.value
		if (
			lastRustNavAction.alertData == null &&
			lastRustNavAction.modalData == null &&
			(
				lastRustNavAction.screenData is ScreenData.Log ||
					lastRustNavAction.screenData is ScreenData.Scan ||
					lastRustNavAction.screenData is ScreenData.SeedSelector ||
					lastRustNavAction.screenData is ScreenData.Settings
				)
		) {
			singleton.activity.moveTaskToBack(true)
		} else
			navigate(Action.GO_BACK)
	}
}

class EmptyNavigator : Navigator {
	override fun navigate(button: Action, details: String, seedPhrase: String) {
		//do nothing
	}

	override fun navigate(action: LocalNavRequest) {
		//do nothing
	}

	override fun backAction() {
	}
}


sealed class LocalNavAction {
	object None : LocalNavAction()
	class ShowExportPrivateKey( //todo dmitry refactor this to show this screen right on old screen without global navigation
        val model: PrivateKeyExportModel,
        val navigator: SignerNavigator
	) : LocalNavAction()
	object ShowScan: LocalNavAction()
}

sealed class LocalNavRequest {
	class ShowExportPrivateKey(val publicKey: String) : LocalNavRequest()
	object ShowScan : LocalNavRequest()
}
