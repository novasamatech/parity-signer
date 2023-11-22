//
//  RuntimePropertiesProviderTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 22/11/2023.
//

import Foundation
@testable import PolkadotVault
import XCTest

final class RuntimePropertiesProviderTests: XCTestCase {
    private var appInformationContainer: ApplicationInformationContainerMock.Type!
    private var subject: RuntimePropertiesProvider!

    override func setUp() {
        super.setUp()
        appInformationContainer = ApplicationInformationContainerMock.self
        subject = RuntimePropertiesProvider(
            appInformationContainer: appInformationContainer
        )
    }

    override func tearDown() {
        appInformationContainer.reset()
        subject = nil
        super.tearDown()
    }

    func test_runtimeMode_whenUnknownMode_returnsProduction() {
        // Given
        appInformationContainer.appRuntimeMode = "fdsfd"

        // When
        let result = subject.runtimeMode

        // Then
        XCTAssertEqual(result, .production)
    }

    func test_runtimeMode_whenQAValue_returnsQA() {
        // Given
        appInformationContainer.appRuntimeMode = "qa"

        // When
        let result = subject.runtimeMode

        // Then
        XCTAssertEqual(result, .qa)
    }

    func test_runtimeMode_whenProductionValue_returnsProduction() {
        // Given
        appInformationContainer.appRuntimeMode = "production"

        // When
        let result = subject.runtimeMode

        // Then
        XCTAssertEqual(result, .production)
    }

    func test_runtimeMode_whenDebugValue_returnsDebug() {
        // Given
        appInformationContainer.appRuntimeMode = "debug"

        // When
        let result = subject.runtimeMode

        // Then
        XCTAssertEqual(result, .debug)
    }

    func test_runtimeMode_whenDynamicDerivationsEnabled_returnsTrue() {
        // Given
        appInformationContainer.dynamicDerivationsEnabled = "true"

        // When
        let result = subject.dynamicDerivationsEnabled

        // Then
        XCTAssert(result)
    }

    func test_runtimeMode_whenDynamicDerivationsDisabled_returnsFalse() {
        // Given
        appInformationContainer.dynamicDerivationsEnabled = "false"

        // When
        let result = subject.dynamicDerivationsEnabled

        // Then
        XCTAssertFalse(result)
    }
}

// MARK: - Mocks

final class ApplicationInformationContainerMock: ApplicationInformationContaining {
    static var dynamicDerivationsEnabled: String = ""
    static var appRuntimeMode: String = ""

    static func reset() {
        dynamicDerivationsEnabled = ""
        appRuntimeMode = ""
    }
}
