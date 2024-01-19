//
//  MAddressCard+Generate.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 10/01/2024.
//

import Foundation
@testable import PolkadotVault

extension MAddressCard {
    static func generate(
        base58: String = "defaultBase58",
        addressKey: String = "defaultAddressKey",
        address: Address = .generate()
    ) -> MAddressCard {
        MAddressCard(
            base58: base58,
            addressKey: addressKey,
            address: address
        )
    }
}

extension Address {
    static func generate(
        path: String = "//polkadot//0",
        hasPwd: Bool = false,
        identicon: Identicon = Identicon.generate(),
        seedName: String = "Main Key Set",
        secretExposed: Bool = false
    ) -> Address {
        Address(
            path: path,
            hasPwd: hasPwd,
            identicon: identicon,
            seedName: seedName,
            secretExposed: secretExposed
        )
    }
}
