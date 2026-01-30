//
//  CreateDerivedKeyService.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 10/01/2023.
//

import Foundation

enum CreateDerivedKeyError: Error {
    case noKeysCreates(errors: [String])
    case keysNotCreated([(network: MmNetwork, error: String)])

    var localizedDescription: String {
        switch self {
        case let .keysNotCreated(networksWithErrors):
            networksWithErrors.reduce(into: "") {
                $0 += (
                    Localizable.CreateKeysForNetwork.Error
                        .derivedKeyForNetwork($1.network.title, $1.error) + "\n"
                )
            }
        case let .noKeysCreates(errors):
            errors.reduce(into: "") { $0 += ($1 + "\n") }
        }
    }
}

enum CreateDerivedKeyForKeySetsError: Error {
    case noNetwork
    case keysNotCreated([(seedName: String, error: String)])

    var localizedDescription: String {
        switch self {
        case let .keysNotCreated(seedNamesWithErrors):
            seedNamesWithErrors.reduce(into: "") {
                $0 += (
                    Localizable.SelectKeySetsForNetworkKey.Error
                        .derivedKeyForNetwork($1.seedName, $1.error) + "\n"
                )
            }
        case .noNetwork:
            Localizable.SelectKeySetsForNetworkKey.Error.noNetwork.string
        }
    }
}

enum ImportDerivedKeyError: Error {
    case noKeysImported(errors: [String])
    case keyNotImported([(key: String, error: String)])

    var localizedDescription: String {
        switch self {
        case let .keyNotImported(errorInfo):
            errorInfo.reduce(into: "") {
                $0 += (
                    Localizable.AddDerivedKeys.Error
                        .DerivedKeyForNetwork.content($1.key, $1.error) + "\n"
                )
            }
        case let .noKeysImported(errors):
            errors.reduce(into: "") { $0 += ($1 + "\n") }
        }
    }
}

// sourcery: AutoMockable
protocol CreateDerivedKeyServicing: AnyObject {
    func createDefaultDerivedKey(
        _ keySet: MKeysNew,
        _ keyName: String,
        _ network: MmNetwork,
        completion: @escaping (Result<Void, ServiceError>) -> Void
    )
    func createDerivedKeys(
        _ seedName: String,
        _ seedPhrase: String,
        networks: [MmNetwork],
        completion: @escaping (Result<Void, CreateDerivedKeyError>) -> Void
    )
    func createDerivedKey(
        _ seedName: String,
        _ path: String,
        _ network: String,
        _ completion: @escaping (Result<Void, ServiceError>) -> Void
    )
    func createDerivedKeyForKeySets(
        _ seedNames: [String],
        _ networkName: String,
        _ completion: @escaping (Result<Void, CreateDerivedKeyForKeySetsError>) -> Void
    )
    func checkForCollision(
        _ seedName: String,
        _ path: String,
        _ network: String,
        completion: @escaping (Result<DerivationCheck, ServiceError>) -> Void
    )
}

extension CreateDerivedKeyService: CreateDerivedKeyServicing {}

final class CreateDerivedKeyService {
    private let backendService: BackendService
    private let seedsMediator: SeedsMediating
    private let createKeyNameService: CreateDerivedKeyNameService

    init(
        seedsMediator: SeedsMediating = ServiceLocator.seedsMediator,
        backendService: BackendService = BackendService(),
        createKeyNameService: CreateDerivedKeyNameService = CreateDerivedKeyNameService()
    ) {
        self.seedsMediator = seedsMediator
        self.backendService = backendService
        self.createKeyNameService = createKeyNameService
    }

    func createDefaultDerivedKey(
        _ keySet: MKeysNew,
        _ keyName: String,
        _ network: MmNetwork,
        completion: @escaping (Result<Void, ServiceError>) -> Void
    ) {
        let seedPhrase = seedsMediator.getSeed(seedName: keyName)
        backendService.performCall({
            try tryCreateAddress(
                seedName: keyName,
                seedPhrase: seedPhrase,
                path: self.createKeyNameService.defaultDerivedKeyName(keySet, network: network),
                network: network.key
            )
        }, completion: completion)
    }

    func createDerivedKeys(
        _ seedName: String,
        _ seedPhrase: String,
        networks: [MmNetwork],
        completion: @escaping (Result<Void, CreateDerivedKeyError>) -> Void
    ) {
        let pathAndNetworks: [(path: String, network: MmNetwork)] = networks
            .map { (path: $0.pathId, network: $0) }
        var occuredErrors: [(network: MmNetwork, error: String)] = []

        pathAndNetworks.forEach { pathAndNetwork in
            backendService.performCall {
                try tryCreateAddress(
                    seedName: seedName,
                    seedPhrase: seedPhrase,
                    path: pathAndNetwork.path,
                    network: pathAndNetwork.network.key
                )
            } completion: { (result: Result<Void, ErrorDisplayed>) in
                if case let .failure(displayedError) = result {
                    occuredErrors.append((network: pathAndNetwork.network, error: displayedError.localizedDescription))
                }
                if let lastElement = pathAndNetworks.last, lastElement == pathAndNetwork {
                    let result: Result<Void, CreateDerivedKeyError> =
                        if occuredErrors.isEmpty {
                            .success(())
                        } else if occuredErrors.count == pathAndNetworks.count {
                            .failure(.noKeysCreates(errors: occuredErrors.map(\.error)))
                        } else {
                            .failure(.keysNotCreated(occuredErrors))
                        }
                    completion(result)
                }
            }
        }
    }

    func createDerivedKey(
        _ seedName: String,
        _ path: String,
        _ network: String,
        _ completion: @escaping (Result<Void, ServiceError>) -> Void
    ) {
        let seedPhrase = seedsMediator.getSeed(seedName: seedName)
        backendService.performCall({
            try tryCreateAddress(
                seedName: seedName,
                seedPhrase: seedPhrase,
                path: path,
                network: network
            )
        }, completion: completion)
    }

    func createDerivedKeyForKeySets(
        _ seedNames: [String],
        _ networkName: String,
        _ completion: @escaping (Result<Void, CreateDerivedKeyForKeySetsError>) -> Void
    ) {
        let seeds = seedsMediator.getSeeds(seedNames: Set(seedNames))
        var occuredErrors: [(seedName: String, error: String)] = []
        let seedKeys = Array(seeds.keys)
        guard let network = try? getManagedNetworks().networks
            .first(where: { $0.title == networkName }) else {
            completion(.failure(.noNetwork))
            return
        }
        seedKeys.forEach { seedKey in
            backendService.performCall {
                try tryCreateAddress(
                    seedName: seedKey,
                    seedPhrase: seeds[seedKey] ?? "",
                    path: network.pathId,
                    network: network.key
                )
            } completion: { (result: Result<Void, ErrorDisplayed>) in
                if case let .failure(displayedError) = result {
                    occuredErrors.append((seedName: seedKey, error: displayedError.localizedDescription))
                }
                if let lastElement = seedKeys.last, lastElement == seedKey {
                    let result: Result<Void, CreateDerivedKeyForKeySetsError> =
                        if occuredErrors.isEmpty {
                            .success(())
                        } else {
                            .failure(.keysNotCreated(occuredErrors))
                        }
                    completion(result)
                }
            }
        }
    }

    func checkForCollision(
        _ seedName: String,
        _ path: String,
        _ network: String,
        completion: @escaping (Result<DerivationCheck, ServiceError>) -> Void
    ) {
        backendService.performCall({
            try substratePathCheck(seedName: seedName, path: path, network: network)
        }, completion: completion)
    }
}
