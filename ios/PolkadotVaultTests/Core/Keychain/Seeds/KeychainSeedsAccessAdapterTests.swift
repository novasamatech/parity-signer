//
//  KeychainSeedsAccessAdapterTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 22/11/2023.
//

import Foundation
@testable import PolkadotVault
import XCTest

final class KeychainSeedsAccessAdapterTests: XCTestCase {
    private var keychainQueryProviderMock: KeychainSeedsQueryProvidingMock!
    private var accessControlProviderMock: AccessControlProvidingMock!
    private var keychainService: KeychainServiceMock!
    private var keychainAccessAdapter: KeychainSeedsAccessAdapter!

    override func setUp() {
        super.setUp()
        keychainQueryProviderMock = KeychainSeedsQueryProvidingMock()
        accessControlProviderMock = AccessControlProvidingMock()
        keychainService = KeychainServiceMock()
        keychainAccessAdapter = KeychainSeedsAccessAdapter(
            keychainService: keychainService,
            acccessControlProvider: accessControlProviderMock,
            queryProvider: keychainQueryProviderMock
        )
    }

    override func tearDown() {
        keychainQueryProviderMock = nil
        accessControlProviderMock = nil
        keychainAccessAdapter = nil
        super.tearDown()
    }

    func testFetchSeedNames_SuccessfulFetch() {
        // Given
        keychainService.copyMatchingReturnValue = errSecSuccess
        let seedData: [[String: Any]] = [[kSecAttrAccount as String: "Seed1"], [kSecAttrAccount as String: "Seed2"]]
        keychainService.copyMatchingData = seedData as CFTypeRef
        keychainQueryProviderMock.queryForReturnValue = [:] as CFDictionary

        // When
        let result = keychainAccessAdapter.fetchSeedNames()

        // Then
        XCTAssertEqual(keychainService.copyMatchingCallsCount, 1)
        switch result {
        case let .success(payload):
            XCTAssertEqual(payload.seeds, ["Seed1", "Seed2"])
        default:
            XCTFail("Expected success but got \(result)")
        }
    }

    func testFetchSeedNames_EmptyResult() {
        // Given
        keychainService.copyMatchingReturnValue = errSecSuccess
        keychainQueryProviderMock.queryForReturnValue = [:] as CFDictionary
        keychainService.copyMatchingData = [] as CFTypeRef

        // When
        let result = keychainAccessAdapter.fetchSeedNames()

        // Then
        XCTAssertEqual(keychainService.copyMatchingCallsCount, 1)
        switch result {
        case let .success(payload):
            XCTAssertTrue(payload.seeds.isEmpty)
        default:
            XCTFail("Expected success with empty result but got \(result)")
        }
    }

    func testFetchSeedNames_Failure() {
        // Given
        keychainService.copyMatchingReturnValue = errSecAuthFailed
        keychainQueryProviderMock.queryForReturnValue = [:] as CFDictionary

        // When
        let result = keychainAccessAdapter.fetchSeedNames()

        // Then
        XCTAssertEqual(keychainService.copyMatchingCallsCount, 1)
        if case let .failure(error) = result {
            XCTAssertEqual(error, KeychainError.fetchError)
        } else {
            XCTFail("Expected failure but got \(result)")
        }
    }

    func testSaveSeed_Successful() {
        // Given
        keychainService.addReturnValue = errSecSuccess
        accessControlProviderMock.accessControlReturnValue = SecAccessControlCreateWithFlags(
            nil,
            kSecAttrAccessibleWhenUnlocked,
            .privateKeyUsage,
            nil
        )
        keychainQueryProviderMock.queryForReturnValue = [:] as CFDictionary

        // When
        let result = keychainAccessAdapter.saveSeed(with: "SeedName", seedPhrase: Data("seedPhrase".utf8))

        // Then
        XCTAssertEqual(keychainService.addCallsCount, 1)
        if case .failure = result {
            XCTFail("Expected success but got \(result)")
        }
    }

    func testSaveSeed_FailureKeychainIssue() {
        // Given
        keychainService.addReturnValue = errSecAuthFailed
        accessControlProviderMock.accessControlReturnValue = SecAccessControlCreateWithFlags(
            nil,
            kSecAttrAccessibleWhenUnlocked,
            .privateKeyUsage,
            nil
        )
        keychainQueryProviderMock.queryForReturnValue = [:] as CFDictionary

        // When
        let result = keychainAccessAdapter.saveSeed(with: "SeedName", seedPhrase: Data("seedPhrase".utf8))

        // Then
        XCTAssertEqual(keychainService.addCallsCount, 1)
        if case .success = result {
            XCTFail("Expected failure but got \(result)")
        }
    }

    func testSaveSeed_FailureAccessControlNotAvailable() {
        // Given
        accessControlProviderMock.accessControlThrowableError = KeychainError.accessControlNotAvailable

        // When
        let result = keychainAccessAdapter.saveSeed(with: "SeedName", seedPhrase: Data("seedPhrase".utf8))

        // Then
        XCTAssertEqual(accessControlProviderMock.accessControlCallsCount, 1)
        if case .success = result {
            XCTFail("Expected failure due to access control not available but got \(result)")
        }
    }

    func testRetrieveSeed_Successful() {
        // Given
        let seedName = "SeedName"
        let expectedSeed = "seedPhrase"
        keychainService.copyMatchingReturnValue = errSecSuccess
        keychainService.copyMatchingData = expectedSeed.data(using: .utf8) as CFTypeRef?
        keychainQueryProviderMock.queryForReturnValue = [:] as CFDictionary

        // When
        let result = keychainAccessAdapter.retrieveSeed(with: seedName)

        // Then
        XCTAssertEqual(keychainService.copyMatchingCallsCount, 1)
        switch result {
        case let .success(seed):
            XCTAssertEqual(seed, expectedSeed)
        default:
            XCTFail("Expected success but got \(result)")
        }
    }

    func testRetrieveSeed_ItemNotFound() {
        // Given
        let seedName = "SeedName"
        keychainService.copyMatchingReturnValue = errSecItemNotFound
        keychainQueryProviderMock.queryForReturnValue = [:] as CFDictionary

        // When
        let result = keychainAccessAdapter.retrieveSeed(with: seedName)

        // Then
        XCTAssertEqual(keychainService.copyMatchingCallsCount, 1)
        switch result {
        case let .failure(error):
            XCTAssertEqual(error, KeychainError.fetchError)
        default:
            XCTFail("Expected failure due to item not found but got \(result)")
        }
    }

    func testRetrieveSeed_Failure() {
        // Given
        let seedName = "SeedName"
        keychainService.copyMatchingReturnValue = errSecAuthFailed
        keychainQueryProviderMock.queryForReturnValue = [:] as CFDictionary

        // When
        let result = keychainAccessAdapter.retrieveSeed(with: seedName)

        // Then
        XCTAssertEqual(keychainService.copyMatchingCallsCount, 1)
        if case .success = result {
            XCTFail("Expected failure but got \(result)")
        }
    }

    func testRetrieveSeeds_Successful() {
        // Given
        let seedNamesToFetch = Set(["Seed1", "Seed2"])
        let seedData: [[String: Any]] = [
            [kSecAttrAccount as String: "Seed1", kSecValueData as String: "seedPhrase1".data(using: .utf8)!],
            [kSecAttrAccount as String: "Seed2", kSecValueData as String: "seedPhrase2".data(using: .utf8)!]
        ]
        keychainService.copyMatchingReturnValue = errSecSuccess
        keychainService.copyMatchingData = seedData as CFTypeRef?
        keychainQueryProviderMock.queryForReturnValue = [:] as CFDictionary

        // When
        let result = keychainAccessAdapter.retrieveSeeds(with: seedNamesToFetch)

        // Then
        XCTAssertEqual(keychainService.copyMatchingCallsCount, 1)
        switch result {
        case let .success(seeds):
            XCTAssertEqual(seeds.count, 2)
            XCTAssertEqual(seeds["Seed1"], "seedPhrase1")
            XCTAssertEqual(seeds["Seed2"], "seedPhrase2")
        default:
            XCTFail("Expected success but got \(result)")
        }
    }

    func testRetrieveSeeds_EmptyResult() {
        // Given
        let seedNamesToFetch = Set(["Seed1", "Seed2"])
        keychainService.copyMatchingReturnValue = errSecSuccess
        keychainService.copyMatchingData = [] as CFTypeRef?
        keychainQueryProviderMock.queryForReturnValue = [:] as CFDictionary

        // When
        let result = keychainAccessAdapter.retrieveSeeds(with: seedNamesToFetch)

        // Then
        XCTAssertEqual(keychainService.copyMatchingCallsCount, 1)
        switch result {
        case let .success(seeds):
            XCTAssertTrue(seeds.isEmpty)
        default:
            XCTFail("Expected success with empty result but got \(result)")
        }
    }

    func testRetrieveSeeds_Failure() {
        // Given
        let seedNamesToFetch = Set(["Seed1", "Seed2"])
        keychainService.copyMatchingReturnValue = errSecAuthFailed
        keychainQueryProviderMock.queryForReturnValue = [:] as CFDictionary

        // When
        let result = keychainAccessAdapter.retrieveSeeds(with: seedNamesToFetch)

        // Then
        XCTAssertEqual(keychainService.copyMatchingCallsCount, 1)
        if case .success = result {
            XCTFail("Expected failure but got \(result)")
        }
    }

    func testRemoveSeed_Successful() {
        // Given
        let seedName = "SeedName"
        keychainService.deleteReturnValue = errSecSuccess
        keychainQueryProviderMock.queryForReturnValue = [:] as CFDictionary

        // When
        let result = keychainAccessAdapter.removeSeed(seedName: seedName)

        // Then
        XCTAssertEqual(keychainService.deleteCallsCount, 1)
        switch result {
        case .success:
            break // Test passes if we reach here
        default:
            XCTFail("Expected success but got \(result)")
        }
    }

    func testRemoveSeed_Failure() {
        // Given
        let seedName = "SeedName"
        keychainService.deleteReturnValue = errSecAuthFailed
        keychainQueryProviderMock.queryForReturnValue = [:] as CFDictionary

        // When
        let result = keychainAccessAdapter.removeSeed(seedName: seedName)

        // Then
        XCTAssertEqual(keychainService.deleteCallsCount, 1)
        switch result {
        case let .failure(error):
            guard case let .deleteError(message) = error else {
                XCTFail("Expected .deleteError but got \(error)")
                return
            }
            XCTAssertFalse(message.isEmpty)
        default:
            XCTFail("Expected failure but got \(result)")
        }
    }

    func testCheckIfSeedPhraseAlreadyExists_Found() {
        // Given
        let seedPhrase = Data("seedPhrase".utf8)
        keychainService.copyMatchingReturnValue = errSecSuccess
        keychainService.copyMatchingData = [seedPhrase] as CFTypeRef?
        keychainQueryProviderMock.queryForReturnValue = [:] as CFDictionary

        // When
        let result = keychainAccessAdapter.checkIfSeedPhraseAlreadyExists(seedPhrase: seedPhrase)

        // Then
        XCTAssertEqual(keychainService.copyMatchingCallsCount, 1)
        switch result {
        case let .success(exists):
            XCTAssertTrue(exists)
        default:
            XCTFail("Expected success with true but got \(result)")
        }
    }

    func testCheckIfSeedPhraseAlreadyExists_NotFound() {
        // Given
        let seedPhrase = Data("seedPhrase".utf8)
        keychainService.copyMatchingReturnValue = errSecItemNotFound
        keychainQueryProviderMock.queryForReturnValue = [:] as CFDictionary

        // When
        let result = keychainAccessAdapter.checkIfSeedPhraseAlreadyExists(seedPhrase: seedPhrase)

        // Then
        XCTAssertEqual(keychainService.copyMatchingCallsCount, 1)
        switch result {
        case let .success(exists):
            XCTAssertFalse(exists)
        default:
            XCTFail("Expected success with false but got \(result)")
        }
    }

    func testCheckIfSeedPhraseAlreadyExists_Failure() {
        // Given
        let seedPhrase = Data("seedPhrase".utf8)
        keychainService.copyMatchingReturnValue = errSecAuthFailed
        keychainQueryProviderMock.queryForReturnValue = [:] as CFDictionary

        // When
        let result = keychainAccessAdapter.checkIfSeedPhraseAlreadyExists(seedPhrase: seedPhrase)

        // Then
        XCTAssertEqual(keychainService.copyMatchingCallsCount, 1)
        if case .success = result {
            XCTFail("Expected failure but got \(result)")
        }
    }

    func testRemoveAllSeeds_WhenDeletionSuccessful_ReturnsTrue() {
        // Given
        keychainService.deleteReturnValue = errSecSuccess
        keychainQueryProviderMock.queryForReturnValue = [:] as CFDictionary

        // When
        let result = keychainAccessAdapter.removeAllSeeds()

        // Then
        XCTAssertEqual(keychainService.deleteCallsCount, 1)
        XCTAssertTrue(result)
    }

    func testRemoveAllSeeds_WhenDeletionFails_ReturnsFalse() {
        // Given
        keychainService.deleteReturnValue = errSecAuthFailed
        keychainQueryProviderMock.queryForReturnValue = [:] as CFDictionary

        // When
        let result = keychainAccessAdapter.removeAllSeeds()

        // Then
        XCTAssertEqual(keychainService.deleteCallsCount, 1)
        XCTAssertFalse(result)
    }

    func testRemoveAllSeeds_WhenItemNotFound_ReturnsTrue() {
        // Given
        keychainService.deleteReturnValue = errSecItemNotFound
        keychainQueryProviderMock.queryForReturnValue = [:] as CFDictionary

        // When
        let result = keychainAccessAdapter.removeAllSeeds()

        // Then
        XCTAssertEqual(keychainService.deleteCallsCount, 1)
        XCTAssertTrue(result)
    }
}

final class KeychainServiceMock: KeychainServicing {
    // Mock return values
    var addReturnValue: OSStatus = errSecSuccess
    var copyMatchingReturnValue: OSStatus = errSecSuccess
    var deleteReturnValue: OSStatus = errSecSuccess

    // Mock received properties
    var receivedAdd: [(query: CFDictionary, result: UnsafeMutablePointer<CFTypeRef?>?)] = []
    var receivedCopyMatching: [(query: CFDictionary, result: UnsafeMutablePointer<CFTypeRef?>?)] = []
    var receivedDelete: [CFDictionary] = []

    // Number of times each function was called
    var addCallsCount = 0
    var copyMatchingCallsCount = 0
    var deleteCallsCount = 0

    // Mock data to be returned
    var addData: CFTypeRef?
    var copyMatchingData: CFTypeRef?

    func add(_ query: CFDictionary, _ result: UnsafeMutablePointer<CFTypeRef?>?) -> OSStatus {
        receivedAdd.append((query, result))
        addCallsCount += 1
        if let data = addData {
            result?.pointee = data
        }
        return addReturnValue
    }

    func copyMatching(_ query: CFDictionary, _ result: UnsafeMutablePointer<CFTypeRef?>?) -> OSStatus {
        receivedCopyMatching.append((query, result))
        copyMatchingCallsCount += 1
        if let data = copyMatchingData {
            result?.pointee = data
        }
        return copyMatchingReturnValue
    }

    func delete(_ query: CFDictionary) -> OSStatus {
        receivedDelete.append(query)
        deleteCallsCount += 1
        return deleteReturnValue
    }
}
