//
//  KeyListService.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 14/04/2023.
//

import Foundation

final class KeyListService {
    private let navigation: NavigationCoordinator

    init(
        navigation: NavigationCoordinator = NavigationCoordinator()
    ) {
        self.navigation = navigation
    }

    func getKeyList() -> MSeeds! {
        navigation.performFake(navigation: .init(action: .start))
        guard case let .seedSelector(value) = navigation
            .performFake(navigation: .init(action: .navbarKeys))?.screenData else { return nil }
        return value
    }
}
