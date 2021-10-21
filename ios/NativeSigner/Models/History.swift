//
//  History.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 12.8.2021.
//

import Foundation

enum Event: Decodable, Hashable, Equatable {
    case databaseInitiated
    case deviceWasOnline
    case error(String)
    case generalVerifierSet(Verifier)
    case historyCleared
    case identitiesWiped
    case identityAdded(IdentityEvent)
    case identityRemoved(IdentityEvent)
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
    case transactionSignError(SignDisplayError)
    case transactionSigned(SignDisplay)
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
        case "add_network_message_signed":
            self = .signedAddNetwork(try values.decode(NetworkSigned.self, forKey: .payload))
        case "load_metadata_message_signed":
            self = .signedLoadMetadata(try values.decode(MetadataSigned.self, forKey: .payload))
        case "load_types_message_signed":
            self = .signedTypes(try values.decode(TypesSigned.self, forKey: .payload))
        case "system_entered_event":
            self = .systemEntry(try values.decode(String.self, forKey: .payload))
        case "sign_error":
            self = .transactionSignError(try values.decode(SignDisplayError.self, forKey: .payload))
        case "transaction_signed":
            self = .transactionSigned(try values.decode(SignDisplay.self, forKey: .payload))
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
            self = .error(try values.decode(String.self, forKey: .payload))
        }
    }
}

struct CurrentVerifier: Decodable, Hashable {
    var type: String
    var details: Verifier
}

struct IdentityEvent: Decodable, Hashable {
    var seed_name: String
    var encryption: String
    var public_key: String
    var path: String
    var network_genesis_hash: String
}

struct NetworkDisplay: Decodable, Hashable {
    var base58prefix: String
    var color: String
    var decimals: String
    var encryption: String
    var genesis_hash: String
    var logo: String
    var name: String
    var order: String
    var path_id: String
    var secondary_color: String
    var title: String
    var unit: String
    var current_verifier: CurrentVerifier
}

struct NetworkSigned: Decodable, Hashable {
    var base58prefix: String
    var color: String
    var decimals: String
    var encryption: String
    var genesis_hash: String
    var logo: String
    var name: String
    var path_id: String
    var secondary_color: String
    var title: String
    var unit: String
    var signed_by: Verifier
}

struct NetworkVerifierDisplay: Decodable, Hashable {
    var genesis_hash: String
    var current_verifier: Verifier
}

struct SignDisplay: Decodable, Hashable {
    var transaction: String
    var signed_by: Verifier
    var user_comment: String
}

struct SignDisplayError: Decodable, Hashable {
    var transaction: String
    var signed_by: Verifier
    var user_comment: String
    var error: String
}

struct TypesDisplay: Decodable, Hashable {
    var types_hash: String
    var verifier: Verifier
}

struct TypesSigned: Decodable, Hashable {
    var types_hash: String
    var signed_by: Verifier
}

struct MetadataSigned: Decodable, Hashable {
    var specname: String
    var spec_version: String
    var meta_hash: String
    var signed_by: Verifier
}

struct History: Decodable {
    var order: Int
    var timestamp: String
    var events: [Event]
}

extension SignerDataModel {
    func getHistory() {
        var err = ExternError()
        let err_ptr: UnsafeMutablePointer<ExternError> = UnsafeMutablePointer(&err)
        let res = print_history(err_ptr, self.dbName)
        if err_ptr.pointee.code == 0 {
            if let historyJSON = String(cString: res!).data(using: .utf8) {
                guard let history = try? JSONDecoder().decode([History].self, from: historyJSON) else {
                    print("JSON decoder failed on history")
                    print(String(cString: res!))
                    print(historyJSON)
                    signer_destroy_string(res!)
                    return
                }
                print(self.history)
                print(String(cString: res!))
                self.history = history.sorted(by: {$0.order > $1.order})
            } else {
                print("keysJSON corrupted")
            }
            signer_destroy_string(res!)
        } else {
            self.lastError = String(cString: err_ptr.pointee.message)
            print(self.lastError)
            signer_destroy_string(err_ptr.pointee.message)
        }
    }
}
