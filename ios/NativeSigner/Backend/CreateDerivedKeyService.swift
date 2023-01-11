//
//  CreateDerivedKeyService.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 10/01/2023.
//

import Foundation

final class CreateDerivedKeyService {
    private let databaseMediator: DatabaseMediating
    private let callQueue: Dispatching
    private let callbackQueue: Dispatching
    private let seedsMediator: SeedsMediating

    init(
        databaseMediator: DatabaseMediating = DatabaseMediator(),
        seedsMediator: SeedsMediating = ServiceLocator.seedsMediator,
        callQueue: Dispatching = DispatchQueue(label: "CreateDerivedKeyService", qos: .userInitiated),
        callbackQueue: Dispatching = DispatchQueue.main
    ) {
        self.databaseMediator = databaseMediator
        self.seedsMediator = seedsMediator
        self.callQueue = callQueue
        self.callbackQueue = callbackQueue
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
                    network: network,
                    dbname: self.databaseMediator.databaseName
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
                try getAllNetworks(
                    dbname: self.databaseMediator.databaseName
                )
                .forEach {
                    try tryCreateAddress(
                        seedName: seedName,
                        seedPhrase: seedPhrase,
                        path: path,
                        network: $0.key,
                        dbname: self.databaseMediator.databaseName
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
}
