//
//  Event+EntryType.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 07/12/2022.
//

import Foundation

extension Event {
    var entryType: LogEntryRenderable.EntryType {
        switch self {
        case .databaseInitiated,
             .deviceWasOnline,
             .generalVerifierSet,
             .historyCleared,
             .identitiesWiped,
             .metadataAdded,
             .metadataRemoved,
             .networkSpecsAdded,
             .networkSpecsRemoved,
             .resetDangerRecord,
             .seedCreated,
             .seedRemoved,
             .seedNameWasShown,
             .networkSpecsSigned,
             .systemEntry,
             .typesAdded,
             .typesRemoved,
             .userEntry,
             .warning,
             .wrongPassword,
             .messageSignError,
             .messageSigned:
            return .basic
        case .identityAdded,
             .identityRemoved,
             .secretWasExported,
             .networkVerifierSet,
             .metadataSigned,
             .typesSigned:
            return .bottomDetails
        case .transactionSignError:
            return .fullDetails
        case .transactionSigned:
            return .fullDetails
        }
    }
}
