//
//  BackendNavigationAdapter.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 05/08/2022.
//

import Foundation

struct NavigationError: Error, CustomStringConvertible {
    let message: String

    var description: String {
        [Localizable.Error.Navigation.Label.prefix.string, message, Localizable.Error.Navigation.Label.suffix.string]
            .joined(separator: "\n")
    }
}

/// Protocol that reflects backend ability to navigate
protocol BackendNavigationPerforming: AnyObject {
    /// Wrapper for Rust backend navigation public function that performs navigation.
    /// Enables mocking and unit testing
    /// - Parameters:
    ///   - action: Action to be performed in Rust backend
    ///   - details: Additional data to be send along with `action`
    ///   - seedPhrase: Seed phrase required to be sent along with some `action`s
    /// - Returns: `ActionResult` if action was valid, `nil` otherwise
    @discardableResult
    func performBackend(action: Action, details: String, seedPhrase: String) -> Result<ActionResult, NavigationError>
    func performTransaction(with payload: String) -> Result<ActionResult, TransactionError>
}

/// We don't want to use module-wide public functions as there is no way of mocking them in unit  / UI tests
/// This adapters acts as a wrapper for public function for navigation
final class BackendNavigationAdapter: BackendNavigationPerforming {
    @discardableResult
    func performBackend(action: Action, details: String, seedPhrase: String) -> Result<ActionResult, NavigationError> {
        do {
            let actionResult = try backendAction(
                action: action,
                details: details,
                seedPhrase: seedPhrase
            )
            return .success(actionResult)
        } catch {
            return .failure(.init(message: Localizable.Error.Navigation.Label.message(error.localizedDescription)))
        }
    }

    func performTransaction(with payload: String) -> Result<ActionResult, TransactionError> {
        do {
            let actionResult = try backendAction(
                action: .transactionFetched,
                details: payload,
                seedPhrase: ""
            )
            return .success(actionResult)
        } catch let errorDisplayed as ErrorDisplayed {
            return .failure(errorDisplayed.transactionError)
        } catch {
            return .failure(.generic(NavigationError(
                message: Localizable.Error.Navigation.Label
                    .message(error.localizedDescription)
            ).description))
        }
    }
}
