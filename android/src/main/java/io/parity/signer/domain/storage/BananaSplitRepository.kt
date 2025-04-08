package io.parity.signer.domain.storage

import androidx.fragment.app.FragmentActivity
import io.parity.signer.domain.AuthResult
import io.parity.signer.domain.Authentication
import io.parity.signer.domain.backend.OperationResult
import io.parity.signer.domain.backend.UniffiInteractor
import io.parity.signer.domain.backend.toOperationResult
import io.parity.signer.screens.scan.bananasplitcreate.BananaSplit
import io.parity.signer.uniffi.ErrorDisplayed
import io.parity.signer.uniffi.QrData
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext


class BananaSplitRepository(
	private val seedStorage: SeedStorage,
	private val clearCryptedStorage: ClearCryptedStorage,
	private val authentication: Authentication,
	private val activity: FragmentActivity,
	private val uniffiInteractor: UniffiInteractor,
) {

	suspend fun creaseBs(
		seedName: String,
		maxShards: Int,
		passPhrase: String
	): OperationResult<Unit, ErrorDisplayed> {

		return when (val authResult = authentication.authenticate(activity)) {
			AuthResult.AuthSuccess -> {
				val phrase = seedStorage.getSeed(seedName, false)
				val qrResults: OperationResult<List<QrData>, ErrorDisplayed> =
					uniffiInteractor.generateBananaSplit(
						secret = phrase,
						title = seedName,
						passphrase = passPhrase,
						totalShards = maxShards.toUInt(),
						requiredShards = BananaSplit.getMinShards(maxShards).toUInt()
					).toOperationResult()
				when (qrResults) {
					is OperationResult.Err -> qrResults
					is OperationResult.Ok -> {
						//saving data
						seedStorage.saveBsData(seedName, passPhrase)
						clearCryptedStorage.saveBsQRCodes(seedName, qrResults.result)
						OperationResult.Ok(Unit)
					}
				}
			}

			AuthResult.AuthError,
			AuthResult.AuthFailed,
			AuthResult.AuthUnavailable -> {
				OperationResult.Err(ErrorDisplayed.Str("auth error - $authResult"))
			}
		}
	}

	fun getBsQrs(seedName: String): List<QrData>? {
		return clearCryptedStorage.getBsQrCodes(seedName)
	}

	suspend fun removeBS(seedName: String): OperationResult<Unit, ErrorDisplayed> {
		return when (val authResult = authentication.authenticate(activity)) {
			AuthResult.AuthSuccess -> {
				//removing bs data data
				withContext(Dispatchers.IO) {
					seedStorage.removeBSData(seedName)
					clearCryptedStorage.removeQrCode(seedName)
				}
				OperationResult.Ok(Unit)
			}

			AuthResult.AuthError,
			AuthResult.AuthFailed,
			AuthResult.AuthUnavailable -> {
				OperationResult.Err(ErrorDisplayed.Str("auth error - $authResult"))
			}
		}
	}

	suspend fun getBsPassword(seedName: String): OperationResult<String, ErrorDisplayed> {
		return when (val authResult = authentication.authenticate(activity)) {
			AuthResult.AuthSuccess -> {
				val bsPassword = withContext(Dispatchers.IO) {
					seedStorage.getBsPassword(seedName)
				}
				OperationResult.Ok(bsPassword)
			}

			AuthResult.AuthError,
			AuthResult.AuthFailed,
			AuthResult.AuthUnavailable -> {
				OperationResult.Err(ErrorDisplayed.Str("auth error - $authResult"))
			}
		}
	}
}


