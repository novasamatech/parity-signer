package io.parity.signer.models

import io.parity.signer.components.ImageContent
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
	val network: NetworkInfoModel,
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
				secretExposed = false,
				seedName = "sdsdsd",
			),
			network = NetworkInfoModel(
				networkTitle = "network title",
				networkLogo = "network logo",
				networkSpecsKey = "01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"
			),
		)
	}
}

fun MKeys.toKeySetDetailsModel() = KeySetDetailsModel(
	keys = set.map { it.toKeysModel() },
	root = root.toKeysModel(),
	network = network.toNetworkModel(),
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
	val secretExposed: Boolean
) {
	companion object {
		fun createStub() = KeyModel(
			addressKey = "address key",
			base58 = "5F3sa2TJAWMqDhXG6jhV4N8ko9SxwGy8TpaNS1repo5EYjQX",
			identicon = PreviewData.exampleIdenticonPng,
			hasPwd = true,
			path = "//polkadot//path2",
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


data class KeyCardModel(
	val network: String,
	val base58: String,
	val path: String,
	val identIcon: ImageContent,
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
				multiselect = null,
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
				identIcon = address_card.address.identicon.toImageContent(),
				seedName = address_card.address.seedName,
				multiselect = null
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
				identIcon = address.identicon.toImageContent(),
				seedName = address.seedName,
				multiselect = false,
			)

		fun createStub() = KeyCardModel(
			network = "Polkadot",
			base58 = "5F3sa2TJAWMqDhXG6jhV4N8ko9SxwGy8TpaNS1repo5EYjQX",
			path = "//polkadot//path",
			identIcon = PreviewData.exampleIdenticonPng,
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
	val qrData: List<UByte>,
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
				qrData = PreviewData.exampleQRData,
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
				qrData = PreviewData.exampleQRData,
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

