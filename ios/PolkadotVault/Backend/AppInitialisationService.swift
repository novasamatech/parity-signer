//
//  AppInitialisationService.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 05/05/2023.
//

import Foundation

final class AppInitialisationService {
    private let navigation: NavigationCoordinator

    init(
        navigation: NavigationCoordinator = NavigationCoordinator()
    ) {
        self.navigation = navigation
    }

    func initialiseAppSession() {
        navigation.performFake(navigation: .init(action: .start))
    }
}
