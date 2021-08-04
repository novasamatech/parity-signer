//
//  Identities.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 3.8.2021.
//

import Foundation

struct Identity: Codable, Equatable {
    var public_key: String
    var ss58: String
    var path: String
    var has_password: String
    var name: String
}

class IdentityProposal: ObservableObject {
    @Published var name: String
    @Published var path: String
    @Published var password: String
    
    init(data: SignerDataModel) {
        name = "name"
        path = data.selectedIdentity?.path ?? "" + "//"
        password = ""
    }
}

extension Identity {
    static var identityData: [Identity] {
        [
            Identity(public_key: "1691a3ce28763a83e094bd5b06835bc5bba4d38d770710f8f327d4f2c71afb21", ss58: "1WbKRCtpZ6XbTdf9w8i7KVwctANhQhQg27qfE54RbamvfrD", path: "", has_password: "false", name: "root address"),
            Identity(public_key: "1791a3ce28763a83e094bd5b06835bc5bba4d38d770710f8f327d4f2c71afb21", ss58: "11bKRCtpZ6XbTdf9w8i7KVwctANhQhQg27qfE54RbamvfrD", path: "", has_password: "true", name: "Some other address")
        ]
    }
}

//MARK: identity management

extension SignerDataModel {
    func fetchKeys() {
        print("fetch keys")
        let err_ptr: UnsafeMutablePointer<ExternError> = UnsafeMutablePointer(&err)
        let dbName = NSHomeDirectory() + "/Documents/Database"
        let res = get_relevant_identities(err_ptr, self.selectedSeed, self.selectedNetwork!.key, dbName)
        if err_ptr.pointee.code == 0 {
            if let keysJSON = String(cString: res!).data(using: .utf8) {
                guard let keys = try? JSONDecoder().decode([Identity].self, from: keysJSON) else {
                    print("JSON decoder failed on keys")
                    print(String(cString: res!))
                    print(keysJSON)
                    signer_destroy_string(res!)
                    return
                }
                self.identities = keys.sorted(by: {$0.path < $1.path})
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
    
    func deleteActiveIdentity() {
        let err_ptr: UnsafeMutablePointer<ExternError> = UnsafeMutablePointer(&err)
        let dbName = NSHomeDirectory() + "/Documents/Database"
        delete_identity(err_ptr, self.selectedIdentity?.public_key, self.selectedNetwork!.key, dbName)
        if err_ptr.pointee.code == 0 {
            self.selectedIdentity = nil
            self.fetchKeys()
        } else {
            self.lastError = String(cString: err_ptr.pointee.message)
            print(self.lastError)
            signer_destroy_string(err_ptr.pointee.message)
        }
    }
    
    func proposeDerive() {
        if self.selectedIdentity == nil {
            self.suggestedPath = "//"
        } else {
            self.suggestedPath = self.selectedIdentity!.path
        }
        //self.suggestName = String(cString: suggest_name(nil, path))
    }
    
    func proposeIncrement() {
        let err_ptr: UnsafeMutablePointer<ExternError> = UnsafeMutablePointer(&err)
        let dbName = NSHomeDirectory() + "/Documents/Database"
        if self.selectedIdentity == nil {
            self.suggestedPath = "//"
        } else {
            let res = suggest_n_plus_one(err_ptr, self.selectedIdentity!.path, self.selectedSeed, self.selectedNetwork!.key, dbName)
            if err_ptr.pointee.code == 0 {
                self.suggestedPath = String(cString: res!)
            } else {
                self.lastError = String(cString: err_ptr.pointee.message)
                print(self.lastError)
                signer_destroy_string(err_ptr.pointee.message)
            }
        }
    }
    
    func createIdentity(password: String) {
        let err_ptr: UnsafeMutablePointer<ExternError> = UnsafeMutablePointer(&err)
        let dbName = NSHomeDirectory() + "/Documents/Database"
        var fullPath = ""
        if password == "" {
            fullPath = self.suggestedPath
        } else {
            fullPath = self.suggestedPath + "///" + password
        }
        let res = check_path(err_ptr, fullPath)
        if err_ptr.pointee.code != 0 {
            self.lastError = String(cString: err_ptr.pointee.message)
            print(self.lastError)
            signer_destroy_string(err_ptr.pointee.message)
            return
        }
        try_create_identity(err_ptr, self.suggestedName, self.selectedSeed, self.getSeed(seedName: self.selectedSeed), "sr25519", fullPath, self.selectedNetwork!.key, res, dbName)
        if err_ptr.pointee.code == 0 {
            self.fetchKeys()
        } else {
            self.lastError = String(cString: err_ptr.pointee.message)
            print(self.lastError)
            signer_destroy_string(err_ptr.pointee.message)
        }
    }
}
