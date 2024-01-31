//
//  RecoverKeySetNameViewModelTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 29/01/2024.
//

import Foundation
@testable import PolkadotVault
import SwiftUI
import XCTest

final class RecoverKeySetNameViewModelTests: XCTestCase {
    private var viewModel: RecoverKeySetNameView.ViewModel!
    private var seedsMediatorMock: SeedsMediatingMock!
    private var isPresented: Bool = false
    private var onCompletionExecuted: Bool = false

    override func setUp() {
        super.setUp()   
        seedsMediatorMock = SeedsMediatingMock()
        isPresented = false
        onCompletionExecuted = false
        viewModel = RecoverKeySetNameView.ViewModel(
            seedsMediator: seedsMediatorMock,
            isPresented: Binding(get: { self.isPresented }, set: { self.isPresented = $0 }),
            onCompletion: { _ in self.onCompletionExecuted = true }
        )
    }

    override func tearDown() {
        viewModel = nil
        seedsMediatorMock = nil
        super.tearDown()
    }

    func testInitialSetup() {
        // Then
        XCTAssertTrue(viewModel.seedName.isEmpty)
        XCTAssertFalse(viewModel.isPresentingDetails)
    }

    func testOnBackTap() {
        // When
        viewModel.onBackTap()

        // Then
        XCTAssertFalse(isPresented)
    }

    func testOnNextTap() {
        // When
        viewModel.onNextTap()

        // Then
        XCTAssertTrue(viewModel.isPresentingDetails)
    }

    func testIsActionAvailable() {
        // Given
        viewModel.seedName = "ValidSeedName"
        seedsMediatorMock.checkSeedCollisionSeedNameReturnValue = false

        // When
        let result = viewModel.isActionAvailable()

        // Then
        XCTAssertTrue(result)
    }

    func testIsActionNotAvailableDueToEmptySeedName() {
        // Given
        viewModel.seedName = ""

        // When
        let result = viewModel.isActionAvailable()

        // Then
        XCTAssertFalse(result)
    }

    func testIsActionNotAvailableDueToSeedNameCollision() {
        // Given
        viewModel.seedName = "CollidingSeedName"
        seedsMediatorMock.checkSeedCollisionSeedNameReturnValue = true

        // When
        let result = viewModel.isActionAvailable()

        // Then
        XCTAssertFalse(result)
    }

    func testOnSubmitTap_ActionAvailable() {
        // Given
        viewModel.seedName = "ValidSeedName"
        seedsMediatorMock.checkSeedCollisionSeedNameReturnValue = false

        // When
        viewModel.onSubmitTap()

        // Then
        XCTAssertTrue(viewModel.isPresentingDetails)
    }

    func testOnSubmitTap_ActionNotAvailable() {
        // Given
        viewModel.seedName = "CollidingSeedName"
        seedsMediatorMock.checkSeedCollisionSeedNameReturnValue = true

        // When
        viewModel.onSubmitTap()

        // Then
        XCTAssertFalse(viewModel.isPresentingDetails)
    }
}
