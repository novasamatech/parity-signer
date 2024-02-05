//
//  DevicePincodeRequiredViewModelTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 05/02/2024.
//

import Foundation
@testable import PolkadotVault
import XCTest

final class DevicePincodeRequiredViewModelTests: XCTestCase {
    private var viewModel: DevicePincodeRequired.ViewModel!
    private var mockURLOpener: URLOpeningMock!

    override func setUp() {
        super.setUp()
        mockURLOpener = URLOpeningMock()
        viewModel = DevicePincodeRequired.ViewModel(urlOpener: mockURLOpener)
    }

    override func tearDown() {
        viewModel = nil
        mockURLOpener = nil
        super.tearDown()
    }

    func testOnOpenTap_WhenURLCanBeOpened_opensURL() {
        // Given
        mockURLOpener.canOpenURLReturnValue = true
        let settingsUrl = URL(string: UIApplication.openSettingsURLString)!

        // When
        viewModel.onOpenTap()

        // Then
        XCTAssertEqual(mockURLOpener.canOpenURLCallsCount, 1)
        XCTAssertEqual(mockURLOpener.canOpenURLReceivedUrl, [settingsUrl])
        XCTAssertEqual(mockURLOpener.openCallsCount, 1)
        XCTAssertEqual(mockURLOpener.openReceivedUrl, [settingsUrl])
    }

    func testOnOpenTap_WhenURLCannotBeOpened_doesNotOpenURL() {
        // Given
        mockURLOpener.canOpenURLReturnValue = false

        // When
        viewModel.onOpenTap()

        // Then
        XCTAssertEqual(mockURLOpener.canOpenURLCallsCount, 1)
        XCTAssertEqual(mockURLOpener.canOpenURLReceivedUrl, [UIApplication.openSettingsURLString])
        XCTAssertEqual(mockURLOpener.openCallsCount, 0)
    }
}
