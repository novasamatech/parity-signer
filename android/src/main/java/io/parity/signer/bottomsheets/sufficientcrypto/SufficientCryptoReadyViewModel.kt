package io.parity.signer.bottomsheets.sufficientcrypto

import io.parity.signer.backend.mapError
import io.parity.signer.dependencygraph.ServiceLocator
import kotlinx.coroutines.runBlocking


object SufficientCryptoReadyViewModel {
	fun getQrCodeBitmapFromQrCodeData(data: List<UByte>): List<UByte>? {
		val interactor = ServiceLocator.uniffiInteractor
		return runBlocking {
			interactor.encodeToQrImages(listOf(data))
				.mapError()
				?.firstOrNull()
		}
	}
}
