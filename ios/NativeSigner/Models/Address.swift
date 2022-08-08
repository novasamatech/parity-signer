//
//  Identities.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 3.8.2021.
//

import Foundation
import SwiftUI

/// Address-related operations in data model
extension SignerDataModel {
    /// Creates address in database with checks and features
    func createAddress(path: String, seedName: String) {
        let seedPhrase = getSeed(seedName: seedName)
        if !seedPhrase.isEmpty {
            navigation.perform(navigation: .init(action: .goForward, details: path, seedPhrase: seedPhrase))
        }
    }
}
