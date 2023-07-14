package io.parity.signer.domain

import io.parity.signer.components.ImageContent
import io.parity.signer.ui.helpers.PreviewData
import io.parity.signer.uniffi.*


object UnifiiStubs {
	/**
	 * sample of add netowork specs from Talisman
	 */
	fun makeTransactionStubList(): List<MTransaction> {
		return listOf<MTransaction>(
			MTransaction(
				content = TransactionCardSet(
					author = null,
					error = null,
					extensions = null,
					importingDerivations = null,
					message = null,
					meta = null,
					method = null,
					newSpecs = listOf(
						TransactionCard(
							1.toUInt(), indent = 0.toUInt(),
							card = Card.NewSpecsCard(
								f = NetworkSpecs(
									base58prefix = 5u.toUShort(),
									color = "#03b0fb",
									encryption = Encryption.SR25519,
									decimals = 18.toUByte(),
									genesisHash = listOf(
										158,
										183,
										108,
										81,
										132,
										196,
										171,
										134,
										121,
										210,
										213,
										216,
										25,
										253,
										249,
										11,
										156,
										0,
										20,
										3,
										233,
										225,
										125,
										162,
										225,
										75,
										109,
										138,
										236,
										64,
										41,
										198
									).map { it.toUByte() },
									logo = "",
									name = "astar",
									pathId = "//astar",
									secondaryColor = "#000000",
									title = "Astar",
									unit = "ASTR",
								)
							),
						),
					),
					verifier = listOf(
						TransactionCard(
							0.toUInt(), 0.toUInt(), Card.VerifierCard(
								f = MVerifierDetails(
									publicKey = "b04b58ffedd058a81a625819d437d7a35485c63cfac9fc9f0907c16b3e3e9d6c",
									identicon = SignerImage.Png((PreviewData.Identicon.exampleIdenticonPng as ImageContent.Png).image),
									encryption = Encryption.SR25519.toString(),
								)
							)
						)
					),
					warning = null,
					typesInfo = null,
				),
				ttype = TransactionType.STUB,
				authorInfo = null,
				networkInfo = null,
			)
		)
	}

	fun makeTransactionAddNetworksNovasamaStub(): List<MTransaction> {
		return listOf<MTransaction>(
			MTransaction(
				content = TransactionCardSet(
					author = null,
					error = null,
					extensions = null,
					importingDerivations = null,
					message = null,
					meta = null,
					method = null,
					newSpecs = listOf(
						TransactionCard(
							1.toUInt(), indent = 0.toUInt(),
							card = Card.NewSpecsCard(
								f = NetworkSpecs(
									base58prefix = 5u.toUShort(),
									color = "#660D35",
									encryption = Encryption.SR25519,
									decimals = 18.toUByte(),
									genesisHash = listOf(
										158,
										183,
										108,
										81,
										132,
										196,
										171,
										134,
										121,
										210,
										213,
										216,
										25,
										253,
										249,
										11,
										156,
										0,
										20,
										3,
										233,
										225,
										125,
										162,
										225,
										75,
										109,
										138,
										236,
										64,
										41,
										198
									).map { it.toUByte() },
									logo = "astar",
									name = "astar",
									pathId = "#262626",
									secondaryColor = "#262626",
									title = "astar-sr25519",
									unit = "ASTR",
								)
							),
						),
					),
					verifier = listOf(
						TransactionCard(
							0.toUInt(), 0.toUInt(), Card.VerifierCard(
								f = MVerifierDetails(
									publicKey = "16d5a6266345874d8f5b7f88a6619711b2829b52b2865826b1ecefb62beef34f",
									identicon = SignerImage.Png((PreviewData.Identicon.exampleIdenticonPng as ImageContent.Png).image),
									encryption = Encryption.SR25519.toString(),
								)
							)
						)
					),
					warning = null,
					typesInfo = null,
				),
				ttype = TransactionType.STUB,
				authorInfo = null,
				networkInfo = null,
			)
		)
	}
}
