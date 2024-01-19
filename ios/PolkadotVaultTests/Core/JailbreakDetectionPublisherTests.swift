//
//  JailbreakDetectionPublisherTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 17/01/2024.
//

import Combine
import Foundation
@testable import PolkadotVault
import XCTest

final class JailbreakDetectionPublisherTests: XCTestCase {
    private var sut: JailbreakDetectionPublisher!
    private var deviceMock: DeviceProtocolMock!
    private var runtimePropertiesProviderMock: RuntimePropertiesProvidingMock!
    private var fileManagerMock: FileManagingProtocolMock!
    private var urlOpenerMock: URLOpeningMock!
    private var processInfoMock: ProcessInfoProtocolMock!
    private var cancelBag: CancelBag!

    override func setUp() {
        super.setUp()
        cancelBag = CancelBag()
        deviceMock = DeviceProtocolMock()
        runtimePropertiesProviderMock = RuntimePropertiesProvidingMock()
        fileManagerMock = FileManagingProtocolMock()
        urlOpenerMock = URLOpeningMock()
        processInfoMock = ProcessInfoProtocolMock()
        runtimePropertiesProviderMock.runtimeMode = .production
        deviceMock.underlyingIsSimulator = false
        setupMocksForJailbrokenDevice()
        sut = JailbreakDetectionPublisher(
            runtimePropertiesProvider: runtimePropertiesProviderMock,
            device: deviceMock,
            fileManager: fileManagerMock,
            urlOpener: urlOpenerMock,
            processInfo: processInfoMock
        )
    }

    override func tearDown() {
        sut = nil
        deviceMock = nil
        runtimePropertiesProviderMock = nil
        fileManagerMock = nil
        urlOpenerMock = nil
        processInfoMock = nil
        cancelBag = nil
        super.tearDown()
    }

    private func setupMocksForJailbrokenDevice() {
        fileManagerMock.fileExistsAtPathReturnValue = true
        urlOpenerMock.canOpenURLReturnValue = true
        processInfoMock.environment = ["DYLD_INSERT_LIBRARIES": "anyValue"]
    }

    private func setupMocksForNonJailbrokenDevice() {
        fileManagerMock.fileExistsAtPathReturnValue = false
        urlOpenerMock.canOpenURLReturnValue = false
        processInfoMock.environment = [:]
    }

    func testDetectJailbreak_WhenNotProduction_doesNotUpdateIsJailbroken() {
        runtimePropertiesProviderMock.runtimeMode = .debug
        sut = JailbreakDetectionPublisher(
            runtimePropertiesProvider: runtimePropertiesProviderMock,
            device: deviceMock,
            fileManager: fileManagerMock,
            urlOpener: urlOpenerMock,
            processInfo: processInfoMock
        )
        let expectation = XCTestExpectation()

        sut.$isJailbroken
            .sink { isJailbroken in
                XCTAssertFalse(isJailbroken)
                expectation.fulfill()
            }
            .store(in: cancelBag)

        NotificationCenter.default.post(name: UIApplication.didBecomeActiveNotification, object: nil)
        wait(for: [expectation], timeout: 1.0)
    }

    func testDetectJailbreak_WhenDeviceIsJailbroken() {
        setupMocksForJailbrokenDevice()
        let expectation = XCTestExpectation()

        sut.$isJailbroken
            .dropFirst()
            .sink { isJailbroken in
                XCTAssertTrue(isJailbroken)
                expectation.fulfill()
            }
            .store(in: cancelBag)

        NotificationCenter.default.post(name: UIApplication.didBecomeActiveNotification, object: nil)
        wait(for: [expectation], timeout: 1.0)
    }

    func testDetectJailbreak_WhenDeviceIsNotJailbroken() {
        setupMocksForNonJailbrokenDevice()
        let expectation = XCTestExpectation()

        sut.$isJailbroken
            .sink { isJailbroken in
                XCTAssertFalse(isJailbroken)
                expectation.fulfill()
            }
            .store(in: cancelBag)

        NotificationCenter.default.post(name: UIApplication.didBecomeActiveNotification, object: nil)
        wait(for: [expectation], timeout: 1.0)
    }

    func testDetectJailbreak_WhenJailbreakPathsExistAndDeviceIsNotSimulator() {
        deviceMock.isSimulator = false
        fileManagerMock.fileExistsAtPathReturnValue = true
        let expectation = XCTestExpectation()

        sut.$isJailbroken
            .dropFirst()
            .sink { isJailbroken in
                XCTAssertTrue(isJailbroken)
                expectation.fulfill()
            }
            .store(in: cancelBag)

        NotificationCenter.default.post(name: UIApplication.didBecomeActiveNotification, object: nil)
        wait(for: [expectation], timeout: 1.0)
    }

    func testDetectJailbreak_WhenJailbreakToolsPresentAndDeviceIsNotSimulator() {
        deviceMock.isSimulator = false
        urlOpenerMock.canOpenURLReturnValue = true
        let expectation = XCTestExpectation()

        sut.$isJailbroken
            .dropFirst()
            .sink { isJailbroken in
                XCTAssertTrue(isJailbroken)
                expectation.fulfill()
            }
            .store(in: cancelBag)

        NotificationCenter.default.post(name: UIApplication.didBecomeActiveNotification, object: nil)
        wait(for: [expectation], timeout: 1.0)
    }

    func testDetectJailbreak_WhenEnvironmentVariablesIndicateJailbreakAndDeviceIsNotSimulator() {
        deviceMock.isSimulator = false
        processInfoMock.environment = ["DYLD_INSERT_LIBRARIES": "anyValue"]
        let expectation = XCTestExpectation()

        sut.$isJailbroken
            .dropFirst()
            .sink { isJailbroken in
                XCTAssertTrue(isJailbroken)
                expectation.fulfill()
            }
            .store(in: cancelBag)

        NotificationCenter.default.post(name: UIApplication.didBecomeActiveNotification, object: nil)
        wait(for: [expectation], timeout: 1.0)
    }

    func testDetectJailbreak_WhenSystemDoesNotSeemJailbrokenButDeviceIsSimulator() {
        deviceMock.isSimulator = true
        fileManagerMock.fileExistsAtPathReturnValue = false
        urlOpenerMock.canOpenURLReturnValue = false
        processInfoMock.environment = [:]
        let expectation = XCTestExpectation()

        sut.$isJailbroken
            .dropFirst()
            .sink { isJailbroken in
                XCTAssertTrue(isJailbroken)
                expectation.fulfill()
            }
            .store(in: cancelBag)

        NotificationCenter.default.post(name: UIApplication.didBecomeActiveNotification, object: nil)
        wait(for: [expectation], timeout: 1.0)
    }
}
