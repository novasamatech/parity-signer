//
//  KeyDetailsActionService.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 24/04/2023.
//

import Foundation

final class KeyDetailsActionService {
    private let backendService: BackendService

    init(
        backendService: BackendService = BackendService()
    ) {
        self.backendService = backendService
    }

    func performBackupSeed(
        seedName: String,
        _ completion: @escaping (Result<Void, ServiceError>) -> Void
    ) {
        backendService.performCall({
            try historySeedWasShown(seedName: seedName)
        }, completion: completion)
    }

    func publicKey(
        addressKey: String,
        networkSpecsKey: String,
        _ completion: @escaping (Result<MKeyDetails, ServiceError>) -> Void
    ) {
        backendService.performCall({
            try getKeySetPublicKey(address: addressKey, networkSpecsKey: networkSpecsKey)
        }, completion: completion)
    }

    func forgetKeySet(
        seedName: String,
        _ completion: @escaping (Result<Void, ServiceError>) -> Void
    ) {
        backendService.performCall({
            try PolkadotVault.removeKeySet(addressKey: seedName)
        }, completion: completion)
    }
}
