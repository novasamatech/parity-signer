package io.parity.signer.screens.keysetdetails.export

import io.parity.signer.backend.UniffiInteractor
import io.parity.signer.backend.mapError
import io.parity.signer.components.qrcode.AnimatedQrKeysProvider
import io.parity.signer.dependencyGraph.ServiceLocator
import io.parity.signer.models.KeyModel
import io.parity.signer.models.NetworkModel


class KeySetDetailsExportService() :
	AnimatedQrKeysProvider<KeySetDetailsExportService.GetQrCodesListRequest> {
	private val uniffiInteractor: UniffiInteractor =
		ServiceLocator.backendLocator.uniffiInteractor
	override suspend fun getQrCodesList(input: GetQrCodesListRequest): List<List<UByte>>? {
		return uniffiInteractor.exportSeedWithKeys(input.seedName,
			input.keys.map { key -> key.addressKey to input.networkModel.title }) //todo dmitry network title is wrong
				.mapError()
			?.let { keyInfo -> uniffiInteractor.encodeToQrImages(keyInfo.frames) }
			?.mapError()
	}

	data class GetQrCodesListRequest(
		val seedName: String,
		val networkModel: NetworkModel,
		val keys: List<KeyModel>,
	)
}
