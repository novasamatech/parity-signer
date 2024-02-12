//
//  AddKeySetUpNetworksStepOneViewModelTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 08/02/2024.
//

import Combine
import Foundation
@testable import PolkadotVault
import XCTest

final class AddKeySetUpNetworksStepOneViewModelTests: XCTestCase {
    private var viewModel: AddKeySetUpNetworksStepOneView.ViewModel!
    private var cancelBag: CancelBag!

    override func setUp() {
        super.setUp()
        viewModel = AddKeySetUpNetworksStepOneView.ViewModel()
        cancelBag = CancelBag()
    }

    override func tearDown() {
        viewModel = nil
        cancelBag.cancel()
        cancelBag = nil
        super.tearDown()
    }

    func testOnNextButtonTap_PresentsStepTwo() {
        // When
        viewModel.onNextButtonTap()

        // Then
        XCTAssertTrue(viewModel.isPresentingStepTwo)
    }

    func testOnScanTap_ShowsQRScanner() {
        // When
        viewModel.onScanTap()

        // Then
        XCTAssertTrue(viewModel.isShowingQRScanner)
    }

    func testOnStepTwoComplete_DismissesView() {
        // Given
        var dismissTriggered = false
        viewModel.dismissViewRequest.sink {
            dismissTriggered = true
        }.store(in: cancelBag)

        // When
        viewModel.onStepTwoComplete()

        // Then
        XCTAssertTrue(dismissTriggered)
    }

    func testOnQRScannerDismiss_DismissesView() {
        // Given
        var dismissTriggered = false
        viewModel.dismissViewRequest.sink {
            dismissTriggered = true
        }.store(in: cancelBag)

        // When
        viewModel.onQRScannerDismiss()

        // Then
        XCTAssertTrue(dismissTriggered)
    }
}
