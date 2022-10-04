//
//  ForgetSingleKeyAction.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 15/09/2022.
//

import SwiftUI

final class ForgetSingleKeyAction {
    private let snackbarPresentation: BottomSnackbarPresentation
    @EnvironmentObject private var navigation: NavigationCoordinator

    init(
        snackbarPresentation: BottomSnackbarPresentation = ServiceLocator.bottomSnackbarPresentation
    ) {
        self.snackbarPresentation = snackbarPresentation
    }

    func forgetSingleKey(_: String) {
        // This triggers key deletion and moves user to Logs tab
        navigation.perform(navigation: .init(action: .removeKey), skipDebounce: true)
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
