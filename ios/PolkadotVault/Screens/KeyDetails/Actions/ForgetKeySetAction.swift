//
//  ForgetKeySetAction.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 09/09/2022.
//

import SwiftUI

final class ForgetKeySetAction {
    private let seedsMediator: SeedsMediating
    private let snackbarPresentation: BottomSnackbarPresentation
    private weak var navigation: NavigationCoordinator!

    init(
        snackbarPresentation: BottomSnackbarPresentation = ServiceLocator.bottomSnackbarPresentation,
        seedsMediator: SeedsMediating = ServiceLocator.seedsMediator
    ) {
        self.snackbarPresentation = snackbarPresentation
        self.seedsMediator = seedsMediator
    }

    func use(navigation: NavigationCoordinator) {
        self.navigation = navigation
    }

    func forgetKeySet(_ keySet: String) {
        // Remove from keychain first
        let isRemoved = seedsMediator.removeSeed(seedName: keySet)
        guard isRemoved else { return }
        // Now update UI state -> this moves user to Logs
        navigation.performFake(navigation: .init(action: .removeSeed))

        // We need this call to Rust state machine to move user manually from Logs to Keys tab as per new design
        navigation.perform(navigation: .init(action: .navbarKeys))
        // After moving user to Keys, present snackbar from bottom as action confirmation
        snackbarPresentation.viewModel = .init(
            title: Localizable.KeySetsModal.Confirmation.snackbar.string,
            style: .warning
        )
        snackbarPresentation.isSnackbarPresented = true
    }
}
