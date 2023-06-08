//
//  CreateKeySetService.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 14/04/2023.
//

import Foundation

final class CreateKeySetService {
    private let callQueue: Dispatching
    private let callbackQueue: Dispatching

    init(
        callQueue: Dispatching = DispatchQueue(label: "CreateKeySetService", qos: .userInitiated),
        callbackQueue: Dispatching = DispatchQueue.main
    ) {
        self.callQueue = callQueue
        self.callbackQueue = callbackQueue
    }

    func createKeySet(
        seedName: String,
        _ completion: @escaping (Result<MNewSeedBackup, Error>) -> Void
    ) {
        callQueue.async {
            let result: Result<MNewSeedBackup, Error>
            do {
                let seedBackup = try printNewSeed(newSeedName: seedName)
                result = .success(seedBackup)
            } catch {
                result = .failure(error)
            }
            self.callbackQueue.async {
                completion(result)
            }
        }
    }

    func confirmKeySetCreation(
        seedName: String,
        seedPhrase: String,
        _ completion: @escaping (Result<Void, Error>) -> Void
    ) {
        callQueue.async {
            let result: Result<Void, Error>
            do {
                try PolkadotVault.createKeySet(seedName: seedName, seedPhrase: seedPhrase, networks: [])
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
