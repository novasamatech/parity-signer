package io.parity.signer.domain

import io.parity.signer.components.sharedcomponents.KeyCardModel
import io.parity.signer.domain.backend.OperationResult
import io.parity.signer.ui.helpers.PreviewData
import io.parity.signer.uniffi.*
import java.util.*

/**
 * reflection of uniffi models so compose will work properly
 */

/**
 * Local copy of shared [MKeys] class
 */
data class KeySetDetailsModel(
	val keysAndNetwork: List<KeyAndNetworkModel>,
	val root: KeyModel,
) {
	companion object {
		fun createStub(): KeySetDetailsModel = KeySetDetailsModel(
			keysAndNetwork = listOf(
				KeyAndNetworkModel(
					key = KeyModel.createStub(addressKey = "address key"),
					network = NetworkInfoModel.createStub()
				),
				KeyAndNetworkModel(
					key = KeyModel.createStub(addressKey = "address key2"),
					network = NetworkInfoModel.createStub(networkName = "Some")
				),
				KeyAndNetworkModel(
					key = KeyModel.createStub(addressKey = "address key3")
						.copy(path = "//polkadot//path3"),
					network = NetworkInfoModel.createStub()
				),
			),
			root = KeyModel.createStub()
				.copy(path = "//polkadot", identicon = PreviewData.Identicon.jdenticonIcon),
		)
	}
}


fun MKeysNew.toKeySetDetailsModel(): OperationResult<KeySetDetailsModel, ErrorDisplayed> {
	return if (root == null) {
		OperationResult.Err(ErrorDisplayed.Str("Key Set is missing in DB or storage inconsistent"))
	} else {
		OperationResult.Ok(
			KeySetDetailsModel(
				keysAndNetwork = set.map { it.toKeyAndNetworkModel() },
				root = root!!.toKeysModel(),
			)
		)
	}
}


data class KeyAndNetworkModel(
	val key: KeyModel,
	val network: NetworkInfoModel
) {
	companion object {
		fun createStub() =
			KeyAndNetworkModel(KeyModel.createStub(), NetworkInfoModel.createStub())
	}
}

fun MKeyAndNetworkCard.toKeyAndNetworkModel() = KeyAndNetworkModel(
	key = key.toKeyModel(),
	network = network.toNetworkInfoModel()
)

/**
 * Local copy of shared [MKeysCard] class
 */
data class KeyModel(
	val identicon: Identicon,
	val addressKey: String,
	val seedName: String,
	val base58: String,
	val hasPwd: Boolean,
	val path: String,
	val secretExposed: Boolean,
) {
	companion object {
		fun createStub(
			addressKey: String = "address key",
		) = KeyModel(
			addressKey = addressKey,
			base58 = "5F3sa2TJAWMqDhXG6jhV4N8ko9SxwGy8TpaNS1repo5EYjQX",
			identicon = PreviewData.Identicon.dotIcon,
			hasPwd = true,
			path = "//polkadot//path2",
			secretExposed = false,
			seedName = "sdsdsd",
		)
	}
}

fun MAddressCard.toKeysModel() = KeyModel(
	addressKey = addressKey,
	base58 = base58,
	identicon = address.identicon,
	hasPwd = address.hasPwd,
	path = address.path,
	secretExposed = address.secretExposed,
	seedName = address.seedName,
)

/**
 * [MKeysCard] is the same as MAddressCard but can be swiped.
 */
fun MKeysCard.toKeyModel() = KeyModel(
	addressKey = addressKey,
	base58 = base58,
	identicon = address.identicon,
	hasPwd = address.hasPwd,
	path = address.path,
	secretExposed = address.secretExposed,
	seedName = address.seedName,
)

/**
 * Local copy of shared [MNetworkCard] class
 */
data class NetworkBasicModel(
	val title: String,
	val logo: String,
)

fun MNetworkCard.toNetworkBasicModel() = NetworkBasicModel(
	title = title,
	logo = logo,
)

/**
 * Local copy of shared [MSeeds] class
 */
data class KeySetsListModel(val keys: List<KeySetModel>)

fun MSeeds.toKeySetsSelectModel() = KeySetsListModel(
	seedNameCards.map { it.toSeedModel() }
)

/**
 * Local copy of shared [SeedNameCard] class
 */
data class KeySetModel(
	val seedName: String,
	val identicon: Identicon,
	val usedInNetworks: List<String>,
	val derivedKeysCount: UInt,
) {
	companion object {
		fun createStub(name: String? = null, number: Int? = null) =
			KeySetModel(
				name ?: "first seed name",
				PreviewData.Identicon.jdenticonIcon,
				listOf("westend", "some"),
				number?.toUInt() ?: 1.toUInt()
			)
	}
}


fun SeedNameCard.toSeedModel() =
	KeySetModel(
		seedName,
		identicon,
		usedInNetworks,
		derivedKeysCount
	)

/**
 * Local copy of shared [MKeyDetails] class
 */
data class KeyDetailsModel(
	val qrData: List<UByte>,
	val pubkey: String,
	val networkInfo: NetworkInfoModel,
	val address: KeyCardModel,
	val base58: String,
	val secretExposed: Boolean,
) {
	val isRootKey = address.cardBase.path.isEmpty()

	companion object {
		fun createStubDerived(): KeyDetailsModel {
			val keyCard = KeyCardModel.createStub()
			return KeyDetailsModel(
				qrData = PreviewData.exampleQRData,
				pubkey = "public key",
				networkInfo = NetworkInfoModel(
					"network title",
					"network logo", "network specs"
				),
				address = keyCard,
				base58 = keyCard.cardBase.base58,
				secretExposed = true,
			)
		}

		fun createStubRoot(): KeyDetailsModel {
			val keyCard = KeyCardModel.createStub()
			return KeyDetailsModel(
				qrData = PreviewData.exampleQRData,
				pubkey = "public key",
				networkInfo = NetworkInfoModel(
					"network title",
					"network logo", "network specs"
				),
				address = KeyCardModel(
					keyCard.network, keyCard.cardBase.copy(path = "")
				),
				base58 = keyCard.cardBase.base58,
				secretExposed = true,
			)
		}
	}
}

fun MKeyDetails.toKeyDetailsModel() =
	KeyDetailsModel(
		qrData = qr.getData(),
		pubkey = pubkey,
		networkInfo = networkInfo.toNetworkInfoModel(),
		address = KeyCardModel.fromAddress(
			address = address,
			base58 = base58,
			network = networkInfo.toNetworkInfoModel(),
		),
		base58 = base58,
		secretExposed = address.secretExposed,
	)


/**
 * Local copy of shared [MscNetworkInfo] class
 */
data class NetworkInfoModel(
	val networkTitle: String,
	val networkLogo: String,
	val networkSpecsKey: String
) {
	companion object {
		fun createStub(networkName: String? = null): NetworkInfoModel =
			NetworkInfoModel(
				networkTitle = networkName ?: "Westend",
				networkLogo = networkName?.lowercase() ?: "westend",
				networkSpecsKey = "01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"
			)
	}
}

fun MscNetworkInfo.toNetworkInfoModel() =
	NetworkInfoModel(networkTitle.replaceFirstChar {
		if (it.isLowerCase()) it.titlecase() else it.toString()
	}, networkLogo, networkSpecsKey)

fun QrData.getData(): List<UByte> =
	when (this) {
		is QrData.Regular -> this.data
		is QrData.Sensitive -> this.data
	}


/**
 * Local copy of shared [Network] and [MmNetwork]
 */
data class NetworkModel(
	val key: String,
	val logo: String,
	val title: String,
	val pathId: String, //default path for this network
) {
	companion object {
		fun createStub(networkName: String? = null): NetworkModel = NetworkModel(
			key = "0191b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3",
			logo = networkName ?: "polkadot",
			title = networkName?.lowercase() ?: "Polkadot",
			pathId = "polkadot"
		)
	}
}

fun NetworkInfoModel.toNetworkModel(): NetworkModel = NetworkModel(
	key = networkSpecsKey,
	logo = networkLogo,
	title = networkTitle.replaceFirstChar {
		if (it.isLowerCase()) it.titlecase() else it.toString()
	},
	pathId = "//${networkTitle.toLowerCase()}",
)

fun MmNetwork.toNetworkModel(): NetworkModel = NetworkModel(
	key = key,
	logo = logo,
	title = title.replaceFirstChar {
		if (it.isLowerCase()) it.titlecase() else it.toString()
	},
	pathId = pathId,
)


data class VerifierDetailsModel(
	val publicKey: String,
	val identicon: Identicon,
	val encryption: String,
) {
	companion object {
		fun createStub() = VerifierDetailsModel(
			publicKey = "5DCmwXp8XLzSMUyE4uhJMKV4vwvsWqqBYFKJq38CW53VHEVq",
			identicon = PreviewData.Identicon.dotIcon,
			encryption = "sr25519",
		)
	}
}

fun MVerifierDetails.toVerifierDetailsModel() = VerifierDetailsModel(
	publicKey = publicKey,
	identicon = identicon,
	encryption = encryption,
)



