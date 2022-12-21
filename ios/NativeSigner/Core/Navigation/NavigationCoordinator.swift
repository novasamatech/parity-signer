//
//  NavigationCoordinator.swift
//  NativeSigner
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

    /// Stores view model of currently selected tab
    ///
    /// This should preceed information from `ActionResult.footerButton` as `FooterButton` enum contains also `back`
    /// value which is irrelevant to bottom navigation system that mimics system `TabView`
    /// This should be removed once navigation is moved to native system.
    @Published var selectedTab: Tab = .keys

    /// Responsible for presentation of generic error bottom sheet alert component
    /// Currently error is based on `actionResult.alertData` component when app receives `.errorData(message)` value
    @Published var genericError = GenericErrorViewModel()

    @Published var shouldPresentQRScanner = false

    /// Enables to override old logic based on `ActionResult` to not include additional components in main view
    /// hierarchy
    /// for screens with updated design approach.
    ///
    /// This will enable to slowly move into proper view hierachy in newer screens and then update navigation
    var shouldSkipInjectedViews: Bool = false

    /// Stores currently selected Key Set Details
    ///
    /// This is a temporary fix that should be removed after introduction of Rust API
    @Published var currentKeyDetails: MKeyDetails!

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

    func perform(navigation: Navigation, skipDebounce: Bool = false) {
        guard isActionAvailable else { return }
        defer { handleDebounce(skipDebounce) }

        isActionAvailable = false

        let result = backendActionPerformer.performBackend(
            action: navigation.action,
            details: navigation.details,
            seedPhrase: navigation.seedPhrase
        )
        switch result {
        case let .success(actionResult):
            updateIntermediateNavigation(actionResult)
            updateIntermediateDataModels(actionResult)
            updateGlobalViews(actionResult)
            self.actionResult = actionResult
            updateTabBar()
        case let .failure(error):
            genericError.errorMessage = error.description
            genericError.isPresented = true
        }
    }
}

private extension NavigationCoordinator {
    func updateIntermediateNavigation(_ actionResult: ActionResult) {
        var updatedShouldSkipInjectedViews: Bool
        switch actionResult.screenData {
        case .seedSelector, // Main `Keys` screen
             .keys, // `Key Details` screen
             .keyDetails, // `Public Key` screen
             .transaction,
             .log,
             .settings,
             .vVerifier,
             .manageNetworks:
            updatedShouldSkipInjectedViews = true
        default:
            updatedShouldSkipInjectedViews = false
        }
        if updatedShouldSkipInjectedViews != shouldSkipInjectedViews {
            shouldSkipInjectedViews = updatedShouldSkipInjectedViews
        }
    }

    func updateIntermediateDataModels(_ actionResult: ActionResult) {
        // Used temporarly in Export Private Key flow. To be removed
        if case let .keyDetails(keyDetails) = actionResult.screenData {
            currentKeyDetails = keyDetails
        }
    }

    func updateGlobalViews(_ actionResult: ActionResult) {
        if case let .errorData(message) = actionResult.alertData {
            genericError.errorMessage = message
            genericError.isPresented = true
        }
    }

    func updateTabBar() {
        guard let tab = Tab(actionResult.footerButton), tab != selectedTab else { return }
        selectedTab = tab
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
