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
    private enum Constants {
        static let debounceTime: Double = 0.2
        static let queueLabel = "navigationCoordinator.debounce"
    }

    private let debounceQueue: Dispatching
    private let backendActionPerformer: BackendNavigationPerforming
    private var isActionAvailable = true

    /// Action handler
    ///
    /// Screen state is stored here
    @Published var actionResult: ActionResult = ActionResult(
        screenLabel: "",
        back: false,
        footer: false,
        footerButton: .settings,
        rightButton: .none,
        screenNameType: .h4,
        screenData: ScreenData.documents,
        modalData: nil,
        alertData: .none
    )

    /// Responsible for presentation of generic error bottom sheet alert component
    /// Currently error is based on `actionResult.alertData` component when app receives `.errorData(message)` value
    @Published var genericError = GenericErrorViewModel()

    @Published var disableSwipeToBack: Bool = false

    init(
        backendActionPerformer: BackendNavigationPerforming = BackendNavigationAdapter(),
        debounceQueue: Dispatching = DispatchQueue(label: Constants.queueLabel)
    ) {
        self.backendActionPerformer = backendActionPerformer
        self.debounceQueue = debounceQueue
    }
}

extension NavigationCoordinator {
    @discardableResult
    func performFake(navigation: Navigation) -> ActionResult {
        let result = backendActionPerformer.performBackend(
            action: navigation.action,
            details: navigation.details,
            seedPhrase: navigation.seedPhrase
        )
        switch result {
        case let .success(action):
            return action
        case .failure:
            return actionResult
        }
    }

    func performTransaction(with payload: String) -> Result<ActionResult, TransactionError> {
        backendActionPerformer.performTransaction(with: payload)
    }

    @discardableResult
    func perform(navigation: Navigation, skipDebounce: Bool = false) -> ActionResult {
        guard isActionAvailable else { return actionResult }
        defer { handleDebounce(skipDebounce) }

        isActionAvailable = false

        let result = backendActionPerformer.performBackend(
            action: navigation.action,
            details: navigation.details,
            seedPhrase: navigation.seedPhrase
        )
        switch result {
        case let .success(actionResult):
            updateIntermediateDataModels(actionResult)
            updateGlobalViews(actionResult)
            self.actionResult = actionResult
        case let .failure(error):
            genericError.errorMessage = error.description
            genericError.isPresented = true
        }
        return actionResult
    }
}

private extension NavigationCoordinator {
    func updateIntermediateDataModels(_: ActionResult) {}

    func updateGlobalViews(_ actionResult: ActionResult) {
        if case let .errorData(message) = actionResult.alertData {
            genericError.errorMessage = message
            genericError.isPresented = true
        }
    }

    func handleDebounce(_ skipDebounce: Bool) {
        guard !isActionAvailable else { return }
        if skipDebounce {
            isActionAvailable = true
        } else {
            debounceQueue.asyncAfter(deadline: .now() + Constants.debounceTime, flags: .barrier) {
                self.isActionAvailable = true
            }
        }
    }
}
