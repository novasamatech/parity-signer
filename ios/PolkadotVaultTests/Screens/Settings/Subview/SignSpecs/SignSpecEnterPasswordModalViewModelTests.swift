//
//  SignSpecEnterPasswordModalViewModelTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 19/01/2024.
//

import Combine
import Foundation
@testable import PolkadotVault
import SwiftUI
import XCTest

final class SignSpecEnterPasswordModalViewModelTests: XCTestCase {
    private var viewModel: SignSpecEnterPasswordModal.ViewModel!
    private var cancelBag: CancelBag!
    private var isPresented: Bool = false
    private var onDoneTapActionExecuted: Bool = false

    override func setUp() {
        super.setUp()
        cancelBag = CancelBag()
        isPresented = false
        onDoneTapActionExecuted = false
        viewModel = SignSpecEnterPasswordModal.ViewModel(
            isPresented: Binding(get: { self.isPresented }, set: { self.isPresented = $0 }),
            selectedKeyRecord: MRawKey.generate(),
            onDoneTapAction: { _ in self.onDoneTapActionExecuted = true }
        )
    }

    override func tearDown() {
        viewModel = nil
        cancelBag.cancel()
        cancelBag = nil
        super.tearDown()
    }

    func testInit_SetsIsActionDisabledToTrue() {
        // Then
        XCTAssertTrue(viewModel.isActionDisabled)
    }

    func testPasswordChange_UpdatesIsActionDisabled() {
        // When
        viewModel.password = "new password"

        // Then
        XCTAssertFalse(viewModel.isActionDisabled)
    }

    func testPasswordChange_ResetsIsValid() {
        // When
        viewModel.isValid = false
        viewModel.password = "new password"

        // Then
        XCTAssertTrue(viewModel.isValid)
    }

    func testOnCancelTap_SetsIsPresentedToFalse() {
        // When
        viewModel.onCancelTap()

        // Then
        XCTAssertFalse(isPresented)
    }

    func testOnDoneTap_ExecutesOnDoneTapAction() {
        // When
        viewModel.onDoneTap()

        // Then
        XCTAssertTrue(onDoneTapActionExecuted)
    }
}
