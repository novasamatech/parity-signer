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
    var multiselect: Bool?
}

/**
 * Checkers for derivation path
 */
struct DerivationCheck: Decodable, Equatable {
    var button_good: Bool?
    var where_to: DerivationDestination?
    var collision: Address?
    var error: String?
}

/**
 * Destination of "next" button in key derivation screen
 */
enum DerivationDestination: String, Decodable {
    case pwd
    case pin
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
