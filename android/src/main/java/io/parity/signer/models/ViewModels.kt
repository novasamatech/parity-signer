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
	val keys: List<KeysModel>,
	val root: SeedKeyModel,
	val network: NetworkModel,
) {
	companion object {
		fun createStub() : KeySetDetailsModel = KeySetDetailsModel(
			keys = listOf(
				KeysModel.createStub(),
				KeysModel(
					addressKey = "address key2",
					base58 = "5F3sa2TJAWMqDhXG6jhV4N8ko9sdfsdfsdfS1repo5EYjGG",
					identicon = PreviewData.exampleIdenticon,
					hasPwd = true,
					path = "//polkadot//path3",
					multiselect = false,
					secretExposed = false,
				),
			),
				root = SeedKeyModel(
					seedName = "seed name",
						identicon = PreviewData.exampleIdenticon,
						addressKey = "address key",
						base58 = "5F3sa2TJAWMqDhXG6jhV4N8ko9SxwGy8TpaNS1repo5EYjQX",
						swiped = false,
						multiselect = false,
						secretExposed = false,
				),
				network = NetworkModel("network title", "network logo"),
		)
	}
}

fun MKeys.toKeySetDetailsModel() = KeySetDetailsModel(
	keys = set.map { it.toKeysModel() },
	root = root.toSeedKeyModel(),
	network = network.toNetworkModel(),
)

/**
 * Local copy of shared [MKeysCard] class
 */
data class KeysModel(
	val identicon: List<UByte>,
	val addressKey: String,
	val base58: String,
	val hasPwd: Boolean,
	val path: String,
	val multiselect: Boolean,
	val secretExposed: Boolean
) {
	companion object {
		fun createStub() = 	KeysModel(
			addressKey = "address key",
			base58 = "5F3sa2TJAWMqDhXG6jhV4N8ko9SxwGy8TpaNS1repo5EYjQX",
			identicon = PreviewData.exampleIdenticon,
			hasPwd = true,
			path = "//polkadot//path2",
			multiselect = false,
			secretExposed = false,
		)
	}
}
fun MKeysCard.toKeysModel() = KeysModel(
	addressKey = addressKey,
	base58 = base58,
	identicon = identicon,
	hasPwd = hasPwd,
	path = path,
	multiselect = multiselect,
	secretExposed = secretExposed,
)

/**
 * Local copy of shared [MKeysCard] class
 */
data class SeedKeyModel(
	val seedName: String,
	val identicon: List<UByte>,
	val addressKey: String,
	val base58: String,
	val swiped: Boolean,
	val multiselect: Boolean,
	val secretExposed: Boolean
)
fun MSeedKeyCard.toSeedKeyModel() = SeedKeyModel(
	seedName = seedName,
	identicon = identicon,
	addressKey = addressKey,
	base58 = base58,
	swiped = swiped,
	multiselect = multiselect,
	secretExposed = secretExposed,
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
		/**
		 * @param networkTitle probably from keyDetails.networkInfo.networkTitle
		 */
		fun fromAddress(address: Address, networkTitle: String): KeyCardModel =
			KeyCardModel(
				network = networkTitle,
				base58 = address.base58,
				path = address.path,
				hasPwd = address.hasPwd,
				identIcon = address.identicon,
				seedName = address.seedName,
				multiselect = address.multiselect,
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
