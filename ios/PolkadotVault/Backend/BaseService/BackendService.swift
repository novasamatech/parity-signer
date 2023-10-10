//
//  BackendService.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 24/08/2023.
//

import Foundation

/// A class defining the behavior of a backend service.
final class BackendService {
    /// The dispatch queue for performing the service call.
    let callQueue: Dispatching
    /// The dispatch queue for handling the completion callback.
    let callbackQueue: Dispatching

    /// Initializes an instance of `BackendService`.
    ///
    /// - Parameters:
    ///   - callQueue: The dispatch queue for performing the service call.
    ///   - callbackQueue: The dispatch queue for handling the completion callback.
    init(
        callQueue: Dispatching = DispatchQueue.global(qos: .userInteractive),
        callbackQueue: Dispatching = DispatchQueue.main
    ) {
        self.callQueue = callQueue
        self.callbackQueue = callbackQueue
    }

    /// Performs a backend service call and process error.
    ///
    /// This method bridges communication between Swift and Rust backend codebases
    /// by using the provided closure to execute Rust-specific logic.
    ///
    /// - Parameters:
    ///   - call: A closure encapsulating the logic of the Rust backend call.
    ///   - completion: A closure to be called when the service call completes.
    func performCall<Success>(
        _ call: @escaping () throws -> some Any,
        completion: @escaping (Result<Success, ServiceError>) -> Void
    ) {
        callQueue.async {
            var result: Result<Success, ServiceError>
            do {
                let successValue = try call()
                if let mappedSuccess = successValue as? Success {
                    result = .success(mappedSuccess)
                } else {
                    result = .failure(.init(message: Localizable.ErrorDisplayed.invalidType.string))
                }
            } catch {
                result = .failure(.init(message: error.backendDisplayError))
            }
            self.callbackQueue.async {
                completion(result)
            }
        }
    }

    /// Performs a backend service call.
    ///
    /// This method bridges communication between Swift and Rust backend codebases
    /// by using the provided closure to execute Rust-specific logic.
    ///
    /// - Parameters:
    ///   - call: A closure encapsulating the logic of the Rust backend call.
    ///   - completion: A closure to be called when the service call completes.
    func performCall<Success>(
        _ call: @escaping () throws -> some Any,
        completion: @escaping (Result<Success, ErrorDisplayed>) -> Void
    ) {
        callQueue.async {
            var result: Result<Success, ErrorDisplayed>
            do {
                let successValue = try call()
                if let mappedSuccess = successValue as? Success {
                    result = .success(mappedSuccess)
                } else {
                    result = .failure(ErrorDisplayed.Str(s: Localizable.ErrorDisplayed.invalidType.string))
                }
            } catch let displayedError as ErrorDisplayed {
                result = .failure(displayedError)
            } catch {
                result = .failure(ErrorDisplayed.Str(s: error.localizedDescription))
            }
            self.callbackQueue.async {
                completion(result)
            }
        }
    }
}
