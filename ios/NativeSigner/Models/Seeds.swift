//
//  Seeds.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 13.9.2021.
//

import Foundation

import Security //for keyring

/**
 * Apple's own crypto boilerplate
 */
enum KeychainError: Error {
    case noPassword
    case unexpectedPasswordData
    case unhandledError(status: OSStatus)
}

/**
 * Seeds management operations - these mostly rely on secure enclave
 * However, some rustnative operations happen here as well (default keys creation and associated keys removal)
 *
 *  Seeds are stored in keyring - it has SQL-like api but is backed by secure enclave
 *  IMPORTANT! The keys from keyring are not removed on app uninstall!
 *  Remember to wipe the app with wipe button in settings.
 */
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
    
    func addSeed(seedName: String, seedPhrase: String) {
        var err = ExternError()
        let err_ptr: UnsafeMutablePointer<ExternError> = UnsafeMutablePointer(&err)
        guard let accessFlags = SecAccessControlCreateWithFlags(kCFAllocatorDefault, kSecAttrAccessibleWhenPasscodeSetThisDeviceOnly, .devicePasscode, &error) else {
            print("Access flags could not be allocated")
            print(error ?? "no error code")
            self.lastError = "iOS key manager error, report a bug"
            return
        }
        print(accessFlags)
        if checkSeedCollision(seedName: seedName) {
            print("Key collision")
            self.lastError = "Seed with this name already exists"
        }
        let res = try_create_seed(err_ptr, seedName, seedPhrase, 24, dbName)
        if err_ptr.pointee.code != 0 {
            self.lastError = String(cString: err_ptr.pointee.message)
            print("Rust returned error")
            print(self.lastError)
            signer_destroy_string(err_ptr.pointee.message)
            return
        }
        let finalSeedPhraseString = String(cString: res!)
        guard let finalSeedPhrase = finalSeedPhraseString.data(using: .utf8) else {
            print("could not encode seed phrase")
            self.lastError = "Seed phrase contains non-0unicode symbols"
            return
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
            return
        }
        self.refreshSeeds()
        self.selectSeed(seedName: seedName)
        self.seedBackup = finalSeedPhraseString
        self.keyManagerModal = .seedBackup
    }
    
    /**
     * Each seed name should be unique, obviously. We do not want to overwrite old seeds.
     */
    func checkSeedCollision(seedName: String) -> Bool {
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrAccount as String: seedName,
            kSecMatchLimit as String: kSecMatchLimitOne
        ]
        let status = SecItemCopyMatching(query as CFDictionary, nil)
        return status == errSecSuccess
    }
    
    /**
     * Selects seed and updates the model accordingly
     */
    func selectSeed(seedName: String) {
        self.selectedSeed = seedName
        self.fetchKeys()
    }
    
    /**
     * This is simple explicit "get" for showing plaintext seedBackup value after it was fetched
     */
    func getRememberedSeedPhrate() -> String {
        if self.seedBackup == "" {
            self.seedBackup = getSeed(seedName: self.selectedSeed, backup: true)
        }
        return self.seedBackup
    }
    
    /**
     * Gets seed by seedName from keyring
     * Calls auth screen automatically; no need to call it specially or wrap
     */
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
                seed_name_was_shown(err_ptr, seedName, self.dbName)
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
                return String(data: (item as! CFData) as Data, encoding: .utf8) ?? ""
            }
        } else {
            self.lastError = SecCopyErrorMessageString(status, nil)! as String
            return ""
        }
    }
    
    /**
     * Removes seed and all derived keys
     */
    func removeSeed(seedName: String) {
        var err = ExternError()
        let err_ptr: UnsafeMutablePointer<ExternError> = UnsafeMutablePointer(&err)
        
        let query = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrAccount as String: seedName
        ] as CFDictionary
        let status = SecItemDelete(query)
        print(status.description)
        if status == errSecSuccess {
            remove_seed(err_ptr, seedName, dbName)
            if err_ptr.pointee.code == 0 {
                self.seedNames = self.seedNames.filter {
                    $0 != seedName
                }
                self.selectedSeed = ""
                self.fetchKeys()
            } else {
                self.lastError = String(cString: err_ptr.pointee.message)
                print(self.lastError)
            }
        } else {
            print(seedName)
            self.lastError = SecCopyErrorMessageString(status, nil)! as String
            print("remove seed from secure storage error: " + self.lastError)
        }
    }
}
