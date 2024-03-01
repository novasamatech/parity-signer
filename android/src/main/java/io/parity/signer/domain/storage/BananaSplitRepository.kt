package io.parity.signer.domain.storage

import androidx.fragment.app.FragmentActivity
import io.parity.signer.domain.Authentication
import io.parity.signer.domain.backend.OperationResult
import io.parity.signer.domain.backend.UniffiInteractor
import io.parity.signer.domain.backend.toOperationResult
import io.parity.signer.screens.scan.bananasplitcreate.BananaSplit
import io.parity.signer.uniffi.ErrorDisplayed
import io.parity.signer.uniffi.QrData


class BananaSplitRepository(
	private val seedStorage: SeedStorage,
	private val clearCryptedStorage: ClearCryptedStorage,
	private val authentication: Authentication,
	private val activity: FragmentActivity,
	private val uniffiInteractor: UniffiInteractor,
) {

	suspend fun creaseBs(seedName: String, maxShards: Int, passPhrase: String): OperationResult<Unit, ErrorDisplayed> {
//todo dmitry auth

		val qrResults: OperationResult<List<QrData>, ErrorDisplayed> =
			uniffiInteractor.generateBananaSplit(
				secret = "",// todo dmitry after auth
				title = seedName,
				passphrase = passPhrase,
				totalShards = maxShards.toUInt(),
				requiredShards = BananaSplit.getMinShards(maxShards).toUInt()
			).toOperationResult()



		//todo dmitry
		return OperationResult.Ok(Unit)
	}
	//todo dmitry implement storage.getBsPassword() etc
}

data class BsPassData(val totalShards: Int, val passPhrase: String) //todo dmitry remove?

data class BsData(val qrData: List<QrData>, val totalShards: Int, val passPhrase: String)
