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
    private var contextMock: LAContextMock!
    private var notificationCenter: NotificationCenter!
    private var subject: PasswordProtectionStatePublisher!

    override func setUp() {
        super.setUp()
        contextMock = LAContextMock()
        notificationCenter = NotificationCenter()
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
        contextMock.canEvaluatePolicyReturnValue = true

        // When initialized
        subject = PasswordProtectionStatePublisher(context: contextMock, notificationCenter: notificationCenter)

        // Then
        XCTAssertTrue(subject.isProtected)
    }

    func testProtectionStatus_WhenEnteringForeground_AndProtected() {
        // Given
        let expectation = expectation(description: "Notification is received")
        contextMock.canEvaluatePolicyReturnValue = true

        // When
        notificationCenter.post(name: UIApplication.willEnterForegroundNotification, object: nil)
        DispatchQueue.main.asyncAfter(deadline: .now() + 1) {
            expectation.fulfill()
        }

        // Then
        waitForExpectations(timeout: 2)
        XCTAssertTrue(subject.isProtected)
    }

    func testProtectionStatus_WhenEnteringForeground_AndNotProtected() {
        // Given
        let expectation = expectation(description: "Notification is received")
        contextMock.canEvaluatePolicyReturnValue = false

        // When
        notificationCenter.post(name: UIApplication.willEnterForegroundNotification, object: nil)
        DispatchQueue.main.asyncAfter(deadline: .now() + 1) {
            expectation.fulfill()
        }

        // Then
        waitForExpectations(timeout: 2)
        XCTAssertFalse(subject.isProtected)
    }
}

// MARK: - Mocks

final class LAContextMock: LAContextProtocol {
    var canEvaluatePolicyCallsCount = 0
    var canEvaluatePolicyReceivedPolicy: LAPolicy?
    var canEvaluatePolicyReturnValue: Bool = true

    func canEvaluatePolicy(_ policy: LAPolicy, error _: NSErrorPointer) -> Bool {
        canEvaluatePolicyCallsCount += 1
        canEvaluatePolicyReceivedPolicy = policy
        return canEvaluatePolicyReturnValue
    }
}
