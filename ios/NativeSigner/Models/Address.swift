//
//  Identities.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 3.8.2021.
//

import Foundation
import SwiftUI

/**
 * Address-related operations in data model
 */
extension SignerDataModel {
    /**
     * Creates address in database with checks and features
     */
    func createAddress(path: String, seedName: String) {
        let seedPhrase = self.getSeed(seedName: seedName)
        if !seedPhrase.isEmpty {
            pushButton(action: .goForward, details: path, seedPhrase: seedPhrase)
        }
    }
}
