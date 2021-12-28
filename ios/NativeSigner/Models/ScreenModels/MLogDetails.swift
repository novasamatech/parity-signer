//
//  MLogDetails.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 27.12.2021.
//

import Foundation

struct MLogDetails: Decodable {
    var timestamp: String
    var events: [EventDetailed]
}

enum EventDetailed: Decodable, Hashable {
    case databaseInitiated
    case deviceWasOnline
    case generalVerifierSet(Verifier)
    case historyCleared
    case identitiesWiped
    case identityAdded(IdentityEvent)
    case identityRemoved(IdentityEvent)
    case messageSignError(SignMessageError)
    case messageSigned(SignMessage)
    case metadataAdded(MetaSpecs)
    case metadataRemoved(MetaSpecs)
    case networkAdded(NetworkDisplay)
    case networkRemoved(NetworkDisplay)
    case networkVerifierSet(NetworkVerifierDisplay)
    case resetDangerRecord
    case seedNameWasShown(String)
    case signedAddNetwork(NetworkSigned)
    case signedLoadMetadata(MetadataSigned)
    case signedTypes(TypesSigned)
    case systemEntry(String)
    case transactionSignError(HDTransactionFailed)
    case transactionSigned(HDTransactionSigned)
    case typesAdded(TypesDisplay)
    case typesRemoved(TypesDisplay)
    case userEntry(String)
    case warning(String)
    case wrongPassword
    
    enum CodingKeys: String, CodingKey {
        case event
        case payload
    }
    
    init(from decoder: Decoder) throws {
        let values = try decoder.container(keyedBy: CodingKeys.self)
        let type = try values.decode(String.self, forKey: .event)
        
        switch type {
        case "database_initiated":
            self = .databaseInitiated
        case "device_online":
            self = .deviceWasOnline
        case "general_verifier_added":
            self = .generalVerifierSet(try values.decode(Verifier.self, forKey: .payload))
        case "history_cleared":
            self = .historyCleared
        case "identities_wiped":
            self = .identitiesWiped
        case "identity_added":
            self = .identityAdded(try values.decode(IdentityEvent.self, forKey: .payload))
        case "identity_removed":
            self = .identityRemoved(try values.decode(IdentityEvent.self, forKey: .payload))
        case "message_sign_error":
            self = .messageSignError(try values.decode(SignMessageError.self, forKey: .payload))
        case "message_signed":
            self = .messageSigned(try values.decode(SignMessage.self, forKey: .payload))
        case "metadata_added":
            self = .metadataAdded(try values.decode(MetaSpecs.self, forKey: .payload))
        case "metadata_removed":
            self = .metadataRemoved(try values.decode(MetaSpecs.self, forKey: .payload))
        case "network_added":
            self = .networkAdded(try values.decode(NetworkDisplay.self, forKey: .payload))
        case "network_removed":
            self = .networkRemoved(try values.decode(NetworkDisplay.self, forKey: .payload))
        case "network_verifier_set":
            self = .networkVerifierSet(try values.decode(NetworkVerifierDisplay.self, forKey: .payload))
        case "reset_danger_record":
            self = .resetDangerRecord
        case "seed_name_shown":
            self = .seedNameWasShown(try values.decode(String.self, forKey: .payload))
        case "add_specs_message_signed":
            self = .signedAddNetwork(try values.decode(NetworkSigned.self, forKey: .payload))
        case "load_metadata_message_signed":
            self = .signedLoadMetadata(try values.decode(MetadataSigned.self, forKey: .payload))
        case "load_types_message_signed":
            self = .signedTypes(try values.decode(TypesSigned.self, forKey: .payload))
        case "system_entered_event":
            self = .systemEntry(try values.decode(String.self, forKey: .payload))
        case "transaction_sign_error":
            self = .transactionSignError(try values.decode(HDTransactionFailed.self, forKey: .payload))
        case "transaction_signed":
            self = .transactionSigned(try values.decode(HDTransactionSigned.self, forKey: .payload))
        case "types_info_updated":
            self = .typesAdded(try values.decode(TypesDisplay.self, forKey: .payload))
        case "types_removed":
            self = .typesRemoved(try values.decode(TypesDisplay.self, forKey: .payload))
        case "user_entered_event":
            self = .userEntry(try values.decode(String.self, forKey: .payload))
        case "warning":
            self = .warning(try values.decode(String.self, forKey: .payload))
        case "wrong_password_entered":
            self = .wrongPassword
        default:
            self = .warning("Record corrupted")
        }
    }
}

struct HDTransactionSigned: Decodable, Hashable {
    var transaction: TransactionCardSet
    var signed_by: HDSignedBy
    var network_name: String
    var user_comment: String
}

struct HDSignedBy: Decodable, Hashable {
    var hex: String
    var identicon: String
    var encryption: String
}

struct HDTransactionFailed: Decodable, Hashable {
    var transaction: TransactionCardSet
    var network_name: String
    var signed_by: HDSignedBy
    var user_comment: String
    var error: String
}
