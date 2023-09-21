package io.parity.signer.screens.keydetails

import androidx.lifecycle.ViewModel
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.KeyDetailsModel
import io.parity.signer.domain.backend.OperationResult
import io.parity.signer.domain.backend.UniffiResult
import io.parity.signer.domain.storage.RepoResult
import io.parity.signer.screens.keydetails.exportprivatekey.PrivateKeyExportModel


class KeyDetailsScreenViewModel: ViewModel() {
	private val uniFfi = ServiceLocator.uniffiInteractor
	private val repo = ServiceLocator.activityScope!!.seedRepository

	suspend fun fetchModel(keyAddr: String, networkSpecs: String) =
		uniFfi.getKeyPublicKey(keyAddr, networkSpecs)

	suspend fun getPrivateExportKey(
		model: KeyDetailsModel,
	): OperationResult<PrivateKeyExportModel, Any> {
		val seedResult =
			repo.getSeedPhraseForceAuth(model.address.cardBase.seedName)
		return when (seedResult) {
			is RepoResult.Failure -> OperationResult.Err(seedResult)
			is RepoResult.Success -> {
				val secretKeyDetailsQR = uniFfi.generateSecretKeyQr(
						publicKey = model.pubkey,
						expectedSeedName = model.address.cardBase.seedName,
						networkSpecsKey = model.networkInfo.networkSpecsKey,
						seedPhrase = seedResult.result,
						keyPassword = null,
					)
				when (secretKeyDetailsQR) {
					is UniffiResult.Error -> OperationResult.Err(secretKeyDetailsQR.error)
					is UniffiResult.Success -> OperationResult.Ok(secretKeyDetailsQR.result)
				}
			}
		}
	}

	suspend fun removeDerivedKey(
		addressKey: String,
		networkSpecsKey: String,
	):  UniffiResult<Unit>  {
		return uniFfi.removedDerivedKey(addressKey, networkSpecsKey)
	}
}
