//
//  NavigationCoordinator.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 05/08/2022.
//

import Foundation

typealias NavigationRequest = (Navigation) -> Void

/// This is a custom navigator to keep this code somewhat similar to what android has
/// and implement some simple shallow navigation without pulling legacy or experimental libs
/// Slightly non-trivial navigation
/// We should keep this to minimum
final class NavigationCoordinator: ObservableObject {
    private let backendActionPerformer: BackendNavigationPerforming

    /// Responsible for presentation of generic error bottom sheet alert component
    /// Currently error is based on `actionResult.alertData` component when app receives `.errorData(message)` value
    @Published var genericError = GenericErrorViewModel()

    init(
        backendActionPerformer: BackendNavigationPerforming = BackendNavigationAdapter()
    ) {
        self.backendActionPerformer = backendActionPerformer
    }
}

extension NavigationCoordinator {
    @discardableResult
    func performFake(navigation: Navigation) -> ActionResult? {
        let result = backendActionPerformer.performBackend(
            action: navigation.action,
            details: navigation.details,
            seedPhrase: navigation.seedPhrase
        )
        switch result {
        case let .success(action):
            updateGlobalViews(action)
            return action
        case let .failure(error):
            genericError.errorMessage = error.description
            genericError.isPresented = true
            return nil
        }
    }

    func performTransaction(with payload: String) -> Result<ActionResult, TransactionError> {
        backendActionPerformer.performTransaction(with: payload)
    }
}

private extension NavigationCoordinator {
    func updateGlobalViews(_ actionResult: ActionResult) {
        if case let .errorData(message) = actionResult.alertData {
            genericError.errorMessage = message
            genericError.isPresented = true
        }
    }
}
