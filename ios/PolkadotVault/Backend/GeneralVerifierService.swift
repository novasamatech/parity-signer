//
//  GeneralVerifierService.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 08/05/2023.
//

import Foundation

final class GeneralVerifierService {
    private let navigation: NavigationCoordinator

    init(
        navigation: NavigationCoordinator = NavigationCoordinator()
    ) {
        self.navigation = navigation
    }

    func getGeneralVerifier() -> MVerifierDetails? {
        navigation.performFake(navigation: .init(action: .start))
        navigation.performFake(navigation: .init(action: .navbarSettings))
        guard case let .vVerifier(value) = navigation
            .performFake(navigation: .init(action: .viewGeneralVerifier))?.screenData else { return nil }
        navigation.performFake(navigation: .init(action: .goBack))
        return value
    }
}
