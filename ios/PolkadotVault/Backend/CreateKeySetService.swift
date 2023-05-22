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

    func createKeySet(seedName: String) -> MNewSeedBackup! {
        navigation.performFake(navigation: .init(action: .start))
        navigation.performFake(navigation: .init(action: .navbarKeys))
        navigation.performFake(navigation: .init(action: .rightButtonAction))
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
