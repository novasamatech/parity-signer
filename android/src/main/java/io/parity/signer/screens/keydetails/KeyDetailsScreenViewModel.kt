package io.parity.signer.screens.keydetails

import android.content.Context
import android.widget.Toast
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.KeyDetailsModel
import io.parity.signer.domain.storage.getSeed
import io.parity.signer.screens.keydetails.exportprivatekey.PrivateKeyExportModel
import io.parity.signer.screens.keydetails.exportprivatekey.toPrivateKeyExportModel
import io.parity.signer.uniffi.generateSecretKeyQr


class KeyDetailsScreenViewModel {
	private val uniFfi = ServiceLocator.uniffiInteractor

	suspend fun fetchModel(keyAddr: String, networkSpecs: String) =
		uniFfi.getKeyPublicKey(keyAddr, networkSpecs)

	fun getPrivateExportKey(model: KeyDetailsModel, context: Context): PrivateKeyExportModel? {
		val seed = singleton.getSeed(keyDetails.address.seedName),

		val secretKeyDetailsQR = try {
			generateSecretKeyQr(
				publicKey = model.pubkey,
				expectedSeedName = model.address.cardBase.seedName,
				networkSpecsKey = model.networkInfo.networkSpecsKey,
				seedPhrase = seed,
				keyPassword = null,
			).toPrivateKeyExportModel()
		} catch (e: Exception) {
			//todo issue #1533
			Toast.makeText(
				context,
				"For passworded keys not yet supported",
				Toast.LENGTH_LONG
			).show()
			null
		}
		return secretKeyDetailsQR
	}
}
