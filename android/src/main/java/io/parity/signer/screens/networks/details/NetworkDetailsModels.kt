package io.parity.signer.screens.networks.details

import io.parity.signer.components.ImageContent
import io.parity.signer.components.toImageContent
import io.parity.signer.screens.scan.transaction.transactionElements.MetadataModel
import io.parity.signer.screens.scan.transaction.transactionElements.toMetadataModel
import io.parity.signer.uniffi.MNetworkDetails
import io.parity.signer.uniffi.MVerifier


data class NetworkDetailsModel(
	val base58prefix: UShort,
	val color: String,
	val decimals: UByte,
	val encryptionType: String,
	val genesisHash: List<UByte>,
	val logo: String,
	val name: String,
	val order: String,
	val pathId: String,
	val secondaryColor: String,
	val title: String,
	val unit: String,
	val currentVerifier: VerifierModel,
	val meta: List<MetadataModel>
)

fun MNetworkDetails.toNetworkDetailsModel() = NetworkDetailsModel(
	base58prefix = base58prefix,
	color = color,
	decimals = decimals,
	encryptionType = encryption.name,
	genesisHash = genesisHash,
	logo = logo,
	name = name,
	order = order,
	pathId = pathId,
	secondaryColor = secondaryColor,
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
