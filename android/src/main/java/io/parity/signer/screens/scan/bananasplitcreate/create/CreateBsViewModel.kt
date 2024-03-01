package io.parity.signer.screens.scan.bananasplitcreate.create

import androidx.lifecycle.ViewModel
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.backend.OperationResult
import io.parity.signer.domain.backend.map
import io.parity.signer.domain.backend.toOperationResult
import io.parity.signer.screens.scan.bananasplitcreate.BananaSplit
import io.parity.signer.uniffi.ErrorDisplayed
import io.parity.signer.uniffi.QrData
import kotlinx.coroutines.runBlocking


class CreateBsViewModel : ViewModel() {
	private val uniffiInteractor = ServiceLocator.uniffiInteractor
	private val bsRepository = ServiceLocator.activityScope!!.bsRepository

	fun generatePassPhrase(totalShards: Int): OperationResult<String, ErrorDisplayed> {
		return runBlocking {
			uniffiInteractor.bsGeneratePassphrase(totalShards)
		}.toOperationResult()
	}

	suspend fun createBS(
		seedName: String,
		maxShards: Int,
		passPhrase: String
	): OperationResult<Unit, ErrorDisplayed> {
		return bsRepository.creaseBs(seedName, maxShards, passPhrase)
	}

}
