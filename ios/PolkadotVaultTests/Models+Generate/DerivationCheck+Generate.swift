//
//  DerivationCheck+Generate.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 05/02/2024.
//

import Foundation
@testable import PolkadotVault

extension DerivationCheck {
    static func generate(
        buttonGood: Bool = true,
        whereTo: DerivationDestination? = nil,
        collision: MAddressCard? = nil,
        error: String? = nil
    ) -> DerivationCheck {
        DerivationCheck(
            buttonGood: buttonGood,
            whereTo: whereTo,
            collision: collision,
            error: error
        )
    }
}
