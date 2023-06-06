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
            return networksWithErrors.reduce(into: "") {
                $0 += (
                    Localizable.CreateKeysForNetwork.Error
                        .derivedKeyForNetwork($1.network.title, $1.error) + "\n"
                )
            }
        case let .noKeysCreates(errors):
            return errors.reduce(into: "") { $0 += ($1 + "\n") }
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
        callQueue: Dispatching = DispatchQueue(label: "CreateDerivedKeyService", qos: .userInitiated),
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
        networks: [MmNetwork],
        completion: @escaping (Result<Void, CreateDerivedKeyError>) -> Void
    ) {
        let pathAndNetworks: [(path: String, network: MmNetwork)] = networks
            .map { (path: "//\($0.title)", network: $0) }
        var occuredErrors: [(network: MmNetwork, error: String)] = []
        callQueue.async {
            let result: Result<Void, CreateDerivedKeyError>
            let seedPhrase = self.seedsMediator.getSeed(seedName: seedName)
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

    func createDerivedKeyOnAllNetworks(
        _ seedName: String,
        _ path: String,
        _ completion: @escaping (Result<Void, Error>) -> Void
    ) {
        callQueue.async {
            let result: Result<Void, Error>
            let seedPhrase = self.seedsMediator.getSeed(seedName: seedName)
            do {
                try getAllNetworks()
                    .forEach {
                        try tryCreateAddress(
                            seedName: seedName,
                            seedPhrase: seedPhrase,
                            path: path,
                            network: $0.key
                        )
                    }
                result = .success(())
            } catch {
                result = .failure(error)
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
            return .failure(.init(message: error.localizedDescription))
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
