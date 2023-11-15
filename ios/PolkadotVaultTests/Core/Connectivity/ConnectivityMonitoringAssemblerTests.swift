//
//  ConnectivityMonitoringAssemblerTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 02/08/2022.
//

@testable import PolkadotVault
import XCTest

final class ConnectivityMonitoringAssemblerTests: XCTestCase {
    private var runtimePropertiesProvider: RuntimePropertiesProvidingMock!
    private var subject: ConnectivityMonitoringAssembler!

    override func setUp() {
        super.setUp()
        runtimePropertiesProvider = RuntimePropertiesProvidingMock()
        subject = ConnectivityMonitoringAssembler(
            runtimePropertiesProvider: runtimePropertiesProvider
        )
    }

    func test_assemble_whenDebug_returnsStub() {
        // Given
        runtimePropertiesProvider.runtimeMode = .debug

        // When
        let result = subject.assemble()

        // Then
        XCTAssertTrue(result is ConnectivityMonitoringStub)
    }

    func test_assemble_whenProduction_returnsSystemAdapter() {
        // Given
        runtimePropertiesProvider.runtimeMode = .production

        // When
        let result = subject.assemble()

        // Then
        XCTAssertTrue(result is ConnectivityMonitoringAdapter)
    }

    func test_assemble_whenQA_returnsSystemAdapter() {
        // Given
        runtimePropertiesProvider.runtimeMode = .qa

        // When
        let result = subject.assemble()

        // Then
        XCTAssertTrue(result is ConnectivityMonitoringAdapter)
    }
}
