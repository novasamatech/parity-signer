//
//  ApplicationUpdateRequiredViewModelTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 05/02/2024.
//

import Foundation
@testable import PolkadotVault
import XCTest

final class ApplicationUpdateRequiredViewModelTests: XCTestCase {
    private var viewModel: ApplicationUpdateRequiredView.ViewModel!

    override func setUp() {
        super.setUp()
        viewModel = ApplicationUpdateRequiredView.ViewModel()
    }

    override func tearDown() {
        viewModel = nil
        super.tearDown()
    }

    func testOnBackupTap_SetsIsBackupPresentedToTrue() {
        // When
        viewModel.onBackupTap()

        // Then
        XCTAssertTrue(viewModel.isBackupPresented)
    }
}
