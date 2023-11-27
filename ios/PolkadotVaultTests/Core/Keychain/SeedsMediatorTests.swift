//
//  SeedsMediatorTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 28/11/2023.
//

import Foundation
@testable import PolkadotVault
import XCTest

final class SeedsMediatorTests: XCTestCase {
    private var keychainAccessAdapterMock: KeychainAccessAdapterMock!
    private var databaseMediatorMock: DatabaseMediatorMock!
    private var authenticationStateMediatorMock: AuthenticatedStateMediatorMock!
    private var seedsMediator: SeedsMediator!

    override func setUp() {
        super.setUp()
        keychainAccessAdapterMock = KeychainAccessAdapterMock()
        databaseMediatorMock = DatabaseMediatorMock()
        authenticationStateMediatorMock = AuthenticatedStateMediatorMock()
        seedsMediator = SeedsMediator(
            keychainAccessAdapter: keychainAccessAdapterMock,
            databaseMediator: databaseMediatorMock,
            authenticationStateMediator: authenticationStateMediatorMock
        )
    }

    override func tearDown() {
        keychainAccessAdapterMock = nil
        databaseMediatorMock = nil
        authenticationStateMediatorMock = nil
        seedsMediator = nil
        super.tearDown()
    }

    func testRefreshSeeds_Successful() {
        // Given
        let seedNames = ["Seed1", "Seed2"]
        keychainAccessAdapterMock.fetchSeedNamesReturnValue = .success(FetchSeedsPayload(seeds: seedNames))

        // When
        seedsMediator.refreshSeeds()

        // Then
        XCTAssertEqual(seedsMediator.seedNames, seedNames)
        XCTAssertTrue(authenticationStateMediatorMock.authenticated)
        XCTAssertEqual(authenticationStateMediatorMock.authenticatedSetCallsCount, 1)
    }

    func testRefreshSeeds_Failure() {
        // Given
        keychainAccessAdapterMock.fetchSeedNamesReturnValue = .failure(.fetchError)

        // When
        seedsMediator.refreshSeeds()

        // Then
        XCTAssertTrue(seedsMediator.seedNames.isEmpty)
        XCTAssertFalse(authenticationStateMediatorMock.authenticated)
        XCTAssertEqual(authenticationStateMediatorMock.authenticatedSetCallsCount, 1)
    }

    func testCreateSeed_Successful() {
        // Given
        let seedName = "Seed1"
        let seedPhrase = "seedPhrase"
        keychainAccessAdapterMock.saveSeedReturnValue = .success(())

        // When
        let result = seedsMediator.createSeed(
            seedName: seedName,
            seedPhrase: seedPhrase,
            shouldCheckForCollision: false
        )

        // Then
        XCTAssertTrue(result)
        XCTAssertEqual(seedsMediator.seedNames, [seedName])
        XCTAssertEqual(keychainAccessAdapterMock.saveSeedCallsCount, 1)
    }

    func testCreateSeed_FailureDueToEmptySeedName() {
        // Given
        let seedName = ""
        let seedPhrase = "seedPhrase"

        // When
        let result = seedsMediator.createSeed(
            seedName: seedName,
            seedPhrase: seedPhrase,
            shouldCheckForCollision: false
        )

        // Then
        XCTAssertFalse(result)
        XCTAssertTrue(seedsMediator.seedNames.isEmpty)
    }

    func testCreateSeed_FailureDueToCollision() {
        // Given
        let seedName = "Seed1"
        let seedPhrase = "seedPhrase"
        // Simulate collision
        seedsMediator.seedNames = [seedPhrase]

        // When
        let result = seedsMediator.createSeed(seedName: seedName, seedPhrase: seedPhrase, shouldCheckForCollision: true)

        // Then
        XCTAssertFalse(result)
        XCTAssertEqual(seedsMediator.seedNames, [seedPhrase])
    }

    func testCreateSeed_KeychainFailure() {
        // Given
        let seedName = "Seed1"
        let seedPhrase = "seedPhrase"
        keychainAccessAdapterMock.saveSeedReturnValue = .failure(.saveError(message: "Error"))

        // When
        let result = seedsMediator.createSeed(
            seedName: seedName,
            seedPhrase: seedPhrase,
            shouldCheckForCollision: false
        )

        // Then
        XCTAssertFalse(result)
        XCTAssertTrue(seedsMediator.seedNames.isEmpty)
        XCTAssertEqual(keychainAccessAdapterMock.saveSeedCallsCount, 1)
    }

    func testCheckSeedCollision_Exists() {
        // Given
        let existingSeedName = "Seed1"
        seedsMediator.seedNames = [existingSeedName]

        // When
        let result = seedsMediator.checkSeedCollision(seedName: existingSeedName)

        // Then
        XCTAssertTrue(result)
    }

    func testCheckSeedCollision_NotExists() {
        // Given
        let nonExistingSeedName = "Seed2"
        seedsMediator.seedNames = ["Seed1"]

        // When
        let result = seedsMediator.checkSeedCollision(seedName: nonExistingSeedName)

        // Then
        XCTAssertFalse(result)
    }

    func testGetSeedBackup_Success() {
        // Given
        let seedName = "Seed1"
        let seedBackup = "BackupData"
        keychainAccessAdapterMock.retrieveSeedReturnValue = .success(seedBackup)

        // When
        let result = seedsMediator.getSeedBackup(seedName: seedName)

        // Then
        XCTAssertEqual(result, seedBackup)
    }

    func testGetSeedBackup_Failure() {
        // Given
        let seedName = "Seed1"
        keychainAccessAdapterMock.retrieveSeedReturnValue = .failure(.fetchError)

        // When
        let result = seedsMediator.getSeedBackup(seedName: seedName)

        // Then
        XCTAssertEqual(result, "")
        XCTAssertFalse(authenticationStateMediatorMock.authenticated)
    }

    func testGetSeed_Success() {
        // Given
        let seedName = "Seed1"
        let expectedSeed = "seedData"
        keychainAccessAdapterMock.retrieveSeedReturnValue = .success(expectedSeed)

        // When
        let result = seedsMediator.getSeed(seedName: seedName)

        // Then
        XCTAssertEqual(result, expectedSeed)
    }

    func testGetSeed_Failure() {
        // Given
        let seedName = "Seed1"
        keychainAccessAdapterMock.retrieveSeedReturnValue = .failure(.fetchError)

        // When
        let result = seedsMediator.getSeed(seedName: seedName)

        // Then
        XCTAssertEqual(result, "")
        XCTAssertFalse(authenticationStateMediatorMock.authenticated)
    }

    func testGetSeeds_Success() {
        // Given
        let seedNames = Set(["Seed1", "Seed2"])
        let expectedSeeds = ["Seed1": "seedData1", "Seed2": "seedData2"]
        keychainAccessAdapterMock.retrieveSeedsReturnValue = .success(expectedSeeds)

        // When
        let result = seedsMediator.getSeeds(seedNames: seedNames)

        // Then
        XCTAssertEqual(result, expectedSeeds)
    }

    func testGetSeeds_Failure() {
        // Given
        let seedNames = Set(["Seed1", "Seed2"])
        keychainAccessAdapterMock.retrieveSeedsReturnValue = .failure(.fetchError)

        // When
        let result = seedsMediator.getSeeds(seedNames: seedNames)

        // Then
        XCTAssertTrue(result.isEmpty)
        XCTAssertFalse(authenticationStateMediatorMock.authenticated)
    }

    func testGetAllSeeds() {
        // Given
        seedsMediator.seedNames = ["Seed1", "Seed2"]
        let expectedSeeds = ["Seed1": "seedData1", "Seed2": "seedData2"]
        keychainAccessAdapterMock.retrieveSeedsReturnValue = .success(expectedSeeds)

        // When
        let result = seedsMediator.getAllSeeds()

        // Then
        XCTAssertEqual(result, expectedSeeds)
    }

    func testRemoveSeed_Successful() {
        // Given
        let seedName = "Seed1"
        seedsMediator.seedNames = [seedName, "Seed2"]
        keychainAccessAdapterMock.retrieveSeedReturnValue = .success("seedData")
        keychainAccessAdapterMock.removeSeedReturnValue = .success(())

        // When
        let result = seedsMediator.removeSeed(seedName: seedName)

        // Then
        XCTAssertTrue(result)
        XCTAssertEqual(seedsMediator.seedNames, ["Seed2"])
    }

    func testRemoveSeed_Failure() {
        // Given
        let seedName = "Seed1"
        seedsMediator.seedNames = [seedName, "Seed2"]
        keychainAccessAdapterMock.retrieveSeedReturnValue = .failure(.fetchError)

        // When
        let result = seedsMediator.removeSeed(seedName: seedName)

        // Then
        XCTAssertFalse(result)
        XCTAssertEqual(seedsMediator.seedNames, [seedName, "Seed2"])
    }

    func testRemoveAllSeeds_Successful() {
        // Given
        seedsMediator.seedNames = ["Seed1", "Seed2"]
        keychainAccessAdapterMock.retrieveSeedsReturnValue = .success(["Seed1": "seedData1", "Seed2": "seedData2"])
        keychainAccessAdapterMock.removeAllSeedsReturnValue = true

        // When
        let result = seedsMediator.removeAllSeeds()

        // Then
        XCTAssertTrue(result)
        XCTAssertTrue(seedsMediator.seedNames.isEmpty)
    }

    func testRemoveAllSeeds_Failure() {
        // Given
        seedsMediator.seedNames = ["Seed1", "Seed2"]
        keychainAccessAdapterMock.retrieveSeedsReturnValue = .failure(.fetchError)

        // When
        let result = seedsMediator.removeAllSeeds()

        // Then
        XCTAssertFalse(result)
        XCTAssertEqual(seedsMediator.seedNames, ["Seed1", "Seed2"])
    }

    func testCheckSeedPhraseCollision_Exists() {
        // Given
        let seedPhrase = "existingSeedPhrase"
        keychainAccessAdapterMock.checkIfSeedPhraseAlreadyExistsReturnValue = .success(true)

        // When
        let result = seedsMediator.checkSeedPhraseCollision(seedPhrase: seedPhrase)

        // Then
        XCTAssertTrue(result)
    }

    func testCheckSeedPhraseCollision_NotExists() {
        // Given
        let seedPhrase = "newSeedPhrase"
        keychainAccessAdapterMock.checkIfSeedPhraseAlreadyExistsReturnValue = .success(false)

        // When
        let result = seedsMediator.checkSeedPhraseCollision(seedPhrase: seedPhrase)

        // Then
        XCTAssertFalse(result)
    }

    func testCheckSeedPhraseCollision_Failure() {
        // Given
        let seedPhrase = "seedPhrase"
        keychainAccessAdapterMock.checkIfSeedPhraseAlreadyExistsReturnValue = .failure(.checkError)

        // When
        let result = seedsMediator.checkSeedPhraseCollision(seedPhrase: seedPhrase)

        // Then
        XCTAssertFalse(result)
        XCTAssertFalse(authenticationStateMediatorMock.authenticated)
    }

    func testRemoveStalledSeeds() {
        // Given
        seedsMediator.seedNames = ["Seed1", "Seed2"]
        keychainAccessAdapterMock.removeAllSeedsReturnValue = true

        // When
        seedsMediator.removeStalledSeeds()

        // Then
        XCTAssertTrue(seedsMediator.seedNames.isEmpty)
    }
}

// MARK: - Mocks

final class AuthenticatedStateMediatorMock: AuthenticatedStateMediator {
    var authenticatedSetCallsCount = 0
    override var authenticated: Bool {
        didSet {
            authenticatedSetCallsCount += 1
        }
    }
}

final class DatabaseMediatorMock: DatabaseMediating {
    // Properties to track method calls and arguments
    var databaseNameCallsCount = 0
    var isDatabaseAvailableCallsCount = 0
    var recreateDatabaseFileCallsCount = 0
    var wipeDatabaseCallsCount = 0

    // Properties to control return values
    var databaseNameReturnValue = "MockDatabase"
    var isDatabaseAvailableReturnValue = false
    var recreateDatabaseFileReturnValue = false
    var wipeDatabaseReturnValue = false

    // Implementations of protocol methods
    var databaseName: String {
        databaseNameCallsCount += 1
        return databaseNameReturnValue
    }

    func isDatabaseAvailable() -> Bool {
        isDatabaseAvailableCallsCount += 1
        return isDatabaseAvailableReturnValue
    }

    @discardableResult
    func recreateDatabaseFile() -> Bool {
        recreateDatabaseFileCallsCount += 1
        return recreateDatabaseFileReturnValue
    }

    @discardableResult
    func wipeDatabase() -> Bool {
        wipeDatabaseCallsCount += 1
        return wipeDatabaseReturnValue
    }
}

final class KeychainAccessAdapterMock: KeychainAccessAdapting {
    // Properties to track method calls and arguments
    var fetchSeedNamesCallsCount = 0
    var saveSeedCallsCount = 0
    var retrieveSeedCallsCount = 0
    var retrieveSeedsCallsCount = 0
    var removeSeedCallsCount = 0
    var checkIfSeedPhraseAlreadyExistsCallsCount = 0
    var removeAllSeedsCallsCount = 0

    // Properties to store passed arguments
    var saveSeedArguments: [(seedName: String, seedPhrase: Data)] = []
    var retrieveSeedArguments: [String] = []
    var retrieveSeedsArguments: [Set<String>] = []
    var removeSeedArguments: [String] = []
    var checkIfSeedPhraseAlreadyExistsArguments: [Data] = []

    // Properties to control return values
    var fetchSeedNamesReturnValue: Result<FetchSeedsPayload, KeychainError> = .failure(.fetchError)
    var saveSeedReturnValue: Result<Void, KeychainError> = .failure(.saveError(message: ""))
    var retrieveSeedReturnValue: Result<String, KeychainError> = .failure(.fetchError)
    var retrieveSeedsReturnValue: Result<[String: String], KeychainError> = .failure(.fetchError)
    var removeSeedReturnValue: Result<Void, KeychainError> = .failure(.deleteError(message: ""))
    var checkIfSeedPhraseAlreadyExistsReturnValue: Result<Bool, KeychainError> = .failure(.checkError)
    var removeAllSeedsReturnValue: Bool = false

    // Implementations of protocol methods
    func fetchSeedNames() -> Result<FetchSeedsPayload, KeychainError> {
        fetchSeedNamesCallsCount += 1
        return fetchSeedNamesReturnValue
    }

    func saveSeed(with seedName: String, seedPhrase: Data) -> Result<Void, KeychainError> {
        saveSeedCallsCount += 1
        saveSeedArguments.append((seedName, seedPhrase))
        return saveSeedReturnValue
    }

    func retrieveSeed(with seedName: String) -> Result<String, KeychainError> {
        retrieveSeedCallsCount += 1
        retrieveSeedArguments.append(seedName)
        return retrieveSeedReturnValue
    }

    func retrieveSeeds(with seedNames: Set<String>) -> Result<[String: String], KeychainError> {
        retrieveSeedsCallsCount += 1
        retrieveSeedsArguments.append(seedNames)
        return retrieveSeedsReturnValue
    }

    func removeSeed(seedName: String) -> Result<Void, KeychainError> {
        removeSeedCallsCount += 1
        removeSeedArguments.append(seedName)
        return removeSeedReturnValue
    }

    func checkIfSeedPhraseAlreadyExists(seedPhrase: Data) -> Result<Bool, KeychainError> {
        checkIfSeedPhraseAlreadyExistsCallsCount += 1
        checkIfSeedPhraseAlreadyExistsArguments.append(seedPhrase)
        return checkIfSeedPhraseAlreadyExistsReturnValue
    }

    func removeAllSeeds() -> Bool {
        removeAllSeedsCallsCount += 1
        return removeAllSeedsReturnValue
    }
}
