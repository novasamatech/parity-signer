package io.parity.signer.screens.keysets.export

import io.parity.signer.backend.UniffiInteractor
import io.parity.signer.backend.mapError
import io.parity.signer.components.qrcode.AnimatedQrImages
import io.parity.signer.components.qrcode.AnimatedQrKeysProvider
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.models.KeySetModel


class KeySetsExportService : AnimatedQrKeysProvider<List<KeySetModel>> {
	private val uniffiInteractor: UniffiInteractor =
		ServiceLocator.backendLocator.uniffiInteractor

	override suspend fun getQrCodesList(input: List<KeySetModel>): AnimatedQrImages? {
		return uniffiInteractor.exportSeedKeyInfos(input.map { it.seedName })
			.mapError()
			?.let { keyInfo -> uniffiInteractor.encodeToQrImages(keyInfo.frames) }
			?.mapError()
			?.let { AnimatedQrImages(it) }
	}
}

