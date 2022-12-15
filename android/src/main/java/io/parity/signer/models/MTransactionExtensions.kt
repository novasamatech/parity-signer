package io.parity.signer.models

import io.parity.signer.uniffi.MTransaction
import io.parity.signer.uniffi.TransactionCard

val MTransaction.nonIssuesCardsFiltered: List<TransactionCard>
	get() {
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
	}

val MTransaction.issuesCardsFiltered: List<TransactionCard>
	get() {
		listOfNotNull(
			content.error,
			content.warning,
		).flatten()
	}

fun MTransaction.isDisplayingErrorOnly() : Boolean {
	nonIssuesCardsFiltered.isEmpty() && issuesCardsFiltered.isNotEmpty()
}


//todo dmitry ios functions may become needed
//    func transactionIssues() -> String {
//        [
//            content.error,
//            content.warning
//        ]
//        .compactMap { $0 }
//        .flatMap { $0 }
//        .sorted { $0.index < $1.index }
//        .compactMap {
//            if case let .errorCard(text) = $0.card {
//                return text
//            }
//            if case let .warningCard(text) = $0.card {
//                return text
//            }
//            return nil
//        }
//        .joined(separator: "\n")
//    }

//    func sortedValueCards() -> [TransactionCard] {
//        [
//            content.author,
//            content.extensions,
//            content.importingDerivations,
//            content.message,
//            content.meta,
//            content.method,
//            content.newSpecs,
//            content.verifier,
//            content.typesInfo
//        ]
//        .compactMap { $0 }
//        .flatMap { $0 }
//        .sorted { $0.index < $1.index }
//    }
