package io.parity.signer.screens.keydetails

import androidx.lifecycle.ViewModel
import io.parity.signer.bottomsheets.password.EnterPasswordModel
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.KeyDetailsModel
import io.parity.signer.domain.backend.OperationResult
import io.parity.signer.domain.backend.UniffiResult
import io.parity.signer.domain.storage.RepoResult
import io.parity.signer.screens.keydetails.exportprivatekey.PrivateKeyExportModel
import io.parity.signer.uniffi.MKeyDetails


class KeyDetailsScreenViewModel : ViewModel() {
	private val uniFfi = ServiceLocator.uniffiInteractor
	private val repo = ServiceLocator.activityScope!!.seedRepository

	suspend fun fetchModel(
		keyAddr: String,
		networkSpecs: String
	): UniffiResult<MKeyDetails> {
		return uniFfi.getKeyPublicKey(keyAddr, networkSpecs)
	}

	suspend fun getPrivateExportKey(
		model: KeyDetailsModel,
		password: String?
	): OperationResult<PrivateKeyExportModel, Any> {
		//todo dmitry think of not asking twice in case of password
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
					keyPassword = password,
				)
				when (secretKeyDetailsQR) {
					is UniffiResult.Error -> OperationResult.Err(secretKeyDetailsQR.error)
					is UniffiResult.Success -> OperationResult.Ok(secretKeyDetailsQR.result)
				}
			}
		}
	}

	fun createPasswordModel(keyModel: KeyDetailsModel): EnterPasswordModel {
		return EnterPasswordModel(
			keyCard = keyModel.address.cardBase,
			showError = false,
			attempt = 0,
		)
	}

	fun tryPassword(
		keyModel: KeyDetailsModel,
		passwordModel: EnterPasswordModel,
		password: String
	): EnterPasswordReply {
		val result = getPrivateExportKey(keyModel, password)
		when (result) {
			is OperationResult.Err -> TODO()
			is OperationResult.Ok -> TODO()
		}
//todo dmitry finalize
	}

	sealed class EnterPasswordReply {
		data class OK(val password: String) : EnterPasswordReply()
		data class UpdatePass(val model: EnterPasswordModel) : EnterPasswordReply()
		data object ErrorAttemptsExceeded : EnterPasswordReply()
	}

	suspend fun removeDerivedKey(
		addressKey: String,
		networkSpecsKey: String,
	): UniffiResult<Unit> {
		return uniFfi.removedDerivedKey(addressKey, networkSpecsKey)
	}
}
