//
//  BananaSplitQRCodeModalViewModelTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 04/03/2024.
//

import Combine
import Foundation
@testable import PolkadotVault
import SwiftUI
import XCTest

final class BananaSplitQRCodeModalViewModelTests: XCTestCase {
    private var viewModel: BananaSplitQRCodeModalView.ViewModel!
    private var mediatorMock: KeychainBananaSplitAccessMediatingMock!
    private var completionAction: BananaSplitQRCodeModalView.OnCompletionAction?
    private var testSeedName: String!
    private var testBananaSplitBackup: BananaSplitBackup!

    override func setUp() {
        super.setUp()
        testSeedName = "testSeed"
        testBananaSplitBackup = BananaSplitBackup(qrCodes: [[10]])
        mediatorMock = KeychainBananaSplitAccessMediatingMock()
        viewModel = BananaSplitQRCodeModalView.ViewModel(
            seedName: testSeedName,
            bananaSplitBackup: testBananaSplitBackup,
            bananaSplitMediator: mediatorMock,
            onCompletion: { [weak self] action in
                self?.completionAction = action
            }
        )
    }

    override func tearDown() {
        testSeedName = nil
        testBananaSplitBackup = nil
        viewModel = nil
        mediatorMock = nil
        completionAction = nil
        super.tearDown()
    }

    func testOnMoreButtonTap_PresentsActionSheet() {
        // When
        viewModel.onMoreButtonTap()

        // Then
        XCTAssertTrue(viewModel.isPresentingActionSheet)
    }

    func testOnCloseTap_CompletesWithClose() {
        // When
        viewModel.onCloseTap()

        // Then
        XCTAssertEqual(completionAction, .close)
    }

    func testCheckForActionsPresentation_PresentsPassphraseModal() {
        // Given
        viewModel.shouldPresentPassphraseModal = true

        // When
        viewModel.checkForActionsPresentation()

        // Then
        XCTAssertTrue(viewModel.isPresentingPassphraseModal)
    }

    func testCheckForActionsPresentation_PresentsDeleteBackupWarningModal() {
        // Given
        viewModel.shouldPresentDeleteBackupWarningModal = true

        // When
        viewModel.checkForActionsPresentation()

        // Then
        XCTAssertTrue(viewModel.isPresentingDeleteBackupWarningModal)
    }

    func testOnDeleteBackupTap_DeletesBackupSuccessfully() {
        // Given
        mediatorMock.removeBananaSplitBackupSeedNameReturnValue = .success(())

        // When
        viewModel.onDeleteBackupTap()

        // Then
        XCTAssertEqual(completionAction, .backupDeleted)
    }

    func testOnDeleteBackupTap_FailsToDeleteBackup() {
        // Given
        let expectedError = KeychainError.checkError
        mediatorMock.removeBananaSplitBackupSeedNameReturnValue = .failure(expectedError)

        // When
        viewModel.onDeleteBackupTap()

        // Then
        XCTAssertTrue(viewModel.isPresentingError)
        XCTAssertEqual(viewModel.presentableError, .alertError(message: expectedError.localizedDescription))
    }
}
