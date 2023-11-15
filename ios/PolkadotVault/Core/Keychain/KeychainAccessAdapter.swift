//
//  KeychainAccessAdapter.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 26/08/2022.
//

import Foundation

struct FetchSeedsPayload {
    let seeds: [String]
}

/// Protocol that provides access to Keychain's C-like API using modern approach
protocol KeychainAccessAdapting: AnyObject {
    /// Attempts to fetch list of seeds name from Keychain
    /// - Returns: closure with `.success` and requested `FetchSeedsPayload`
    ///            otherwise `.failure` with `KeychainError`
    func fetchSeedNames() -> Result<FetchSeedsPayload, KeychainError>
    /// Saves given `seedPhrase` as data attached to given `seedName`
    /// - Parameters:
    ///   - seedName: seed name
    ///   - seedPhrase: seed phrase to save
    /// - Returns: closure with `.success` if successfully saved to Keychain
    ///            otherwise `.failure` with `KeychainError`
    func saveSeed(with seedName: String, seedPhrase: Data) -> Result<Void, KeychainError>
    /// Saves given `seedPhrase` as data attached to given `seedName`
    /// - Parameters:
    ///   - seedName: seed name
    /// - Returns: closure with `.success` if successfully saved to Keychain
    ///            otherwise `.failure` with `KeychainError`
    func retrieveSeed(with seedName: String) -> Result<String, KeychainError>
    func retrieveSeeds(with seedNames: Set<String>) -> Result<[String: String], KeychainError>
    /// Removes seed with `seedName` from Keychain
    /// - Parameter seedName: seed name
    /// - Returns: closure with `.success` if successfully removed from Keychain
    ///            otherwise `.failure` with `KeychainError`
    func removeSeed(seedName: String) -> Result<Void, KeychainError>
    /// Checks Keychain for any items with identifier equal to `seedName`
    /// - Parameter seedName: seed name
    /// - Returns: closure with `.success` with information on whether seed phase already exists
    ///            otherwise `.failure` with `KeychainError`
    func checkIfSeedPhraseAlreadyExists(seedPhrase: Data) -> Result<Bool, KeychainError>
    /// Remove all seeds from Keychain
    func removeAllSeeds() -> Bool
}

final class KeychainAccessAdapter: KeychainAccessAdapting {
    private let queryProvider: KeychainQueryProviding
    private let acccessControlProvider: AccessControlProviding

    init(
        acccessControlProvider: AccessControlProviding = AccessControlProvidingAssembler().assemble(),
        queryProvider: KeychainQueryProviding = KeychainQueryProvider()
    ) {
        self.acccessControlProvider = acccessControlProvider
        self.queryProvider = queryProvider
    }

    func fetchSeedNames() -> Result<FetchSeedsPayload, KeychainError> {
        let query = queryProvider.query(for: .fetch)
        var fetchResult: CFTypeRef?
        let osStatus = SecItemCopyMatching(query, &fetchResult)
        // Keychain returned success and non-nil payload
        if case errSecSuccess = osStatus, let resultAsItems = fetchResult as? [[String: Any]] {
            let seedNames = resultAsItems
                .compactMap { seed in seed[kSecAttrAccount as String] as? String }
                .sorted()
            return .success(FetchSeedsPayload(seeds: seedNames))
        }
        // Keychain returned success but no data
        // We should confirm why we are not updating `authenticated` state in that case in original code
        if case errSecSuccess = osStatus {
            return .success(FetchSeedsPayload(seeds: []))
        }
        // Kechain stores no data for given query
        if case errSecItemNotFound = osStatus {
            return .success(FetchSeedsPayload(seeds: []))
        }
        // Different result status, return generic error
        return .failure(.fetchError)
    }

    func saveSeed(with seedName: String, seedPhrase: Data) -> Result<Void, KeychainError> {
        do {
            var result: CFTypeRef?
            let accessControl = try acccessControlProvider.accessControl()
            let query = queryProvider.query(
                for: .restoreQuery(
                    seedName: seedName,
                    finalSeedPhrase: seedPhrase,
                    accessControl: accessControl
                )
            )

            let osStatus = SecItemAdd(query, &result)
            if osStatus == errSecSuccess {
                return .success(())
            } else {
                let message = SecCopyErrorMessageString(osStatus, nil) as? String ?? ""
                return .failure(.saveError(message: message))
            }
        } catch KeychainError.accessControlNotAvailable {
            return .failure(.accessControlNotAvailable)
        } catch {
            let message = "Unkown error occured while saving seed"
            return .failure(.saveError(message: message))
        }
    }

    func retrieveSeed(with seedName: String) -> Result<String, KeychainError> {
        var item: CFTypeRef?
        let query = queryProvider.query(for: .search(seedName: seedName))
        let osStatus = SecItemCopyMatching(query, &item)
        if osStatus == errSecSuccess || osStatus == errSecItemNotFound,
           let itemAsData = item as? Data,
           let result = String(data: itemAsData, encoding: .utf8) {
            return .success(result)
        }
        return .failure(.fetchError)
    }

    func retrieveSeeds(with seedNames: Set<String>) -> Result<[String: String], KeychainError> {
        let query = queryProvider.query(for: .fetchWithData)
        var fetchResult: CFTypeRef?
        let osStatus = SecItemCopyMatching(query, &fetchResult)
        // Keychain returned success and non-nil payload
        if case errSecSuccess = osStatus, let resultAsItems = fetchResult as? [[String: Any]] {
            let seedNames: [(String, String)] = resultAsItems
                .compactMap { seed in
                    guard
                        let seedPhraseAsData = seed[kSecValueData as String] as? Data,
                        let seedPhrase = String(data: seedPhraseAsData, encoding: .utf8),
                        let seedName = seed[kSecAttrAccount as String] as? String,
                        seedNames.contains(seedName)
                    else { return nil }
                    return (seedName, seedPhrase)
                }

            let result: [String: String] = Dictionary(uniqueKeysWithValues: seedNames)
            return .success(result)
        }
        // Keychain returned success but no data
        if case errSecSuccess = osStatus {
            return .success([:])
        }
        // Kechain stores no data for given query
        if case errSecItemNotFound = osStatus {
            return .success([:])
        }
        // Different result status, return generic error
        return .failure(.fetchError)
    }

    func removeSeed(seedName: String) -> Result<Void, KeychainError> {
        let query = queryProvider.query(for: .delete(seedName: seedName))
        let osStatus = SecItemDelete(query)
        if osStatus == errSecSuccess {
            return .success(())
        }
        let errorMessage = SecCopyErrorMessageString(osStatus, nil) as? String ?? ""
        return .failure(.deleteError(message: errorMessage))
    }

    func checkIfSeedPhraseAlreadyExists(seedPhrase: Data) -> Result<Bool, KeychainError> {
        let query = queryProvider.query(for: .check)
        var queryResult: AnyObject?
        let osStatus = SecItemCopyMatching(query, &queryResult)
        if osStatus != errSecSuccess, osStatus != errSecItemNotFound {
            return .failure(.checkError)
        }
        if osStatus == errSecItemNotFound { return .success(false) }
        if let foundItem = queryResult as? [Data] {
            return .success(foundItem.contains(seedPhrase))
        }
        return .success(false)
    }

    func removeAllSeeds() -> Bool {
        let query = queryProvider.query(for: .deleteAll)
        let osStatus = SecItemDelete(query)
        if osStatus == errSecSuccess || osStatus == errSecItemNotFound {
            return true
        } else {
            return false
        }
    }
}
