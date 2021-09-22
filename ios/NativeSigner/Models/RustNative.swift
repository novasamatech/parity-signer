//
//  RustNative.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 22.7.2021.
//

//TODO: this model crashes if no network is selected. This should be impossible, but it should be more elegant and safely handled.

import Foundation
import Security
import UIKit

enum KeychainError: Error {
    case noPassword
    case unexpectedPasswordData
    case unhandledError(status: OSStatus)
}

class DevTestObject: ObservableObject {
    var value: String = ""
    var err = ExternError()
    @Published var pictureData: Data?
    @Published var image: UIImage?
    
    init() {
        self.refresh(input: "")
    }
    
    func refresh(input: String) {
        let err_ptr: UnsafeMutablePointer<ExternError> = UnsafeMutablePointer(&self.err)
        let res = development_test(err_ptr, input)
        if err_ptr.pointee.code == 0 {
            value = String(cString: res!)
            self.pictureData = Data(fromHexEncodedString: value)
            self.image = UIImage(data: self.pictureData!)
            signer_destroy_string(res!)
        } else {
            value = String(cString: err_ptr.pointee.message)
            signer_destroy_string(err_ptr.pointee.message)
        }
    }
}

//NOTE: this object should always be synchronous
class SignerDataModel: ObservableObject {
    @Published var seedNames: [String] = []
    @Published var networks: [Network] = []
    @Published var identities: [Identity] = []
    @Published var selectedSeed: String = ""
    @Published var selectedNetwork: Network?
    @Published var selectedIdentity: Identity?
    @Published var newSeed: Bool = true
    @Published var newIdentity: Bool = false
    @Published var exportIdentity: Bool = false
    @Published var suggestedPath: String = "//"
    @Published var suggestedName: String = ""
    @Published var onboardingDone: Bool = false
    @Published var lastError: String = ""
    @Published var document: ShownDocument = .none
    @Published var networkSettings: NetworkSettings?
    @Published var history: [History] = []
    
    var error: Unmanaged<CFError>?
    var dbName: String
    
    init() {
        self.dbName = NSHomeDirectory() + "/Documents/Database"
        self.onboardingDone = FileManager.default.fileExists(atPath: NSHomeDirectory() + "/Documents/Database")
        if self.onboardingDone {
            self.refreshSeeds()
            self.totalRefresh()
        }
    }
    
    func totalRefresh() {
        self.lastError = ""
        self.document = .none
        self.newSeed = self.seedNames.count == 0
        self.newIdentity = false
        self.exportIdentity = false
        self.refreshNetworks()
        if self.networks.count > 0 {
            self.selectedNetwork = self.networks[0]
            self.fetchKeys()
        } else {
            print("No networks found; not handled yet")
        }
        self.networkSettings = nil
        self.getHistory()
    }
}



//MARK: messy seed management

extension SignerDataModel {
        
    func refreshSeeds() {
        var item: CFTypeRef?
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecMatchLimit as String: kSecMatchLimitAll,
            kSecReturnAttributes as String: true,
            kSecReturnData as String: false
        ]
        let status = SecItemCopyMatching(query as CFDictionary, &item)
        print("refresh seeds")
        print(status)
        print(SecCopyErrorMessageString(status, nil) ?? "Success")
        guard let itemFound = item as? [[String : Any]]
        else {
            print("no seeds available")
            self.seedNames = []
            return
        }
        print("some seeds fetched")
        print(itemFound)
        print(kSecAttrAccount)
        let seedNames = itemFound.map{item -> String in
            guard let seedName = item[kSecAttrAccount as String] as? String
            else {
                print("seed name decoding error")
                return "error!"
            }
            return seedName
        }
        print(seedNames)
        self.seedNames = seedNames.sorted()
    }
    
    func addSeed(seedName: String, seedPhrase: String) -> String {
        var err = ExternError()
        let err_ptr: UnsafeMutablePointer<ExternError> = UnsafeMutablePointer(&err)
        guard let accessFlags = SecAccessControlCreateWithFlags(kCFAllocatorDefault, kSecAttrAccessibleWhenPasscodeSetThisDeviceOnly, .devicePasscode, &error) else {
            print("Access flags could not be allocated")
            print(error ?? "no error code")
            self.lastError = "iOS key manager error, report a bug"
            return ""
        }
        print(accessFlags)
        if checkSeedCollision(seedName: seedName) {
            print("Key collision")
            self.lastError = "Seed with this name already exists"
            return ""
        }
        let res = try_create_seed(err_ptr, seedName, "sr25519", seedPhrase, 24, dbName)
        if err_ptr.pointee.code != 0 {
            self.lastError = String(cString: err_ptr.pointee.message)
            print("Rust returned error")
            print(self.lastError)
            signer_destroy_string(err_ptr.pointee.message)
            return ""
        }
        let finalSeedPhraseString = String(cString: res!)
        guard let finalSeedPhrase = finalSeedPhraseString.data(using: .utf8) else {
            print("could not encode seed phrase")
            self.lastError = "Seed phrase contains non-0unicode symbols"
            return ""
        }
        signer_destroy_string(res)
        print(finalSeedPhrase)
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrAccessControl as String: accessFlags,
            kSecAttrAccount as String: seedName,
            kSecValueData as String: finalSeedPhrase,
            kSecReturnData as String: true
        ]
        var resultAdd: AnyObject?
        let status = SecItemAdd(query as CFDictionary, &resultAdd)
        guard status == errSecSuccess else {
            print("key add failure")
            print(status)
            self.lastError = SecCopyErrorMessageString(status, nil)! as String
            return ""
        }
        self.refreshSeeds()
        self.selectSeed(seedName: seedName)
        return finalSeedPhraseString
    }
    
    func checkSeedCollision(seedName: String) -> Bool {
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrAccount as String: seedName,
            kSecMatchLimit as String: kSecMatchLimitOne
        ]
        let status = SecItemCopyMatching(query as CFDictionary, nil)
        return status == errSecSuccess
    }
    
    func selectSeed(seedName: String) {
        self.selectedSeed = seedName
        self.fetchKeys()
    }

    func getSeed(seedName: String, backup: Bool = false) -> String {
        var err = ExternError()
        let err_ptr: UnsafeMutablePointer<ExternError> = UnsafeMutablePointer(&err)
        var item: CFTypeRef?
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrAccount as String: seedName,
            kSecMatchLimit as String: kSecMatchLimitOne,
            kSecReturnData as String: true
        ]
        let status = SecItemCopyMatching(query as CFDictionary, &item)
        if status == errSecSuccess {
            if backup {
                seeds_were_shown(err_ptr, self.dbName)
            } else {
                //seeds_were_accessed(err_ptr, self.dbName)
                //This breaks things badly
                //TODO: cleanup
                return String(data: (item as! CFData) as Data, encoding: .utf8) ?? ""
            }
            if err_ptr.pointee.code == 0 {
                return String(data: (item as! CFData) as Data, encoding: .utf8) ?? ""
            } else {
                print("Seed access logging error! This system is broken and should not be used anymore.")
                self.lastError = String(cString: err_ptr.pointee.message)
                print(self.lastError)
                signer_destroy_string(err_ptr.pointee.message)
                //Attempt to log this anyway one last time;
                //if this fails too - complain to joulu pukki
                history_entry_system(nil, "Seed access logging failed!", self.dbName)
                return ""
            }
        } else {
            self.lastError = SecCopyErrorMessageString(status, nil)! as String
            return ""
        }
    }
}

//MARK: Onboarding

extension SignerDataModel {
    func onboard() {
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
                init_history(err_ptr, self.dbName)
                if (err_ptr.pointee.code == 0) {
                    self.onboardingDone = true
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
    }
}

//MARK: Actual rust bridge should be here

extension SignerDataModel {
    func stub(){
        return
    }
}
