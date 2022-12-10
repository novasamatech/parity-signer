//
//  Event+Title.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 02/12/2022.
//

import Foundation

extension Event {
    var eventTitle: String {
        switch self {
        case .databaseInitiated:
            return Localizable.HistoryCard.database.string
        case .deviceWasOnline:
            return Localizable.HistoryCard.deviceConnected.string
        case .generalVerifierSet:
            return Localizable.HistoryCard.verifierSet.string
        case .historyCleared:
            return Localizable.HistoryCard.historyCleared.string
        case .identitiesWiped:
            return Localizable.HistoryCard.keysCleared.string
        case .identityAdded:
            return Localizable.HistoryCard.keysCreated.string
        case .identityRemoved:
            return Localizable.HistoryCard.keysRemoved.string
        case .secretWasExported:
            return Localizable.HistoryCard.secretWasExported.string
        case .metadataAdded:
            return Localizable.HistoryCard.metadataAdded.string
        case .metadataRemoved:
            return Localizable.HistoryCard.metadataRemoved.string
        case .networkSpecsAdded:
            return Localizable.HistoryCard.networkAdded.string
        case .networkSpecsRemoved:
            return Localizable.HistoryCard.networkRemoved.string
        case .networkVerifierSet:
            return Localizable.HistoryCard.networkVerifier.string
        case .resetDangerRecord:
            return Localizable.HistoryCard.resetDanger.string
        case .seedCreated:
            return Localizable.HistoryCard.seedCreated.string
        case .seedRemoved:
            return Localizable.HistoryCard.seedRemoved.string
        case .seedNameWasShown:
            return Localizable.HistoryCard.seedShown.string
        case .networkSpecsSigned:
            return Localizable.HistoryCard.networkSpecsSigned.string
        case .metadataSigned:
            return Localizable.HistoryCard.metadataSigned.string
        case .typesSigned:
            return Localizable.HistoryCard.typesSigned.string
        case .systemEntry:
            return Localizable.HistoryCard.systemRecord.string
        case .transactionSignError:
            return Localizable.HistoryCard.signingFailure.string
        case .transactionSigned:
            return Localizable.HistoryCard.transactionSigned.string
        case .typesAdded:
            return Localizable.HistoryCard.typesAdded.string
        case .typesRemoved:
            return Localizable.HistoryCard.typesRemoved.string
        case .userEntry:
            return Localizable.HistoryCard.userRecord.string
        case let .warning(text):
            return Localizable.HistoryCard.warning(text)
        case .wrongPassword:
            return Localizable.HistoryCard.WrongPassword.title.string
        case .messageSignError:
            return Localizable.HistoryCard.messageSignError.string
        case .messageSigned:
            return Localizable.HistoryCard.messageSigned.string
        }
    }
}
