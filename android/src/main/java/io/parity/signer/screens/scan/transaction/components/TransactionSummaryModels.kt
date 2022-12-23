package io.parity.signer.screens.scan.transaction.components

import io.parity.signer.components.ImageContent
import io.parity.signer.components.sharedcomponents.KeyCardModelBase
import io.parity.signer.components.toImageContent
import io.parity.signer.models.BASE58_STYLE_ABBREVIATE
import io.parity.signer.models.abbreviateString
import io.parity.signer.ui.helpers.PreviewData
import io.parity.signer.uniffi.Address
import io.parity.signer.uniffi.Card
import io.parity.signer.uniffi.MTransaction

/**
 * Local version of [Mtransaction] for the case of signing elements
 */

data class SigningTransactionModel(
	val summaryModels: List<TransactionSummaryModel>,
	val keyModel: KeyCardModelBase?,
) {
	companion object {
		fun createStub(): SigningTransactionModel =
			SigningTransactionModel(
				summaryModels = listOf(
					TransactionSummaryModel(
						pallet = "Balances",
						method = "transfer_keep_alive",
						destination = "1219xC79CXV31543DDXoQMjuA",
						value = "0.2 WND",
						mTransactionIndex = 0,
					),
					TransactionSummaryModel(
						pallet = "Balances2",
						method = "transfer_keep_alive2",
						destination = "1219xC79CXV31543DDXoQMjuA2",
						value = "0.3 WND",
						mTransactionIndex = 1,
					)
				),
				keyModel = KeyCardModelBase(
					path = "//polkadot//1",
					seedName = "Parity Keys",
					base58 = "1219xC79CXV31543DDXoQMjuA",
					identIcon = PreviewData.exampleIdenticonPng,
					hasPassword = true
				)
			)
	}
}

fun List<MTransaction>.toSigningTransactionModels(): List<SigningTransactionModel> {
	val fullModelsList: List<SigningTransactionModel> =
		map { it.toSigningTransactionModel() }
	val signatures = fullModelsList.map { it.keyModel }.toSet()
	return signatures.map { signature ->
		SigningTransactionModel(keyModel = signature,
			summaryModels = fullModelsList
				.filter { it.keyModel == signature }
				.map { it.summaryModels }
				.flatten())
	}
}

private fun MTransaction.toSigningTransactionModel(): SigningTransactionModel {
	var pallet: String = ""
	var method: String = ""
	var destination: String = ""
	var value: String = ""

	val methodCards = content.method?.map { it.card } ?: emptyList()
	for (methodCard in methodCards) {
		when (methodCard) {
			is Card.PalletCard -> {
				pallet = methodCard.f
			}
			is Card.CallCard -> {
				method = methodCard.f.methodName
			}
			is Card.IdCard -> {
				destination = methodCard.f.base58.abbreviateString(
					BASE58_STYLE_ABBREVIATE
				)
			}
			is Card.BalanceCard -> {
				value = "${methodCard.f.amount} ${methodCard.f.units}"
			}
			else -> {
				//ignore the rest of the cards
			}
		}
	}
	return SigningTransactionModel(
		summaryModels = listOf(
			TransactionSummaryModel(
				pallet = pallet,
				method = method,
				destination = destination,
				value = value,
				mTransactionIndex = 0 //todo scan pass index
			)
		),
		keyModel = authorInfo?.let { author ->
			KeyCardModelBase(
				path = author.address.toDisplayablePathString(),
				seedName = author.address.seedName,
				base58 = author.base58,
				identIcon = author.address.identicon.toImageContent(),//.svgPayload, on iOS
				hasPassword = author.address.hasPwd
			)
		}
	)
}

fun Address.toDisplayablePathString(): String {
	return if (hasPwd) "$path •••• " else path
}

data class TransactionSummaryModel(
	val pallet: String,
	val method: String,
	val destination: String,
	val value: String,
	val mTransactionIndex: Int
)



