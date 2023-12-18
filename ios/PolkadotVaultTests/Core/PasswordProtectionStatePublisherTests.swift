//
//  PasswordProtectionStatePublisherTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 22/11/2023.
//

import Foundation
import LocalAuthentication
@testable import PolkadotVault
import XCTest

final class PasswordProtectionStatePublisherTests: XCTestCase {
    private var contextMock: LAContextProtocolMock!
    private var notificationCenter: NotificationCenter!
    private var subject: PasswordProtectionStatePublisher!

    override func setUp() {
        super.setUp()
        contextMock = LAContextProtocolMock()
        notificationCenter = NotificationCenter()
        contextMock.canEvaluatePolicyErrorReturnValue = false
        subject = PasswordProtectionStatePublisher(
            context: contextMock, notificationCenter: notificationCenter
        )
    }

    override func tearDown() {
        subject = nil
        contextMock = nil
        notificationCenter = nil
        super.tearDown()
    }

    func testInitialProtectionStatus() {
        // Given
        contextMock.canEvaluatePolicyErrorReturnValue = true

        // When initialized
        subject = PasswordProtectionStatePublisher(context: contextMock, notificationCenter: notificationCenter)

        // Then
        XCTAssertTrue(subject.isProtected)
    }

    func testProtectionStatus_WhenEnteringForeground_AndProtected() {
        // Given
        let expectation = expectation(description: "Notification is received")
        contextMock.canEvaluatePolicyErrorReturnValue = true

        // When
        notificationCenter.post(name: UIApplication.willEnterForegroundNotification, object: nil)
        DispatchQueue.main.asyncAfter(deadline: .now() + 1) {
            expectation.fulfill()
        }

        // Then
        waitForExpectations(timeout: 10)
        XCTAssertTrue(subject.isProtected)
    }

    func testProtectionStatus_WhenEnteringForeground_AndNotProtected() {
        // Given
        let expectation = expectation(description: "Notification is received")
        contextMock.canEvaluatePolicyErrorReturnValue = false

        // When
        notificationCenter.post(name: UIApplication.willEnterForegroundNotification, object: nil)
        DispatchQueue.main.asyncAfter(deadline: .now() + 1) {
            expectation.fulfill()
        }

        // Then
        waitForExpectations(timeout: 10)
        XCTAssertFalse(subject.isProtected)
    }
}
