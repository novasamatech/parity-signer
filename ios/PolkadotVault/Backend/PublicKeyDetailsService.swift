//
//  PublicKeyDetailsService.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 26/04/2023.
//

import Foundation

final class PublicKeyDetailsService {
    private let navigation: NavigationCoordinator

    init(
        navigation: NavigationCoordinator = NavigationCoordinator()
    ) {
        self.navigation = navigation
    }

    func forgetSingleKey(_ keyName: String) {
        navigation.performFake(navigation: .init(action: .rightButtonAction))
        navigation.performFake(navigation: .init(action: .removeKey))
        navigation.performFake(navigation: .init(action: .navbarKeys))
        navigation.performFake(navigation: .init(action: .selectSeed, details: keyName))
    }

    func resetNavigationState(_ keyName: String, _ publicKeyDetails: String) {
        navigation.performFake(navigation: .init(action: .start))
        navigation.performFake(navigation: .init(action: .navbarKeys))
        navigation.performFake(navigation: .init(action: .selectSeed, details: keyName))
        navigation.performFake(navigation: .init(action: .selectKey, details: publicKeyDetails))
    }
}
