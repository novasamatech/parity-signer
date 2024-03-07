//
//  KeychainSeedsQueryProvider.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 26/08/2022.
//

import Foundation

/// Available queries for accessing Keychain
enum KeychainSeedsQuery {
    case fetch
    case fetchWithData
    case check
    case deleteAll
    case search(seedName: String)
    case delete(seedName: String)
    case restoreQuery(seedName: String, finalSeedPhrase: Data, accessControl: SecAccessControl)
}

// sourcery: AutoMockable
/// Protocol that provides access to query payload
protocol KeychainSeedsQueryProviding: AnyObject {
    /// Generates payload query for given query type with given input
    /// - Parameter queryType: query type and payload if needed
    /// - Returns: query payload as dictionary that can be used in Keychain querying
    func query(for queryType: KeychainSeedsQuery) -> CFDictionary
}

final class KeychainSeedsQueryProvider: KeychainSeedsQueryProviding {
    func query(for queryType: KeychainSeedsQuery) -> CFDictionary {
        var dictionary: [CFString: Any] = [
            kSecClass: kSecClassGenericPassword
        ]
        switch queryType {
        case .fetch:
            dictionary[kSecMatchLimit] = kSecMatchLimitAll // return all items
            dictionary[kSecReturnAttributes] = true // return item attributes
            dictionary[kSecReturnData] = false // don't return item data
        case .fetchWithData:
            dictionary[kSecMatchLimit] = kSecMatchLimitAll // return all items
            dictionary[kSecReturnAttributes] = true // return item attributes
            dictionary[kSecReturnData] = true // return item data
        case .check:
            dictionary[kSecMatchLimit] = kSecMatchLimitAll
            dictionary[kSecReturnData] = true
        case let .search(seedName):
            dictionary[kSecMatchLimit] = kSecMatchLimitOne // return only one item
            dictionary[kSecAttrAccount] = seedName // `Account` name, under which data and attributes can be saved
            dictionary[kSecReturnData] = true // return item data
        case let .delete(seedName):
            dictionary[kSecAttrAccount] = seedName
        case let .restoreQuery(seedName, finalSeedPhrase, accessControl):
            dictionary[kSecAttrAccessControl] = accessControl
            dictionary[kSecAttrAccount] = seedName
            dictionary[kSecValueData] = finalSeedPhrase // actual data for given `Account`
            dictionary[kSecReturnData] = true
        case .deleteAll:
            ()
        }
        return dictionary as CFDictionary
    }
}
