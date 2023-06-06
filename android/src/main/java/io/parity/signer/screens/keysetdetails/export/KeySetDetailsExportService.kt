package io.parity.signer.screens.keysetdetails.export

import io.parity.signer.domain.backend.UniffiInteractor
import io.parity.signer.domain.backend.mapError
import io.parity.signer.components.qrcode.AnimatedQrImages
import io.parity.signer.components.qrcode.AnimatedQrKeysProvider
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.KeyModel
import io.parity.signer.domain.getData


class KeySetDetailsExportService :
	AnimatedQrKeysProvider<KeySetDetailsExportService.GetQrCodesListRequest> {
	private val uniffiInteractor: UniffiInteractor =
		ServiceLocator.uniffiInteractor

	override suspend fun getQrCodesList(input: GetQrCodesListRequest): AnimatedQrImages? {
		return uniffiInteractor.exportSeedWithKeys(
			seed = input.seedName,
			derivedKeyAddr = input.keys.map { key -> key.addressKey })
			.mapError()
			?.let { keyInfo -> uniffiInteractor.encodeToQrImages(keyInfo.frames.map { it.getData() }) }
			?.mapError()
			?.let { AnimatedQrImages(it) }
	}

	data class GetQrCodesListRequest(
		val seedName: String,
		val keys: List<KeyModel>,
	)
}
