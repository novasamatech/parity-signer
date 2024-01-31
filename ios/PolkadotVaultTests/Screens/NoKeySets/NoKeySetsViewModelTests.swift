//
//  NoKeySetsViewModelTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 31/01/2024.
//

import Combine
import Foundation
@testable import PolkadotVault
import XCTest

final class NoKeySetsViewModelTests: XCTestCase {
    private var viewModel: NoKeySetsView.ViewModel!
    private var onCompletionActionExecuted: CreateKeysForNetworksView.OnCompletionAction?

    override func setUp() {
        super.setUp()
        viewModel = NoKeySetsView.ViewModel { [weak self] action in
            self?.onCompletionActionExecuted = action
        }
    }

    override func tearDown() {
        viewModel = nil
        onCompletionActionExecuted = nil
        super.tearDown()
    }

    func testInit_InitialState() {
        // Then
        XCTAssertFalse(viewModel.isPresentingAddKeySet)
        XCTAssertFalse(viewModel.isPresentingRecoverKeySet)
    }

    func testOnAddTap_SetsIsPresentingAddKeySetToTrue() {
        // When
        viewModel.onAddTap()

        // Then
        XCTAssertTrue(viewModel.isPresentingAddKeySet)
    }

    func testOnRecoverTap_SetsIsPresentingRecoverKeySetToTrue() {
        // When
        viewModel.onRecoverTap()

        // Then
        XCTAssertTrue(viewModel.isPresentingRecoverKeySet)
    }

    func testOnKeySetAddCompletion_ExecutesOnCompletion() {
        // Given
        let completionAction = CreateKeysForNetworksView.OnCompletionAction.createKeySet(seedName: "seedName")

        // When
        viewModel.onKeySetAddCompletion(completionAction)

        // Then
        XCTAssertEqual(onCompletionActionExecuted, completionAction)
    }
}
