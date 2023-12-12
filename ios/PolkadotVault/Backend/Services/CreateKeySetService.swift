//
//  CreateKeySetService.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 14/04/2023.
//

import Foundation

// sourcery: AutoMockable
protocol CreateKeySetServicing: AnyObject {
    func createKeySet(
        seedName: String,
        _ completion: @escaping (Result<MNewSeedBackup, ServiceError>) -> Void
    )
    func confirmKeySetCreation(
        seedName: String,
        seedPhrase: String,
        networks: [MmNetwork],
        _ completion: @escaping (Result<Void, ServiceError>) -> Void
    )
}

extension CreateKeySetService: CreateKeySetServicing {}

final class CreateKeySetService {
    private let backendService: BackendService

    init(
        backendService: BackendService = BackendService()
    ) {
        self.backendService = backendService
    }

    func createKeySet(
        seedName: String,
        _ completion: @escaping (Result<MNewSeedBackup, ServiceError>) -> Void
    ) {
        backendService.performCall({
            try printNewSeed(newSeedName: seedName)
        }, completion: completion)
    }

    func confirmKeySetCreation(
        seedName: String,
        seedPhrase: String,
        networks: [MmNetwork],
        _ completion: @escaping (Result<Void, ServiceError>) -> Void
    ) {
        backendService.performCall({
            try PolkadotVault.createKeySet(
                seedName: seedName,
                seedPhrase: seedPhrase,
                networks: networks.map(\.key)
            )
        }, completion: completion)
    }
}
