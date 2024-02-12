//
//  MKeysNew+Generate.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 05/02/2024.
//

import Foundation
@testable import PolkadotVault

extension MKeysNew {
    static func generate(
        root: MAddressCard = .generate(),
        set: [MKeyAndNetworkCard] = [.generate()]
    ) -> MKeysNew {
        MKeysNew(
            root: root,
            set: set
        )
    }
}
