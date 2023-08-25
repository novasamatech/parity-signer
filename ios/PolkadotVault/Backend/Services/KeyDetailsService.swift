//
//  KeyDetailsService.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 28/10/2022.
//

import Foundation

final class KeyDetailsService {
    private let backendService: BackendService

    init(
        backendService: BackendService = BackendService()
    ) {
        self.backendService = backendService
    }

    func getKeys(
        for seedName: String,
        _ completion: @escaping (Result<MKeysNew, ServiceError>) -> Void
    ) {
        backendService.performCall({
            try keysBySeedName(seedName: seedName)
        }, completion: completion)
    }
}
