//
//  KeychainBananaSplitAccessAdapter.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 27/02/2024.
//

import Foundation

protocol KeychainBananaSplitAccessAdapting: AnyObject {
    func saveBananaSplit(
        with seedName: String,
        bananaSplitBackup: BananaSplitBackup,
        passphrase: BananaSplitPassphrase
    ) -> Result<Void, KeychainError>
    func retrieveBananaSplit(with seedName: String) -> Result<BananaSplitBackup, KeychainError>
    func retrieveBananaSplitPassphrase(with seedName: String) -> Result<BananaSplitPassphrase, KeychainError>
    func removeBananaSplitBackup(seedName: String) -> Result<Void, KeychainError>
    func checkIfBananaSplitAlreadyExists(seedName: String) -> Result<Bool, KeychainError>
}

final class KeychainBananaSplitAccessAdapter: KeychainBananaSplitAccessAdapting {
    private let keychainService: KeychainServicing
    private let queryProvider: KeychainBananaSplitQueryProviding
    private let acccessControlProvider: AccessControlProviding
    private let jsonDecoder: JSONDecoder

    init(
        keychainService: KeychainServicing = KeychainService(),
        acccessControlProvider: AccessControlProviding = AccessControlProvidingAssembler().assemble(),
        queryProvider: KeychainBananaSplitQueryProviding = KeychainBananaSplitQueryProvider(),
        jsonDecoder: JSONDecoder = JSONDecoder()
    ) {
        self.keychainService = keychainService
        self.acccessControlProvider = acccessControlProvider
        self.queryProvider = queryProvider
        self.jsonDecoder = jsonDecoder
    }

    func saveBananaSplit(
        with seedName: String,
        bananaSplitBackup: BananaSplitBackup,
        passphrase: BananaSplitPassphrase
    ) -> Result<Void, KeychainError> {
        do {
            let accessControl = try acccessControlProvider.accessControl()
            var query = queryProvider.query(
                for: .save(seedName: seedName, bananaSplit: bananaSplitBackup)
            )
            var osStatus = keychainService.add(query, nil)
            if osStatus != errSecSuccess {
                let message = SecCopyErrorMessageString(osStatus, nil) as? String ?? ""
                return .failure(.saveError(message: message))
            }

            query = queryProvider.query(
                for: KeychainBananaSplitPassphraseQuery.save(
                    seedName: seedName,
                    passphrase: passphrase,
                    accessControl: accessControl
                )
            )
            osStatus = keychainService.add(query, nil)
            if osStatus != errSecSuccess {
                let message = SecCopyErrorMessageString(osStatus, nil) as? String ?? ""
                return .failure(.saveError(message: message))
            }
            return .success(())
        } catch {
            return .failure(.accessControlNotAvailable)
        }
    }

    func retrieveBananaSplit(with seedName: String) -> Result<BananaSplitBackup, KeychainError> {
        var item: CFTypeRef?
        let query = queryProvider.query(for: KeychainBananaSplitQuery.fetch(seedName: seedName))
        let osStatus = keychainService.copyMatching(query, &item)
        if osStatus == errSecSuccess, let itemAsData = item as? Data {
            do {
                let result = try jsonDecoder.decode(BananaSplitBackup.self, from: itemAsData)
                return .success(result)
            } catch {
                return .failure(.dataDecodingError)
            }
        }
        return .failure(.fetchError)
    }

    func retrieveBananaSplitPassphrase(with seedName: String) -> Result<BananaSplitPassphrase, KeychainError> {
        var item: CFTypeRef?
        let query = queryProvider.query(for: KeychainBananaSplitPassphraseQuery.fetch(seedName: seedName))
        let osStatus = keychainService.copyMatching(query, &item)
        if osStatus == errSecSuccess, let itemAsData = item as? Data {
            do {
                let result = try jsonDecoder.decode(BananaSplitPassphrase.self, from: itemAsData)
                return .success(result)
            } catch {
                return .failure(.dataDecodingError)
            }
        }
        return .failure(.fetchError)
    }

    func removeBananaSplitBackup(seedName: String) -> Result<Void, KeychainError> {
        let bananaSplitQuery = queryProvider.query(for: KeychainBananaSplitQuery.delete(seedName: seedName))
        var osStatus = keychainService.delete(bananaSplitQuery)
        if osStatus != errSecSuccess {
            let errorMessage = SecCopyErrorMessageString(osStatus, nil) as? String ?? ""
            return .failure(.deleteError(message: errorMessage))
        }
        let passphraseQuery = queryProvider.query(for: KeychainBananaSplitPassphraseQuery.delete(seedName: seedName))
        osStatus = keychainService.delete(passphraseQuery)
        if osStatus != errSecSuccess {
            let errorMessage = SecCopyErrorMessageString(osStatus, nil) as? String ?? ""
            return .failure(.deleteError(message: errorMessage))
        }
        return .success(())
    }

    func checkIfBananaSplitAlreadyExists(seedName: String) -> Result<Bool, KeychainError> {
        let query = queryProvider.query(for: .check(seedName: seedName))
        var queryResult: AnyObject?
        let osStatus = keychainService.copyMatching(query, &queryResult)
        switch osStatus {
        case errSecItemNotFound:
            return .success(false)
        case errSecSuccess:
            return .success(true)
        default:
            return .failure(.checkError)
        }
    }
}
