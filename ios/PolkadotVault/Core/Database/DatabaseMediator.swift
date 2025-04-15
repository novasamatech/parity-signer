//
//  DatabaseMediator.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 03/08/2022.
//

import Foundation

/// Protocol mediating operations on database
protocol DatabaseMediating: AnyObject {
    /// Provides name of main database
    var databaseName: String { get }
    /// Informs whether database already exists and can be returned from file system
    /// - Returns: `true` if database is accessible, `false` otherwise
    func isDatabaseAvailable() -> Bool
    /// Attempts to recreate database
    /// - Returns: `true` if database is recreated successfully, `false` if any of process steps fails
    @discardableResult
    func recreateDatabaseFile() -> Bool
    /// Wipes database file
    /// - Returns: `true` if database was deleted or didn't exist, `false` if any of process steps fails
    @discardableResult
    func wipeDatabase() -> Bool
}

/// Class that should act as main accessor for database
final class DatabaseMediator: DatabaseMediating {
    private enum Constants {
        static let bundleResource = "Database"
        static let destinationResource = "DatabaseV7_0_0"
    }

    private let bundle: BundleProtocol
    private let fileManager: FileManagingProtocol

    var databaseName: String { databasePath }

    private var databasePath: String {
        let documentsURL = try? fileManager.url(
            for: .documentDirectory,
            in: .userDomainMask,
            appropriateFor: nil,
            create: false
        )
        return documentsURL?
            .appendingPathComponent(Constants.destinationResource).path ?? ""
    }

    init(
        bundle: BundleProtocol = Bundle.main,
        fileManager: FileManagingProtocol = FileManager.default
    ) {
        self.bundle = bundle
        self.fileManager = fileManager
    }

    func isDatabaseAvailable() -> Bool {
        fileManager.fileExists(atPath: databasePath)
    }

    @discardableResult
    func recreateDatabaseFile() -> Bool {
        guard let source = bundle.url(forResource: Constants.bundleResource, withExtension: "") else {
            return false
        }
        do {
            var destination = try fileManager.url(
                for: .documentDirectory,
                in: .userDomainMask,
                appropriateFor: nil,
                create: true
            )
            destination.appendPathComponent(Constants.destinationResource)
            if fileManager.fileExists(atPath: databasePath) {
                do {
                    try fileManager.removeItem(at: destination)
                } catch {
                    return false
                }
            }
            try fileManager.copyItem(at: source, to: destination)
            return true
        } catch {
            return false
        }
    }

    @discardableResult
    func wipeDatabase() -> Bool {
        do {
            try fileManager.removeItem(atPath: databasePath)
            return true
        } catch {
            return false
        }
    }
}
