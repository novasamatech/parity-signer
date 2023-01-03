package io.parity.signer.screens.scan.transaction

import io.parity.signer.uniffi.Card
import io.parity.signer.uniffi.MTransaction
import io.parity.signer.uniffi.TransactionCard
import io.parity.signer.uniffi.TransactionType

val MTransaction.nonIssuesCardsFiltered: List<TransactionCard>
	get() =
		listOfNotNull(
			content.extensions,
			content.importingDerivations,
			content.message,
			content.meta,
			content.method,
			content.newSpecs,
			content.verifier,
			content.typesInfo,
		).flatten()


val MTransaction.issuesCardsFiltered: List<TransactionCard>
	get() =
		listOfNotNull(
			content.error,
			content.warning,
		).flatten()


fun MTransaction.isDisplayingErrorOnly(): Boolean =
	nonIssuesCardsFiltered.isEmpty() && issuesCardsFiltered.isNotEmpty()

fun MTransaction.transactionIssues(): String =
	issuesCardsFiltered
		.sortedBy { it.index }
		.mapNotNull {
			when (val card = it.card) {
				is Card.ErrorCard -> {
					card.f
				}
				is Card.WarningCard -> {
					card.f
				}
				else -> null
			}
		}
		.joinToString("\n")


val MTransaction.sortedValueCards: List<TransactionCard>
	get() =
		listOfNotNull(
			content.author,
			content.extensions,
			content.importingDerivations,
			content.message,
			content.meta,
			content.method,
			content.newSpecs,
			content.verifier,
			content.typesInfo,
		).flatten()
			.sortedBy { it.index }


sealed class TransactionPreviewType {
	data class AddNetwork(val network: String) : TransactionPreviewType()
	data class Metadata(val network: String, val version: String) :
		TransactionPreviewType()

	object Transfer : TransactionPreviewType()
	object Unknown : TransactionPreviewType()
}

val MTransaction.previewType: TransactionPreviewType
	get() = when (ttype) {
		TransactionType.STUB -> {
			sortedValueCards.firstNotNullOfOrNull {
				when (val card = it.card) {
					is Card.MetaCard -> {
						TransactionPreviewType.Metadata(
							network = card.f.specname,
							version = card.f.specsVersion
						)
					}
					is Card.NewSpecsCard -> {
						TransactionPreviewType.AddNetwork(network = card.f.name)
					}
					else -> null
				}
			} ?: TransactionPreviewType.Unknown
		}
		TransactionType.SIGN -> {
			TransactionPreviewType.Transfer
		}
		TransactionType.READ, TransactionType.IMPORT_DERIVATIONS, TransactionType.DONE -> {
			TransactionPreviewType.Unknown
		}
	}
