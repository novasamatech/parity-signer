//
//  TransactionCardSet+TransactionCards.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 14/11/2022.
//

import Foundation

extension MTransaction {
    func sortedValueCards() -> [TransactionCard] {
        [
            content.author,
            content.error
            content.extensions,
            content.importingDerivations,
            content.message,
            content.meta,
            content.method,
            content.newSpecs,
            content.verifier,
            content.warning,
            content.typesInfo
        ]
        .compactMap { $0 }
        .flatMap { $0 }
        .sorted { $0.index < $1.index }
    }

    func transactionIssuesCards() -> [TransactionCard] {
        [
            content.error,
            content.warning
        ]
        .compactMap { $0 }
        .flatMap { $0 }
        .sorted { $0.index < $1.index }
    }
}
