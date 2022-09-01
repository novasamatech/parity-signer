//
//  KeychainAccessAdapter.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 26/08/2022.
//

import Foundation

struct FetchSeedsPayload {
    let seeds: [String]
    let authenticated: Bool?
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
    func removeAllSeeds()
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
            return .success(FetchSeedsPayload(seeds: seedNames, authenticated: true))
        }
        // Keychain returned success but no data
        // We should confirm why we are not updating `authenticated` state in that case in original code
        if case errSecSuccess = osStatus {
            return .success(FetchSeedsPayload(seeds: [], authenticated: nil))
        }
        // Kechain stores no data for given query
        if case errSecItemNotFound = osStatus {
            return .success(FetchSeedsPayload(seeds: [], authenticated: true))
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
                print("Key set addition failure \(osStatus) \(message)")
                return .failure(.saveError(message: message))
            }
        } catch KeychainError.accessControlNotAvailable {
            print("Access flags could not be allocated")
            return .failure(.accessControlNotAvailable)
        } catch {
            let message = "Unkown error occured while saving seed"
            print(message)
            return .failure(.saveError(message: message))
        }
    }

    func retrieveSeed(with seedName: String) -> Result<String, KeychainError> {
        var item: CFTypeRef?
        let query = queryProvider.query(for: .search(seedName: seedName))
        let osStatus = SecItemCopyMatching(query, &item)
        if osStatus == errSecSuccess,
           let itemAsData = item as? Data,
           let result = String(data: itemAsData, encoding: .utf8) {
            return .success(result)
        }
        return .failure(.fetchError)
    }

    func removeSeed(seedName: String) -> Result<Void, KeychainError> {
        let query = queryProvider.query(for: .delete(seedName: seedName))
        let osStatus = SecItemDelete(query)
        if osStatus == errSecSuccess {
            return .success(())
        }
        let errorMessage = SecCopyErrorMessageString(osStatus, nil) as? String ?? ""
        print("Remove seed from secure storage error: \(errorMessage)")
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

    func removeAllSeeds() {
        let query = queryProvider.query(for: .deleteAll)
        SecItemDelete(query)
    }
}
