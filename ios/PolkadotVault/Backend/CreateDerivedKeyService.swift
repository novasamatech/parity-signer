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

final class CreateDerivedKeyService {
    private let databaseMediator: DatabaseMediating
    private let callQueue: Dispatching
    private let callbackQueue: Dispatching
    private let seedsMediator: SeedsMediating
    private let navigation: NavigationCoordinator

    init(
        databaseMediator: DatabaseMediating = DatabaseMediator(),
        seedsMediator: SeedsMediating = ServiceLocator.seedsMediator,
        callQueue: Dispatching = DispatchQueue.global(qos: .userInteractive),
        callbackQueue: Dispatching = DispatchQueue.main,
        navigation: NavigationCoordinator = NavigationCoordinator()
    ) {
        self.databaseMediator = databaseMediator
        self.seedsMediator = seedsMediator
        self.callQueue = callQueue
        self.callbackQueue = callbackQueue
        self.navigation = navigation
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
        callQueue.async {
            let result: Result<Void, CreateDerivedKeyError>
            pathAndNetworks.forEach {
                do {
                    try tryCreateAddress(
                        seedName: seedName,
                        seedPhrase: seedPhrase,
                        path: $0.path,
                        network: $0.network.key
                    )
                } catch let displayedError as ErrorDisplayed {
                    occuredErrors.append((network: $0.network, error: displayedError.localizedDescription))
                } catch {
                    occuredErrors.append((network: $0.network, error: error.localizedDescription))
                }
            }
            if occuredErrors.isEmpty {
                result = .success(())
            } else if occuredErrors.count == pathAndNetworks.count {
                result = .failure(.noKeysCreates(errors: occuredErrors.map(\.error)))
            } else {
                result = .failure(.keysNotCreated(occuredErrors))
            }
            self.callbackQueue.async {
                completion(result)
            }
        }
    }

    func createDerivedKey(
        _ seedName: String,
        _ path: String,
        _ network: String,
        _ completion: @escaping (Result<Void, Error>) -> Void
    ) {
        callQueue.async {
            let result: Result<Void, Error>
            let seedPhrase = self.seedsMediator.getSeed(seedName: seedName)
            do {
                try tryCreateAddress(
                    seedName: seedName,
                    seedPhrase: seedPhrase,
                    path: path,
                    network: network
                )
                result = .success(())
            } catch {
                result = .failure(error)
            }
            self.callbackQueue.async {
                completion(result)
            }
        }
    }

    func createDerivedKeyForKeySets(
        _ seedNames: [String],
        _ networkName: String,
        _ completion: @escaping (Result<Void, CreateDerivedKeyForKeySetsError>) -> Void
    ) {
        callQueue.async {
            let result: Result<Void, CreateDerivedKeyForKeySetsError>
            let seeds = self.seedsMediator.getSeeds(seedNames: Set(seedNames))
            var occuredErrors: [(seedName: String, error: String)] = []

            guard let network = try? getManagedNetworks().networks
                .first(where: { $0.title == networkName }) else {
                result = .failure(.noNetwork)
                return
            }
            seeds.forEach {
                do {
                    try tryCreateAddress(
                        seedName: $0.key,
                        seedPhrase: $0.value,
                        path: network.pathId,
                        network: network.key
                    )
                } catch let displayedError as ErrorDisplayed {
                    occuredErrors.append((seedName: $0.key, error: displayedError.localizedDescription))
                } catch {
                    occuredErrors.append((seedName: $0.key, error: error.localizedDescription))
                }
            }
            if occuredErrors.isEmpty {
                result = .success(())
            } else {
                result = .failure(.keysNotCreated(occuredErrors))
            }
            self.callbackQueue.async {
                completion(result)
            }
        }
    }

    func createDerivedKeys(
        _ seedName: String,
        _ seedPhrase: String,
        keysToImport: [DdDetail],
        completion: @escaping (Result<Void, ImportDerivedKeyError>) -> Void
    ) {
        var occuredErrors: [(key: String, error: String)] = []
        callQueue.async {
            let result: Result<Void, ImportDerivedKeyError>
            keysToImport.forEach {
                do {
                    try tryCreateAddress(
                        seedName: seedName,
                        seedPhrase: seedPhrase,
                        path: $0.path,
                        network: $0.networkSpecsKey
                    )
                } catch {
                    occuredErrors.append((key: $0.path, error: error.backendDisplayError))
                }
            }
            if occuredErrors.isEmpty {
                result = .success(())
            } else if occuredErrors.count == keysToImport.count {
                result = .failure(.noKeysImported(errors: occuredErrors.map(\.error)))
            } else {
                result = .failure(.keyNotImported(occuredErrors))
            }
            self.callbackQueue.async {
                completion(result)
            }
        }
    }

    func checkForCollision(
        _ seedName: String,
        _ path: String,
        _ network: String
    ) -> Result<DerivationCheck, ServiceError> {
        do {
            return try .success(substratePathCheck(seedName: seedName, path: path, network: network))
        } catch {
            return .failure(.init(message: error.backendDisplayError))
        }
    }

    func resetNavigationState(_ keyName: String) {
        navigation.performFake(navigation: .init(action: .start))
        navigation.performFake(navigation: .init(action: .navbarKeys))
        navigation.performFake(navigation: .init(action: .selectSeed, details: keyName))
        navigation.performFake(navigation: .init(action: .newKey))
        navigation.performFake(navigation: .init(action: .goBack))
    }
}
