//
//  ConnectivityMonitoringAdapterTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 02/08/2022.
//

import Network
@testable import PolkadotVault
import XCTest

final class ConnectivityMonitoringAdapterTests: XCTestCase {
    private var adaptee: PathMonitorProtocolMock!
    private var monitoringQueue: DispatchQueue!
    private var notificationQueue: DispatchQueue!
    private var subject: ConnectivityMonitoringAdapter!

    override func setUp() {
        super.setUp()
        adaptee = PathMonitorProtocolMock()
        notificationQueue = DispatchQueue.main
        monitoringQueue = DispatchQueue.global(qos: .background)
        subject = ConnectivityMonitoringAdapter(
            adaptee: adaptee,
            monitoringQueue: monitoringQueue,
            notificationQueue: notificationQueue
        )
    }

    func test_startMonitoring_setsUpdateHandler() {
        // Then
        XCTAssertNil(adaptee.pathUpdateHandler)

        // When
        subject.startMonitoring { _ in }

        // Then
        XCTAssertNotNil(adaptee.pathUpdateHandler)
    }

    func test_startMonitoring_startsListeningOnMonitoringQueue() {
        // When
        subject.startMonitoring { _ in }

        // Then
        XCTAssertEqual(adaptee.startQueueCallsCount, 1)
        XCTAssertEqual(adaptee.startQueueReceivedQueue, [monitoringQueue])
    }
}

// MARK: - Mocks

final class PathMonitorProtocolMock: PathMonitorProtocol {
    @preconcurrency
    var pathUpdateHandler: ((NWPath) -> Void)?

    var startQueueCallsCount = 0
    var startQueueReceivedQueue: [DispatchQueue] = []

    func start(queue: DispatchQueue) {
        startQueueCallsCount += 1
        startQueueReceivedQueue.append(queue)
    }
}
