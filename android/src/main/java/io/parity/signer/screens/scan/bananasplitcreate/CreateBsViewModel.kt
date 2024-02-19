package io.parity.signer.screens.scan.bananasplitcreate

import androidx.lifecycle.ViewModel
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.backend.OperationResult
import io.parity.signer.domain.backend.UniffiInteractor
import io.parity.signer.domain.backend.toOperationResult
import io.parity.signer.uniffi.ErrorDisplayed
import kotlinx.coroutines.runBlocking


class CreateBsViewModel: ViewModel() {
	private val uniffiInteractor: UniffiInteractor =
		ServiceLocator.uniffiInteractor


	fun generatePassPhrase(totalShards: Int): OperationResult<String, ErrorDisplayed> {
		return runBlocking {
			uniffiInteractor.bsGeneratePassphrase(totalShards)
		}.toOperationResult()
	}

	fun createBS(shards: Int, passPhrase: String) {
//		todo dmitry
//		return uniffiInteractor.createBananaSplit(shards)
	}

}
