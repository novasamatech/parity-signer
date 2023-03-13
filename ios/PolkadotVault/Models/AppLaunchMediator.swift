//
//  AppLaunchMediator.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 13/03/2023.
//

import Foundation

protocol AppLaunchMediating: AnyObject {
    func onboard(verifierRemoved: Bool)
    func initialiseFirstRun()
    func initialiseOnboardedUserRun()
}

final class AppLaunchMediator: AppLaunchMediating {
    private let navigationInitialisationService: NavigationInitialisationService
    private let seedsMediator: SeedsMediating
    private let databaseMediator: DatabaseMediating

    init(
        navigationInitialisationService: NavigationInitialisationService = NavigationInitialisationService(),
        seedsMediator: SeedsMediating = ServiceLocator.seedsMediator,
        databaseMediator: DatabaseMediating = DatabaseMediator()
    ) {
        self.navigationInitialisationService = navigationInitialisationService
        self.seedsMediator = seedsMediator
        self.databaseMediator = databaseMediator
    }

    func onboard(verifierRemoved: Bool) {
        seedsMediator.refreshSeeds()
        localDataCleanup()
        databaseMediator.recreateDatabaseFile()
        navigationInitialisationService.initialiseNavigation(verifierRemoved: verifierRemoved)
        seedsMediator.refreshSeeds()
    }

    func initialiseFirstRun() {
        localDataCleanup()
    }

    func initialiseOnboardedUserRun() {
        seedsMediator.initialRefreshSeeds()
        do {
            try initNavigation(dbname: databaseMediator.databaseName, seedNames: seedsMediator.seedNames)
        } catch {}
    }
}

private extension AppLaunchMediator {
    func localDataCleanup() {
        seedsMediator.removeAllSeeds()
        databaseMediator.wipeDatabase()
    }
}
