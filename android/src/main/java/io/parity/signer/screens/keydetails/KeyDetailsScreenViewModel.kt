package io.parity.signer.screens.keydetails

import android.content.Context
import android.widget.Toast
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.KeyDetailsModel
import io.parity.signer.domain.backend.OperationResult
import io.parity.signer.domain.backend.UniffiResult
import io.parity.signer.domain.storage.RepoResult
import io.parity.signer.screens.keydetails.exportprivatekey.PrivateKeyExportModel


class KeyDetailsScreenViewModel {
	private val uniFfi = ServiceLocator.uniffiInteractor
	private val repo = ServiceLocator.activityScope!!.seedRepository

	suspend fun fetchModel(keyAddr: String, networkSpecs: String) =
		uniFfi.getKeyPublicKey(keyAddr, networkSpecs)

	suspend fun getPrivateExportKey(
		model: KeyDetailsModel,
	): OperationResult<PrivateKeyExportModel, Any> {
		val seedResult =
			repo.getSeedPhraseForceAuth(model.address.cardBase.seedName)
		when (seedResult) {
			is RepoResult.Failure -> return OperationResult.Err(seedResult)
			is RepoResult.Success -> {
				val secretKeyDetailsQR = uniFfi.generateSecretKeyQr(
						publicKey = model.pubkey,
						expectedSeedName = model.address.cardBase.seedName,
						networkSpecsKey = model.networkInfo.networkSpecsKey,
						seedPhrase = seedResult.result,
						keyPassword = null,
					)
				return when (secretKeyDetailsQR) {
					is UniffiResult.Error -> OperationResult.Err(secretKeyDetailsQR.error)
					is UniffiResult.Success -> OperationResult.Ok(secretKeyDetailsQR.result)
				}
			}
		}
	}

	suspend fun removeDerivedKey(
		addressKey: String,
		networkSpecsKey: String,
	): Boolean {
		val result = uniFfi.removedDerivedKey(addressKey, networkSpecsKey)
		return when (result) {
			is UniffiResult.Error -> false
			is UniffiResult.Success -> true
		}
	}
}
