//
//  Error+RustDisplayError.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 31/07/2023.
//

import Foundation

extension Error {
    var backendDisplayError: String {
        (self as? ErrorDisplayed)?.localizedDescription ?? localizedDescription
    }
}
