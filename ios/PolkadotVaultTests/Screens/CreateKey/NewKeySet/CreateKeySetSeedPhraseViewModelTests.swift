//
//  CreateKeySetSeedPhraseViewModelTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 25/01/2024.
//

import Combine
import Foundation
@testable import PolkadotVault
import SwiftUI
import XCTest

final class CreateKeySetSeedPhraseViewModelTests: XCTestCase {
    private var viewModel: CreateKeySetSeedPhraseView.ViewModel!
    private var serviceMock: CreateKeySetServicingMock!
    private var seedsMediatorMock: SeedsMediatingMock!
    private var isPresented: Bool!
    private var onCompletionExecuted: Bool!
    private var dataModel: MNewSeedBackup!

    override func setUp() {
        super.setUp()
        dataModel = MNewSeedBackup.generate()
        serviceMock = CreateKeySetServicingMock()
        seedsMediatorMock = SeedsMediatingMock()
        isPresented = false
        onCompletionExecuted = false
        viewModel = CreateKeySetSeedPhraseView.ViewModel(
            dataModel: dataModel,
            isPresented: Binding(get: { self.isPresented }, set: { self.isPresented = $0 }),
            service: serviceMock,
            seedsMediator: seedsMediatorMock,
            onCompletion: { _ in self.onCompletionExecuted = true }
        )
    }

    override func tearDown() {
        viewModel = nil
        serviceMock = nil
        seedsMediatorMock = nil
        dataModel = nil
        isPresented = nil
        onCompletionExecuted = nil
        super.tearDown()
    }

    func testInitialSetup() {
        // Then
        XCTAssertFalse(viewModel.isPresentingDetails)
        XCTAssertFalse(viewModel.isPresentingInfo)
        XCTAssertFalse(viewModel.confirmBackup)
    }

    func testOnCreateTap_SetsIsPresentingDetailsToTrue() {
        // When
        viewModel.onCreateTap()

        // Then
        XCTAssertTrue(viewModel.isPresentingDetails)
    }

    func testOnInfoBoxTap_SetsIsPresentingInfoToTrue() {
        // When
        viewModel.onInfoBoxTap()

        // Then
        XCTAssertTrue(viewModel.isPresentingInfo)
    }

    func testCreateDerivedKeys_ReturnsViewModelWithExpectedProperties() {
        // When
        let derivedKeysViewModel = viewModel.createDerivedKeys()

        // Then
        XCTAssertEqual(derivedKeysViewModel.isPresented, isPresented)
        XCTAssertEqual(derivedKeysViewModel.seedName, dataModel.seed)
        XCTAssertEqual(derivedKeysViewModel.seedPhrase, dataModel.seedPhrase)
        XCTAssertEqual(derivedKeysViewModel.mode, .createKeySet)
    }
}
