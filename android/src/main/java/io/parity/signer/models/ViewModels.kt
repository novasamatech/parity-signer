package io.parity.signer.models

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
					identicon = PreviewData.exampleIdenticon,
					hasPwd = true,
					path = "//polkadot//path3",
					multiselect = false,
					secretExposed = false,
					seedName = "sdsdsd",
				),
			),
			root = KeyModel(
				identicon = PreviewData.exampleIdenticon,
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
	val identicon: List<UByte>,
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
			identicon = PreviewData.exampleIdenticon,
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
	identicon = address.identicon,
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
	val identicon: List<UByte>,
	val derivedKeysCount: UInt
)

fun SeedNameCard.toSeedModel() =
	KeySetModel(seedName, identicon, derivedKeysCount)


data class KeyCardModel(
	val network: String,
	val base58: String,
	val path: String,
	val identIcon: List<UByte>,
	val seedName: String,
	val hasPwd: Boolean = false,
	val multiselect: Boolean? = null,
) {
	companion object {

		fun fromKeyModel(model: KeyModel, networkTitle: String): KeyCardModel =
			KeyCardModel(
				network = networkTitle,
				base58 = model.base58,
				path = model.path,
				identIcon = model.identicon,
				seedName = model.seedName,
				hasPwd = model.hasPwd,
				multiselect = model.multiselect,
			)

		/**
		 * @param networkTitle probably from keyDetails.networkInfo.networkTitle
		 */
		fun fromAddress(
			address_card: MAddressCard,
			networkTitle: String
		): KeyCardModel =
			KeyCardModel(
				network = networkTitle,
				base58 = address_card.base58,
				path = address_card.address.path,
				hasPwd = address_card.address.hasPwd,
				identIcon = address_card.address.identicon,
				seedName = address_card.address.seedName,
				multiselect = address_card.multiselect,
			)

		fun fromAddress(
			address: Address,
			base58: String,
			networkTitle: String
		): KeyCardModel =
			KeyCardModel(
				network = networkTitle,
				base58 = base58,
				path = address.path,
				hasPwd = address.hasPwd,
				identIcon = address.identicon,
				seedName = address.seedName,
				multiselect = false,
			)

		fun createStub() = KeyCardModel(
			network = "Polkadot",
			base58 = "5F3sa2TJAWMqDhXG6jhV4N8ko9SxwGy8TpaNS1repo5EYjQX",
			path = "//polkadot//path",
			identIcon = PreviewData.exampleIdenticon,
			seedName = "Seed Name",
			hasPwd = false,
			multiselect = null,
		)
	}
}

/**
 * Local copy of shared [MKeyDetails] class
 */
data class KeyDetailsModel(
	val qr: List<UByte>,
	val pubkey: String,
	val networkInfo: NetworkInfoModel,
	val address: KeyCardModel,
	val base58: String,
	val secretExposed: Boolean,
) {
	val isRootKey = address.path.isEmpty()

	companion object {
		fun createStubDerived(): KeyDetailsModel {
			val keyCard = KeyCardModel.createStub()
			return KeyDetailsModel(
				qr = PreviewData.exampleQRCode,
				pubkey = "public key",
				networkInfo = NetworkInfoModel(
					"network title",
					"network logo", "network specs"
				),
				address = keyCard,
				base58 = keyCard.base58,
				secretExposed = true,
			)
		}

		fun createStubRoot(): KeyDetailsModel {
			val keyCard = KeyCardModel.createStub()
			return KeyDetailsModel(
				qr = PreviewData.exampleQRCode,
				pubkey = "public key",
				networkInfo = NetworkInfoModel(
					"network title",
					"network logo", "network specs"
				),
				address = KeyCardModel(
					keyCard.network, keyCard.base58, "",
					keyCard.identIcon, keyCard.seedName, false
				),
				base58 = keyCard.base58,
				secretExposed = true,
			)
		}
	}
}

fun MKeyDetails.toKeyDetailsModel() =
	KeyDetailsModel(
		qr = qr, pubkey = pubkey, networkInfo = networkInfo.toNetworkInfoModel(),
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

