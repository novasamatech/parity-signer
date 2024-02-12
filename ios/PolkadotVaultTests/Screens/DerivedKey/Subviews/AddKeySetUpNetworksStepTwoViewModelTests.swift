//
//  AddKeySetUpNetworksStepTwoViewModelTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 05/02/2024.
//

import Combine
import Foundation
@testable import PolkadotVault
import XCTest

final class AddKeySetUpNetworksStepTwoViewModelTests: XCTestCase {
    private var viewModel: AddKeySetUpNetworksStepTwoView.ViewModel!
    private var onDoneTapExecuted: Bool = false

    override func setUp() {
        super.setUp()
        onDoneTapExecuted = false
        viewModel = AddKeySetUpNetworksStepTwoView.ViewModel(
            onDoneTap: { [weak self] in
                self?.onDoneTapExecuted = true
            }
        )
    }

    override func tearDown() {
        viewModel = nil
        super.tearDown()
    }

    func testOnDoneButtonTap_ExecutesOnDoneTap() {
        // When
        viewModel.onDoneButtonTap()

        // Then
        XCTAssertTrue(onDoneTapExecuted)
    }

    func testOnScanTap_SetsIsShowingQRScannerToTrue() {
        // When
        viewModel.onScanTap()

        // Then
        XCTAssertTrue(viewModel.isShowingQRScanner)
    }

    func testOnQRScannerDismiss_ExecutesOnDoneTap() {
        // When
        viewModel.onQRScannerDismiss()

        // Then
        XCTAssertTrue(onDoneTapExecuted)
    }
}
