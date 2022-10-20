package io.parity.signer.models

import io.parity.signer.ui.helpers.PreviewData
import io.parity.signer.uniffi.*

/**
 * reflection of uniffi models so compose will work properly
 */


/**
 * Local copy of shared [MKeys] class
 */
data class KeySetDetailsViewModel(
	val set: List<KeysViewModel>,
	val root: SeedKeyViewModel,
	val network: NetworkViewModel,
){
	companion object {
		fun createStub() : KeySetDetailsViewModel = KeySetDetailsViewModel(
			set = listOf(
				KeysViewModel(
					addressKey = "address key",
						base58 = "5F3sa2TJAWMqDhXG6jhV4N8ko9SxwGy8TpaNS1repo5EYjQX",
						identicon = PreviewData.exampleIdenticon,
						hasPwd = true,
						path = "//polkadot//path2",
						multiselect = false,
						secretExposed = false,
				),
				KeysViewModel(
					addressKey = "address key2",
					base58 = "5F3sa2TJAWMqDhXG6jhV4N8ko9sdfsdfsdfS1repo5EYjGG",
					identicon = PreviewData.exampleIdenticon,
					hasPwd = true,
					path = "//polkadot//path3",
					multiselect = false,
					secretExposed = false,
				),
			),
				root = SeedKeyViewModel(
					seedName = "seed name",
						identicon = PreviewData.exampleIdenticon,
						addressKey = "address key",
						base58 = "5F3sa2TJAWMqDhXG6jhV4N8ko9SxwGy8TpaNS1repo5EYjQX",
						swiped = false,
						multiselect = false,
						secretExposed = false,
				),
				network = NetworkViewModel("network title", "network logo"),
		)
	}
}

fun MKeys.toKeySetDetailsViewModel() = KeySetDetailsViewModel(
	set = set.map { it.toKeysViewModel() },
	root = root.toSeedKeyViewModel(),
	network = network.toNetworkViewModel(),
)

/**
 * Local copy of shared [MKeysCard] class
 */
data class KeysViewModel(
	val identicon: List<UByte>,
	val addressKey: String,
	val base58: String,
	val hasPwd: Boolean,
	val path: String,
	val multiselect: Boolean,
	val secretExposed: Boolean
)
fun MKeysCard.toKeysViewModel() = KeysViewModel(
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
data class SeedKeyViewModel(
	val seedName: String,
	val identicon: List<UByte>,
	val addressKey: String,
	val base58: String,
	val swiped: Boolean,
	val multiselect: Boolean,
	val secretExposed: Boolean
)
fun MSeedKeyCard.toSeedKeyViewModel() = SeedKeyViewModel(
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
data class NetworkViewModel(
	val title: String,
	val logo: String,
)
fun MNetworkCard.toNetworkViewModel() = NetworkViewModel(
	title = title,
	logo = logo,
)

/**
 * Local copy of shared [MSeeds] class
 */
data class KeySetsSelectViewModel(val keys: List<KeySetViewModel>)

fun MSeeds.toKeySetsSelectViewModel() = KeySetsSelectViewModel(
	seedNameCards.map { it.toSeedViewModel() }
)

/**
 * Local copy of shared [SeedNameCard] class
 */
data class KeySetViewModel(
	val seedName: String,
	val identicon: List<UByte>,
	val derivedKeysCount: UInt
)

fun SeedNameCard.toSeedViewModel() =
	KeySetViewModel(seedName, identicon, derivedKeysCount)


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
