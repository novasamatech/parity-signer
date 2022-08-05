//
//  BackendNavigationAdapter.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 05/08/2022.
//

import Foundation

/// Protocol that reflects backend ability to navigate
protocol BackendNavigationPerforming: AnyObject {
    /// Wrapper for Rust backend navigation public function that performs navigation.
    /// Enables mocking and unit testing
    /// - Parameters:
    ///   - action: Action to be performed in Rust backend
    ///   - details: Additional data to be send along with `action`
    ///   - seedPhrase: Seed phrase required to be sent along with some `action`s
    /// - Returns: `ActionResult` if action was valid, `nil` otherwise
    func performBackend(action: Action, details: String, seedPhrase: String) -> ActionResult?
}


/// We don't want to use module-wide public functions as there is no way of mocking them in unit  / UI tests
/// This adapters acts as a wrapper for public function for navigation
final class BackendNavigationAdapter: BackendNavigationPerforming {
    func performBackend(action: Action, details: String, seedPhrase: String) -> ActionResult? {
        guard let result = try? backendAction(
            action: action,
            details: details,
            seedPhrase: seedPhrase
        ) else { return nil }
        if case let .sufficientCryptoReady(value) = result.modalData {
            print(value)
        }
        return result
    }
}
