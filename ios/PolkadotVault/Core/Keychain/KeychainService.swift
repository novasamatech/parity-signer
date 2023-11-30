//
//  KeychainService.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 25/11/2023.
//

import Foundation

/// A protocol that abstracts the keychain operations used by KeychainMediator.
protocol KeychainServicing {
    /// Adds a keychain item that matches a query.
    /// - Parameters:
    ///   - query: A dictionary that contains an item class key and attribute keys and values.
    ///   - result: On return, contains the item data from the keychain.
    /// - Returns: A result code. See "Security Result Codes"
    /// (https://developer.apple.com/documentation/security/1542001-security_framework_result_codes).
    func add(_ query: CFDictionary, _ result: UnsafeMutablePointer<CFTypeRef?>?) -> OSStatus

    /// Returns one or more keychain items that match a search query.
    /// - Parameters:
    ///   - query: A dictionary that contains an item class key and attribute keys and values.
    ///   - result: On return, contains the item data from the keychain.
    /// - Returns: A result code. See "Security Result Codes"
    /// (https://developer.apple.com/documentation/security/1542001-security_framework_result_codes).
    func copyMatching(_ query: CFDictionary, _ result: UnsafeMutablePointer<CFTypeRef?>?) -> OSStatus

    /// Deletes keychain items that match a search query.
    /// - Parameter query: A dictionary that contains an item class key and attribute keys and values.
    /// - Returns: A result code. See "Security Result Codes"
    /// (https://developer.apple.com/documentation/security/1542001-security_framework_result_codes).
    func delete(_ query: CFDictionary) -> OSStatus
}

/// A class that implements `KeychainService` by calling the actual `Keychain API`.
final class KeychainService: KeychainServicing {
    init() {}
    func add(_ query: CFDictionary, _ result: UnsafeMutablePointer<CFTypeRef?>?) -> OSStatus {
        SecItemAdd(query, result)
    }

    func copyMatching(_ query: CFDictionary, _ result: UnsafeMutablePointer<CFTypeRef?>?) -> OSStatus {
        SecItemCopyMatching(query, result)
    }

    func delete(_ query: CFDictionary) -> OSStatus {
        SecItemDelete(query)
    }
}
