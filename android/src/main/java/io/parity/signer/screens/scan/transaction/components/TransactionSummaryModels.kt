package io.parity.signer.screens.scan.transaction.components

import io.parity.signer.components.ImageContent
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
	val signature: TransactionSignatureRenderable?,
) {
	companion object {
		fun createStub(): SigningTransactionModel =
			SigningTransactionModel(
				summaryModels = listOf(
					TransactionSummaryModel(
						pallet = "Balances",
						method = "transfer_keep_alive",
						destination = "1219xC79CXV31543DDXoQMjuA",
						value = "0.2 WND"
					),
					TransactionSummaryModel(
						pallet = "Balances2",
						method = "transfer_keep_alive2",
						destination = "1219xC79CXV31543DDXoQMjuA2",
						value = "0.3 WND"
					)
				),
				signature = TransactionSignatureRenderable(
					path = "//polkadot//1",
					name = "Parity Keys",
					base58 = "1219xC79CXV31543DDXoQMjuA",
					identicon = PreviewData.exampleIdenticonPng,
					hasPassword = true
				)
			)
	}
}

fun List<MTransaction>.toSigningTransactionModels(): List<SigningTransactionModel> {
	val fullModelsList: List<SigningTransactionModel> =
		map { it.toSigningTransactionModel() }
	val signatures = fullModelsList.map { it.signature }.toSet()
	return signatures.map { signature ->
		SigningTransactionModel(signature = signature,
			summaryModels = fullModelsList
				.filter { it.signature == signature }
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
			)
		),
		signature = authorInfo?.let { author ->
			TransactionSignatureRenderable(
				path = author.address.toDisplayablePathString(),
				name = author.address.seedName,
				base58 = author.base58,
				identicon = author.address.identicon.toImageContent(),//.svgPayload, on iOS
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
)

//fun TransactionSummaryModel.toDetailsRows(): kotlin.collections.List<TransactionDetailsRow> {
//	return listOf(TransactionDetailsRow("Key"), )
//}


data class TransactionSignatureRenderable(
	val path: String,
	val name: String,
	val base58: String,
	val identicon: ImageContent,
	val hasPassword: Boolean,
)

data class TransactionDetailsRow(
	val key: String,
	val value: String,
)


//    var asRenderable: [TransactionDetailsRow] {
//        let labelKey = Localizable.TransactionSign.Label.Details.self
//        return [.init(key: labelKey.pallet.string, value: pallet),
//                .init(key: labelKey.method.string, value: method),
//                .init(key: labelKey.destination.string, value: destination),
//                .init(key: labelKey.value.string, value: value)]
//    }
//}

// todo scan
//ios/NativeSigner/Screens/Scan/Models/TransactionSummaryModels.swift:59


