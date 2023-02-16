//
//  SeedKeysPreview+Helpers.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 05/01/2023.
//

import Foundation

extension SeedKeysPreview {
    /// Returns count of `DerivedKeyPreview`
    var importableKeysCount: Int {
        derivedKeys.filter { $0.status == .importable }.count
    }

    /// Whether `SeedKeysPreview` can be
    var isImportable: Bool {
        importableKeysCount > 0
    }
}
