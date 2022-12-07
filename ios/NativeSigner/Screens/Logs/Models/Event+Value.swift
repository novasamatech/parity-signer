//
//  Event+Value.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 02/12/2022.
//

import Foundation

extension Event {
    var displayValue: String? {
        switch self {
        case let .generalVerifierSet(value):
            return value.show()
        case let .identityAdded(value),
             let .identityRemoved(value),
             let .secretWasExported(value):
            return value.seedName + value.path
        case let .metadataAdded(value),
             let .metadataRemoved(value):
            return value.name + " version " + String(value.version)
        case let .networkSpecsAdded(value),
             let .networkSpecsRemoved(value):
            return value.network.specs.title
        case let .networkVerifierSet(value):
            return value.validCurrentVerifier == .general ?
                "general" :
                "custom" + " for network with genesis hash " + value.genesisHash.formattedAsString
        case let .seedCreated(text),
             let .seedRemoved(text),
             let .seedNameWasShown(text),
             let .systemEntry(text),
             let .userEntry(text):
            return text
        case let .networkSpecsSigned(value):
            return value.specsToSend.title
        case let .metadataSigned(value):
            return value.name + String(value.version)
        case let .transactionSignError(value),
             let .transactionSigned(value):
            return value.userComment
        case let .messageSignError(value),
             let .messageSigned(value):
            return value.userComment
        case .wrongPassword:
            return Localizable.HistoryCard.WrongPassword.subtitle.string
        default:
            return nil
        }
    }
}
