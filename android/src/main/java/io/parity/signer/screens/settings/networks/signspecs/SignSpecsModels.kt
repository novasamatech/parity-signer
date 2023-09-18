package io.parity.signer.screens.settings.networks.signspecs

import io.parity.signer.components.sharedcomponents.KeyCardModelBase
import io.parity.signer.domain.toKeysModel
import io.parity.signer.ui.helpers.PreviewData
import io.parity.signer.uniffi.MAddressCard
import io.parity.signer.uniffi.MRawKey
import io.parity.signer.uniffi.MSignSufficientCrypto
import io.parity.signer.uniffi.MSufficientCryptoReady
import io.parity.signer.uniffi.MscContent


data class SignSpecsListModel(val keys: List<KeyCardModelBase>) {

	companion object {
		fun createStub(): SignSpecsListModel = SignSpecsListModel(
			listOf(
				KeyCardModelBase.createStub(),
				KeyCardModelBase.createStub(),
				KeyCardModelBase.createStub(),
				KeyCardModelBase.createStub(),
				KeyCardModelBase.createStub(),
			)
		)
	}
}

fun MSignSufficientCrypto.toSignSpecsListModel() = SignSpecsListModel(
	keys = identities.map { it.toKeyCardModel() }
)

fun MRawKey.toKeyCardModel() =
	KeyCardModelBase.fromAddress(
		address= address,
		base58 = publicKey,
		networkLogo,
	)




data class SignSpecsResultModel(
	val key: KeyCardModelBase,
	val sufficientSignature: List<UByte>,
	val content: MscContent,
) {
	companion object {
		fun createStub(): SignSpecsResultModel = SignSpecsResultModel(
			key = KeyCardModelBase.createStub(),
				sufficientSignature = PreviewData.exampleQRData,
				content = MscContent.LoadMetadata("metadata name", 2015u)
		)

	}
}
internal fun MSufficientCryptoReady.toSignSpecsResultModel(): SignSpecsResultModel {
	return SignSpecsResultModel(
		key = KeyCardModelBase.fromKeyModel(authorInfo.toKeysModel(), networkLogo),
		sufficientSignature = sufficient,
		content =content,
	)
}
