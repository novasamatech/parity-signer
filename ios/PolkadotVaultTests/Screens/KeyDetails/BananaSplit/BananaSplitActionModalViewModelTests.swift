//
//  BananaSplitActionModalViewModelTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 04/03/2024.
//

import Combine
import Foundation
@testable import PolkadotVault
import SwiftUI
import XCTest

final class BananaSplitActionModalViewModelTests: XCTestCase {
    private var viewModel: BananaSplitActionModal.ViewModel!
    private var isPresented: Bool = false
    private var shouldPresentDeleteBackupWarningModal: Bool!
    private var shouldPresentPassphraseModal: Bool!

    override func setUp() {
        super.setUp()
        isPresented = true
        shouldPresentDeleteBackupWarningModal = false
        shouldPresentPassphraseModal = false
        viewModel = BananaSplitActionModal.ViewModel(
            isPresented: Binding(
                get: { self.isPresented },

                set: { self.isPresented = $0 }
            ),
            shouldPresentDeleteBackupWarningModal: Binding(
                get: { self.shouldPresentDeleteBackupWarningModal },
                set: { self.shouldPresentDeleteBackupWarningModal = $0 }
            ),
            shouldPresentPassphraseModal: Binding(
                get: { self.shouldPresentPassphraseModal },
                set: { self.shouldPresentPassphraseModal = $0 }
            )
        )
    }

    override func tearDown() {
        shouldPresentDeleteBackupWarningModal = nil
        shouldPresentPassphraseModal = nil
        isPresented = false
        viewModel = nil
        super.tearDown()
    }

    func testRemoveBackup_TriggerWarningModalAndDismiss() {
        // Given
        let expectation = expectation(description: "RemoveBackupDismissal")

        // When
        viewModel.removeBackup()

        // Then
        XCTAssertTrue(shouldPresentDeleteBackupWarningModal)

        DispatchQueue.main.asyncAfter(deadline: .now() + 1) {
            XCTAssertFalse(self.isPresented)
            expectation.fulfill()
        }

        waitForExpectations(timeout: 2)
    }

    func testShowPassphrase_TriggerPassphraseModalAndDismiss() {
        // Given
        let expectation = expectation(description: "ShowPassphraseDismissal")

        // When
        viewModel.showPassphrase()

        // Then
        XCTAssertTrue(shouldPresentPassphraseModal)

        DispatchQueue.main.asyncAfter(deadline: .now() + 1) {
            XCTAssertFalse(self.isPresented)
            expectation.fulfill()
        }

        waitForExpectations(timeout: 2)
    }

    func testDismissActionSheet_TriggersAnimationAndDismissal() {
        // Given
        let expectation = expectation(description: "AnimateDismissal")

        // When
        viewModel.dismissActionSheet()

        // Then
        DispatchQueue.main.asyncAfter(deadline: .now() + 1) {
            XCTAssertFalse(self.isPresented)
            expectation.fulfill()
        }

        waitForExpectations(timeout: 2)
    }
}
