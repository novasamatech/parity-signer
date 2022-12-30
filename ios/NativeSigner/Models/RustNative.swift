//
//  RustNative.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 22.7.2021.
//

import Foundation
import LocalAuthentication // to detect if password is set
import SwiftUI

/// Object to store all data; since the data really is mostly stored in RustNative side,
/// just one object (to describe it) is used here.
final class SignerDataModel: ObservableObject {
    private let seedsMediator: SeedsMediating

    @ObservedObject private(set) var navigation: NavigationCoordinator
    @ObservedObject private var connectivityMediator: ConnectivityMediator

    // Data state
    @Published var onboardingDone: Bool = false
    @Published var authenticated: Bool = false

    // Alert indicator
    @Published var alert: Bool = false
    @Published var alertShow: Bool = false

    /// internal boilerplate
    private var dbName: String {
        databaseMediator.databaseName
    }

    /// did user set up password?
    let protected = LAContext().canEvaluatePolicy(.deviceOwnerAuthentication, error: nil)

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

    /// refresh everything except for seedNames
    /// should be called as often as reasonably possible - on flow interrupts, changes, events, etc.
    func totalRefresh() {
        navigation.perform(navigation: .init(action: .start))
        checkAlert()
    }

    /// Should be called once on factory-new state of the app
    /// Populates database with starting values
    func onboard(jailbreak: Bool = false) {
        wipe()
        guard databaseMediator.recreateDatabaseFile() else {
            print("Database could not be recreated")
            return
        }
        do {
            if jailbreak {
                try historyInitHistoryNoCert(dbname: dbName)
            } else {
                try historyInitHistoryWithCert(dbname: dbName)
            }
            onboardingDone = true
            // Mean app mode:
            // if self.isConnectivityOn {
            // device_was_online(nil, self.dbName)
            // }
            try initNavigation(dbname: dbName, seedNames: seedsMediator.seedNames)
            totalRefresh()
            seedsMediator.refreshSeeds()
        } catch {
            print("History init failed! This will not do.")
        }
    }
}

private extension SignerDataModel {
    func setUpConnectivityMonitoring() {
        alert = connectivityMediator.isConnectivityOn
    }

    func finaliseInitialisation() {
        if onboardingDone {
            seedsMediator.refreshSeeds()
            do {
                try initNavigation(dbname: dbName, seedNames: seedsMediator.seedNames)
            } catch {
                print("InitNavigation has failed! This will not do.")
            }
            totalRefresh()
        } else {
            // remove secrets first
            seedsMediator.removeAllSeeds()
            // then everything else
            databaseMediator.wipeDatabase()
        }
    }
}

extension SignerDataModel {
    /// Restores the Signer to factory new state
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
        do {
            try initNavigation(dbname: dbName, seedNames: seedsMediator.seedNames)
        } catch {
            print("InitNavigation has failed. This will not do.")
        }
    }
}

extension SignerDataModel {
    /// Remove general verifier; wipes everything, obviously
    func jailbreak() {
        wipe()
        if !onboardingDone {
            onboard(jailbreak: true)
        }
    }
}

extension SignerDataModel {
    func sign(seedName: String, comment: String) {
        if alert {
            alertShow = true
        } else {
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
}

/// Address-related operations in data model
extension SignerDataModel {
    /// Creates address in database with checks and features
    func createAddress(path: String, seedName: String) {
        let seedPhrase = seedsMediator.getSeed(seedName: seedName)
        if !seedPhrase.isEmpty {
            navigation.perform(navigation: .init(action: .goForward, details: path, seedPhrase: seedPhrase))
        }
    }
}

extension SignerDataModel {
    /// Check if alert was triggered
    func checkAlert() {
        if onboardingDone {
            do {
                let res = try historyGetWarnings(dbname: dbName)
                if res {
                    alert = true
                } else {
                    alert = false
                }
            } catch {
                print("History init failed! This will not do.")
                alert = true
            }
        }
    }
}
