package io.parity.signer.models

import io.parity.signer.components.ImageContent
import io.parity.signer.components.sharedcomponents.KeyCardModel
import io.parity.signer.components.toImageContent
import io.parity.signer.ui.helpers.PreviewData
import io.parity.signer.uniffi.*

/**
 * reflection of uniffi models so compose will work properly
 */


/**
 * Local copy of shared [MKeys] class
 */
data class KeySetDetailsModel(
	val keys: List<KeyModel>,
	val root: KeyModel,
	val network: NetworkModel,
	val multiselectMode: Boolean,
	val multiselectCount: String,
) {
	companion object {
		fun createStub(): KeySetDetailsModel = KeySetDetailsModel(
			keys = listOf(
				KeyModel.createStub(),
				KeyModel(
					addressKey = "address key2",
					base58 = "5F3sa2TJAWMqDhXG6jhV4N8ko9sdfsdfsdfS1repo5EYjGG",
					identicon = PreviewData.exampleIdenticonPng,
					hasPwd = true,
					path = "//polkadot//path3",
					multiselect = false,
					secretExposed = false,
					seedName = "sdsdsd",
				),
			),
			root = KeyModel(
				identicon = PreviewData.exampleIdenticonPng,
				addressKey = "address key",
				base58 = "5F3sa2TJAWMqDhXG6jhV4N8ko9SxwGy8TpaNS1repo5EYjQX",
				hasPwd = true,
				path = "//polkadot",
				multiselect = false,
				secretExposed = false,
				seedName = "sdsdsd",
			),
			network = NetworkModel("network title", "network logo"),
			multiselectCount = "5",
			multiselectMode = false,
		)
	}
}

fun MKeys.toKeySetDetailsModel() = KeySetDetailsModel(
	keys = set.map { it.toKeysModel() },
	root = root.toKeysModel(),
	network = network.toNetworkModel(),
	multiselectMode = multiselectMode,
	multiselectCount = multiselectCount,
)

/**
 * Local copy of shared [MKeysCard] class
 */
data class KeyModel(
	val identicon: ImageContent,
	val addressKey: String,
	val seedName: String,
	val base58: String,
	val hasPwd: Boolean,
	val path: String,
	val multiselect: Boolean,
	val secretExposed: Boolean
) {
	companion object {
		fun createStub() = KeyModel(
			addressKey = "address key",
			base58 = "5F3sa2TJAWMqDhXG6jhV4N8ko9SxwGy8TpaNS1repo5EYjQX",
			identicon = PreviewData.exampleIdenticonPng,
			hasPwd = true,
			path = "//polkadot//path2",
			multiselect = false,
			secretExposed = false,
			seedName = "sdsdsd",
		)
	}
}

fun MKeysCard.toKeysModel() = KeyModel(
	addressKey = addressKey,
	base58 = base58,
	identicon = address.identicon.toImageContent(),
	hasPwd = address.hasPwd,
	path = address.path,
	multiselect = multiselect,
	secretExposed = address.secretExposed,
	seedName = address.seedName,
)

/**
 * Local copy of shared [MNetworkCard] class
 */
data class NetworkModel(
	val title: String,
	val logo: String,
)

fun MNetworkCard.toNetworkModel() = NetworkModel(
	title = title,
	logo = logo,
)

/**
 * Local copy of shared [MSeeds] class
 */
data class KeySetsSelectModel(val keys: List<KeySetModel>)

fun MSeeds.toKeySetsSelectModel() = KeySetsSelectModel(
	seedNameCards.map { it.toSeedModel() }
)

/**
 * Local copy of shared [SeedNameCard] class
 */
data class KeySetModel(
	val seedName: String,
	val identicon: ImageContent,
	val derivedKeysCount: UInt
)

fun SeedNameCard.toSeedModel() =
	KeySetModel(seedName, identicon.toImageContent(), derivedKeysCount)

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
		qrData = qr.getData(), pubkey = pubkey, networkInfo = networkInfo.toNetworkInfoModel(),
		address = KeyCardModel.fromAddress(
			address = address,
			base58 = base58,
			networkTitle = networkInfo.networkTitle,
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
)

fun MscNetworkInfo.toNetworkInfoModel() =
	NetworkInfoModel(networkTitle, networkLogo, networkSpecsKey)

fun QrData.getData(): List<UByte> =
	when (this) {
		is QrData.Regular -> this.data
		is QrData.Sensitive -> this.data
	}

