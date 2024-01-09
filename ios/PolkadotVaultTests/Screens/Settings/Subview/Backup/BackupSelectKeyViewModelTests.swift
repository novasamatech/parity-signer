//
//  BackupSelectKeyViewModelTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 08/01/2024.
//

import Combine
import Foundation
@testable import PolkadotVault
import XCTest

final class BackupSelectKeyViewModelTests: XCTestCase {
    private var viewModel: BackupSelectKeyView.ViewModel!
    private var seedsMediatorMock: SeedsMediatingMock!

    override func setUp() {
        super.setUp()
        seedsMediatorMock = SeedsMediatingMock()
        viewModel = BackupSelectKeyView.ViewModel(seedsMediator: seedsMediatorMock)
    }

    override func tearDown() {
        viewModel = nil
        seedsMediatorMock = nil
        super.tearDown()
    }

    func testOnSeedNameTap_PresentsBackupModal() {
        // Given
        let seedName = "TestSeed"
        let expectedSeedPhrase = "SeedPhrase"
        seedsMediatorMock.getSeedBackupSeedNameReturnValue = expectedSeedPhrase
        let expectedViewModel = SettingsBackupViewModel(
            keyName: seedName,
            seedPhrase: .init(seedPhrase: expectedSeedPhrase)
        )
        // When
        viewModel.onSeedNameTap(seedName)

        // Then
        XCTAssertTrue(viewModel.isPresentingBackupModal)
        XCTAssertEqual(viewModel.seedPhraseToPresent, expectedViewModel)
    }
}
