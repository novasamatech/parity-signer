package io.parity.signer.screens.settings.networks.signnetworkcrypto

import androidx.lifecycle.ViewModel
import io.parity.signer.domain.backend.SignSufficientCryptoInteractor
import io.parity.signer.uniffi.MSignSufficientCrypto


class SignSufficientCryptoViewModel : ViewModel() {
	val interactor = SignSufficientCryptoInteractor()

	suspend fun getNetworkModel(networkKey: String): MSignSufficientCrypto? =
		interactor.signNetworkSpecs(networkKey)

	suspend fun getMetadataModel(
		networkKey: String,
		versionSpec: String
	): MSignSufficientCrypto? = interactor.signMetadataSpecInfo(networkKey, versionSpec)
}
