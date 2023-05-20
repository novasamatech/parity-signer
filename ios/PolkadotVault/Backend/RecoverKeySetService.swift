//
//  RecoverKeySetService.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 18/04/2023.
//

import Foundation

final class RecoverKeySetService {
    private let navigation: NavigationCoordinator

    init(
        navigation: NavigationCoordinator = NavigationCoordinator()
    ) {
        self.navigation = navigation
    }

    func recoverKeySetStart(_ isFirstKeySet: Bool) -> MRecoverSeedName! {
        navigation.performFake(navigation: .init(action: .start))
        navigation.performFake(navigation: .init(action: .navbarKeys))
        // We need to call this conditionally, as if there are no seeds,
        // Rust does not expect `rightButtonAction` called before `addSeed` / `recoverSeed`
        if !isFirstKeySet {
            navigation.performFake(navigation: .init(action: .rightButtonAction))
        }
        guard case let .recoverSeedName(value) = navigation.performFake(navigation: .init(action: .recoverSeed))?
            .screenData else { return nil }
        return value
    }

    func continueKeySetRecovery(_ seedName: String) -> MRecoverSeedPhrase! {
        guard case let .recoverSeedPhrase(value) = navigation
            .performFake(navigation: .init(action: .goForward, details: seedName))?.screenData else { return nil }
        return value
    }

    func updateGuess(_ guess: String) -> MRecoverSeedPhrase? {
        let result = navigation.performFake(navigation: .init(action: .pushWord, details: guess))
        guard case let .recoverSeedPhrase(updatedContent) = result?.screenData else { return nil }
        return updatedContent
    }

    func onUserEntry(_ wordToSend: String) -> MRecoverSeedPhrase? {
        let result = navigation.performFake(navigation: .init(action: .textEntry, details: wordToSend))
        guard case let .recoverSeedPhrase(updatedContent) = result?.screenData else { return nil }
        return updatedContent
    }

    func finishKeySetRecover(_ seedPhrase: String) {
        navigation.performFake(navigation: .init(
            action: .goForward,
            details: BackendConstants.true,
            seedPhrase: seedPhrase
        ))
    }
}
