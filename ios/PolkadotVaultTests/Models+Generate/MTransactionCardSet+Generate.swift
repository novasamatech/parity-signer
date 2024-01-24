//
//  MTransactionCardSet+Generate.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 22/01/2024.
//

import Foundation
@testable import PolkadotVault

extension TransactionCardSet {
    static func generate(
        author: [TransactionCard]? = [TransactionCard.generate()],
        error: [TransactionCard]? = [TransactionCard.generate()],
        extensions: [TransactionCard]? = [TransactionCard.generate()],
        importingDerivations: [TransactionCard]? = [TransactionCard.generate()],
        message: [TransactionCard]? = [TransactionCard.generate()],
        meta: [TransactionCard]? = [TransactionCard.generate()],
        method: [TransactionCard]? = [TransactionCard.generate()],
        newSpecs: [TransactionCard]? = [TransactionCard.generate()],
        verifier: [TransactionCard]? = [TransactionCard.generate()],
        warning: [TransactionCard]? = [TransactionCard.generate()],
        typesInfo: [TransactionCard]? = [TransactionCard.generate()]
    ) -> TransactionCardSet {
        TransactionCardSet(
            author: author,
            error: error,
            extensions: extensions,
            importingDerivations: importingDerivations,
            message: message,
            meta: meta,
            method: method,
            newSpecs: newSpecs,
            verifier: verifier,
            warning: warning,
            typesInfo: typesInfo
        )
    }
}

extension TransactionCard {
    static func generate(
        index: UInt32 = 0,
        indent: UInt32 = 0,
        card: Card = Card.generate()
    ) -> TransactionCard {
        TransactionCard(
            index: index,
            indent: indent,
            card: card
        )
    }
}

extension Card {
    static func generate() -> Card {
        .eraImmortalCard
    }
}
