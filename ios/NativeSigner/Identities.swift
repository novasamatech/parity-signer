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
        var err = ExternError()
        let err_ptr: UnsafeMutablePointer<ExternError> = UnsafeMutablePointer(&err)
        print("fetch keys")
        let res = get_relevant_identities(err_ptr, self.selectedSeed, self.selectedNetwork!.key, self.dbName)
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
            print("Rust returned error")
            print(self.lastError)
            signer_destroy_string(err_ptr.pointee.message)
        }
    }
    
    func deleteActiveIdentity() {
        var err = ExternError()
        let err_ptr: UnsafeMutablePointer<ExternError> = UnsafeMutablePointer(&err)
        delete_identity(err_ptr, self.selectedIdentity?.public_key, self.selectedNetwork!.key, self.dbName)
        if err_ptr.pointee.code == 0 {
            self.selectedIdentity = nil
            self.fetchKeys()
        } else {
            self.lastError = String(cString: err_ptr.pointee.message)
            print("Rust returned error")
            print(self.lastError)
            signer_destroy_string(err_ptr.pointee.message)
        }
    }
    
    func proposeDerive() {
        self.lastError = ""
        if self.selectedIdentity == nil {
            self.suggestedPath = "//"
        } else {
            self.suggestedPath = self.selectedIdentity!.path
        }
        self.suggestedName = String(cString: suggest_name(nil, self.suggestedPath))
    }
    
    func proposeIncrement() {
        var err = ExternError()
        let err_ptr: UnsafeMutablePointer<ExternError> = UnsafeMutablePointer(&err)
        self.lastError = ""
        if self.selectedIdentity == nil {  //this should be impossible but plug it anyway
            self.suggestedPath = "//"
        } else {
            let res = suggest_n_plus_one(err_ptr, self.selectedIdentity!.path, self.selectedSeed, self.selectedNetwork!.key, self.dbName)
            if err_ptr.pointee.code == 0 {
                self.suggestedPath = String(cString: res!)
            } else {
                self.lastError = String(cString: err_ptr.pointee.message)
                print("Rust returned error")
                print(self.lastError)
                signer_destroy_string(err_ptr.pointee.message)
            }
        }
        self.suggestedName = String(cString: suggest_name(nil, self.suggestedPath))
    }
    
    //This does not report error if created identity is identical with already existing one.
    //This is intended behavior unless there are objections
    func createIdentity(password: String) {
        var err = ExternError()
        let err_ptr: UnsafeMutablePointer<ExternError> = UnsafeMutablePointer(&err)
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
        if password == "" && res != 0 {
            self.lastError = "The sequence /// is not allowed in path; use password field to include password (omitting ///)"
            print("password was entered in path field")
            return
        }
        try_create_identity(err_ptr, self.suggestedName, self.selectedSeed, self.getSeed(seedName: self.selectedSeed), "sr25519", fullPath, self.selectedNetwork!.key, res, self.dbName)
        if err_ptr.pointee.code == 0 {
            print("Identity added!")
            self.fetchKeys()
        } else {
            self.lastError = String(cString: err_ptr.pointee.message)
            print("Rust returned error")
            print(self.lastError)
            signer_destroy_string(err_ptr.pointee.message)
        }
    }
}
