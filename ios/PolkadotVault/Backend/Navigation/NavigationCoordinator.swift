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
    @Published var navigationErrorPresentation: NavigationErrorPresentation

    init(
        backendActionPerformer: BackendNavigationPerforming = BackendNavigationAdapter(),
        navigationErrorPresentation: NavigationErrorPresentation = ServiceLocator.navigationErrorPresentation
    ) {
        self.backendActionPerformer = backendActionPerformer
        _navigationErrorPresentation = .init(initialValue: navigationErrorPresentation)
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
            navigationErrorPresentation.errorMessage = error.description
            navigationErrorPresentation.isPresented = true
            return nil
        }
    }
}

private extension NavigationCoordinator {
    func updateGlobalViews(_ actionResult: ActionResult) {
        if case let .errorData(message) = actionResult.alertData {
            navigationErrorPresentation.errorMessage = message
            navigationErrorPresentation.isPresented = true
        }
    }
}
