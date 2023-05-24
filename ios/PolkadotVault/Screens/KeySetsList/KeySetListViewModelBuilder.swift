//
//  KeySetListViewModelBuilder.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 22/08/2022.
//

import SwiftUI

/// View model to render `KeySetList` table view
struct KeySetListViewModel: Equatable {
    let list: [KeySetViewModel]
}

/// View model to render single row in `KeySetList`
struct KeySetViewModel: Equatable, Identifiable {
    let id = UUID()
    let seed: SeedNameCard
    let keyName: String
    let derivedKeys: String?
    let identicon: SignerImage
    let networks: [String]

    init(
        seed: SeedNameCard,
        keyName: String,
        derivedKeys: String?,
        identicon: SignerImage,
        networks: [String]
    ) {
        self.seed = seed
        self.keyName = keyName
        self.derivedKeys = derivedKeys
        self.identicon = identicon
        self.networks = networks
    }
}

/// Builds view model for `KeySetList` table view
final class KeySetListViewModelBuilder {
    func build(for seeds: [SeedNameCard]) -> KeySetListViewModel {
        KeySetListViewModel(
            list: seeds.map {
                KeySetViewModel(
                    seed: $0,
                    keyName: $0.seedName,
                    derivedKeys: derivedKeys(for: $0),
                    identicon: $0.identicon,
                    networks: $0.usedInNetworks
                )
            }
        )
    }
}

private extension KeySetListViewModelBuilder {
    func derivedKeys(for seed: SeedNameCard) -> String? {
        switch seed.derivedKeysCount {
        case 0:
            return nil
        case 1:
            return Localizable.KeySets.Label.DerivedKeys.single(1)
        default:
            return Localizable.KeySets.Label.DerivedKeys.plural(seed.derivedKeysCount)
        }
    }
}
