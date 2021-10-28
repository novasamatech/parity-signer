//
//  RustNative.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 22.7.2021.
//

//TODO: this model crashes if no network is selected. This should be impossible, but it should be more elegant and safely handled.

import Foundation
import UIKit //for converting raw png to UIImage
import Network //to detect network connection and raise alert

/**
 * Object to store all data; since the data really is mostly stored in RustNative side, just one object (to describe it) is used here.
 */
class SignerDataModel: ObservableObject {
    //Data state
    @Published var seedNames: [String] = []
    @Published var networks: [Network] = []
    @Published var addresses: [Address] = []
    @Published var onboardingDone: Bool = false
    @Published var lastError: String = ""
    @Published var networkSettings: NetworkSettings?
    @Published var generalVerifier: Verifier?
    
    //Key manager state
    @Published var selectedSeed: String = ""
    @Published var selectedNetwork: Network?
    @Published var selectedAddress: Address?
    @Published var searchKey: String = ""
    @Published var suggestedPath: String = "//"
    @Published var suggestedName: String = ""
    @Published var multiSelected: [Address] = []
    
    //History screen data state
    @Published var history: [History] = []
    @Published var selectedRecord: History?
    
    //Navigation
    @Published var signerScreen: SignerScreen = .history
    @Published var keyManagerModal: KeyManagerModal = .none
    @Published var settingsModal: SettingsModal = .none
    @Published var transactionState: TransactionState = .none
    
    //Transaction content
    @Published var cards: [TransactionCard] = []
    @Published var payloadStr: String = ""
    @Published var transactionError: String = ""
    @Published var action: Action?
    @Published var qr: UIImage?
    @Published var result: String? //TODO: remove this?
    @Published var author: Author?
    @Published var comment: String = ""
    @Published var resetCamera: Bool = false
    
    //internal boilerplate
    var error: Unmanaged<CFError>?
    var dbName: String
    
    //This is the secret - thus it's made non-reactive
    var seedBackup: String = ""
    
    //Alert indicator
    @Published var canaryDead: Bool = false
    let monitor = NWPathMonitor()
    let queue = DispatchQueue.global(qos: .background)
    @Published var alert: Bool = false
    
    init() {
        self.dbName = NSHomeDirectory() + "/Documents/Database"
        self.onboardingDone = FileManager.default.fileExists(atPath: NSHomeDirectory() + "/Documents/Database")
        if self.onboardingDone {
            self.monitor.pathUpdateHandler = {path in
                if path.availableInterfaces.count == 0 {
                    DispatchQueue.main.async {
                        self.canaryDead = false
                    }
                } else {
                    DispatchQueue.main.async {
                        device_was_online(nil, self.dbName)
                        self.canaryDead = true
                        self.alert = true
                    }
                }
            }
            
            monitor.start(queue: self.queue)
            self.refreshSeeds()
            self.totalRefresh()
        }
    }
    
    /**
     * Mild refresh for situations when no interaction with data was really performed.
     * Should not call stuff in signer.h
     */
    func refreshUI() {
        self.seedBackup = ""
        self.lastError = ""
        disableMutliSelectionMode()
        self.networkSettings = nil
        self.selectedRecord = nil
        resetTransaction()
        if self.seedNames.count == 0 {
            self.signerScreen = .keys
            self.keyManagerModal = .newSeed
        } else {
            self.keyManagerModal = .seedSelector
        }
        self.settingsModal = .none
        if self.signerScreen == .scan {
            self.resetCamera = true
        }
        self.searchKey = ""
    }
    
    /**
     * refresh everything except for navigation and seedNames
     * should be called as often as reasonably possible - on flow interrupts, changes, events, etc.
     */
    func totalRefresh() {
        print("heavy reset")
        self.checkAlert()
        self.refreshNetworks()
        if self.networks.count > 0 {
            self.selectedNetwork = self.networks[0]
            self.fetchKeys()
        } else {
            print("No networks found; not handled yet")
        }
        self.getHistory()
        self.getGeneralVerifier()
        resetTransaction()
        self.refreshUI()
    }
}

//MARK: Onboarding

extension SignerDataModel {
    /**
     * Should be called once on factory-new state of the app
     * Populates database with starting values
     */
    func onboard(jailbreak: Bool = false) {
        var err = ExternError()
        let err_ptr: UnsafeMutablePointer<ExternError> = UnsafeMutablePointer(&err)
        do {
            if let source = Bundle.main.url(forResource: "Database", withExtension: "") {
                print(source)
                var destination = try FileManager.default.url(for: .documentDirectory, in: .userDomainMask, appropriateFor: nil, create: false)
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
                if jailbreak {
                    init_history_no_cert(err_ptr, self.dbName)
                } else {
                    init_history_with_cert(err_ptr, self.dbName)
                }
                if (err_ptr.pointee.code == 0) {
                    self.onboardingDone = true
                    if self.canaryDead {
                        device_was_online(nil, self.dbName)
                    }
                    self.totalRefresh()
                    self.refreshSeeds()
                } else {
                    print("History init failed! This will not do.")
                    print(String(cString: err_ptr.pointee.message))
                    signer_destroy_string(err_ptr.pointee.message)
                }
            }
        } catch {
            print("DB init failed")
        }
    }
    
    /**
     * Restores the Signer to factory new state
     * Should be called before app uninstall/upgrade!
     */
    func wipe() {
        do {
            var destination = try FileManager.default.url(for: .documentDirectory, in: .userDomainMask, appropriateFor: nil, create: false)
            destination.appendPathComponent("Database")
            print(destination)
            print(self.dbName)
            try FileManager.default.removeItem(at: destination)
        } catch {
            print("FileManager failed to delete db")
            return
        }
        let query = [
            kSecClass as String: kSecClassGenericPassword
        ] as CFDictionary
        SecItemDelete(query)
        self.onboardingDone = false
        self.seedNames = []
        self.signerScreen = .keys
        self.keyManagerModal = .newSeed
    }
    
    /**
     * Remove general verifier; wipes everything, obviously
     */
    func jailbreak() {
        self.wipe()
        self.onboard(jailbreak: true)
    }
    
    func getGeneralVerifier() {
        var err = ExternError()
        let err_ptr: UnsafeMutablePointer<ExternError> = UnsafeMutablePointer(&err)
        let res = get_general_certificate(err_ptr, dbName)
        if (err_ptr.pointee.code == 0) {
            if let generalCert = String(cString: res!).data(using: .utf8) {
                self.generalVerifier = try? JSONDecoder().decode(Verifier.self, from: generalCert)
            } else {
                print("General verifier corrupted")
            }
            signer_destroy_string(res!)
        } else {
            print("General verifier fetch failed")
            print(String(cString: err_ptr.pointee.message))
            signer_destroy_string(err_ptr.pointee.message)
        }
    }
}

