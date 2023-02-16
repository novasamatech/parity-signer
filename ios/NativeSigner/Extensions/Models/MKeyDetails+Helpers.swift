//
//  MKeyDetails+Helpers.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 13/09/2022.
//

import Foundation

extension MKeyDetails {
    var isRootKey: Bool {
        address.path.isEmpty
    }
}
