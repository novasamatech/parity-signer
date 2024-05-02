//
//  KeychainBananaSplitQueryProvider.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 26/02/2024.
//

import Foundation

struct BananaSplitPassphrase: Codable, Equatable {
    let passphrase: String
}

enum KeychainBananaSplitQuery {
    case fetch(seedName: String)
    case check(seedName: String)
    case delete(seedName: String)
    case save(seedName: String, bananaSplit: BananaSplitBackup)
}

enum KeychainBananaSplitPassphraseQuery {
    case fetch(seedName: String)
    case delete(seedName: String)
    case save(seedName: String, passphrase: BananaSplitPassphrase, accessControl: SecAccessControl)
}

// sourcery: AutoMockable
protocol KeychainBananaSplitQueryProviding: AnyObject {
    func query(for queryType: KeychainBananaSplitQuery) -> CFDictionary
    func passhpraseQuery(for queryType: KeychainBananaSplitPassphraseQuery) -> CFDictionary
}

final class KeychainBananaSplitQueryProvider: KeychainBananaSplitQueryProviding {
    enum Constants {
        static let bananaSplitSuffix = "_bananaSplit"
        static let passphraseSuffix = "_passphrase"
    }

    private let jsonEncoder: JSONEncoder

    init(jsonEncoder: JSONEncoder = JSONEncoder()) {
        self.jsonEncoder = jsonEncoder
    }

    func query(for queryType: KeychainBananaSplitQuery) -> CFDictionary {
        var dictionary: [CFString: Any] = [
            kSecClass: kSecClassGenericPassword
        ]
        switch queryType {
        case let .fetch(seedName):
            dictionary[kSecMatchLimit] = kSecMatchLimitOne
            dictionary[kSecAttrAccount] = backupName(seedName)
            dictionary[kSecReturnData] = true
        case let .check(seedName):
            dictionary[kSecMatchLimit] = kSecMatchLimitOne
            dictionary[kSecAttrAccount] = backupName(seedName)
            dictionary[kSecReturnData] = false
        case let .delete(seedName):
            dictionary[kSecAttrAccount] = backupName(seedName)
        case let .save(seedName, bananaSplit):
            dictionary[kSecAttrAccount] = backupName(seedName)
            if let data = try? jsonEncoder.encode(bananaSplit) {
                dictionary[kSecValueData] = data
            }
            dictionary[kSecReturnData] = false
        }
        return dictionary as CFDictionary
    }

    func passhpraseQuery(for queryType: KeychainBananaSplitPassphraseQuery) -> CFDictionary {
        var dictionary: [CFString: Any] = [
            kSecClass: kSecClassGenericPassword
        ]
        switch queryType {
        case let .fetch(seedName):
            dictionary[kSecMatchLimit] = kSecMatchLimitOne
            dictionary[kSecAttrAccount] = passphraseName(seedName)
            dictionary[kSecReturnData] = true
        case let .delete(seedName):
            dictionary[kSecAttrAccount] = passphraseName(seedName)
        case let .save(seedName, passphrase, accessControl):
            dictionary[kSecAttrAccessControl] = accessControl
            dictionary[kSecAttrAccount] = passphraseName(seedName)
            if let data = try? jsonEncoder.encode(passphrase) {
                dictionary[kSecValueData] = data
            }

            dictionary[kSecReturnData] = false
        }
        return dictionary as CFDictionary
    }

    private func backupName(_ seedName: String) -> String {
        seedName + Constants.bananaSplitSuffix
    }

    private func passphraseName(_ seedName: String) -> String {
        seedName + Constants.passphraseSuffix
    }
}
