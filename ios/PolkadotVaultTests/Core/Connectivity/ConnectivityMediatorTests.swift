//
//  ConnectivityMediatorTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 20/11/2023.
//

import Foundation
@testable import PolkadotVault
import XCTest

final class ConnectivityMediatorTests: XCTestCase {
    private var connectivityMonitoringMock: ConnectivityMonitoringMock!
    private var connectivityMediator: ConnectivityMediator!

    override func setUp() {
        super.setUp()
        connectivityMonitoringMock = ConnectivityMonitoringMock()
        connectivityMediator = ConnectivityMediator(
            connectivityMonitor: connectivityMonitoringMock
        )
    }

    override func tearDown() {
        connectivityMonitoringMock = nil
        connectivityMediator = nil
        super.tearDown()
    }

    func testConnectivityMediator_StartMonitoring_ShouldBeCalledOnce() {
        // Given: ConnectivityMediator is initialized

        // When: setUpConnectivityMonitoring is called during initialization

        // Then: startMonitoring should be called once
        XCTAssertEqual(connectivityMonitoringMock.startMonitoringCallsCount, 1)
    }

    func testConnectivityMediator_WhenConnected_ShouldUpdateIsConnectivityOnToTrue() {
        // Given
        XCTAssertFalse(connectivityMediator.isConnectivityOn)

        // When
        connectivityMonitoringMock.simulateConnectivityChange(isConnected: true)

        // Then
        XCTAssertTrue(connectivityMediator.isConnectivityOn)
    }

    func testConnectivityMediator_WhenDisconnected_ShouldUpdateIsConnectivityOnToFalse() {
        // Given
        connectivityMonitoringMock.simulateConnectivityChange(isConnected: true)

        // When
        connectivityMonitoringMock.simulateConnectivityChange(isConnected: false)

        // Then
        XCTAssertFalse(connectivityMediator.isConnectivityOn)
    }
}

// MARK: - Mocks

final class ConnectivityMonitoringMock: ConnectivityMonitoring {
    var startMonitoringCallsCount = 0
    var startMonitoringReceivedUpdate: ((Bool) -> Void)?

    func startMonitoring(_ update: @escaping (Bool) -> Void) {
        startMonitoringCallsCount += 1
        startMonitoringReceivedUpdate = update
    }

    // Helper method to simulate connectivity change
    func simulateConnectivityChange(isConnected: Bool) {
        startMonitoringReceivedUpdate?(isConnected)
    }
}
