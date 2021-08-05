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
        let res = get_relevant_identities(self.err_ptr, self.selectedSeed, self.selectedNetwork!.key, self.dbName)
        if self.err_ptr.pointee.code == 0 {
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
            self.handleRustError()
        }
    }
    
    func deleteActiveIdentity() {
        delete_identity(self.err_ptr, self.selectedIdentity?.public_key, self.selectedNetwork!.key, self.dbName)
        if self.err_ptr.pointee.code == 0 {
            self.selectedIdentity = nil
            self.fetchKeys()
        } else {
            self.handleRustError()
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
        if self.selectedIdentity == nil {  //this should be impossible but plug it anyway
            self.suggestedPath = "//"
        } else {
            let res = suggest_n_plus_one(self.err_ptr, self.selectedIdentity!.path, self.selectedSeed, self.selectedNetwork!.key, self.dbName)
            if self.err_ptr.pointee.code == 0 {
                self.suggestedPath = String(cString: res!)
            } else {
                self.handleRustError()
            }
        }
    }
    
    func createIdentity(password: String) {
        var fullPath = ""
        if password == "" {
            fullPath = self.suggestedPath
        } else {
            fullPath = self.suggestedPath + "///" + password
        }
        let res = check_path(self.err_ptr, fullPath)
        if self.err_ptr.pointee.code != 0 {
            self.lastError = String(cString: self.err_ptr.pointee.message)
            print(self.lastError)
            signer_destroy_string(self.err_ptr.pointee.message)
            return
        }
        try_create_identity(self.err_ptr, self.suggestedName, self.selectedSeed, self.getSeed(seedName: self.selectedSeed), "sr25519", fullPath, self.selectedNetwork!.key, res, self.dbName)
        if self.err_ptr.pointee.code == 0 {
            self.fetchKeys()
        } else {
            self.handleRustError()
        }
    }
}
