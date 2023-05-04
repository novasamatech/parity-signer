//
//  CreateDerivedKeyService.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 10/01/2023.
//

import Foundation

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
