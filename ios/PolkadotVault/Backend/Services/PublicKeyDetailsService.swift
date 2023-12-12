//
//  PublicKeyDetailsService.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 26/04/2023.
//

import Foundation

// sourcery: AutoMockable
protocol PublicKeyDetailsServicing: AnyObject {
    func forgetSingleKey(
        address: String,
        networkSpecsKey: String,
        _ completion: @escaping (Result<Void, ServiceError>) -> Void
    )
}

extension PublicKeyDetailsService: PublicKeyDetailsServicing {}

final class PublicKeyDetailsService {
    private let backendService: BackendService

    init(
        backendService: BackendService = BackendService()
    ) {
        self.backendService = backendService
    }

    func forgetSingleKey(
        address: String,
        networkSpecsKey: String,
        _ completion: @escaping (Result<Void, ServiceError>) -> Void
    ) {
        backendService.performCall({
            try removeDerivedKey(address: address, networkSpecsKey: networkSpecsKey)
        }, completion: completion)
    }
}
