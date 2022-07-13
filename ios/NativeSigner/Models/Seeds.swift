//
//  Seeds.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 13.9.2021.
//

import Foundation

import Security // for keyring
import SwiftUI

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
        switch status {
        case errSecSuccess: do {
            guard let itemFound = item as? [[String: Any]]
            else {
                print("no seeds available")
                self.seedNames = []
                updateSeedNames(seedNames: seedNames)
                return
            }
            let seedNames = itemFound.map {item -> String in
                guard let seedName = item[kSecAttrAccount as String] as? String
                else {
                    print("seed name decoding error")
                    return "error!"
                }
                return seedName
            }
            self.seedNames = seedNames.sorted()
            updateSeedNames(seedNames: seedNames)
            self.authenticated = true
        }
        case errSecItemNotFound: do {
            print("no seeds available")
            self.seedNames = []
            updateSeedNames(seedNames: seedNames)
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
        guard let accessFlags = SecAccessControlCreateWithFlags(
            kCFAllocatorDefault,
            kSecAttrAccessibleWhenPasscodeSetThisDeviceOnly,
            .devicePasscode,
            &error
        ) else {
            print("Access flags could not be allocated")
            print(error ?? "no error code")
            return
        }
        if checkSeedPhraseCollision(seedPhrase: seedPhrase) {
            print("Key collision")
            return
        }
        if !authenticated { return }
        guard let finalSeedPhrase = seedPhrase.data(using: .utf8) else {
            print("could not encode seed phrase")
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
            let lastError = SecCopyErrorMessageString(status, nil)! as String
            print(lastError)
            return
        }
        self.seedNames.append(seedName)
        self.seedNames = self.seedNames.sorted()
        updateSeedNames(seedNames: self.seedNames)
        self.pushButton(action: .goForward, details: createRoots ? "true" : "false", seedPhrase: seedPhrase)
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
        var item: AnyObject?
        guard let finalSeedPhrase = seedPhrase.data(using: .utf8) else {
            print("could not encode seed phrase")
            return true
        }
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecMatchLimit as String: kSecMatchLimitAll,
            kSecReturnData as String: true
        ]
        let status = SecItemCopyMatching(query as CFDictionary, &item)
        if !(status == errSecSuccess || status == errSecItemNotFound) {
            self.authenticated = false
        }
        if status == errSecItemNotFound { return false }
        if item == nil {return false} else {
            let found = item as! NSArray // swiftlint:disable:this force_cast
            return found.contains(finalSeedPhrase)
        }
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
                do {
                    try historySeedNameWasShown(seedName: seedName, dbname: self.dbName)
                } catch {
                    print("Seed access logging error! This system is broken and should not be used anymore.")
                    // Attempt to log this anyway one last time;
                    // if this fails too - complain to joulu pukki
                    do {
                        try historyEntrySystem(
                            event: .systemEntry(systemEntry: "Seed access logging failed!"),
                            dbname: dbName
                        )
                    } catch {
                        logSuccess = false
                        authenticated = false
                        return ""
                    }
                    logSuccess = false
                }
                return logSuccess ? String(
                    data: (item as! CFData) as Data, // swiftlint:disable:this force_cast
                    encoding: .utf8
                ) ?? "" : ""
            }
            return String(
                data: (item as! CFData) as Data, // swiftlint:disable:this force_cast
                encoding: .utf8
            ) ?? ""
        } else {
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
                updateSeedNames(seedNames: self.seedNames)
                pushButton(action: .removeSeed)
            } else {
                let lastError = SecCopyErrorMessageString(status, nil)! as String
                print("remove seed from secure storage error: " + lastError)
            }
        }
    }

    /**
     * Wrapper for signing with use of seed material
     */
    func sign(seedName: String, comment: String) {
        if self.alert {
            self.alertShow = true
        } else {
            self.pushButton(
                action: .goForward,
                details: comment,
                seedPhrase: self.getSeed(seedName: seedName)
            )
        }
    }
}
