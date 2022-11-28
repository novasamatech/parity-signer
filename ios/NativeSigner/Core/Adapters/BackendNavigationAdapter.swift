//
//  BackendNavigationAdapter.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 05/08/2022.
//

import Foundation

enum NavigationError: Error, CustomStringConvertible {
    case general(String)
    case unknownNetwork(genesisHash: H256, encryption: Encryption)
    case invalidNetworkVersion(asDecoded: String, inMetadata: UInt32)

    /// This is temporary debug error message, until `Scan` redesigned is finished
    var message: String {
        switch self {
        case let .general(message):
            return message
        case let .unknownNetwork(genesisHash, encryption):
            return "Unknown network.\nGenesis hash:\(genesisHash.formattedAsString)\nEncryption:\(encryption.rawValue)"
        case let .invalidNetworkVersion(asDecoded, inMetadata):
            return "Invalid network version.\nDecoded as:\(asDecoded)\nIn Metadata:\(String(inMetadata))"
        }
    }

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
    func performBackend(action: Action, details: String, seedPhrase: String) -> Result<ActionResult, NavigationError>
}

/// We don't want to use module-wide public functions as there is no way of mocking them in unit  / UI tests
/// This adapters acts as a wrapper for public function for navigation
final class BackendNavigationAdapter: BackendNavigationPerforming {
    func performBackend(action: Action, details: String, seedPhrase: String) -> Result<ActionResult, NavigationError> {
        do {
            let actionResult = try backendAction(
                action: action,
                details: details,
                seedPhrase: seedPhrase
            )
            return .success(actionResult)
        } catch let ErrorDisplayed.Str(details) {
            return .failure(.general(details))
        } catch let ErrorDisplayed.UnknownNetwork(genesisHash, encryption) {
            return .failure(.unknownNetwork(genesisHash: genesisHash, encryption: encryption))
        } catch let ErrorDisplayed.WrongNetworkVersion(asDecoded, inMetadata) {
            return .failure(.invalidNetworkVersion(asDecoded: asDecoded, inMetadata: inMetadata))
        } catch {
            return .failure(.general(Localizable.Error.Navigation.Label.message(error.localizedDescription)))
        }
    }
}
