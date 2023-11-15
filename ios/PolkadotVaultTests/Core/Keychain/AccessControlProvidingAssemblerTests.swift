//
//  AccessControlProvidingAssemblerTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 29/08/2022.
//

@testable import PolkadotVault
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

    func test_assemble_whenDebug_returnsSimulatorClass() {
        // Given
        runtimePropertiesProvider.runtimeMode = .debug

        // When
        let result = subject.assemble()

        // Then
        XCTAssertTrue(result is SimulatorAccessControlProvider)
    }

    func test_assemble_whenProduction_returnsStandardProvider() {
        // Given
        runtimePropertiesProvider.runtimeMode = .production

        // When
        let result = subject.assemble()

        // Then
        XCTAssertTrue(result is AccessControlProvider)
    }

    func test_assemble_whenQA_returnsStandardProvider() {
        // Given
        runtimePropertiesProvider.runtimeMode = .qa

        // When
        let result = subject.assemble()

        // Then
        XCTAssertTrue(result is AccessControlProvider)
    }
}
