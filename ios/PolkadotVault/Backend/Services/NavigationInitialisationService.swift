//
//  NavigationInitialisationService.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 06/03/2023.
//

import Foundation

final class NavigationInitialisationService {
    private let seedsMediator: SeedsMediating
    private let databaseMediator: DatabaseMediating
    private let backendService: BackendService

    init(
        databaseMediator: DatabaseMediating = DatabaseMediator(),
        backendService: BackendService = BackendService(),
        seedsMediator: SeedsMediating = ServiceLocator.seedsMediator
    ) {
        self.databaseMediator = databaseMediator
        self.backendService = backendService
        self.seedsMediator = seedsMediator
    }

    func initialiseNavigation(verifierRemoved: Bool, completion: @escaping (Result<Void, ServiceError>) -> Void) {
        backendService.performCall({
            try initNavigation(
                dbname: self.databaseMediator.databaseName,
                seedNames: self.seedsMediator.seedNames
            )
            if verifierRemoved {
                try historyInitHistoryNoCert()
            } else {
                try historyInitHistoryWithCert()
            }
        }, completion: completion)
    }
}
