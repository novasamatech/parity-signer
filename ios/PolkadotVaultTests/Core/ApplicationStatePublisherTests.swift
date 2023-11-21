//
//  ApplicationStatePublisherTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 20/11/2023.
//

import Foundation
@testable import PolkadotVault
import XCTest

final class ApplicationStatePublisherTests: XCTestCase {
    private var notificationCenter: NotificationCenter!
    private var subject: ApplicationStatePublisher!

    override func setUp() {
        super.setUp()
        notificationCenter = NotificationCenter()
        subject = ApplicationStatePublisher(notificationCenter: notificationCenter)
    }

    override func tearDown() {
        subject = nil
        notificationCenter = nil
        super.tearDown()
    }

    func testApplicationState_WhenBecomesInactive_ShouldBeInactive() {
        // Given
        let expectation = expectation(description: "Notification is received")

        // When
        notificationCenter.post(name: UIApplication.willResignActiveNotification, object: nil)
        DispatchQueue.main.asyncAfter(deadline: .now() + 1) {
            expectation.fulfill()
        }

        // Then
        waitForExpectations(timeout: 2)
        XCTAssertEqual(subject.applicationState, .inactive)
    }

    func testApplicationState_WhenBecomesActive_ShouldBeActive() {
        // Given
        let expectation = expectation(description: "Notification is received")
        notificationCenter.post(name: UIApplication.willResignActiveNotification, object: nil)

        // When
        notificationCenter.post(name: UIApplication.didBecomeActiveNotification, object: nil)
        DispatchQueue.main.asyncAfter(deadline: .now() + 1) {
            expectation.fulfill()
        }

        // Then
        waitForExpectations(timeout: 2)
        XCTAssertEqual(subject.applicationState, .active)
    }
}
