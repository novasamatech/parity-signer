package io.parity.signer.screens.scan.bananasplitcreate.show

import androidx.lifecycle.ViewModel
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.backend.OperationResult
import io.parity.signer.domain.backend.UniffiInteractor
import io.parity.signer.domain.backend.toOperationResult
import io.parity.signer.uniffi.ErrorDisplayed
import io.parity.signer.uniffi.QrData
import kotlinx.coroutines.runBlocking


class ShowBananaSplitViewModel : ViewModel() {
	private val uniffiInteractor: UniffiInteractor =
		ServiceLocator.uniffiInteractor
//	private val usecase = BananaSplitU
//	todo dmitry


	fun getBananaSplit(seedName: String): OperationResult<List<QrData>, ErrorDisplayed> {
		//todo dmitry implement
		return OperationResult.Ok(emptyList())
	}

	fun removeBS(seedName: String) {
		//todo dmtiry remove bs and remove qr codes
	}

	fun getPassword(seedName: String): String {
//require auth
		//todo dmitry
		return ""
	}
}
