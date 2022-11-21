//
//  MTransaction+Summary.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 20/11/2022.
//

import Foundation
import SwiftUI

extension TransactionPreviewRenderable {
    init(_ transaction: MTransaction) {
        var pallet: String = ""
        var method: String = ""
        var destination: String = ""
        var value: String = ""
        let methodCards = transaction.content.method?.map(\.card) ?? []

        for methodCard in methodCards {
            if case let .palletCard(value) = methodCard {
                pallet = value
            }
            if case let .callCard(value) = methodCard {
                method = value.methodName
            }
            if case let .idCard(value) = methodCard {
                destination = value.base58.truncateMiddle()
            }
            if case let .balanceCard(cardValue) = methodCard {
                value = [cardValue.amount, cardValue.units].joined(separator: " ")
            }
        }
        summary = .init(
            pallet: pallet,
            method: method,
            destination: destination,
            value: value
        )
        signature = .init(
            path: transaction.authorInfo?.address.path ?? "",
            name: transaction.authorInfo?.address.seedName ?? "",
            base58: transaction.authorInfo?.base58 ?? "",
            identicon: transaction.authorInfo?.address.identicon ?? []
        )
    }
}

struct TransactionDetailsRow: Equatable, Identifiable {
    let id = UUID()
    let key: String
    let value: String
}

struct TransactionPreviewRenderable: Equatable {
    let summary: TransactionSummaryModel
    let signature: TransactionSignatureRenderable
}

struct TransactionSummaryModel: Equatable {
    let pallet: String
    let method: String
    let destination: String
    let value: String

    init(_: MTransaction) {
        pallet = ""
        method = ""
        destination = ""
        value = ""
    }

    init(
        pallet: String = "",
        method: String = "",
        destination: String = "",
        value: String = ""
    ) {
        self.pallet = pallet
        self.method = method
        self.destination = destination
        self.value = value
    }

    var asRenderable: [TransactionDetailsRow] {
        let labelKey = Localizable.TransactionSign.Label.Details.self
        return [.init(key: labelKey.pallet.string, value: pallet),
                .init(key: labelKey.method.string, value: method),
                .init(key: labelKey.destination.string, value: destination),
                .init(key: labelKey.value.string, value: value)]
    }
}

struct TransactionSignatureRenderable: Equatable {
    let path: String
    let name: String
    let base58: String
    let identicon: [UInt8]

    init(_: MTransaction) {
        path = ""
        name = ""
        base58 = ""
        identicon = PreviewData.exampleIdenticon
    }

    init(
        path: String = "",
        name: String = "",
        base58: String = "",
        identicon: [UInt8] = []
    ) {
        self.path = path
        self.name = name
        self.base58 = base58
        self.identicon = identicon
    }
}
