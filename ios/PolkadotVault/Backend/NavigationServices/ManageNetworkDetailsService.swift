//
//  ManageNetworkDetailsService.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 11/04/2023.
//

import Foundation

final class ManageNetworkDetailsService {
    private let navigation: NavigationCoordinator
    private let backendService: BackendService

    init(
        navigation: NavigationCoordinator = NavigationCoordinator(),
        backendService: BackendService = BackendService()
    ) {
        self.navigation = navigation
        self.backendService = backendService
    }

    func refreshCurrentNavigationState(_ networkKey: String) -> MNetworkDetails! {
        navigation.performFake(navigation: .init(action: .start))
        navigation.performFake(navigation: .init(action: .navbarSettings))
        navigation.performFake(navigation: .init(action: .manageNetworks))
        guard case let .nNetworkDetails(value) = navigation
            .performFake(navigation: .init(action: .goForward, details: networkKey))?.screenData else { return nil }
        return value
    }

    func attemptSigning(_ keyRecord: MRawKey, _ seedPhrase: String?) -> ActionResult? {
        navigation.performFake(
            navigation: .init(
                action: .goForward,
                details: keyRecord.addressKey,
                seedPhrase: seedPhrase
            )
        )
    }

    @discardableResult
    func signSpecList(_ networkKey: String) -> MSignSufficientCrypto! {
        _ = refreshCurrentNavigationState(networkKey)
        navigation.performFake(navigation: .init(action: .rightButtonAction))
        guard case let .signSufficientCrypto(value) = navigation
            .performFake(navigation: .init(action: .signNetworkSpecs))?.screenData else { return nil }
        return value
    }

    func signMetadataSpecList(_ networkKey: String, _ specsVersion: String) -> MSignSufficientCrypto! {
        _ = refreshCurrentNavigationState(networkKey)
        navigation.performFake(navigation: .init(action: .manageMetadata, details: specsVersion))
        guard case let .signSufficientCrypto(value) = navigation
            .performFake(navigation: .init(action: .signMetadata))?.screenData else { return nil }
        return value
    }

    func attemptPassword(_ password: String) -> ActionResult? {
        navigation.performFake(navigation: .init(action: .goForward, details: password))
    }

    func deleteNetwork(
        _ networkKey: String,
        _ completion: @escaping (Result<Void, ServiceError>) -> Void
    ) {
        backendService.performCall({
            try removeManagedNetwork(networkKey: networkKey)
        }, completion: completion)
    }

    func deleteNetworkMetadata(
        _ networkKey: String,
        _ specsVersion: String,
        _ completion: @escaping (Result<Void, ServiceError>) -> Void
    ) {
        backendService.performCall({
            try removeMetadataOnManagedNetwork(networkKey: networkKey, metadataSpecsVersion: specsVersion)
        }, completion: completion)
    }
}
