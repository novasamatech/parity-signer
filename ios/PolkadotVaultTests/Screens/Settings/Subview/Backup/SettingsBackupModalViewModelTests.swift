//
//  SettingsBackupModalViewModelTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 22/01/2024.
//

import Combine
import Foundation
@testable import PolkadotVault
import SwiftUI
import XCTest

final class SettingsBackupModalViewModelTests: XCTestCase {
    private var viewModel: SettingsBackupModal.ViewModel!
    private var isPresented: Bool!

    override func setUp() {
        super.setUp()
        isPresented = false
        viewModel = SettingsBackupModal.ViewModel(
            isPresented: Binding(get: { self.isPresented }, set: { self.isPresented = $0 }),
            viewModel: SettingsBackupViewModel(
                keyName: "Test Key",
                seedPhrase: SeedPhraseViewModel(
                    seedPhrase: "Test Phrase"
                )
            )
        )
    }

    override func tearDown() {
        viewModel = nil
        super.tearDown()
    }

    func testInit_SetsAnimateBackgroundToFalse() {
        // Then
        XCTAssertFalse(viewModel.animateBackground)
    }

    func testOnAppear_SetsSnackbar() {
        // When
        viewModel.onAppear()

        // Then
        XCTAssertEqual(viewModel.snackbar.viewModel.title, Localizable.Settings.BackupModal.Label.snackbar.string)
        XCTAssertTrue(viewModel.snackbar.viewModel.tapToDismiss == false)
        XCTAssertTrue(viewModel.snackbar.isSnackbarPresented)
    }

    func testDismissModal_TogglesAnimateBackground() {
        // Given
        viewModel.animateBackground = false

        // When
        viewModel.dismissModal()

        // Then
        XCTAssertTrue(viewModel.animateBackground)
    }

    func testDismissModal_HidesModal() {
        // Given
        let expectation = XCTestExpectation()
        viewModel.animateBackground = false
        viewModel.dismissModal()

        DispatchQueue.main.asyncAfter(deadline: .now() + 1) {
            // Then
            XCTAssertFalse(self.isPresented)
            expectation.fulfill()
        }

        // When
        wait(for: [expectation], timeout: 2)
    }
}
