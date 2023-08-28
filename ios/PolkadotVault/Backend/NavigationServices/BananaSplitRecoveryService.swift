//
//  BananaSplitRecoveryService.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 01/05/2023.
//

import Foundation

final class BananaSplitRecoveryService {
    private let navigation: NavigationCoordinator

    init(
        navigation: NavigationCoordinator = NavigationCoordinator()
    ) {
        self.navigation = navigation
    }

    func startBananaSplitRecover(_ seedName: String) {
        navigation.performFake(navigation: .init(action: .start))
        navigation.performFake(navigation: .init(action: .navbarKeys))
        navigation.performFake(navigation: .init(action: .rightButtonAction))
        navigation.performFake(navigation: .init(action: .recoverSeed))
        navigation.performFake(navigation: .init(action: .goForward, details: seedName))
    }

    func completeBananaSplitRecovery(_ seedPhrase: String) {
        navigation.performFake(navigation: .init(
            action: .goForward,
            details: BackendConstants.true,
            seedPhrase: seedPhrase
        ))
        navigation.performFake(navigation: .init(action: .start))
    }
}
