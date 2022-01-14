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
 *
 *  Seeds are stored in keyring - it has SQL-like api but is backed by secure enclave
 *  IMPORTANT! The keys from keyring are not removed on app uninstall!
 *  Remember to wipe the app with wipe button in settings.
 */
extension SignerDataModel {
    /**
     * Get all seed names from secure storage
     *
     * this is also used as generic auth request operation that will lock the app on failure
     */
    func refreshSeeds() {
        var item: CFTypeRef?
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecMatchLimit as String: kSecMatchLimitAll,
            kSecReturnAttributes as String: true,
            kSecReturnData as String: false
        ]
        let status = SecItemCopyMatching(query as CFDictionary, &item)
        switch (status) {
        case errSecSuccess: do {
            guard let itemFound = item as? [[String : Any]]
            else {
                print("no seeds available")
                self.seedNames = []
                update_seed_names(nil, seedNames.joined(separator: ","))
                return
            }
            let seedNames = itemFound.map{item -> String in
                guard let seedName = item[kSecAttrAccount as String] as? String
                else {
                    print("seed name decoding error")
                    return "error!"
                }
                return seedName
            }
            self.seedNames = seedNames.sorted()
            update_seed_names(nil, self.seedNames.joined(separator: ","))
            self.authenticated = true
        }
        case errSecItemNotFound: do {
            print("no seeds available")
            self.seedNames = []
            update_seed_names(nil, seedNames.joined(separator: ","))
            self.authenticated = true
            return
        }
        default:
            self.authenticated = false
        }
    }
    
    /**
     * Creates seed; this is the only way to create seed.
     * createRoots: choose whether empty derivations for every network should be created
     */
    func restoreSeed(seedName: String, seedPhrase: String, createRoots: Bool) {
        var error: Unmanaged<CFError>?
        guard let accessFlags = SecAccessControlCreateWithFlags(kCFAllocatorDefault, kSecAttrAccessibleWhenPasscodeSetThisDeviceOnly, .devicePasscode, &error) else {
            print("Access flags could not be allocated")
            print(error ?? "no error code")
            self.lastError = "iOS key manager error, report a bug"
            return
        }
        if checkSeedPhraseCollision(seedPhrase: seedPhrase) {
            print("Key collision")
            self.lastError = "This seed phrase already exists"
            return
        }
        if !authenticated { return }
        guard let finalSeedPhrase = seedPhrase.data(using: .utf8) else {
            print("could not encode seed phrase")
            self.lastError = "Seed phrase contains non-0unicode symbols"
            return
        }
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
            print(self.lastError)
            return
        }
        self.seedNames.append(seedName)
        self.seedNames = self.seedNames.sorted()
        update_seed_names(nil, self.seedNames.joined(separator: ","))
        self.pushButton(buttonID: .GoForward, details: createRoots ? "true" : "false", seedPhrase: seedPhrase)
    }
    
    /**
     * Each seed name should be unique, obviously. We do not want to overwrite old seeds.
     */
    func checkSeedCollision(seedName: String) -> Bool {
        return self.seedNames.contains(seedName)
    }
    
    /**
     * Check if proposed seed phrase is already saved. But mostly require auth on seed creation.
     */
    func checkSeedPhraseCollision(seedPhrase: String) -> Bool {
        var item: CFTypeRef?
        guard let finalSeedPhrase = seedPhrase.data(using: .utf8) else {
            print("could not encode seed phrase")
            self.lastError = "Seed phrase contains non-unicode symbols"
            return true
        }
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecValueData as String: finalSeedPhrase,
            kSecMatchLimit as String: kSecMatchLimitOne
        ]
        let status = SecItemCopyMatching(query as CFDictionary, &item)
        if !(status == errSecSuccess || status == errSecItemNotFound) {
            self.authenticated = false
        }
        return status == errSecSuccess
    }
    
    /**
     * Gets seed by seedName from keyring
     * Calls auth screen automatically; no need to call it specially or wrap
     */
    func getSeed(seedName: String, backup: Bool = false) -> String {
        if self.alert {
            self.alertShow = true
            return ""
        }
        var err = ExternError()
        var item: CFTypeRef?
        var logSuccess = true
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrAccount as String: seedName,
            kSecMatchLimit as String: kSecMatchLimitOne,
            kSecReturnData as String: true
        ]
        let status = SecItemCopyMatching(query as CFDictionary, &item)
        if status == errSecSuccess {
            if backup {
                withUnsafeMutablePointer(to: &err) {err_ptr in
                    seed_name_was_shown(err_ptr, seedName, self.dbName)
                    if err_ptr.pointee.code != 0 {
                        print("Seed access logging error! This system is broken and should not be used anymore.")
                        self.lastError = String(cString: err_ptr.pointee.message)
                        print(self.lastError)
                        signer_destroy_string(err_ptr.pointee.message)
                        //Attempt to log this anyway one last time;
                        //if this fails too - complain to joulu pukki
                        history_entry_system(nil, "Seed access logging failed!", self.dbName)
                        logSuccess = false
                    }
                }
                return logSuccess ? String(data: (item as! CFData) as Data, encoding: .utf8) ?? "" : ""
            }
            return String(data: (item as! CFData) as Data, encoding: .utf8) ?? ""
        } else {
            self.lastError = SecCopyErrorMessageString(status, nil)! as String
            authenticated = false
            return ""
        }
    }
    
    
    /**
     * Removes seed and all derived keys
     */
    func removeSeed(seedName: String) {
        refreshSeeds()
        if self.authenticated {
            let query = [
                kSecClass as String: kSecClassGenericPassword,
                kSecAttrAccount as String: seedName
            ] as CFDictionary
            let status = SecItemDelete(query)
            if status == errSecSuccess {
                self.seedNames = self.seedNames.filter {element in
                    return element != seedName
                }
                self.seedNames = seedNames.sorted()
                update_seed_names(nil, self.seedNames.joined(separator: ","))
                pushButton(buttonID: .RemoveSeed)
            } else {
                self.lastError = SecCopyErrorMessageString(status, nil)! as String
                print("remove seed from secure storage error: " + self.lastError)
            }
        }
    }
    
    
    /*
     * Guess possible seed word(s) from user input
     */
    func guessWord(word: String) -> [String] {
        let res = guess_word(nil, word)
        if let wordsJSON = String(cString: res!).data(using: .utf8) {
            guard let words = try? JSONDecoder().decode([String].self, from: wordsJSON)
            else { return [] }
            return words
        } else {
            return []
        }
    }
    
    /**
     * Check if seedphrase is valid; returns error message or nothing
     */
    func validatePhrase(seedPhrase: String) -> String? {
        var err = ExternError()
        var errorMessage: String? = nil
        withUnsafeMutablePointer(to: &err) {err_ptr in
            validate_phrase(err_ptr, seedPhrase)
            if (err_ptr.pointee.code != 0)
            {
                errorMessage = String(cString: err_ptr.pointee.message)
                signer_destroy_string(err_ptr.pointee.message)
            }
        }
        return errorMessage
    }
}
