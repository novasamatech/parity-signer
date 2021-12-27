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
import LocalAuthentication //to detect if password is set

/**
 * Object to store all data; since the data really is mostly stored in RustNative side, just one object (to describe it) is used here.
 */
class SignerDataModel: ObservableObject {
    
    //Action handler
    var actionAvailable = true //debouncer
    @Published var actionResult: ActionResult = ActionResult() //Screen state should pretty much be here
    let debounceTime: Double = 0.2 //Debounce time
    
    //Data state
    @Published var seedNames: [String] = []
    @Published var onboardingDone: Bool = false
    @Published var lastError: String = ""
    @Published var authenticated: Bool = false
    
    //Key manager state
    @Published var selectedSeed: String = ""
    @Published var searchKey: String = ""
    @Published var suggestedPath: String = "//"
    @Published var suggestedName: String = ""
    
    //Navigation
    @Published var keyManagerModal: SignerModal = .Empty
    @Published var parsingAlert: Bool = false
    
    //Transaction content
    @Published var payloadStr: String = ""
    @Published var transactionError: String = ""
    @Published var qr: UIImage?
    @Published var result: String? //TODO: remove this?
    @Published var comment: String = ""
    @Published var resetCamera: Bool = false
    @Published var cards: [TransactionCard] = []
    @Published var action: Action?
    @Published var author: Author?
    
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
    
    //version
    let appVersion = Bundle.main.infoDictionary?["CFBundleShortVersionString"] as? String
    
    //did user set up password?
    let protected = LAContext().canEvaluatePolicy(.deviceOwnerAuthentication, error: nil)
    
    init() {
        self.dbName = NSHomeDirectory() + "/Documents/Database"
        self.onboardingDone = FileManager.default.fileExists(atPath: NSHomeDirectory() + "/Documents/Database")
        self.monitor.pathUpdateHandler = {path in
            if path.availableInterfaces.count == 0 {
                DispatchQueue.main.async {
                    self.canaryDead = false
                }
            } else {
                DispatchQueue.main.async {
                    if self.onboardingDone {
                        device_was_online(nil, self.dbName)
                        self.alert = true
                    }
                    self.canaryDead = true
                }
            }
        }
        monitor.start(queue: self.queue)
        if self.onboardingDone {
            self.refreshSeeds()
            init_navigation(nil, dbName, seedNames.joined(separator: ","))
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
        pushButton(buttonID: .Start)
        self.checkAlert()
        //self.refreshUI()
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
        if !self.canaryDead {
            do {
                print("onboarding...")
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
                    withUnsafeMutablePointer(to: &err) {err_ptr in
                        if jailbreak {
                            init_history_no_cert(err_ptr, self.dbName)
                        } else {
                            init_history_with_cert(err_ptr, self.dbName)
                        }
                        if (err_ptr.pointee.code == 0) {
                            self.onboardingDone = true
                            /* Mean app mode:
                             if self.canaryDead {
                             device_was_online(nil, self.dbName)
                             }*/
                            init_navigation(nil, dbName, seedNames.joined(separator: ","))
                            self.totalRefresh()
                            self.refreshSeeds()
                        } else {
                            print("History init failed! This will not do.")
                            print(String(cString: err_ptr.pointee.message))
                            signer_destroy_string(err_ptr.pointee.message)
                        }
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
            init_navigation(nil, dbName, seedNames.joined(separator: ","))
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

