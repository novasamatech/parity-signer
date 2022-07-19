//
//  RustNative.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 22.7.2021.
//

import Foundation
import UIKit // for converting raw png to UIImage
import Network // to detect network connection and raise alert
import LocalAuthentication // to detect if password is set
// import CoreBluetooth // to check for bluetooth

/**
 * Object to store all data; since the data really is mostly stored in RustNative side,
 * just one object (to describe it) is used here.
 */
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
    let monitor = NWPathMonitor()
    let queue = DispatchQueue.global(qos: .background)
    // var manager: CBCentralManager
    // var bsDetector: BluetoothDetector = BluetoothDetector()
    let queueBT = DispatchQueue.global(qos: .background)
    @Published var alert: Bool = false
    @Published var alertShow: Bool = false

    // version
    let appVersion = Bundle.main.infoDictionary?["CFBundleShortVersionString"] as? String

    // did user set up password?
    let protected = LAContext().canEvaluatePolicy(.deviceOwnerAuthentication, error: nil)

    init() {
        self.dbName = NSHomeDirectory() + "/Documents/Database"
        self.onboardingDone = FileManager.default.fileExists(atPath: NSHomeDirectory() + "/Documents/Database")
        /*
         manager = CBCentralManager(
         delegate: bsDetector,
         queue: queueBT,
         options: [CBCentralManagerOptionShowPowerAlertKey: false]
         )
         */
        self.monitor.pathUpdateHandler = {path in
            if path.availableInterfaces.count == 0 {
                DispatchQueue.main.async {
                    self.canaryDead = false
                }
            } else {
                DispatchQueue.main.async {
                    if self.onboardingDone {
                        do {
                            try historyDeviceWasOnline(dbname: self.dbName)
                        } catch {
                            return
                        }
                        self.alert = true
                    }
                    self.canaryDead = true
                }
            }
        }
        monitor.start(queue: self.queue)
        if self.onboardingDone {
            self.refreshSeeds()
            initNavigation(dbname: dbName, seedNames: seedNames)
            self.totalRefresh()
        }
    }

    /**
     * Mild refresh for situations when no interaction with data was really performed.
     * Should not call stuff in signer.h
     */
    func refreshUI() {
    }

    /**
     * refresh everything except for seedNames
     * should be called as often as reasonably possible - on flow interrupts, changes, events, etc.
     */
    func totalRefresh() {
        print("heavy reset")
        pushButton(action: .start)
        self.checkAlert()
        // self.refreshUI()
    }
}

extension SignerDataModel {
    /**
     * Should be called once on factory-new state of the app
     * Populates database with starting values
     */
    func onboard(jailbreak: Bool = false) {
        if !self.canaryDead {
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
                        self.onboardingDone = true
                        /* Mean app mode:
                         if self.canaryDead {
                         device_was_online(nil, self.dbName)
                         }*/
                        initNavigation(dbname: dbName, seedNames: seedNames)
                        self.totalRefresh()
                        self.refreshSeeds()
                    } catch {
                        print("History init failed! This will not do.")
                    }
                }
            } catch {
                print("DB init failed")
            }
        }
    }

    /**
     * Restores the Signer to factory new state
     * Should be called before app uninstall/upgrade!
     */
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
            self.onboardingDone = false
            self.seedNames = []
            initNavigation(dbname: dbName, seedNames: seedNames)
        }
    }

    /**
     * Remove general verifier; wipes everything, obviously
     */
    func jailbreak() {
        self.wipe()
        if !onboardingDone {
            self.onboard(jailbreak: true)
        }
    }
}

/**
 * Maybe this could show errors?
 */
extension ErrorDisplayed {
    func show() {
        if case .Str(let payload) = self {
            print(payload)
        }
    }
}

/*
 /**
  * An object to monitor for bluetooth
  * This should not do anything else, of course
  */
 class BluetoothDetector: NSObject, CBCentralManagerDelegate {
 @Published var canaryDead = false
 
 /**
  * Just mark current bluetooth state
  */
 func centralManagerDidUpdateState(_ central: CBCentralManager) {
 switch central.state {
 case .unknown:
 DispatchQueue.main.async {
 self.canaryDead = true
 }
 case .resetting:
 DispatchQueue.main.async {
 self.canaryDead = false
 }
 case .unsupported:
 DispatchQueue.main.async {
 self.canaryDead = false
 }
 case .unauthorized:
 DispatchQueue.main.async {
 self.canaryDead = true
 }
 case .poweredOff:
 DispatchQueue.main.async {
 self.canaryDead = false
 }
 case .poweredOn:
 DispatchQueue.main.async {
 self.canaryDead = true
 }
 @unknown default:
 DispatchQueue.main.async {
 self.canaryDead = true
 }
 }
 
 //print(central.state.rawValue)
 }
 }
 */
