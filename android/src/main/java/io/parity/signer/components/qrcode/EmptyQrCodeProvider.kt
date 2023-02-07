package io.parity.signer.components.qrcode

import io.parity.signer.backend.UniffiInteractor
import io.parity.signer.backend.mapError
import io.parity.signer.dependencygraph.ServiceLocator


/**
 * Get ready key into into qr image
 */
class EmptyQrCodeProvider : AnimatedQrKeysProvider<List<List<UByte>>> {
	private val uniffiInteractor: UniffiInteractor =
		ServiceLocator.uniffiInteractor

	override suspend fun getQrCodesList(input: List<List<UByte>>): AnimatedQrImages? {
		return uniffiInteractor.encodeToQrImages(input).mapError()
			?.let { AnimatedQrImages(it) }
	}
}
