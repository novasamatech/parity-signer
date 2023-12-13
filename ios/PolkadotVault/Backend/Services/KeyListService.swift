//
//  KeyListService.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 14/04/2023.
//

import Foundation

// sourcery: AutoMockable
protocol KeyListServicing: AnyObject {
    func getKeyList(
        _ completion: @escaping (Result<MSeeds, ServiceError>) -> Void
    )
}

extension KeyListService: KeyListServicing {}

final class KeyListService {
    private let backendService: BackendService
    private let seedsMediator: SeedsMediating

    init(
        backendService: BackendService = BackendService(),
        seedsMediator: SeedsMediating = ServiceLocator.seedsMediator
    ) {
        self.backendService = backendService
        self.seedsMediator = seedsMediator
    }

    func getKeyList(
        _ completion: @escaping (Result<MSeeds, ServiceError>) -> Void
    ) {
        backendService.performCall({
            try getSeeds(namesPhoneKnows: self.seedsMediator.seedNames)
        }, completion: completion)
    }
}
