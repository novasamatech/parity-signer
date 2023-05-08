//
//  CreateKeySetService.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 14/04/2023.
//

import Foundation

final class CreateKeySetService {
    private let navigation: NavigationCoordinator

    init(
        navigation: NavigationCoordinator = NavigationCoordinator()
    ) {
        self.navigation = navigation
    }

    func createKeySet(_ isFirstKeySet: Bool, seedName: String) -> MNewSeedBackup! {
        navigation.performFake(navigation: .init(action: .start))
        navigation.performFake(navigation: .init(action: .navbarKeys))
        // We need to call this conditionally, as if there are no seeds,
        // Rust does not expect `rightButtonAction` called before `addSeed` / `recoverSeed`
        if !isFirstKeySet {
            navigation.performFake(navigation: .init(action: .rightButtonAction))
        }
        navigation.performFake(navigation: .init(action: .newSeed))
        guard case let .newSeedBackup(value) = navigation
            .performFake(navigation: .init(action: .goForward, details: seedName))?.modalData else { return nil }
        return value
    }

    func confirmKeySetCreation(_ seedPhrase: String) {
        navigation.performFake(navigation: .init(
            action: .goForward,
            details: BackendConstants.true,
            seedPhrase: seedPhrase
        ))
    }
}
