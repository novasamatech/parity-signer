//
//  ForgetSingleKeyAction.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 15/09/2022.
//

import SwiftUI

final class ForgetSingleKeyAction {
    private let snackbarPresentation: BottomSnackbarPresentation
    private weak var navigation: NavigationCoordinator!

    init(
        snackbarPresentation: BottomSnackbarPresentation = ServiceLocator.bottomSnackbarPresentation
    ) {
        self.snackbarPresentation = snackbarPresentation
    }

    func use(navigation: NavigationCoordinator) {
        self.navigation = navigation
    }

    func forgetSingleKey(_: String) {
        // This triggers key deletion and moves user to Logs tab
        navigation.performFake(navigation: .init(action: .removeKey))
        // We need this call to Rust state machine to move user manually from Logs to Keys tab as per new design
        navigation.perform(navigation: .init(action: .navbarKeys))
        // After moving user to Keys, present snackbar from bottom as action confirmation
        snackbarPresentation.viewModel = .init(
            title: Localizable.PublicKeyDetailsModal.Confirmation.snackbar.string,
            style: .warning
        )
        snackbarPresentation.isSnackbarPresented = true
    }
}
