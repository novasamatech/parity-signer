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
    private let navigation: NavigationCoordinator

    init(
        snackbarPresentation: BottomSnackbarPresentation = ServiceLocator.bottomSnackbarPresentation,
        seedsMediator: SeedsMediating = ServiceLocator.seedsMediator,
        navigation: NavigationCoordinator
    ) {
        self.snackbarPresentation = snackbarPresentation
        self.seedsMediator = seedsMediator
        self.navigation = navigation
    }

    func forgetKeySet(_ keySet: String) {
        // This calls `navigation.perform(navigation: .init(action: .removeSeed), skipDebounce: true)` underneath,
        // which will call Rust state machine and user will be taken to Logs tab to see new history card regarding key
        // set
        // removal
        // I'll move it outside of `seedsMediator` when all removal actions are refactored
        seedsMediator.removeSeed(seedName: keySet)
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
