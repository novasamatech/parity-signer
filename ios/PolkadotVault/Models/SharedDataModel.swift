//
//  SharedDataModel.swift
//  Polkadot Vault
//
//  Created by Alexander Slesarev on 22.7.2021.
//

import Foundation
import SwiftUI

final class SharedDataModel: ObservableObject {
    private let seedsMediator: SeedsMediating
    private let databaseMediator: DatabaseMediating
    private let appLaunchMediator: AppLaunchMediating
    private let warningStateMediator: WarningStateMediator

    // Data state
    @Published var onboardingDone: Bool = false
    @Published var authenticated: Bool = false

    init(
        databaseMediator: DatabaseMediating = DatabaseMediator(),
        appLaunchMediator: AppLaunchMediating = AppLaunchMediator(),
        seedsMediator: SeedsMediating = ServiceLocator.seedsMediator,
        warningStateMediator: WarningStateMediator = ServiceLocator.warningStateMediator
    ) {
        self.seedsMediator = seedsMediator
        self.databaseMediator = databaseMediator
        self.appLaunchMediator = appLaunchMediator
        self.warningStateMediator = warningStateMediator
        onboardingDone = databaseMediator.isDatabaseAvailable()

        seedsMediator.set(sharedDataModel: self)
        finaliseInitialisation()
    }

    func onboard(verifierRemoved: Bool = false) {
        appLaunchMediator.onboard(verifierRemoved: verifierRemoved)
        onboardingDone = true
        warningStateMediator.updateWarnings()
    }
}

private extension SharedDataModel {
    func finaliseInitialisation() {
        if onboardingDone {
            appLaunchMediator.initialiseOnboardedUserRun()
        } else {
            appLaunchMediator.initialiseFirstRun()
        }
        warningStateMediator.updateWarnings()
    }
}
