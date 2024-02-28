package io.parity.signer.domain.storage

import androidx.fragment.app.FragmentActivity
import io.parity.signer.domain.Authentication
import io.parity.signer.domain.backend.OperationResult
import io.parity.signer.domain.backend.UniffiInteractor
import io.parity.signer.uniffi.ErrorDisplayed
import io.parity.signer.uniffi.QrData


class BananaSplitRepository(
	private val seedStorage: SeedStorage,
	private val clearCryptedStorage: ClearCryptedStorage,
	private val authentication: Authentication,
	private val activity: FragmentActivity,
	private val uniffiInteractor: UniffiInteractor,
) {

	fun creaseBs(seedName: String, shards: Int, passPhrase: String): OperationResult<Unit, ErrorDisplayed> {
		//todo dmitry
		return OperationResult.Ok(Unit)
	}
	//todo dmitry implement storage.getBsPassword() etc
}

data class BsPassData(val totalShards: Int, val passPhrase: String)

data class BsData(val qrData: List<QrData>, val totalShards: Int, val passPhrase: String)
