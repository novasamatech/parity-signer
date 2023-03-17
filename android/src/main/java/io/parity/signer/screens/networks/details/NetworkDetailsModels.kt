package io.parity.signer.screens.networks.details

import io.parity.signer.components.ImageContent
import io.parity.signer.components.toImageContent
import io.parity.signer.domain.encodeHex
import io.parity.signer.screens.scan.transaction.transactionElements.MetadataModel
import io.parity.signer.screens.scan.transaction.transactionElements.toMetadataModel
import io.parity.signer.ui.helpers.PreviewData
import io.parity.signer.uniffi.MNetworkDetails
import io.parity.signer.uniffi.MVerifier


data class NetworkDetailsModel(
	val base58prefix: UShort,
//	val color: String,
	val decimals: UByte,
//	val encryptionType: String,
	val genesisHash: String,
	val logo: String,
	val name: String,
//	val order: String,
//	val pathId: String,
//	val secondaryColor: String,
	val title: String,
	val unit: String,
	val currentVerifier: VerifierModel,
	val meta: List<MetadataModel>
) {
	companion object {
		fun createStub() = NetworkDetailsModel(
			base58prefix = 0u,
			decimals = 10.toUByte(),
			genesisHash = "5DCmwXp8XLzSMUyE4uhJMKV4vwvsWqqBYFKJq38CW53VHEVq",
			logo = "polkadot",
			name = "Polkadot",
			title = "Polkadot",
			unit = "DOT",
			currentVerifier = VerifierModel(
				"custom",
				"vwvsWqqBYFK",
				PreviewData.exampleIdenticonPng,
				"src3322"
			),
			meta = listOf(MetadataModel.createStub(), MetadataModel.createStub())
		)
	}
}

fun MNetworkDetails.toNetworkDetailsModel() = NetworkDetailsModel(
	base58prefix = base58prefix,
	decimals = decimals,
	genesisHash = genesisHash.toUByteArray().toByteArray().encodeHex(),
	logo = logo,
	name = name,
	title = title,
	unit = unit,
	currentVerifier = currentVerifier.toVerifierModel(),
	meta = meta.map { it.toMetadataModel() },
)

data class VerifierModel(
	val ttype: String,
	val publicKey: String,
	val identicon: ImageContent,
	val encryption: String
)

fun MVerifier.toVerifierModel() = VerifierModel(
	ttype = ttype,
	publicKey = details.publicKey,
	identicon = details.identicon.toImageContent(),
	encryption = details.encryption,
)
