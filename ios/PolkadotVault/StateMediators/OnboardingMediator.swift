//
//  OnboardingMediator.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 13/03/2023.
//

import Foundation
import SwiftUI

final class OnboardingMediator: ObservableObject {
    private let navigationInitialisationService: NavigationInitialisationService
    private let seedsMediator: SeedsMediating
    private let databaseMediator: DatabaseMediating
    private let warningStateMediator: WarningStateMediator
    private let initialisationService: AppInitialisationService
    @Published var onboardingDone: Bool = false

    init(
        navigationInitialisationService: NavigationInitialisationService = NavigationInitialisationService(),
        seedsMediator: SeedsMediating = ServiceLocator.seedsMediator,
        databaseMediator: DatabaseMediating = DatabaseMediator(),
        warningStateMediator: WarningStateMediator = ServiceLocator.warningStateMediator,
        initialisationService: AppInitialisationService = AppInitialisationService()
    ) {
        self.navigationInitialisationService = navigationInitialisationService
        self.seedsMediator = seedsMediator
        self.databaseMediator = databaseMediator
        self.warningStateMediator = warningStateMediator
        self.initialisationService = initialisationService
        onboardingDone = databaseMediator.isDatabaseAvailable()
    }

    func onboard(verifierRemoved: Bool = false) {
        seedsMediator.refreshSeeds()
        localDataCleanup()
        databaseMediator.recreateDatabaseFile()
        navigationInitialisationService.initialiseNavigation(verifierRemoved: verifierRemoved)
        seedsMediator.refreshSeeds()
        onboardingDone = true
        warningStateMediator.updateWarnings()
        initialisationService.initialiseAppSession()
    }
}

private extension OnboardingMediator {
    func localDataCleanup() {
        seedsMediator.removeAllSeeds()
        databaseMediator.wipeDatabase()
    }
}
