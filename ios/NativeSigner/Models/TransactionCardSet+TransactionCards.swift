//
//  TransactionCardSet+TransactionCards.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 14/11/2022.
//

import Foundation

extension TransactionCardSet {
    func asSortedCards() -> [TransactionCard] {
        [
            author,
            error,
            extensions,
            importingDerivations,
            message,
            meta,
            method,
            newSpecs,
            verifier,
            warning,
            typesInfo
        ]
        .compactMap { $0 }
        .flatMap { $0 }
        .sorted { $0.index < $1.index }
    }
}
