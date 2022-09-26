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

    @ObservedObject var navigation: NavigationCoordinator
    @Published var parsingAlert: Bool = false

    // Data state
    @Published var onboardingDone: Bool = false
    @Published var authenticated: Bool = false

    // Alert indicator
    @Published var isConnectivityOn: Bool = false
    @Published var alert: Bool = false
    @Published var alertShow: Bool = false

    /// internal boilerplate
    var dbName: String

    /// did user set up password?
    let protected = LAContext().canEvaluatePolicy(.deviceOwnerAuthentication, error: nil)

    private let bundle: BundleProtocol
    private let connectivityMonitor: ConnectivityMonitoring
    private let databaseMediator: DatabaseMediating
    private let fileManager: FileManagingProtocol

    init(
        navigation: NavigationCoordinator,
        bundle: BundleProtocol = Bundle.main,
        connectivityMonitor: ConnectivityMonitoring = ConnectivityMonitoringAssembler().assemble(),
        databaseMediator: DatabaseMediating = DatabaseMediator(),
        fileManager: FileManagingProtocol = FileManager.default,
        seedsMediator: SeedsMediating = ServiceLocator.seedsMediator
    ) {
        self.seedsMediator = seedsMediator
        self.navigation = navigation
        self.bundle = bundle
        self.connectivityMonitor = connectivityMonitor
        self.databaseMediator = databaseMediator
        self.fileManager = fileManager
        dbName = databaseMediator.databaseName
        onboardingDone = databaseMediator.isDatabaseAvailable()

        seedsMediator.set(signerDataModel: self)
        setUpConnectivityMonitoring()
        finaliseInitialisation()
    }

    /// Mild refresh for situations when no interaction with data was really performed.
    /// Should not call stuff in signer.h
    func refreshUI() {}

    /// refresh everything except for seedNames
    /// should be called as often as reasonably possible - on flow interrupts, changes, events, etc.
    func totalRefresh() {
        navigation.perform(navigation: .init(action: .start))
        checkAlert()
        // self.refreshUI()
    }

    /// Should be called once on factory-new state of the app
    /// Populates database with starting values
    func onboard(jailbreak: Bool = false) {
        guard !isConnectivityOn else { return }
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
            initNavigation(dbname: dbName, seedNames: seedsMediator.seedNames)
            totalRefresh()
            seedsMediator.refreshSeeds()
        } catch {
            print("History init failed! This will not do.")
        }
    }
}

private extension SignerDataModel {
    func setUpConnectivityMonitoring() {
        connectivityMonitor.startMonitoring { isConnected in
            if isConnected, self.onboardingDone {
                do {
                    try historyDeviceWasOnline(dbname: self.dbName)
                } catch {
                    return
                }
                self.alert = true
            }
            self.isConnectivityOn = isConnected
        }
    }

    func finaliseInitialisation() {
        guard onboardingDone else { return }
        seedsMediator.refreshSeeds()
        initNavigation(dbname: dbName, seedNames: seedsMediator.seedNames)
        totalRefresh()
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
        initNavigation(dbname: dbName, seedNames: seedsMediator.seedNames)
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
