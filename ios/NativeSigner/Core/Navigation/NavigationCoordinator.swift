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
    }

    private let debounceQueue = DispatchQueue(label: "navigationCoordinator.debounce")
    private var isActionAvailable = true

    /// Action handler
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
}

extension NavigationCoordinator {
    func perform(navigation: Navigation) {
        guard isActionAvailable else { return }

        isActionAvailable = false

        if let tempActionResult = try? backendAction(
            action: navigation.action,
            details: navigation.details,
            seedPhrase: navigation.seedPhrase
        ) {
            if case let .sufficientCryptoReady(value) = tempActionResult.modalData {
                print(value)
            }
            actionResult = tempActionResult
        }

        debounceQueue.asyncAfter(deadline: .now() + Constants.debounceTime, flags: .barrier) {
            self.isActionAvailable = true
        }
    }
}
