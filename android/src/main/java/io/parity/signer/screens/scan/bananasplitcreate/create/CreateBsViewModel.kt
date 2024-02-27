package io.parity.signer.screens.scan.bananasplitcreate.create

import androidx.lifecycle.ViewModel
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.backend.OperationResult
import io.parity.signer.domain.backend.UniffiInteractor
import io.parity.signer.domain.backend.map
import io.parity.signer.domain.backend.toOperationResult
import io.parity.signer.screens.scan.bananasplitcreate.BananaSplit
import io.parity.signer.uniffi.ErrorDisplayed
import io.parity.signer.uniffi.QrData
import kotlinx.coroutines.runBlocking


class CreateBsViewModel : ViewModel() {
	private val uniffiInteractor: UniffiInteractor =
		ServiceLocator.uniffiInteractor


	fun generatePassPhrase(totalShards: Int): OperationResult<String, ErrorDisplayed> {
		return runBlocking {
			uniffiInteractor.bsGeneratePassphrase(totalShards)
		}.toOperationResult()
	}

	suspend fun createBS(seedName: String, shards: Int, passPhrase: String) {

//		todo dmitry
		val qrResults: OperationResult<List<QrData>, ErrorDisplayed> =
			uniffiInteractor.generateBananaSplit(
				secret = "",// todo dmitry after auth
				title = seedName,
				passphrase = passPhrase,
				totalShards = shards.toUInt(),
				requiredShards = BananaSplit.getMinShards(shards).toUInt()
			).toOperationResult()

		return //todo dmitry
	}

}
