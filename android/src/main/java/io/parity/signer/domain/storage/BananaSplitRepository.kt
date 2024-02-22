package io.parity.signer.domain.storage

import androidx.fragment.app.FragmentActivity
import io.parity.signer.domain.Authentication
import io.parity.signer.domain.backend.UniffiInteractor
import io.parity.signer.uniffi.QrData


class BananaSplitRepository(
	private val storage: SeedStorage,
	private val authentication: Authentication,
	private val activity: FragmentActivity,
	private val uniffiInteractor: UniffiInteractor,
) {

	//todo dmitry implement storage.getBsPassword() etc
}

data class BsPassData(val totalShards: Int, val passPhrase: String)

data class BsData(val qrData: List<QrData>, val totalShards: Int, val passPhrase: String)
