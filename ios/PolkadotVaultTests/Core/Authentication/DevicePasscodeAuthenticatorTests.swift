//
//  DevicePasscodeAuthenticatorTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 03/01/2024.
//

import Combine
import Foundation
@testable import PolkadotVault
import XCTest

final class DevicePasscodeAuthenticatorTests: XCTestCase {
    private var seedsMediatorMock: SeedsMediatingMock!
    private var authenticator: DevicePasscodeAuthenticator!

    override func setUp() {
        super.setUp()
        seedsMediatorMock = SeedsMediatingMock()
        authenticator = DevicePasscodeAuthenticator(seedsMediator: seedsMediatorMock)
    }

    override func tearDown() {
        seedsMediatorMock = nil
        authenticator = nil
        super.tearDown()
    }

    func testAuthenticateUser_WhenNoSeedNames_ReturnsTrue() {
        // Given
        seedsMediatorMock.seedNames = []

        // When
        let result = authenticator.authenticateUser()

        // Then
        XCTAssertTrue(result)
    }

    func testAuthenticateUser_WhenSeedNamePresentAndSeedNotEmpty_ReturnsTrue() {
        // Given
        let seedName = "testSeed"
        let seedData = "seedData"
        seedsMediatorMock.seedNames = [seedName]
        seedsMediatorMock.getSeedSeedNameReturnValue = seedData

        // When
        let result = authenticator.authenticateUser()

        // Then
        XCTAssertTrue(result)
    }

    func testAuthenticateUser_WhenSeedNamePresentAndSeedEmpty_ReturnsFalse() {
        // Given
        let seedName = "testSeed"
        seedsMediatorMock.seedNames = [seedName]
        seedsMediatorMock.getSeedSeedNameReturnValue = ""

        // When
        let result = authenticator.authenticateUser()

        // Then
        XCTAssertFalse(result)
    }
}
