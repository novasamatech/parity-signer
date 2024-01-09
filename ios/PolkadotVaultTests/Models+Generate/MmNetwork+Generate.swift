//
//  MmNetwork+Generate.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 08/01/2024.
//

import Foundation
@testable import PolkadotVault

extension MmNetwork {
    static func generate(
        key: String = "defaultKey",
        title: String = "Default Title",
        logo: String = "defaultLogo",
        order: UInt8 = 0,
        pathId: String = "defaultPathId"
    ) -> MmNetwork {
        MmNetwork(key: key, title: title, logo: logo, order: order, pathId: pathId)
    }
}
