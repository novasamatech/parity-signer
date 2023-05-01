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

    func startBananaSplitRecover(_ seedName: String, isFirstSeed: Bool) {
        navigation.performFake(navigation: .init(action: .start))
        navigation.performFake(navigation: .init(action: .navbarKeys))
        // Key Set List state has different "modalData" state depending on whether user has at least one key
        // or not
        // So we need to check whether we should actually "pretend" to open "more" navigation bar menu by
        // calling
        // .rightButtonAction
        if !isFirstSeed {
            navigation.performFake(navigation: .init(action: .rightButtonAction))
        }
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
