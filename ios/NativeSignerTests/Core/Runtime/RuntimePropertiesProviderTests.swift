//
//  RuntimePropertiesProviderTests.swift
//  NativeSignerTests
//
//  Created by Krzysztof Rodak on 02/08/2022.
//

@testable import NativeSigner
import XCTest

final class RuntimePropertiesProviderTests: XCTestCase {
    private var processInfo: ProcessInfoProtocolMock!
    private var subject: RuntimePropertiesProvider!

    override func setUp() {
        super.setUp()

        processInfo = ProcessInfoProtocolMock()
        subject = RuntimePropertiesProvider(
            processInfo: processInfo
        )
    }

    func test_isInDevelopmentMode_whenRuntimePropertiesContainsFlagWithTrue_returnsTrue() {
        // Given
        processInfo.environment = [RuntimePropertiesProvider.Properties.developmentMode.description:
            RuntimePropertiesProvider.PropertiesValues.true.description]

        // When
        let result = subject.isInDevelopmentMode

        // Then
        XCTAssertTrue(result)
    }

    func test_isInDevelopmentMode_whenRuntimePropertiesContainsFlagWithFalse_returnsFalse() {
        // Given
        processInfo.environment = [RuntimePropertiesProvider.Properties.developmentMode.description:
            RuntimePropertiesProvider.PropertiesValues.false.description]

        // When
        let result = subject.isInDevelopmentMode

        // Then
        XCTAssertFalse(result)
    }

    func test_isInDevelopmentMode_whenRuntimePropertiesContainsNoFlag_returnsFalse() {
        // Given
        processInfo.environment = [:]
        // When
        let result = subject.isInDevelopmentMode

        // Then
        XCTAssertFalse(result)
    }
}

// MARK: - Mocks

final class ProcessInfoProtocolMock: ProcessInfoProtocol {
    var environment: [String: String] = [:]
}
