//
//  AppLaunchMediator.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 13/03/2023.
//

import Foundation

protocol AppLaunchMediating: AnyObject {
    func finaliseInitialisation()
}

final class AppLaunchMediator: ObservableObject, AppLaunchMediating {
    private let seedsMediator: SeedsMediating
    private let databaseMediator: DatabaseMediating
    private let onboardingMediator: OnboardingMediator

    init(
        seedsMediator: SeedsMediating = ServiceLocator.seedsMediator,
        databaseMediator: DatabaseMediating = DatabaseMediator(),
        onboardingMediator: OnboardingMediator = ServiceLocator.onboardingMediator
    ) {
        self.seedsMediator = seedsMediator
        self.databaseMediator = databaseMediator
        self.onboardingMediator = onboardingMediator
    }

    func finaliseInitialisation() {
        if onboardingMediator.onboardingDone {
            initialiseOnboardedUserRun()
        } else {
            initialiseFirstRun()
        }
    }

    private func initialiseFirstRun() {
        seedsMediator.removeStalledSeeds()
        databaseMediator.wipeDatabase()
    }

    private func initialiseOnboardedUserRun() {
        seedsMediator.initialRefreshSeeds()
        do {
            try initNavigation(dbname: databaseMediator.databaseName, seedNames: seedsMediator.seedNames)
        } catch {}
    }
}
