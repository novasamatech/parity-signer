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
    var multiselect: Bool
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
