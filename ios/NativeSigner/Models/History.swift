//
//  History.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 12.8.2021.
//

import Foundation

enum Event: Decodable, Hashable, Equatable {
    
    case identityAdded(IdentityAddedEvent)
    case plain(String)
    
    enum CodingKeys: String, CodingKey {
        case event
        case payload
    }
    
    init(from decoder: Decoder) throws {
        let values = try decoder.container(keyedBy: CodingKeys.self)
        let type = try values.decode(String.self, forKey: .event)
        
        switch type {
        case "identity_added":
            self = .identityAdded(try values.decode(IdentityAddedEvent.self, forKey: .payload))
        default:
            self = .plain(try values.decode(String.self, forKey: .payload))
        }
    }
}

struct IdentityAddedEvent: Decodable, Hashable {
    var seed_name: String
    var public_key: String
    var path: String
    var network_key: String
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
