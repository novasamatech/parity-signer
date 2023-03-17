//
//  Address+DisplayablePath.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 02/12/2022.
//

import SwiftUI

extension Address {
    /// Returns either `path` or if password protected, available path with path delimeter and lock icon
    var displayablePath: String {
        hasPwd ?
            "\(path)\(Localizable.Shared.Label.passwordedPathDelimeter.string)" :
            path
    }
}
