//
//  Identities.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 3.8.2021.
//

import Foundation
import SwiftUI

/**
 * Displayable information about a public key within a network
 */
struct Address: Codable, Equatable {
    var public_key: String
    var ss58: String
    var path: String
    var has_password: String
    var name: String
    var seed_name: String
}

/**
 * Mock test sample
 */
extension Address {
    static var addressData: [Address] {
        [
            Address(public_key: "1691a3ce28763a83e094bd5b06835bc5bba4d38d770710f8f327d4f2c71afb21", ss58: "1WbKRCtpZ6XbTdf9w8i7KVwctANhQhQg27qfE54RbamvfrD", path: "", has_password: "false", name: "root address", seed_name: "Pupa"),
            Address(public_key: "1791a3ce28763a83e094bd5b06835bc5bba4d38d770710f8f327d4f2c71afb21", ss58: "11bKRCtpZ6XbTdf9w8i7KVwctANhQhQg27qfE54RbamvfrD", path: "", has_password: "true", name: "Some other address", seed_name: "Lupa")
        ]
    }
}

/**
 * Address-related operations in data model
 */
extension SignerDataModel {
    /**
     * Refresh list of known addresses
     */
    func fetchKeys() {
        var err = ExternError()
        let err_ptr: UnsafeMutablePointer<ExternError> = UnsafeMutablePointer(&err)
        print("fetch keys")
        let res = get_relevant_identities(err_ptr, self.selectedSeed, self.selectedNetwork!.key, self.dbName)
        if err_ptr.pointee.code == 0 {
            if let keysJSON = String(cString: res!).data(using: .utf8) {
                guard let keys = try? JSONDecoder().decode([Address].self, from: keysJSON) else {
                    print("JSON decoder failed on keys")
                    signer_destroy_string(res!)
                    return
                }
                self.addresses = keys.sorted(by: {$0.path < $1.path})
            } else {
                print("keysJSON corrupted")
            }
            signer_destroy_string(res!)
            print("success1")
        } else {
            self.lastError = String(cString: err_ptr.pointee.message)
            print("Rust returned error")
            print(self.lastError)
            signer_destroy_string(err_ptr.pointee.message)
        }
    }
    
    /**
     * Removes selected account from database with all checks and features
     */
    func deleteSelectedAddress() {
        var err = ExternError()
        let err_ptr: UnsafeMutablePointer<ExternError> = UnsafeMutablePointer(&err)
        delete_identity(err_ptr, self.selectedAddress?.public_key, self.selectedNetwork!.key, self.dbName)
        if err_ptr.pointee.code == 0 {
            self.selectedAddress = nil
            self.fetchKeys()
        } else {
            self.lastError = String(cString: err_ptr.pointee.message)
            print("Rust returned error")
            print(self.lastError)
            signer_destroy_string(err_ptr.pointee.message)
        }
    }
    
    /**
     * Populate path with blank suggestion for derivation screen
     */
    func proposeDerive() {
        self.lastError = ""
        if self.selectedAddress == nil {
            self.suggestedPath = "//"
        } else {
            self.suggestedPath = self.selectedAddress!.path
        }
        self.suggestedName = String(cString: suggest_name(nil, self.suggestedPath))
    }
    
    /**
     * Populate path with N+1 suggestion for derivation screen
     */
    func proposeIncrement() {
        var err = ExternError()
        let err_ptr: UnsafeMutablePointer<ExternError> = UnsafeMutablePointer(&err)
        self.lastError = ""
        if self.selectedAddress == nil {  //this should be impossible but plug it anyway
            self.suggestedPath = "//"
        } else {
            let res = suggest_n_plus_one(err_ptr, self.selectedAddress!.path, self.selectedSeed, self.selectedNetwork!.key, self.dbName)
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
    
    /**
     * Creates address in database with checks and features
     */
    //This does not report error if created address is identical with already existing one.
    //This is intended behavior unless there are objections
    func createAddress(password: String) {
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
    
    /**
     * Returns QR with exported key for selected address and selected network
     */
    func exportIdentityQR() -> UIImage? {
        var err = ExternError()
        let err_ptr: UnsafeMutablePointer<ExternError> = UnsafeMutablePointer(&err)
        if (self.selectedNetwork == nil) || (self.selectedAddress == nil) {
            self.lastError = "identity not defined!"
            return nil
        }
        let res = export_pubkey(err_ptr, self.selectedAddress!.public_key, self.selectedNetwork!.key, self.dbName)
        if err_ptr.pointee.code == 0 {
            let result = String(cString: res!)
            signer_destroy_string(res!)
            if let imageData = Data(fromHexEncodedString: result ) {
                return UIImage(data: imageData)
            } else {
                self.lastError = "QR code generation error"
            }
        } else {
            self.lastError = String(cString: err_ptr.pointee.message)
            print(self.lastError)
            signer_destroy_string(err_ptr.pointee.message)
        }
        
        return nil
    }
    
    /**
     * Selects next address in key manager
     */
    func selectNextAddress() {
        if self.selectedAddress != nil {
            if let current = self.addresses.firstIndex(of: self.selectedAddress!) {
                if current < (self.addresses.capacity - 1) {
                    self.selectedAddress = self.addresses[current+1]
                }
            }
        }
    }

    /**
     * Selects previous address in key manager
     */
    func selectPreviousAddress() {
        if self.selectedAddress != nil {
            if let current = self.addresses.firstIndex(of: self.selectedAddress!) {
                if current > 0 {
                    self.selectedAddress = self.addresses[current-1]
                }
            }
        }
    }
}
