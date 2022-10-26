package io.parity.signer.screens.keysets.export

import io.parity.signer.backend.UniffiInteractor
import io.parity.signer.backend.UniffiResult
import io.parity.signer.backend.mapError
import io.parity.signer.models.KeySetModel
import io.parity.signer.ui.helpers.PreviewData
import io.parity.signer.uniffi.ErrorDisplayed


class KeySetsExportService(private val uniffiInteractor: UniffiInteractor) {

	suspend fun getQrCodesList(keySets: List<KeySetModel>): List<List<UByte>>? {
		val keyInfo =
			uniffiInteractor.exportKeyInfo(keySets.map { it.seedName }).mapError()
		val images = keyInfo
			?.let { keyInfo -> uniffiInteractor.encodeToQrImages(keyInfo.frames) }
			?.mapError()
		return images
	}
}

