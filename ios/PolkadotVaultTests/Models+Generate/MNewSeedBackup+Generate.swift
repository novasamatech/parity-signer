//
//  MNewSeedBackup+Generate.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 25/01/2024.
//

import Foundation
@testable import PolkadotVault

extension MNewSeedBackup {
    static func generate(
        seed: String = "defaultSeed",
        seedPhrase: String = "defaultSeedPhrase",
        identicon: Identicon = .generate()
    ) -> MNewSeedBackup {
        MNewSeedBackup(
            seed: seed,
            seedPhrase: seedPhrase,
            identicon: identicon
        )
    }
}
