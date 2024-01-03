//
//  DatabaseMediatorTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 03/08/2022.
//

@testable import PolkadotVault
import XCTest

final class DatabaseMediatorTests: XCTestCase {
    private var bundle: BundleProtocolMock!
    private var fileManager: FileManagingProtocolMock!
    private var subject: DatabaseMediator!

    override func setUp() {
        super.setUp()
        bundle = BundleProtocolMock()
        bundle.urlForResourceWithExtensionReturnValue = .generate()
        fileManager = FileManagingProtocolMock()
        fileManager.fileExistsAtPathReturnValue = false
        fileManager.urlForInAppropriateForCreateReturnValue = try? FileManager.default.url(
            for: .documentDirectory,
            in: .userDomainMask,
            appropriateFor: nil,
            create: false
        )

        subject = DatabaseMediator(
            bundle: bundle,
            fileManager: fileManager
        )
    }

    func test_databseName_returnsExpectedValue() {
        // Given
        let documentsURL = try? FileManager.default.url(
            for: .documentDirectory,
            in: .userDomainMask,
            appropriateFor: nil,
            create: false
        )
        let expectedValue = documentsURL?.appendingPathComponent("Database").path ?? ""

        // When
        let result = subject.databaseName

        // Then
        XCTAssertEqual(result, expectedValue)
    }

    func test_isDatabaseAvailable_checksForExistingFileAtExpectedPath() {
        // Given
        let documentsURL = try? FileManager.default.url(
            for: .documentDirectory,
            in: .userDomainMask,
            appropriateFor: nil,
            create: false
        )
        let expectedValue = documentsURL?.appendingPathComponent("Database").path ?? ""

        // When
        _ = subject.isDatabaseAvailable()

        // Then
        XCTAssertEqual(fileManager.fileExistsAtPathCallsCount, 1)
        XCTAssertEqual(fileManager.fileExistsAtPathReceivedPath, [expectedValue])
    }

    func test_isDatabaseAvailable_returnsInformationOnFileExistenceFromFileManager() {
        // Given
        fileManager.fileExistsAtPathReturnValue = true

        // When
        let result = subject.isDatabaseAvailable()

        // Then
        XCTAssertEqual(fileManager.fileExistsAtPathReturnValue, result)
    }

    func test_wipeDatabase_accessesExpectedFile() {
        // Given
        fileManager.fileExistsAtPathReturnValue = true

        // When
        subject.wipeDatabase()

        // Then
        XCTAssertEqual(fileManager.urlForInAppropriateForCreateCallsCount, 1)
        XCTAssertEqual(fileManager.urlForInAppropriateForCreateReceivedDirectory, [.documentDirectory])
        XCTAssertEqual(fileManager.urlForInAppropriateForCreateReceivedDomain, [.userDomainMask])
        XCTAssertEqual(fileManager.urlForInAppropriateForCreateReceivedShouldCreate, [false])
    }

    func test_wipeDatabase_removesFileAtExpectedDestination() {
        // Given
        let expectedPathUrl = fileManager.urlForInAppropriateForCreateReturnValue.appendingPathComponent("Database")

        // When
        subject.wipeDatabase()

        // Then
        XCTAssertEqual(fileManager.removeItemAtPathCallsCount, 1)
        XCTAssertEqual(fileManager.removeItemAtPathReceivedPath, [expectedPathUrl.path])
    }

    func test_wipeDatabase_whenRemoveItemThrowsError_returnsFalse() {
        // Given
        fileManager.urlForInAppropriateForCreateThrowableError = nil
        fileManager.removeItemAtPathThrowableError = ErrorMock.unknown

        // When
        let result = subject.wipeDatabase()

        // Then
        XCTAssertFalse(result)
    }

    func test_wipeDatabase_whenNoThrownErrors_returnsTrue() {
        // Given
        fileManager.urlForInAppropriateForCreateThrowableError = nil
        fileManager.removeItemAtThrowableError = nil

        // When
        let result = subject.wipeDatabase()

        // Then
        XCTAssertTrue(result)
    }

    func test_recreateDatabaseFile_whenNoResourceAvailable_returnsFalse() {
        // Given
        bundle.urlForResourceWithExtensionReturnValue = nil

        // When
        let result = subject.recreateDatabaseFile()

        // Then
        XCTAssertFalse(result)
    }

    func test_recreateDatabaseFile_whenResourceAvailable_returnsFalse() {
        // Given
        bundle.urlForResourceWithExtensionReturnValue = nil

        // When
        let result = subject.recreateDatabaseFile()

        // Then
        XCTAssertFalse(result)
    }

    func test_recreateDatabaseFile_whenResourceAvailable_accessesUrlWithExpectedParameters() {
        // Given
        bundle.urlForResourceWithExtensionReturnValue = .generate()

        // When
        _ = subject.recreateDatabaseFile()

        // Then
        XCTAssertEqual(fileManager.urlForInAppropriateForCreateCallsCount, 2)
        XCTAssertEqual(
            fileManager.urlForInAppropriateForCreateReceivedDirectory,
            [.documentDirectory, .documentDirectory]
        )
        XCTAssertEqual(fileManager.urlForInAppropriateForCreateReceivedDomain, [.userDomainMask, .userDomainMask])
        XCTAssertEqual(fileManager.urlForInAppropriateForCreateReceivedShouldCreate, [true, false])
    }

    func test_recreateDatabaseFile_whenFileExists_removesFileAtExpectedDestination() {
        // Given
        let expectedPathUrl = fileManager.urlForInAppropriateForCreateReturnValue.appendingPathComponent("Database")
        fileManager.fileExistsAtPathReturnValue = true

        // When
        _ = subject.recreateDatabaseFile()

        // Then
        XCTAssertEqual(fileManager.removeItemAtCallsCount, 1)
        XCTAssertEqual(fileManager.removeItemAtReceivedURL, [expectedPathUrl])
    }

    func test_recreateDatabaseFile_whenFileDoesNotExist_doesNotAttemptToRemoveFile() {
        // Given
        fileManager.fileExistsAtPathReturnValue = false

        // When
        _ = subject.recreateDatabaseFile()

        // Then
        XCTAssertEqual(fileManager.removeItemAtCallsCount, 0)
    }

    func test_recreateDatabaseFile_whenDestinationAccessThrowsError_returnsFalse() {
        // Given
        fileManager.urlForInAppropriateForCreateThrowableError = ErrorMock.unknown
        fileManager.removeItemAtThrowableError = nil

        // When
        let result = subject.recreateDatabaseFile()

        // Then
        XCTAssertFalse(result)
    }

    func test_recreateDatabaseFile_whenFileExists_whenRemoveItemThrowsError_returnsFalse() {
        // Given
        fileManager.fileExistsAtPathReturnValue = true
        fileManager.urlForInAppropriateForCreateThrowableError = nil
        fileManager.removeItemAtThrowableError = ErrorMock.unknown

        // When
        let result = subject.recreateDatabaseFile()

        // Then
        XCTAssertFalse(result)
    }

    func test_recreateDatabaseFile_whenCopyItemThrowsError_returnsFalse() {
        // Given
        fileManager.urlForInAppropriateForCreateThrowableError = nil
        fileManager.copyItemAtToThrowableError = ErrorMock.unknown

        // When
        let result = subject.recreateDatabaseFile()

        // Then
        XCTAssertFalse(result)
    }

    func test_recreateDatabaseFile_whenNoThrownErrors_returnsTrue() {
        // Given
        fileManager.urlForInAppropriateForCreateThrowableError = nil
        fileManager.removeItemAtThrowableError = nil
        fileManager.copyItemAtToThrowableError = nil

        // When
        let result = subject.recreateDatabaseFile()

        // Then
        XCTAssertTrue(result)
    }
}
