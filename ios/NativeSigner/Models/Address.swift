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
    var base58: String
    var path: String
    var has_pwd: Bool
    var identicon: String
    var seed_name: String
}

/*
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
 */

/**
 * Useful utility functions for address
 */
extension Address {
    /**
     * Get truncated base58 address representation that fits on screen
     */
    func truncateBase58() -> String {
        return self.base58.prefix(8) + "..." + self.base58.suffix(8)
    }
    
    /**
     * Same as truncateBase58 but shorter for very space-constrained places
     */
    func truncateBase58to8() -> String {
        return self.base58.prefix(4) + "..." + self.base58.suffix(4)
    }
    
    /**
     * Definition of root address
     */
    func isRoot() -> Bool {
        return self.path == "" && !self.has_pwd
    }
}

/*
 * Checkers for derivation path
 */
struct DerivationState {
    var isValid: Bool
    var hasPassword: Bool
}

extension String {
    /*
     * Check whether we need to confirm password
     * and whether next button should be active
     * in one move using tristate error hack
     */
    func checkAsDerivation() -> DerivationState {
        var err = ExternError()
        return withUnsafeMutablePointer(to: &err) {err_ptr in
            let res = check_path(err_ptr, self)
            if err_ptr.pointee.code == 0 {
                return DerivationState(isValid: true, hasPassword: res != 0)
            } else {
                signer_destroy_string(err_ptr.pointee.message)
                return DerivationState(isValid: false, hasPassword: false)
            }
        }
    }
}


/**
 * Address-related operations in data model
 */
extension SignerDataModel {
    
    /**
     * Creates address in database with checks and features
     */
    //This does not report error if created address is identical with already existing one.
    //This is intended behavior unless there are objections
    func createAddress(path: String, seedName: String) {
        let seedPhrase = self.getSeed(seedName: seedName)
        if !seedPhrase.isEmpty {
            pushButton(buttonID: .GoForward, details: path, seedPhrase: seedPhrase)
        }
    }
}
