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
            content.extensions,
            content.importingDerivations,
            content.message,
            content.meta,
            content.method,
            content.newSpecs,
            content.verifier,
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
    }

    func transactionIssues() -> String {
        [
            content.error,
            content.warning
        ]
        .compactMap { $0 }
        .flatMap { $0 }
        .sorted { $0.index < $1.index }
        .compactMap {
            if case let .errorCard(text) = $0.card {
                return text
            }
            if case let .warningCard(text) = $0.card {
                return text
            }
            return nil
        }
        .joined(separator: "\n")
    }

    var isDisplayingErrorOnly: Bool {
        [
            content.extensions,
            content.importingDerivations,
            content.message,
            content.meta,
            content.method,
            content.newSpecs,
            content.verifier,
            content.typesInfo
        ]
        .compactMap { $0 }
        .flatMap { $0 }
        .isEmpty && !transactionIssuesCards().isEmpty
    }
}

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

extension MTransaction {
    enum TransactionPreviewType {
        case addNetwork(network: String)
        case metadata(network: String, version: String)
        case transfer
        case importKeys(keysCount: Int)
        case unknown
    }

    var previewType: TransactionPreviewType {
        switch ttype {
        case .importDerivations:
            var keysCount: Int = 0
            sortedValueCards().forEach {
                if case let .derivationsCard(keys) = $0.card {
                    keysCount += keys.reduce(into: 0) { $0 += ($1.derivedKeys.count + 1) }
                }
            }
            return .importKeys(keysCount: keysCount)
        case .stub:
            return sortedValueCards().compactMap {
                switch $0.card {
                case let .metaCard(record):
                    return TransactionPreviewType.metadata(network: record.specname, version: record.specsVersion)
                case let .newSpecsCard(spec):
                    return TransactionPreviewType.addNetwork(network: spec.name)
                default:
                    return nil
                }
            }.first ?? .unknown
        case .sign:
            return .transfer
        default:
            return .unknown
        }
    }
}
