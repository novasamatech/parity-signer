//
//  DerivationMethodsInfoViewModelTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 07/02/2024.
//

import Combine
import Foundation
@testable import PolkadotVault
import SwiftUI
import XCTest

final class DerivationMethodsInfoViewModelTests: XCTestCase {
    private var isPresented: Bool!
    private var viewModel: DerivationMethodsInfoView.ViewModel!

    override func setUp() {
        super.setUp()
        isPresented = true
        viewModel = DerivationMethodsInfoView.ViewModel(
            isPresented: Binding(get: { self.isPresented }, set: { self.isPresented = $0 })
        )
    }

    override func tearDown() {
        viewModel = nil
        super.tearDown()
    }

    func testAnimateDismissal_SetsAnimateBackgroundAndHidesView() {
        // Given
        let expectation = XCTestExpectation()
        XCTAssertTrue(isPresented)
        XCTAssertFalse(viewModel.animateBackground)

        // When
        viewModel.animateDismissal()

        DispatchQueue.main.asyncAfter(deadline: .now() + 1) {
            // Then
            XCTAssertTrue(self.viewModel.animateBackground)
            XCTAssertFalse(self.isPresented)
            expectation.fulfill()
        }

        wait(for: [expectation], timeout: 2)
    }
}
