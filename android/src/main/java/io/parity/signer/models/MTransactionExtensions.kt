package io.parity.signer.models

import io.parity.signer.uniffi.Card
import io.parity.signer.uniffi.MTransaction
import io.parity.signer.uniffi.TransactionCard

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

