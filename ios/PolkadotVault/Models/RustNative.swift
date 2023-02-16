//
//  RustNative.swift
//  Polkadot Vault
//
//  Created by Alexander Slesarev on 22.7.2021.
//

import Foundation
import SwiftUI

final class SharedDataModel: ObservableObject {
    private let seedsMediator: SeedsMediating

    @ObservedObject private(set) var navigation: NavigationCoordinator
    @ObservedObject private var connectivityMediator: ConnectivityMediator

    // Data state
    @Published var onboardingDone: Bool = false
    @Published var authenticated: Bool = false

    // Alert indicator
    @Published var alert: Bool = false

    private let bundle: BundleProtocol
    private let databaseMediator: DatabaseMediating

    init(
        navigation: NavigationCoordinator = NavigationCoordinator(),
        connectivityMediator: ConnectivityMediator = ConnectivityMediator(),
        bundle: BundleProtocol = Bundle.main,
        databaseMediator: DatabaseMediating = DatabaseMediator(),
        seedsMediator: SeedsMediating = ServiceLocator.seedsMediator
    ) {
        self.navigation = navigation
        self.connectivityMediator = connectivityMediator
        self.seedsMediator = seedsMediator
        self.bundle = bundle
        self.databaseMediator = databaseMediator
        onboardingDone = databaseMediator.isDatabaseAvailable()

        seedsMediator.set(signerDataModel: self)
        setUpConnectivityMonitoring()
        finaliseInitialisation()
    }

    func totalRefresh() {
        navigation.perform(navigation: .init(action: .start))
        checkAlert()
    }

    func onboard(verifierRemoved: Bool = false) {
        wipe()
        guard databaseMediator.recreateDatabaseFile() else {
            return
        }
        do {
            try initNavigation(dbname: databaseMediator.databaseName, seedNames: seedsMediator.seedNames)
            if verifierRemoved {
                try historyInitHistoryNoCert()
            } else {
                try historyInitHistoryWithCert()
            }
            onboardingDone = true
            totalRefresh()
            seedsMediator.refreshSeeds()
        } catch {}
    }
}

private extension SharedDataModel {
    func setUpConnectivityMonitoring() {
        alert = connectivityMediator.isConnectivityOn
    }

    func finaliseInitialisation() {
        if onboardingDone {
            seedsMediator.initialRefreshSeeds()
            do {
                try initNavigation(dbname: databaseMediator.databaseName, seedNames: seedsMediator.seedNames)
            } catch {}
            totalRefresh()
        } else {
            // remove secrets first
            seedsMediator.removeAllSeeds()
            // then everything else
            databaseMediator.wipeDatabase()
        }
    }
}

extension SharedDataModel {
    /// Restores the `Polkadot Vault` to factory new state
    /// Should be called before app uninstall/upgrade!
    func wipe() {
        seedsMediator.refreshSeeds()
        guard authenticated else { return }
        // remove secrets first
        seedsMediator.removeAllSeeds()
        // then everything else
        databaseMediator.wipeDatabase()
        onboardingDone = false
        seedsMediator.seedNames = []
    }
}

extension SharedDataModel {
    /// Remove general verifier; wipes everything, obviously
    func removeGeneralVerifier() {
        wipe()
        onboard(verifierRemoved: true)
    }
}

extension SharedDataModel {
    func sign(seedName: String, comment: String) {
        navigation.perform(
            navigation:
            .init(
                action: .goForward,
                details: comment,
                seedPhrase: seedsMediator.getSeed(seedName: seedName)
            )
        )
    }
}

/// Address-related operations in data model
extension SharedDataModel {
    /// Creates address in database with checks and features
    func createAddress(path: String, seedName: String) {
        let seedPhrase = seedsMediator.getSeed(seedName: seedName)
        if !seedPhrase.isEmpty {
            navigation.perform(navigation: .init(action: .goForward, details: path, seedPhrase: seedPhrase))
        }
    }
}

extension SharedDataModel {
    /// Check if alert was triggered
    func checkAlert() {
        guard onboardingDone else { return }
        do {
            alert = try historyGetWarnings()
        } catch {
            alert = true
        }
    }
}
