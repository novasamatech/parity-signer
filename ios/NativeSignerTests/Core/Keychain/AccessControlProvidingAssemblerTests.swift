//
//  AccessControlProvidingAssemblerTests.swift
//  NativeSignerTests
//
//  Created by Krzysztof Rodak on 29/08/2022.
//

@testable import NativeSigner
import XCTest

final class AccessControlProvidingAssemblerTests: XCTestCase {
    private var runtimePropertiesProvider: RuntimePropertiesProvidingMock!
    private var subject: AccessControlProvidingAssembler!

    override func setUp() {
        super.setUp()
        runtimePropertiesProvider = RuntimePropertiesProvidingMock()
        subject = AccessControlProvidingAssembler(
            runtimePropertiesProvider: runtimePropertiesProvider
        )
    }

    func test_assemble_whenInDevelopmentMode_returnsSimulatorClass() {
        // Given
        runtimePropertiesProvider.isInDevelopmentMode = true

        // When
        let result = subject.assemble()

        // Then
        XCTAssertTrue(result is SimulatorAccessControlProvider)
    }

    func test_assemble_whenNotInDevelopmentMode_returnsStandardProvider() {
        // Given
        runtimePropertiesProvider.isInDevelopmentMode = false

        // When
        let result = subject.assemble()

        // Then
        XCTAssertTrue(result is AccessControlProvider)
    }
}
