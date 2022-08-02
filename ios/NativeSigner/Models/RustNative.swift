//
//  RustNative.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 22.7.2021.
//

import Foundation
import LocalAuthentication // to detect if password is set
import UIKit // for converting raw png to UIImage

/// Object to store all data; since the data really is mostly stored in RustNative side,
/// just one object (to describe it) is used here.
class SignerDataModel: ObservableObject {
    // Action handler
    var actionAvailable = true // debouncer
    @Published var actionResult: ActionResult = ActionResult( // Screen state is stored here
        screenLabel: "",
        back: false,
        footer: false,
        footerButton: .settings,
        rightButton: .none,
        screenNameType: .h4,
        screenData: ScreenData.documents,
        modalData: nil,
        alertData: .none
    )
    @Published var parsingAlert: Bool = false
    let debounceTime: Double = 0.2 // Debounce time

    // Data state
    @Published var seedNames: [String] = []
    @Published var onboardingDone: Bool = false
    @Published var authenticated: Bool = false

    // This just starts camera reset. Could be done cleaner probably.
    @Published var resetCamera: Bool = false

    // internal boilerplate
    var dbName: String

    // Alert indicator
    @Published var canaryDead: Bool = false
    private let connectivityMonitor: ConnectivityMonitoring
    @Published var alert: Bool = false
    @Published var alertShow: Bool = false

    // version
    let appVersion = Bundle.main.infoDictionary?["CFBundleShortVersionString"] as? String

    // did user set up password?
    let protected = LAContext().canEvaluatePolicy(.deviceOwnerAuthentication, error: nil)

    init(
        connectivityMonitor: ConnectivityMonitoring = ConnectivityMonitoringAssembler().assemble()
    ) {
        self.connectivityMonitor = connectivityMonitor
        dbName = NSHomeDirectory() + "/Documents/Database"
        onboardingDone = FileManager.default.fileExists(atPath: NSHomeDirectory() + "/Documents/Database")

        connectivityMonitor.startMonitoring { isConnected in
            if isConnected, self.onboardingDone {
                do {
                    try historyDeviceWasOnline(dbname: self.dbName)
                } catch {
                    return
                }
                self.alert = true
            }
            self.canaryDead = isConnected
        }

        if onboardingDone {
            refreshSeeds()
            initNavigation(dbname: dbName, seedNames: seedNames)
            totalRefresh()
        }
    }

    /// Mild refresh for situations when no interaction with data was really performed.
    /// Should not call stuff in signer.h
    func refreshUI() {}

    /// refresh everything except for seedNames
    /// should be called as often as reasonably possible - on flow interrupts, changes, events, etc.
    func totalRefresh() {
        print("heavy reset")
        pushButton(action: .start)
        checkAlert()
        // self.refreshUI()
    }
}

extension SignerDataModel {
    /// Should be called once on factory-new state of the app
    /// Populates database with starting values
    func onboard(jailbreak: Bool = false) {
        if !canaryDead {
            do {
                print("onboarding...")
                wipe()
                if let source = Bundle.main.url(forResource: "Database", withExtension: "") {
                    var destination = try FileManager.default.url(
                        for: .documentDirectory,
                        in: .userDomainMask,
                        appropriateFor: nil,
                        create: false
                    )
                    destination.appendPathComponent("Database")
                    if FileManager.default.fileExists(atPath: NSHomeDirectory() + "/Documents/Database") {
                        do {
                            try FileManager.default.removeItem(at: destination)
                        } catch {
                            print("db exists but could not be removed; please report bug")
                            return
                        }
                    }
                    try FileManager.default.copyItem(at: source, to: destination)
                    do {
                        if jailbreak {
                            try historyInitHistoryNoCert(dbname: dbName)
                        } else {
                            try historyInitHistoryWithCert(dbname: dbName)
                        }
                        onboardingDone = true
                        // Mean app mode:
                        // if self.canaryDead {
                        // device_was_online(nil, self.dbName)
                        // }
                        initNavigation(dbname: dbName, seedNames: seedNames)
                        totalRefresh()
                        refreshSeeds()
                    } catch {
                        print("History init failed! This will not do.")
                    }
                }
            } catch {
                print("DB init failed")
            }
        }
    }

    /// Restores the Signer to factory new state
    /// Should be called before app uninstall/upgrade!
    func wipe() {
        refreshSeeds()
        if authenticated {
            // remove secrets first
            let query = [
                kSecClass as String: kSecClassGenericPassword
            ] as CFDictionary
            SecItemDelete(query)
            // then everything else
            do {
                var destination = try FileManager.default.url(
                    for: .documentDirectory,
                    in: .userDomainMask,
                    appropriateFor: nil,
                    create: false
                )
                destination.appendPathComponent("Database")
                try FileManager.default.removeItem(at: destination)
            } catch {
                print("FileManager failed to delete db")
            }
            onboardingDone = false
            seedNames = []
            initNavigation(dbname: dbName, seedNames: seedNames)
        }
    }

    /// Remove general verifier; wipes everything, obviously
    func jailbreak() {
        wipe()
        if !onboardingDone {
            onboard(jailbreak: true)
        }
    }
}

/// Maybe this could show errors?
extension ErrorDisplayed {
    func show() {
        if case let .Str(payload) = self {
            print(payload)
        }
    }
}
