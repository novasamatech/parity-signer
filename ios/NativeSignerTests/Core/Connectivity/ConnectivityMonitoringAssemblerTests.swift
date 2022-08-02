//
//  ConnectivityMonitoringAssemblerTests.swift
//  NativeSignerTests
//
//  Created by Krzysztof Rodak on 02/08/2022.
//

@testable import NativeSigner
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

    func test_assemble_whenInDevelopmentMode_returnsStub() {
        // Given
        runtimePropertiesProvider.isInDevelopmentMode = true

        // When
        let result = subject.assemble()

        // Then
        XCTAssertTrue(result is ConnectivityMonitoringStub)
    }

    func test_assemble_whenNotInDevelopmentMode_returnsSystemAdapter() {
        // Given
        runtimePropertiesProvider.isInDevelopmentMode = false

        // When
        let result = subject.assemble()

        // Then
        XCTAssertTrue(result is ConnectivityMonitoringAdapter)
    }
}

// MARK: - Mocks

final class RuntimePropertiesProvidingMock: RuntimePropertiesProviding {
    var isInDevelopmentMode: Bool = false
}
