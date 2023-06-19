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
        guard seedsMediator.removeAllSeeds() else { return }
        databaseMediator.recreateDatabaseFile()
        navigationInitialisationService.initialiseNavigation(verifierRemoved: verifierRemoved) { [weak self] in
            guard let self = self else { return }
            self.seedsMediator.refreshSeeds()
            self.onboardingDone = true
            self.warningStateMediator.updateWarnings()
            self.initialisationService.initialiseAppSession()
        }
    }
}
