//
//  CreateDerivedKeyConfirmationViewModelTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 07/02/2024.
//

import Combine
import Foundation
@testable import PolkadotVault
import XCTest

final class CreateDerivedKeyConfirmationViewModelTests: XCTestCase {
    private var viewModel: CreateDerivedKeyConfirmationView.ViewModel!
    private var onCompletionExecuted: Bool = false
    private let derivationPath = "path"

    override func setUp() {
        super.setUp()
        onCompletionExecuted = false
        viewModel = CreateDerivedKeyConfirmationView.ViewModel(
            derivationPath: derivationPath,
            onCompletion: { [weak self] in self?.onCompletionExecuted = true }
        )
    }

    override func tearDown() {
        viewModel = nil
        super.tearDown()
    }

    func testOnDoneTap_AnimationAndCompletion() {
        // Given
        let expectation = XCTestExpectation()
        viewModel.animateBackground = false
        viewModel.isCheckboxSelected = true

        // When
        viewModel.onDoneTap()

        DispatchQueue.main.asyncAfter(deadline: .now() + 1) {
            // Then
            XCTAssertTrue(self.viewModel.animateBackground)
            XCTAssertTrue(self.onCompletionExecuted)
            expectation.fulfill()
        }

        wait(for: [expectation], timeout: 2)
    }

    func testToggleCheckbox_TogglesStateAndActionAvailability() {
        // Given
        viewModel.isCheckboxSelected = false
        viewModel.isActionDisabled = true

        // When
        viewModel.toggleCheckbox()

        // Then
        XCTAssertTrue(viewModel.isCheckboxSelected)
        XCTAssertFalse(viewModel.isActionDisabled)
    }
}
