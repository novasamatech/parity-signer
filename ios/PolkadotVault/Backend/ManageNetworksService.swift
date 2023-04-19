//
//  ManageNetworksService.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 11/04/2023.
//

import Foundation

final class ManageNetworksService {
    private let navigation: NavigationCoordinator

    init(
        navigation: NavigationCoordinator = NavigationCoordinator()
    ) {
        self.navigation = navigation
    }

    func manageNetworks() -> [MmNetwork] {
        navigation.performFake(navigation: .init(action: .start))
        navigation.performFake(navigation: .init(action: .navbarSettings))
        guard case let .manageNetworks(value) = navigation
            .performFake(navigation: .init(action: .manageNetworks)).screenData else { return [] }
        return value.networks
    }
}
