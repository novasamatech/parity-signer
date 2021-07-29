//
//  Seed.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 27.7.2021.
//

import Foundation
import Security

enum KeychainError: Error {
    case noPassword
    case unexpectedPasswordData
    case unhandledError(status: OSStatus)
}

class Seeds: ObservableObject {
    @Published var seedNames: [String] = []
    var err = ExternError()
    var error: Unmanaged<CFError>?
    
    init() {
        print("seed names init")
        self.refresh()
    }
    
    func refresh() {
        var item: CFTypeRef?
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecMatchLimit as String: kSecMatchLimitAll,
            kSecReturnAttributes as String: true
        ]
        let status = SecItemCopyMatching(query as CFDictionary, &item)
        print("refresh seeds")
        print(status)
        print(SecCopyErrorMessageString(status, nil))
        print(item)
    }
    
    func add(seedName: String, seedPhrase: String) -> String {
        let err_ptr: UnsafeMutablePointer<ExternError> = UnsafeMutablePointer(&err)
        let dbName = NSHomeDirectory() + "/Documents/Database"
        guard let accessFlags = SecAccessControlCreateWithFlags(kCFAllocatorDefault, kSecAttrAccessibleWhenPasscodeSetThisDeviceOnly, .devicePasscode, &error) else {
            print("Access flags could not be allocated")
            print(error)
            return ""
        }
        print(accessFlags)
        if checkCollision(seedName: seedName) {
            print("Key collision")
            return ""
        }
        let res = try_create_seed(err_ptr, seedName, "sr25519", seedPhrase, 24, dbName)
        if err_ptr.pointee.code != 0 {
            print("Seed creation error!")
            print(String(cString: err_ptr.pointee.message)
            )
            signer_destroy_string(err_ptr.pointee.message)
            return ""
        }
        let finalSeedPhraseString = String(cString: res!)
        guard let finalSeedPhrase = finalSeedPhraseString.data(using: .utf8) else {
            print("could not encode seed phrase")
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
            print(SecCopyErrorMessageString(status, nil))
            return ""
        }
        return finalSeedPhraseString
    }
    
    func checkCollision(seedName: String) -> Bool {
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrAccount as String: seedName,
            kSecMatchLimit as String: kSecMatchLimitOne
        ]
        let status = SecItemCopyMatching(query as CFDictionary, nil)
        return status == errSecSuccess
    }
    
}
