//
//  SetUpNetworksIntroViewModelTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 02/02/2024.
//

import Foundation
@testable import PolkadotVault
import XCTest

final class SetUpNetworksIntroViewModelTests: XCTestCase {
    private var viewModel: SetUpNetworksIntroView.ViewModel!
    private var onNextTapCalled: Bool!
    private var onSkipTapCalled: Bool!

    override func setUp() {
        super.setUp()
        onNextTapCalled = false
        onSkipTapCalled = false
        viewModel = SetUpNetworksIntroView.ViewModel(
            onNextTap: { self.onNextTapCalled = true },
            onSkipTap: { self.onSkipTapCalled = true }
        )
    }

    override func tearDown() {
        viewModel = nil
        onNextTapCalled = nil
        onSkipTapCalled = nil
        super.tearDown()
    }

    func testOnSetUpTap_CallsOnNextTap() {
        // When
        viewModel.onSetUpTap()

        // Then
        XCTAssertTrue(onNextTapCalled)
    }

    func testOnLaterTap_CallsOnSkipTap() {
        // When
        viewModel.onLaterTap()

        // Then
        XCTAssertTrue(onSkipTapCalled)
    }
}
