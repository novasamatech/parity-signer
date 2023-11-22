//
//  CameraPermissionHandlerTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 19/11/2023.
//

import AVKit
import Foundation
@testable import PolkadotVault
import XCTest

final class CameraPermissionHandlerTests: XCTestCase {
    private var captureDeviceMock: AVCaptureDeviceProtocolMock.Type!
    private var dispatchingMock: DispatchingMock!
    private var permissionHandler: CameraPermissionHandler!

    override func setUp() {
        super.setUp()
        captureDeviceMock = AVCaptureDeviceProtocolMock.self
        dispatchingMock = DispatchingMock()
        permissionHandler = CameraPermissionHandler(
            captureDevice: captureDeviceMock, dispatcher: dispatchingMock
        )
    }

    override func tearDown() {
        captureDeviceMock.reset()
        dispatchingMock = nil
        permissionHandler = nil
        super.tearDown()
    }

    func testCheckForPermissions_ChecksAuthorizationStatusForVideo() {
        // Given
        var wasCalled = false

        // When
        permissionHandler.checkForPermissions { _ in
            wasCalled = true
        }

        // Then
        XCTAssertTrue(wasCalled)
        XCTAssertEqual(captureDeviceMock.authorizationStatusCallsCount, 1)
        XCTAssertEqual(captureDeviceMock.authorizationStatusReceivedMediaType, [.video])
    }

    func testCheckForPermissions_WhenAuthorized_ShouldGrantPermission() {
        // Given
        captureDeviceMock.authorizationStatusReturnValue = .authorized
        var wasCalled = false

        // When
        permissionHandler.checkForPermissions { granted in
            XCTAssertTrue(granted)
            wasCalled = true
        }

        // Then
        XCTAssertTrue(wasCalled)
    }

    func testCheckForPermissions_WhenNotDetermined_ShouldCallRequestAccessForVideo() {
        // Given
        captureDeviceMock.authorizationStatusReturnValue = .notDetermined
        var wasCalled = false

        // When
        permissionHandler.checkForPermissions { _ in
            wasCalled = true
        }

        // Then
        XCTAssertTrue(wasCalled)
        XCTAssertEqual(captureDeviceMock.requestAccessCallsCount, 1)
        XCTAssertEqual(captureDeviceMock.requestAccessReceivedMediaType, [.video])
    }

    func testCheckForPermissions_WhenNotDeterminedAndAccessGranted_ShouldGrantPermissionAndCallAsync() {
        // Given
        captureDeviceMock.authorizationStatusReturnValue = .notDetermined
        captureDeviceMock.requestAccessGrantedReturnValue = true
        var wasCalled = false

        // When
        permissionHandler.checkForPermissions { granted in
            XCTAssertTrue(granted)
            wasCalled = true
        }

        // Then
        XCTAssertTrue(wasCalled)
        XCTAssertEqual(dispatchingMock.asyncCallsCount, 1)
    }

    func testCheckForPermissions_WhenDenied_ShouldNotGrantPermission() {
        // Given
        captureDeviceMock.authorizationStatusReturnValue = .denied
        var wasCalled = false

        // When
        permissionHandler.checkForPermissions { granted in
            XCTAssertFalse(granted)
            wasCalled = true
        }

        // Then
        XCTAssertTrue(wasCalled)
    }

    func testCheckForPermissions_WhenRestricted_ShouldNotGrantPermission() {
        // Given
        captureDeviceMock.authorizationStatusReturnValue = .restricted
        var wasCalled = false

        // When
        permissionHandler.checkForPermissions { granted in
            XCTAssertFalse(granted)
            wasCalled = true
        }

        // Then
        XCTAssertTrue(wasCalled)
    }
}

final class DispatchingMock: Dispatching {
    var shouldPerformAsyncWork = true
    var asyncAfterReceivedFlags: [DispatchWorkItemFlags] = []
    var syncCallsCount = 0
    var asyncCallsCount = 0
    var asyncAfterCallsCount = 0

    func async(execute work: @escaping @convention(block) () -> Void) {
        asyncCallsCount += 1
        guard shouldPerformAsyncWork else { return }
        work()
    }

    func asyncAfter(deadline _: DispatchTime, execute work: @escaping @convention(block) () -> Void) {
        asyncAfterCallsCount += 1
        guard shouldPerformAsyncWork else { return }
        work()
    }

    func asyncAfter(deadline _: DispatchTime, flags: DispatchWorkItemFlags, execute work: @escaping () -> Void) {
        asyncAfterCallsCount += 1
        asyncAfterReceivedFlags.append(flags)
        guard shouldPerformAsyncWork else { return }
        work()
    }

    func sync<T>(flags _: DispatchWorkItemFlags, execute work: () throws -> T) rethrows -> T {
        syncCallsCount += 1
        return try work()
    }

    func sync<T>(execute work: () throws -> T) rethrows -> T {
        syncCallsCount += 1

        return try work()
    }
}

final class AVCaptureDeviceProtocolMock: AVCaptureDeviceProtocol {
    static var authorizationStatusCallsCount = 0
    static var authorizationStatusReceivedMediaType: [AVMediaType] = []
    static var authorizationStatusReturnValue: AVAuthorizationStatus = .notDetermined

    static var requestAccessCallsCount = 0
    static var requestAccessReceivedMediaType: [AVMediaType] = []
    static var requestAccessGrantedReturnValue: Bool = false

    static func authorizationStatus(for mediaType: AVMediaType) -> AVAuthorizationStatus {
        authorizationStatusCallsCount += 1
        authorizationStatusReceivedMediaType.append(mediaType)
        return authorizationStatusReturnValue
    }

    static func requestAccess(for mediaType: AVMediaType, completionHandler: @escaping (Bool) -> Void) {
        requestAccessCallsCount += 1
        requestAccessReceivedMediaType.append(mediaType)
        completionHandler(requestAccessGrantedReturnValue)
    }

    static func reset() {
        authorizationStatusCallsCount = 0
        authorizationStatusReceivedMediaType = []
        authorizationStatusReturnValue = .notDetermined
        requestAccessCallsCount = 0
        requestAccessReceivedMediaType = []
        requestAccessGrantedReturnValue = false
    }
}
