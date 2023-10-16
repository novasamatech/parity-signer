//
//  Event+EventTitle.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 02/12/2022.
//

import Foundation

extension Event {
    var eventTitle: String {
        switch self {
        case .databaseInitiated:
            Localizable.HistoryCard.database.string
        case .deviceWasOnline:
            Localizable.HistoryCard.deviceConnected.string
        case .generalVerifierSet:
            Localizable.HistoryCard.verifierSet.string
        case .historyCleared:
            Localizable.HistoryCard.historyCleared.string
        case .identitiesWiped:
            Localizable.HistoryCard.keysCleared.string
        case .identityAdded:
            Localizable.HistoryCard.keysCreated.string
        case .identityRemoved:
            Localizable.HistoryCard.keysRemoved.string
        case .secretWasExported:
            Localizable.HistoryCard.secretWasExported.string
        case .metadataAdded:
            Localizable.HistoryCard.metadataAdded.string
        case .metadataRemoved:
            Localizable.HistoryCard.metadataRemoved.string
        case .networkSpecsAdded:
            Localizable.HistoryCard.networkAdded.string
        case .networkSpecsRemoved:
            Localizable.HistoryCard.networkRemoved.string
        case .networkVerifierSet:
            Localizable.HistoryCard.networkVerifier.string
        case .resetDangerRecord:
            Localizable.HistoryCard.resetDanger.string
        case .seedCreated:
            Localizable.HistoryCard.seedCreated.string
        case .seedRemoved:
            Localizable.HistoryCard.seedRemoved.string
        case .seedNameWasShown:
            Localizable.HistoryCard.seedShown.string
        case .networkSpecsSigned:
            Localizable.HistoryCard.networkSpecsSigned.string
        case .metadataSigned:
            Localizable.HistoryCard.metadataSigned.string
        case .typesSigned:
            Localizable.HistoryCard.typesSigned.string
        case .systemEntry:
            Localizable.HistoryCard.systemRecord.string
        case .transactionSignError:
            Localizable.HistoryCard.signingFailure.string
        case .transactionSigned:
            Localizable.HistoryCard.transactionSigned.string
        case .typesAdded:
            Localizable.HistoryCard.typesAdded.string
        case .typesRemoved:
            Localizable.HistoryCard.typesRemoved.string
        case .userEntry:
            Localizable.HistoryCard.userRecord.string
        case let .warning(text):
            Localizable.HistoryCard.warning(text)
        case .wrongPassword:
            Localizable.HistoryCard.WrongPassword.title.string
        case .messageSignError:
            Localizable.HistoryCard.messageSignError.string
        case .messageSigned:
            Localizable.HistoryCard.messageSigned.string
        }
    }
}
