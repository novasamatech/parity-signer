//
//  SeedsMediator.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 25/08/2022.
//

import Combine
import Foundation

enum KeychainError: Error, Equatable {
    case fetchError
    case checkError
    case saveError(message: String)
    case deleteError(message: String)
    case accessControlNotAvailable
}

// sourcery: AutoMockable
/// Protocol that gathers all operations related to Keychain storage
protocol SeedsMediating: AnyObject {
    /// Accessor property for available seed names
    var seedNames: [String] { get }

    var seedNamesPublisher: AnyPublisher<[String], Never> { get }

    /// Get all seed names from secure storage
    ///
    /// This is also used as generic auth request operation that will lock the app on failure
    func refreshSeeds()
    /// Saves a seed within Keychain and adjust app state
    /// - Parameters:
    ///   - seedName: seed name
    ///   - seedPhrase: seed phrase to be saved
    @discardableResult
    func createSeed(seedName: String, seedPhrase: String, shouldCheckForCollision: Bool) -> Bool
    /// Checks for existance of `seedName` in Keychain
    /// Each seed name needs to be unique, this helps to not overwrite old seeds
    /// - Parameter seedName: seedName to be checked
    /// - Returns: informs whethere there is collision or not.
    func checkSeedCollision(seedName: String) -> Bool
    /// Fetches seed by `seedName` from Keychain
    /// Also calls auth screen automatically; no need to call it specially or wrap
    /// - Parameter seedName: seed name to fetch
    func getSeed(seedName: String) -> String
    func getAllSeeds() -> [String: String]
    func getSeeds(seedNames: Set<String>) -> [String: String]
    /// Gets seed backup by `seedName` from Keychain
    /// Calls auth screen automatically; no need to call it specially or wrap
    /// - Parameter seedName: seed name to fetch
    func getSeedBackup(seedName: String) -> String
    /// Removes seed and all deriverd keys
    /// - Parameter seedName: seed name to delete
    func removeSeed(seedName: String) -> Bool
    /// Clear all seeds from Keychain
    func removeAllSeeds() -> Bool
    func checkSeedPhraseCollision(seedPhrase: String) -> Bool
    func removeStalledSeeds()
}

/// Class handling all seeds-related operations that require access to Keychain
/// As this class contains logic related to UI state and data handling,
/// it should not interact with Keychain directly, but through injected dependencies
final class SeedsMediator: SeedsMediating {
    private let queryProvider: KeychainQueryProviding
    private let keychainAccessAdapter: KeychainAccessAdapting
    private let databaseMediator: DatabaseMediating
    private let authenticationStateMediator: AuthenticatedStateMediator
    var seedNamesSubject = CurrentValueSubject<[String], Never>([])
    var seedNamesPublisher: AnyPublisher<[String], Never> {
        seedNamesSubject.eraseToAnyPublisher()
    }

    var seedNames: [String] {
        seedNamesSubject.value
    }

    init(
        queryProvider: KeychainQueryProviding = KeychainQueryProvider(),
        keychainAccessAdapter: KeychainAccessAdapting = KeychainAccessAdapter(),
        databaseMediator: DatabaseMediating = DatabaseMediator(),
        authenticationStateMediator: AuthenticatedStateMediator = ServiceLocator.authenticationStateMediator
    ) {
        self.queryProvider = queryProvider
        self.keychainAccessAdapter = keychainAccessAdapter
        self.databaseMediator = databaseMediator
        self.authenticationStateMediator = authenticationStateMediator
    }

    func refreshSeeds() {
        let result = keychainAccessAdapter.fetchSeedNames()
        switch result {
        case let .success(payload):
            seedNamesSubject.send(payload.seeds)
            authenticationStateMediator.authenticated = true
        case .failure:
            authenticationStateMediator.authenticated = false
        }
    }

    @discardableResult
    func createSeed(
        seedName: String,
        seedPhrase: String,
        shouldCheckForCollision: Bool
    ) -> Bool {
        guard !seedName.isEmpty, let finalSeedPhrase = seedPhrase.data(using: .utf8) else { return false }
        if shouldCheckForCollision, checkSeedPhraseCollision(seedPhrase: seedPhrase) {
            return false
        }
        let saveSeedResult = keychainAccessAdapter.saveSeed(
            with: seedName,
            seedPhrase: finalSeedPhrase
        )
        switch saveSeedResult {
        case .success:
            var seeds = seedNamesSubject.value
            seeds.append(seedName)
            seeds.sort()
            seedNamesSubject.send(seeds)
            return true
        case .failure:
            return false
        }
    }

    func checkSeedCollision(seedName: String) -> Bool {
        seedNamesSubject.value.contains(seedName)
    }

    func getSeedBackup(seedName: String) -> String {
        let result = keychainAccessAdapter.retrieveSeed(with: seedName)
        switch result {
        case let .success(resultSeed):
            do {
                try historySeedWasShown(seedName: seedName)
            } catch {
                do {
                    try historyEntrySystem(
                        event: .systemEntry(systemEntry: "Seed access logging failed!")
                    )
                } catch {
                    return ""
                }
                return ""
            }
            return resultSeed
        case .failure:
            authenticationStateMediator.authenticated = false
            return ""
        }
    }

    func getSeed(seedName: String) -> String {
        let result = keychainAccessAdapter.retrieveSeed(with: seedName)
        switch result {
        case let .success(seed):
            return seed
        case .failure:
            authenticationStateMediator.authenticated = false
            return ""
        }
    }

    func getSeeds(seedNames: Set<String>) -> [String: String] {
        let result = keychainAccessAdapter.retrieveSeeds(with: seedNames)
        switch result {
        case let .success(seed):
            return seed
        case .failure:
            authenticationStateMediator.authenticated = false
            return [:]
        }
    }

    func getAllSeeds() -> [String: String] {
        getSeeds(seedNames: Set(seedNamesSubject.value))
    }

    func removeSeed(seedName: String) -> Bool {
        // Fetch item first, as this will trigger authentication if passcode is not cached
        guard case .success = keychainAccessAdapter.retrieveSeed(with: seedName) else {
            return false
        }
        let result = keychainAccessAdapter.removeSeed(seedName: seedName)
        switch result {
        case .success:
            seedNamesSubject.value = seedNamesSubject.value
                .filter { $0 != seedName }
                .sorted()
            return true
        case .failure:
            return false
        }
    }

    func removeAllSeeds() -> Bool {
        // Fetch seeds first, as this will trigger authentication if passcode is not cached
        guard case .success = keychainAccessAdapter.retrieveSeeds(with: Set(seedNamesSubject.value)) else {
            return false
        }
        guard keychainAccessAdapter.removeAllSeeds() else { return false }
        seedNamesSubject.send([])
        return true
    }

    func checkSeedPhraseCollision(seedPhrase: String) -> Bool {
        guard let seedPhraseAsData = seedPhrase.data(using: .utf8) else {
            return true
        }
        let result = keychainAccessAdapter.checkIfSeedPhraseAlreadyExists(seedPhrase: seedPhraseAsData)
        switch result {
        case let .success(isThereCollision):
            return isThereCollision
        case .failure:
            authenticationStateMediator.authenticated = false
            return false
        }
    }

    func removeStalledSeeds() {
        _ = keychainAccessAdapter.removeAllSeeds()
        seedNamesSubject.send([])
    }
}
