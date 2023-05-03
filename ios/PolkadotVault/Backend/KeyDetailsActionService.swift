//
//  KeyDetailsActionService.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 24/04/2023.
//

import Foundation

final class KeyDetailsActionService {
    private let navigation: NavigationCoordinator

    init(
        navigation: NavigationCoordinator = NavigationCoordinator()
    ) {
        self.navigation = navigation
    }

    func performBackupSeed(_ keyName: String) {
        resetNavigationStateToKeyDetails(keyName)
        navigation.performFake(navigation: .init(action: .rightButtonAction))
        navigation.performFake(navigation: .init(action: .backupSeed))
        navigation.performFake(navigation: .init(action: .rightButtonAction))
    }

    func navigateToPublicKey(_ keyName: String, _ publicKeyDetails: String) -> MKeyDetails? {
        resetNavigationStateToKeyDetails(keyName)
        guard case let .keyDetails(keyDetails) = navigation
            .performFake(navigation: .init(action: .selectKey, details: publicKeyDetails)).screenData,
            let keyDetails = keyDetails else { return nil }
        return keyDetails
    }

    func forgetKeySetAction(_ keyName: String) {
        resetNavigationStateToKeyDetails(keyName)
        navigation.performFake(navigation: .init(action: .rightButtonAction))
        // Now update UI state -> this moves user to Logs
        navigation.performFake(navigation: .init(action: .removeSeed))
        // We need this call to Rust state machine to move user manually from Logs to Keys tab as per new design
        navigation.performFake(navigation: .init(action: .navbarKeys))
        navigation.performFake(navigation: .init(action: .selectSeed, details: keyName))
    }

    func resetNavigationStateToKeyDetails(_ keyName: String) {
        navigation.performFake(navigation: .init(action: .start))
        navigation.performFake(navigation: .init(action: .navbarKeys))
        navigation.performFake(navigation: .init(action: .selectSeed, details: keyName))
    }
}
