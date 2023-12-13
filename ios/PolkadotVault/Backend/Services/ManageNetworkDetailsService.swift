//
//  ManageNetworkDetailsService.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 11/04/2023.
//

import Foundation

enum SpecSignType: Equatable {
    case metadata(metadataSpecsVersion: String)
    case network
}

enum SpecSignError: Error {
    case wrongPassword
    case error(ServiceError)
}

// sourcery: AutoMockable
protocol ManageNetworkDetailsServicing: AnyObject {
    func getNetworkDetails(
        _ networkKey: String,
        _ completion: @escaping (Result<MNetworkDetails, ServiceError>) -> Void
    )
    func signSpecList(
        _ completion: @escaping (Result<MSignSufficientCrypto, ServiceError>) -> Void
    )
    func signSpec(
        _ type: SpecSignType,
        _ networkKey: String,
        signingAddressKey: String,
        seedPhrase: String,
        password: String?,
        _ completion: @escaping (Result<MSufficientCryptoReady, SpecSignError>) -> Void
    )
    func deleteNetwork(
        _ networkKey: String,
        _ completion: @escaping (Result<Void, ServiceError>) -> Void
    )
    func deleteNetworkMetadata(
        _ networkKey: String,
        _ specsVersion: String,
        _ completion: @escaping (Result<Void, ServiceError>) -> Void
    )
}

extension ManageNetworkDetailsService: ManageNetworkDetailsServicing {}

final class ManageNetworkDetailsService {
    private let backendService: BackendService
    private let callQueue: Dispatching
    private let callbackQueue: Dispatching

    init(
        backendService: BackendService = BackendService(),
        callQueue: Dispatching = DispatchQueue.global(qos: .userInteractive),
        callbackQueue: Dispatching = DispatchQueue.main
    ) {
        self.backendService = backendService
        self.callQueue = callQueue
        self.callbackQueue = callbackQueue
    }

    func getNetworkDetails(
        _ networkKey: String,
        _ completion: @escaping (Result<MNetworkDetails, ServiceError>) -> Void
    ) {
        backendService.performCall({
            try getManagedNetworkDetails(networkKey: networkKey)
        }, completion: completion)
    }

    func signSpecList(
        _ completion: @escaping (Result<MSignSufficientCrypto, ServiceError>) -> Void
    ) {
        backendService.performCall({
            try getKeysForSigning()
        }, completion: completion)
    }

    func signSpec(
        _ type: SpecSignType,
        _ networkKey: String,
        signingAddressKey: String,
        seedPhrase: String,
        password: String?,
        _ completion: @escaping (Result<MSufficientCryptoReady, SpecSignError>) -> Void
    ) {
        switch type {
        case let .metadata(specsVersion):
            signMetadataSpecList(
                networkKey,
                specsVersion,
                signingAddressKey: signingAddressKey,
                seedPhrase: seedPhrase,
                password: password,
                completion
            )
        case .network:
            signNetworkSpec(
                networkKey,
                signingAddressKey: signingAddressKey,
                seedPhrase: seedPhrase,
                password: password,
                completion
            )
        }
    }

    func deleteNetwork(
        _ networkKey: String,
        _ completion: @escaping (Result<Void, ServiceError>) -> Void
    ) {
        backendService.performCall({
            try removeManagedNetwork(networkKey: networkKey)
        }, completion: completion)
    }

    func deleteNetworkMetadata(
        _ networkKey: String,
        _ specsVersion: String,
        _ completion: @escaping (Result<Void, ServiceError>) -> Void
    ) {
        backendService.performCall({
            try removeMetadataOnManagedNetwork(networkKey: networkKey, metadataSpecsVersion: specsVersion)
        }, completion: completion)
    }
}

private extension ManageNetworkDetailsService {
    func signMetadataSpecList(
        _ networkKey: String,
        _ specsVersion: String,
        signingAddressKey: String,
        seedPhrase: String,
        password: String?,
        _ completion: @escaping (Result<MSufficientCryptoReady, SpecSignError>) -> Void
    ) {
        signSpec({
            try signMetadataWithKey(
                networkKey: networkKey,
                metadataSpecsVersion: specsVersion,
                signingAddressKey: signingAddressKey,
                seedPhrase: seedPhrase,
                password: password
            )
        }, completion: completion)
    }

    func signNetworkSpec(
        _ networkKey: String,
        signingAddressKey: String,
        seedPhrase: String,
        password: String?,
        _ completion: @escaping (Result<MSufficientCryptoReady, SpecSignError>) -> Void
    ) {
        signSpec({
            try signNetworkSpecWithKey(
                networkKey: networkKey,
                signingAddressKey: signingAddressKey,
                seedPhrase: seedPhrase,
                password: password
            )
        }, completion: completion)
    }

    func signSpec(
        _ call: @escaping () throws -> MSufficientCryptoReady,
        completion: @escaping (Result<MSufficientCryptoReady, SpecSignError>) -> Void
    ) {
        callQueue.async {
            var result: Result<MSufficientCryptoReady, SpecSignError>
            do {
                let successValue = try call()
                result = .success(successValue)
            } catch let displayedError as ErrorDisplayed {
                if case .WrongPassword = displayedError {
                    result = .failure(.wrongPassword)
                } else {
                    result = .failure(.error(.init(message: displayedError.backendDisplayError)))
                }
            } catch {
                result = .failure(.error(.init(message: error.localizedDescription)))
            }
            self.callbackQueue.async {
                completion(result)
            }
        }
    }
}
