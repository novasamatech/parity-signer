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
    /// This should preceed information from `ActionResult.footerButton` as `FooterButton` enum contains also `back` value which is irrelevant to bottom navigation system that mimics system `TabView`
    /// This should be removed once navigation is moved to native system.
    @Published var selectedTab: Tab = .keys

    init(
        backendActionPerformer: BackendNavigationPerforming = BackendNavigationAdapter(),
        debounceQueue: Dispatching = DispatchQueue(label: Constants.queueLabel)
    ) {
        self.backendActionPerformer = backendActionPerformer
        self.debounceQueue = debounceQueue
    }
}

extension NavigationCoordinator {
    func perform(navigation: Navigation) {
        guard isActionAvailable else { return }

        isActionAvailable = false

        if let actionResult = backendActionPerformer.performBackend(
            action: navigation.action,
            details: navigation.details,
            seedPhrase: navigation.seedPhrase
        ) {
            self.actionResult = actionResult
            if let tab = Tab(actionResult.footerButton), tab != selectedTab {
                selectedTab = tab
            }
        }

        debounceQueue.asyncAfter(deadline: .now() + Constants.debounceTime, flags: .barrier) {
            self.isActionAvailable = true
        }
    }
}
