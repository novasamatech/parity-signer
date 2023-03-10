//
//  SharedDataModel.swift
//  Polkadot Vault
//
//  Created by Alexander Slesarev on 22.7.2021.
//

import Foundation
import SwiftUI

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

final class SharedDataModel: ObservableObject {
    private let seedsMediator: SeedsMediating
    private let databaseMediator: DatabaseMediating
    private let appLaunchMediator: AppLaunchMediating

    @ObservedObject private var connectivityMediator: ConnectivityMediator

    // Data state
    @Published var onboardingDone: Bool = false
    @Published var authenticated: Bool = false

    // Alert indicator
    @Published var alert: Bool = false

    init(
        connectivityMediator: ConnectivityMediator = ConnectivityMediator(),
        databaseMediator: DatabaseMediating = DatabaseMediator(),
        appLaunchMediator: AppLaunchMediating = AppLaunchMediator(),
        seedsMediator: SeedsMediating = ServiceLocator.seedsMediator
    ) {
        self.connectivityMediator = connectivityMediator
        self.seedsMediator = seedsMediator
        self.databaseMediator = databaseMediator
        self.appLaunchMediator = appLaunchMediator
        onboardingDone = databaseMediator.isDatabaseAvailable()

        seedsMediator.set(sharedDataModel: self)
        setUpConnectivityMonitoring()
        finaliseInitialisation()
    }

    func updateWarnings() {
        do {
            alert = try historyGetWarnings()
        } catch {
            alert = true
        }
    }

    func onboard(verifierRemoved: Bool = false) {
        appLaunchMediator.onboard(verifierRemoved: verifierRemoved)
        onboardingDone = true
        updateWarnings()
    }
}

private extension SharedDataModel {
    func setUpConnectivityMonitoring() {
        alert = connectivityMediator.isConnectivityOn
    }

    func finaliseInitialisation() {
        if onboardingDone {
            appLaunchMediator.initialiseOnboardedUserRun()
        } else {
            appLaunchMediator.initialiseFirstRun()
        }
        updateWarnings()
    }
}
