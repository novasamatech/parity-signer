//
//  AppLaunchMediator.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 13/03/2023.
//

import Foundation

protocol AppLaunchMediating: AnyObject {
    func finaliseInitialisation(_ completion: @escaping (Result<Void, ServiceError>) -> Void)
}

final class AppLaunchMediator: ObservableObject, AppLaunchMediating {
    private let seedsMediator: SeedsMediating
    private let databaseMediator: DatabaseMediating
    private let backendService: BackendService

    init(
        seedsMediator: SeedsMediating = ServiceLocator.seedsMediator,
        databaseMediator: DatabaseMediating = DatabaseMediator(),
        backendService: BackendService = BackendService()
    ) {
        self.seedsMediator = seedsMediator
        self.databaseMediator = databaseMediator
        self.backendService = backendService
    }

    func finaliseInitialisation(_ completion: @escaping (Result<Void, ServiceError>) -> Void) {
        if databaseMediator.isDatabaseAvailable() {
            initialiseOnboardedUserRun(completion)
        } else {
            initialiseFirstRun(completion)
        }
    }

    private func initialiseFirstRun(_ completion: @escaping (Result<Void, ServiceError>) -> Void) {
        seedsMediator.removeStalledSeeds()
        databaseMediator.wipeDatabase()
        completion(.success(()))
    }

    private func initialiseOnboardedUserRun(_ completion: @escaping (Result<Void, ServiceError>) -> Void) {
        seedsMediator.refreshSeeds()
        backendService.performCall({
            try initNavigation(dbname: self.databaseMediator.databaseName, seedNames: self.seedsMediator.seedNames)
        }, completion: completion)
    }
}
