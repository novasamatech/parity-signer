//
//  GetManagedNetworksService.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 10/01/2023.
//

import Foundation

// sourcery: AutoMockable
protocol GetManagedNetworksServicing: AnyObject {
    func getNetworks(
        _ completion: @escaping (Result<[MmNetwork], ServiceError>) -> Void
    )
}

final class GetManagedNetworksService {
    private let backendService: BackendService

    init(
        backendService: BackendService = BackendService()
    ) {
        self.backendService = backendService
    }

    func getNetworks(
        _ completion: @escaping (Result<[MmNetwork], ServiceError>) -> Void
    ) {
        backendService.performCall({
            try getManagedNetworks().networks
        }, completion: completion)
    }
}
