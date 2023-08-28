//
//  PublicKeyDetailsService.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 26/04/2023.
//

import Foundation

final class PublicKeyDetailsService {
    private let navigation: NavigationCoordinator
    private let backendService: BackendService

    init(
        navigation: NavigationCoordinator = NavigationCoordinator(),
        backendService: BackendService = BackendService()
    ) {
        self.navigation = navigation
        self.backendService = backendService
    }

    func forgetSingleKey(
        address: String,
        networkSpecsKey: String,
        _ completion: @escaping (Result<Void, ServiceError>) -> Void
    ) {
        backendService.performCall({
            try removeDerivedKey(address: address, networkSpecsKey: networkSpecsKey)
        }, completion: completion)
    }

    func resetNavigationState(_ keyName: String, _ publicKeyDetails: String) {
        navigation.performFake(navigation: .init(action: .start))
        navigation.performFake(navigation: .init(action: .navbarKeys))
        navigation.performFake(navigation: .init(action: .selectSeed, details: keyName))
        navigation.performFake(navigation: .init(action: .selectKey, details: publicKeyDetails))
    }
}
