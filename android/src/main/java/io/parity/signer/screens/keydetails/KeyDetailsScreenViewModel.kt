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

	//keeping this state because we don't want to ask for password twice
	private var privateExportStateModel: PrivateKeyExportModel? = null

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

		privateExportStateModel?.let { saved ->
			if (saved.keyCard == model.address) {
				//password check result saved or already saved before config change
				return OperationResult.Ok(saved)
			}
		}

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
					is UniffiResult.Success -> {
						privateExportStateModel = secretKeyDetailsQR.result
						OperationResult.Ok(secretKeyDetailsQR.result)
					}
				}
			}
		}
	}

	fun clearExportResultState() {
		privateExportStateModel = null
	}

	fun createPasswordModel(keyModel: KeyDetailsModel): EnterPasswordModel {
		return EnterPasswordModel(
			keyCard = keyModel.address.cardBase,
			showError = false,
			attempt = 0,
		)
	}

	suspend fun tryPassword(
		keyModel: KeyDetailsModel,
		passwordModel: EnterPasswordModel,
		password: String
	): EnterPasswordReply {
		val seedResult = if (passwordModel.attempt == 0) {
			repo.getSeedPhraseForceAuth(keyModel.address.cardBase.seedName)
		} else {
			repo.getSeedPhrases(listOf(keyModel.address.cardBase.seedName))
		}

		return when (seedResult) {
			is RepoResult.Failure -> {
				EnterPasswordReply.ErrorAuthWrong
			}

			is RepoResult.Success -> {
				val secretKeyDetailsQR = uniFfi.generateSecretKeyQr(
					publicKey = keyModel.pubkey,
					expectedSeedName = keyModel.address.cardBase.seedName,
					networkSpecsKey = keyModel.networkInfo.networkSpecsKey,
					seedPhrase = seedResult.result,
					keyPassword = password,
				)
				when (secretKeyDetailsQR) {
					is UniffiResult.Error -> {
						if (passwordModel.attempt > 3) {
							EnterPasswordReply.ErrorAttemptsExceeded
						} else {
							EnterPasswordReply.UpdatePassword(
								passwordModel.copy(
									showError = true,
									attempt = passwordModel.attempt + 1
								)
							)
						}
					}

					is UniffiResult.Success -> {
						privateExportStateModel = secretKeyDetailsQR.result
						EnterPasswordReply.OK(password)
					}
				}
			}
		}
	}

	sealed class EnterPasswordReply {
		data class OK(val password: String) : EnterPasswordReply()
		data class UpdatePassword(val model: EnterPasswordModel) :
			EnterPasswordReply()

		data object ErrorAttemptsExceeded : EnterPasswordReply()
		data object ErrorAuthWrong : EnterPasswordReply()
	}

	suspend fun removeDerivedKey(
		addressKey: String,
		networkSpecsKey: String,
	): UniffiResult<Unit> {
		return uniFfi.removedDerivedKey(addressKey, networkSpecsKey)
	}
}
