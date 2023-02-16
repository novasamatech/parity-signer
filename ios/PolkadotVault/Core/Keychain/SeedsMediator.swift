//
//  SeedsMediator.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 25/08/2022.
//

import Foundation

enum KeychainError: Error {
    case fetchError
    case checkError
    case saveError(message: String)
    case deleteError(message: String)
    case accessControlNotAvailable
}

/// Protocol that gathers all operations related to Keychain storage
protocol SeedsMediating: AnyObject {
    /// Accessor property for available seed names
    ///
    /// This should be turned to `private` in future refactors
    var seedNames: [String] { get set }
    /// Sets weak dependency to parent due to current architecture limitation (we should not store this class in
    /// `SharedDataModel`)
    /// - Parameter signerDataModel: reference to `SharedDataModel`
    func set(signerDataModel: SharedDataModel)
    /// Get all seed names from secure storage
    ///
    /// This is also used as generic auth request operation that will lock the app on failure
    func refreshSeeds()
    /// Get all seed names from secure storage without sending update to Rust
    func initialRefreshSeeds()
    /// Saves a seed within Keychain and adjust app state
    /// - Parameters:
    ///   - seedName: seed name
    ///   - seedPhrase: seed phrase to be saved
    @discardableResult
    func restoreSeed(seedName: String, seedPhrase: String, navigate: Bool) -> Bool
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
    func removeSeed(seedName: String)
    /// Clear all seeds from Keychain
    func removeAllSeeds()

    func checkSeedPhraseCollision(seedPhrase: String) -> Bool
}

/// Class handling all seeds-related operations that require access to Keychain
/// As this class contains logic related to UI state and data handling,
/// it should not interact with Keychain directly through injected dependencies
///
/// Old documentation below for reference, will be removed later:
/// Seeds management operations - these mostly rely on secure enclave
/// Seeds are stored in Keychain - it has SQL-like api but is backed by secure enclave
/// IMPORTANT! The keys in Keychain are not removed on app uninstall!
/// Remember to wipe the app with wipe button in settings.
final class SeedsMediator: SeedsMediating {
    private enum Constants {
        static let `true` = "true"
        static let `false` = "false"
    }

    private let queryProvider: KeychainQueryProviding
    private let keychainAccessAdapter: KeychainAccessAdapting
    private weak var signerDataModel: SharedDataModel!
    private let databaseMediator: DatabaseMediating

    @Published var seedNames: [String] = []

    init(
        queryProvider: KeychainQueryProviding = KeychainQueryProvider(),
        keychainAccessAdapter: KeychainAccessAdapting = KeychainAccessAdapter(),
        databaseMediator: DatabaseMediating = DatabaseMediator()
    ) {
        self.queryProvider = queryProvider
        self.keychainAccessAdapter = keychainAccessAdapter
        self.databaseMediator = databaseMediator
    }

    func set(signerDataModel: SharedDataModel) {
        self.signerDataModel = signerDataModel
    }

    func refreshSeeds() {
        refreshSeeds(firstRun: false)
    }

    func initialRefreshSeeds() {
        refreshSeeds(firstRun: true)
    }

    private func refreshSeeds(firstRun: Bool) {
        let result = keychainAccessAdapter.fetchSeedNames()
        switch result {
        case let .success(payload):
            seedNames = payload.seeds
            if let authenticated = payload.authenticated {
                signerDataModel.authenticated = authenticated
            }
            if !firstRun {
                attemptToUpdate(seedNames: seedNames)
            }
        case .failure:
            signerDataModel.authenticated = false
        }
    }

    @discardableResult
    func restoreSeed(seedName: String, seedPhrase: String, navigate: Bool = true) -> Bool {
        guard !seedName.isEmpty, let finalSeedPhrase = seedPhrase.data(using: .utf8) else { return false }
        if navigate, checkSeedPhraseCollision(seedPhrase: seedPhrase) {
            return false
        }
        let saveSeedResult = keychainAccessAdapter.saveSeed(
            with: seedName,
            seedPhrase: finalSeedPhrase
        )
        switch saveSeedResult {
        case .success:
            seedNames.append(seedName)
            seedNames.sort()
            attemptToUpdate(seedNames: seedNames)
            if navigate {
                signerDataModel.navigation.perform(navigation: .init(
                    action: .goForward,
                    details: Constants.true,
                    seedPhrase: seedPhrase
                ))
            } else {
                signerDataModel.navigation.performFake(navigation: .init(
                    action: .goForward,
                    details: Constants.true,
                    seedPhrase: seedPhrase
                ))
            }
            return true
        case .failure:
            return false
        }
    }

    func checkSeedCollision(seedName: String) -> Bool {
        seedNames.contains(seedName)
    }

    func getSeedBackup(seedName: String) -> String {
        let result = keychainAccessAdapter.retrieveSeed(with: seedName)
        switch result {
        case let .success(resultSeed):
            do {
                try historySeedNameWasShown(seedName: seedName)
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
            signerDataModel.authenticated = false
            return ""
        }
    }

    func getSeed(seedName: String) -> String {
        let result = keychainAccessAdapter.retrieveSeed(with: seedName)
        switch result {
        case let .success(seed):
            return seed
        case .failure:
            signerDataModel.authenticated = false
            return ""
        }
    }

    func getSeeds(seedNames: Set<String>) -> [String: String] {
        let result = keychainAccessAdapter.retrieveSeeds(with: seedNames)
        switch result {
        case let .success(seed):
            return seed
        case .failure:
            signerDataModel.authenticated = false
            return [:]
        }
    }

    func getAllSeeds() -> [String: String] {
        getSeeds(seedNames: Set(seedNames))
    }

    func removeSeed(seedName: String) {
        refreshSeeds()
        guard signerDataModel.authenticated else {
            return
        }
        let result = keychainAccessAdapter.removeSeed(seedName: seedName)
        switch result {
        case .success:
            seedNames = seedNames
                .filter { $0 != seedName }
                .sorted()
            attemptToUpdate(seedNames: seedNames)
            signerDataModel.navigation.perform(navigation: .init(action: .removeSeed), skipDebounce: true)
        case .failure:
            ()
            // We should inform user with some dedicated UI state for that error, maybe just system alert
        }
    }

    func removeAllSeeds() {
        keychainAccessAdapter.removeAllSeeds()
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
            signerDataModel.authenticated = false
            return false
        }
    }
}

private extension SeedsMediator {
    func attemptToUpdate(seedNames: [String]) {
        do {
            try updateSeedNames(seedNames: seedNames)
        } catch {
            signerDataModel.authenticated = false
        }
    }
}
