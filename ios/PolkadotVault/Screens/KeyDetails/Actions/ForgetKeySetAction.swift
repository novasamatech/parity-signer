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
        navigation: NavigationCoordinator = NavigationCoordinator()
    ) {
        self.snackbarPresentation = snackbarPresentation
        self.seedsMediator = seedsMediator
        self.navigation = navigation
    }

    func forgetKeySet(_ keySet: String) {
        // Remove from keychain first
        let isRemoved = seedsMediator.removeSeed(seedName: keySet)
        guard isRemoved else { return }

        // Present snackbar from bottom as action confirmation
        snackbarPresentation.viewModel = .init(
            title: Localizable.KeySetsModal.Confirmation.snackbar.string,
            style: .warning
        )
        snackbarPresentation.isSnackbarPresented = true
    }
}
