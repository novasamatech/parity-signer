//
//  SeedsMediator.swift
//  NativeSigner
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
    // `SignerDataModel`)
    /// - Parameter signerDataModel: reference to `SignerDataModel`
    func set(signerDataModel: SignerDataModel)
    /// Get all seed names from secure storage
    ///
    /// This is also used as generic auth request operation that will lock the app on failure
    func refreshSeeds()
    /// Saves a seed within Keychain and adjust app state
    /// - Parameters:
    ///   - seedName: seed name
    ///   - seedPhrase: seed phrase to be saved
    ///   - createRoots: choose whether empty derivations for every network should be created
    func restoreSeed(seedName: String, seedPhrase: String, createRoots: Bool)
    /// Checks for existance of `seedName` in Keychain
    /// Each seed name needs to be unique, this helps to not overwrite old seeds
    /// - Parameter seedName: seedName to be checked
    /// - Returns: informs whethere there is collision or not.
    /// Current `false` is also returned if `seedName` cannot be encoded into data
    func checkSeedCollision(seedName: String) -> Bool
    /// Fetches seed by `seedName` from Keychain
    /// Also calls auth screen automatically; no need to call it specially or wrap
    /// - Parameter seedName: seed name to fetch
    func getSeed(seedName: String) -> String
    /// Gets seed backup by `seedName` from Keychain
    /// Calls auth screen automatically; no need to call it specially or wrap
    /// - Parameter seedName: seed name to fetch
    func getSeedBackup(seedName: String) -> String
    /// Removes seed and all deriverd keys
    /// - Parameter seedName: seed name to delete
    func removeSeed(seedName: String)
    /// Clear all seeds from Keychain
    func removeAllSeeds()
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
    private weak var signerDataModel: SignerDataModel!
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

    func set(signerDataModel: SignerDataModel) {
        self.signerDataModel = signerDataModel
    }

    func refreshSeeds() {
        let result = keychainAccessAdapter.fetchSeedNames()
        switch result {
        case let .success(payload):
            seedNames = payload.seeds
            if let authenticated = payload.authenticated {
                signerDataModel.authenticated = authenticated
            }
            attemptToUpdate(seedNames: seedNames)
        case .failure:
            signerDataModel.authenticated = false
        }
    }

    func restoreSeed(seedName: String, seedPhrase: String, createRoots: Bool) {
        guard signerDataModel.authenticated,
              !checkSeedPhraseCollision(seedPhrase: seedPhrase),
              let finalSeedPhrase = seedPhrase.data(using: .utf8) else { return }
        let saveSeedResult = keychainAccessAdapter.saveSeed(
            with: seedName,
            seedPhrase: finalSeedPhrase
        )
        switch saveSeedResult {
        case .success:
            seedNames.append(seedName)
            seedNames.sort()
            attemptToUpdate(seedNames: seedNames)
            signerDataModel.navigation.perform(navigation: .init(
                action: .goForward,
                details: createRoots ? Constants.true : Constants.false,
                seedPhrase: seedPhrase
            ))
        case .failure: ()
            // We should inform user with some dedicated UI state for that error, maybe just system alert
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
                try historySeedNameWasShown(seedName: seedName, dbname: databaseMediator.databaseName)
            } catch {
                // Revisit why exactly we are returning empty String here, if Keychain data is all good
                print("Seed access logging error! This system is broken and should not be used anymore.")
                do {
                    try historyEntrySystem(
                        event: .systemEntry(systemEntry: "Seed access logging failed!"),
                        dbname: databaseMediator.databaseName
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
        case .failure: ()
            // We should inform user with some dedicated UI state for that error, maybe just system alert
        }
    }

    func removeAllSeeds() {
        keychainAccessAdapter.removeAllSeeds()
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

    func checkSeedPhraseCollision(seedPhrase: String) -> Bool {
        guard let seedPhraseAsData = seedPhrase.data(using: .utf8) else {
            // We should probably inform user that their data input can't be parsed
            // due to non-utf characters pasted in textfield
            print("Could not encode seed phrase to using .utf8")
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
