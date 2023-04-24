//
//  NewDerivedKeyService.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 23/04/2023.
//

import Foundation

final class NewDerivedKeyService {
    private let navigation: NavigationCoordinator

    init(
        navigation: NavigationCoordinator = NavigationCoordinator()
    ) {
        self.navigation = navigation
    }

    func startDeriveNewKey(_ keyName: String) {
        navigation.performFake(navigation: .init(action: .start))
        navigation.performFake(navigation: .init(action: .navbarKeys))
        navigation.performFake(navigation: .init(action: .selectSeed, details: keyName))
        navigation.performFake(navigation: .init(action: .newKey))
    }

    func continueKeySetRecovery(_ seedName: String) -> MRecoverSeedPhrase! {
        guard case let .recoverSeedPhrase(value) = navigation
            .performFake(navigation: .init(action: .goForward, details: seedName)).screenData else { return nil }
        return value
    }

    func finishKeySetRecover(_ seedPhrase: String) {
        navigation.performFake(navigation: .init(
            action: .goForward,
            details: BackendConstants.true,
            seedPhrase: seedPhrase
        ))
    }
}
