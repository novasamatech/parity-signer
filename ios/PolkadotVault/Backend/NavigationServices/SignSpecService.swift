//
//  SignSpecService.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 08/05/2023.
//

import Foundation

final class SignSpecService {
    private let navigation: NavigationCoordinator

    init(
        navigation: NavigationCoordinator = NavigationCoordinator()
    ) {
        self.navigation = navigation
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
        navigation.performFake(navigation: .init(action: .start))
        navigation.performFake(navigation: .init(action: .navbarSettings))
        navigation.performFake(navigation: .init(action: .manageNetworks))
        navigation.performFake(navigation: .init(action: .goForward, details: networkKey))
        navigation.performFake(navigation: .init(action: .rightButtonAction))
        guard case let .signSufficientCrypto(value) = navigation
            .performFake(navigation: .init(action: .signNetworkSpecs))?.screenData else { return nil }
        return value
    }

    func signMetadataSpecList(_ networkKey: String, _ specsVersion: String) -> MSignSufficientCrypto! {
        navigation.performFake(navigation: .init(action: .start))
        navigation.performFake(navigation: .init(action: .navbarSettings))
        navigation.performFake(navigation: .init(action: .manageNetworks))
        navigation.performFake(navigation: .init(action: .goForward, details: networkKey))
        navigation.performFake(navigation: .init(action: .manageMetadata, details: specsVersion))
        guard case let .signSufficientCrypto(value) = navigation
            .performFake(navigation: .init(action: .signMetadata))?.screenData else { return nil }
        return value
    }

    func attemptPassword(_ password: String) -> ActionResult? {
        navigation.performFake(navigation: .init(action: .goForward, details: password))
    }
}
