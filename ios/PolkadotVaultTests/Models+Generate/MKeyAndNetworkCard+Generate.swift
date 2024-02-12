//
//  MKeyAndNetworkCard+Generate.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 05/02/2024.
//

import Foundation
@testable import PolkadotVault

extension MKeyAndNetworkCard {
    static func generate(
        key: MKeysCard = .generate(),
        network: MscNetworkInfo = .generate()
    ) -> MKeyAndNetworkCard {
        MKeyAndNetworkCard(
            key: key,
            network: network
        )
    }
}

extension MKeysCard {
    static func generate(
        address: Address = .generate(),
        addressKey: String = "addressKey",
        base58: String = "base58",
        swiped: Bool = false
    ) -> MKeysCard {
        MKeysCard(
            address: address,
            addressKey: addressKey,
            base58: base58,
            swiped: swiped
        )
    }
}
