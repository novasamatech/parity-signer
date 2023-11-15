//
//  Event+Value.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 02/12/2022.
//

import Foundation

extension Event {
    var displayValue: String? {
        switch self {
        case let .generalVerifierSet(value):
            value.show()
        case let .identityAdded(value),
             let .identityRemoved(value),
             let .secretWasExported(value):
            value.seedName + value.path
        case let .metadataAdded(value),
             let .metadataRemoved(value):
            value.name + " version " + String(value.version)
        case let .networkSpecsAdded(value),
             let .networkSpecsRemoved(value):
            value.network.specs.title
        case let .networkVerifierSet(value):
            value.validCurrentVerifier == .general ?
                "general" :
                "custom" + " for network with genesis hash " + value.genesisHash.formattedAsString
        case let .seedCreated(text),
             let .seedRemoved(text),
             let .seedNameWasShown(text),
             let .systemEntry(text),
             let .userEntry(text):
            text
        case let .networkSpecsSigned(value):
            value.specsToSend.title
        case let .metadataSigned(value):
            value.name + String(value.version)
        case let .transactionSignError(value),
             let .transactionSigned(value):
            value.userComment
        case let .messageSignError(value),
             let .messageSigned(value):
            value.userComment
        case .wrongPassword:
            Localizable.HistoryCard.WrongPassword.subtitle.string
        default:
            nil
        }
    }
}
