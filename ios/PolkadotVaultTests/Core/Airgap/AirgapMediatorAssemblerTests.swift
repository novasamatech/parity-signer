//
//  AirgapMediatorAssemblerTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 04/03/2024.
//

@testable import PolkadotVault
import XCTest

final class AirgapMediatorAssemblerTests: XCTestCase {
    private var runtimePropertiesProvider: RuntimePropertiesProvidingMock!
    private var subject: AirgapMediatorAssembler!

    override func setUp() {
        super.setUp()
        runtimePropertiesProvider = RuntimePropertiesProvidingMock()
        subject = AirgapMediatorAssembler(
            runtimePropertiesProvider: runtimePropertiesProvider
        )
    }

    func test_assemble_whenDebug_returnsStub() {
        // Given
        runtimePropertiesProvider.runtimeMode = .debug

        // When
        let result = subject.assemble()

        // Then
        XCTAssertTrue(result is AirgapMediatingStub)
    }

    func test_assemble_whenProduction_returnsSystemAdapter() {
        // Given
        runtimePropertiesProvider.runtimeMode = .production

        // When
        let result = subject.assemble()

        // Then
        XCTAssertTrue(result is AirgapMediator)
    }

    func test_assemble_whenQA_returnsSystemAdapter() {
        // Given
        runtimePropertiesProvider.runtimeMode = .qa

        // When
        let result = subject.assemble()

        // Then
        XCTAssertTrue(result is AirgapMediator)
    }
}
