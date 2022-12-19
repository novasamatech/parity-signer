package io.parity.signer.screens.keysets.export

import io.parity.signer.backend.UniffiInteractor
import io.parity.signer.backend.mapError
import io.parity.signer.components.qrcode.AnimatedQrImages
import io.parity.signer.components.qrcode.AnimatedQrKeysProvider
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.models.KeySetModel
import io.parity.signer.models.getData


class KeySetsExportService : AnimatedQrKeysProvider<List<KeySetModel>> {
	private val uniffiInteractor: UniffiInteractor =
		ServiceLocator.backendScope.uniffiInteractor

	override suspend fun getQrCodesList(input: List<KeySetModel>): AnimatedQrImages? {
		return uniffiInteractor.exportSeedKeyInfos(input.map { it.seedName })
			.mapError()
			?.let { keyInfo -> uniffiInteractor.encodeToQrImages(keyInfo.frames.map { it.getData() }) }
			?.mapError()
			?.let { AnimatedQrImages(it) }
	}
}

