//
//  UnlockDeviceViewModelTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 02/02/2024.
//

import Foundation
@testable import PolkadotVault
import XCTest

final class UnlockDeviceViewModelTests: XCTestCase {
    private var viewModel: UnlockDeviceView.ViewModel!
    private var seedsMediatorMock: SeedsMediatingMock!

    override func setUp() {
        super.setUp()
        seedsMediatorMock = SeedsMediatingMock()
        viewModel = UnlockDeviceView.ViewModel(seedsMediator: seedsMediatorMock)
    }

    override func tearDown() {
        viewModel = nil
        seedsMediatorMock = nil
        super.tearDown()
    }

    func testOnUnlockTap_CallsRefreshSeeds() {
        // When
        viewModel.onUnlockTap()

        // Then
        XCTAssertEqual(seedsMediatorMock.refreshSeedsCallsCount, 1)
    }
}
